use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudflaredStatus {
    pub installed: bool,
    pub path: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveTunnelInfo {
    pub id: String,
    pub service_id: Option<String>,
    pub port: u16,
    pub url: Option<String>,
    pub status: String,
    pub pid: Option<u32>,
    pub started_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelReadyPayload {
    pub id: String,
    pub service_id: Option<String>,
    pub port: u16,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelLogPayload {
    pub id: String,
    pub line: String,
}

pub struct ActiveTunnelProcess {
    pub info: ActiveTunnelInfo,
    pub child: Option<Child>,
}

#[derive(Clone)]
pub struct TunnelManager {
    tunnels: Arc<Mutex<HashMap<String, ActiveTunnelProcess>>>,
}

impl TunnelManager {
    pub fn new() -> Self {
        Self {
            tunnels: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn list(&self) -> Vec<ActiveTunnelInfo> {
        let lock = self.tunnels.lock().await;
        lock.values().map(|t| t.info.clone()).collect()
    }

    pub async fn stop(&self, id: &str) -> Result<bool, String> {
        let mut lock = self.tunnels.lock().await;
        if let Some(mut tunnel) = lock.remove(id) {
            if let Some(mut child) = tunnel.child.take() {
                let _ = child.kill().await;
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn stop_all(&self) {
        let mut lock = self.tunnels.lock().await;
        for (_, mut tunnel) in lock.drain() {
            if let Some(mut child) = tunnel.child.take() {
                let _ = child.kill().await;
            }
        }
    }
}

pub fn find_cloudflared_binary() -> Option<PathBuf> {
    // 1. Check ~/.config/tether/bin/cloudflared
    if let Some(home) = dirs::home_dir() {
        let tether_bin = home.join(".config").join("tether").join("bin").join("cloudflared");
        if tether_bin.is_file() {
            return Some(tether_bin);
        }
        let local_bin = home.join(".local").join("bin").join("cloudflared");
        if local_bin.is_file() {
            return Some(local_bin);
        }
    }

    // 2. Check standard system locations
    let candidates = ["/usr/local/bin/cloudflared", "/usr/bin/cloudflared", "cloudflared"];
    for candidate in &candidates {
        if let Ok(output) = std::process::Command::new(candidate).arg("--version").output() {
            if output.status.success() {
                return Some(PathBuf::from(candidate));
            }
        }
    }

    None
}

#[tauri::command]
pub fn check_cloudflared() -> CloudflaredStatus {
    match find_cloudflared_binary() {
        Some(path) => {
            let output = std::process::Command::new(&path)
                .arg("--version")
                .output();
            
            let version = match output {
                Ok(out) if out.status.success() => {
                    String::from_utf8_lossy(&out.stdout)
                        .lines()
                        .next()
                        .map(|s| s.to_string())
                }
                _ => None,
            };

            CloudflaredStatus {
                installed: true,
                path: Some(path.to_string_lossy().to_string()),
                version,
            }
        }
        None => CloudflaredStatus {
            installed: false,
            path: None,
            version: None,
        },
    }
}

#[tauri::command]
pub async fn start_tunnel(
    app: AppHandle,
    state: tauri::State<'_, TunnelManager>,
    port: u16,
    service_id: Option<String>,
) -> Result<ActiveTunnelInfo, String> {
    let binary_path = find_cloudflared_binary()
        .ok_or_else(|| "cloudflared binary not found. Please install cloudflared or place it in ~/.config/tether/bin/".to_string())?;

    let tunnel_id = format!("tun-{}", uuid_simple());
    let url_regex = Regex::new(r"https://[a-zA-Z0-9-]+\.trycloudflare\.com").unwrap();

    let mut cmd = Command::new(&binary_path);
    cmd.args(["tunnel", "--url", &format!("http://localhost:{}", port), "--no-autoupdate"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd
        .spawn()
        .map_err(|e| format!("Failed to spawn cloudflared process: {}", e))?;

    let pid = child.id();
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let started_at = "Just now".to_string();

    let info = ActiveTunnelInfo {
        id: tunnel_id.clone(),
        service_id: service_id.clone(),
        port,
        url: None,
        status: "starting".into(),
        pid,
        started_at,
    };

    let active = ActiveTunnelProcess {
        info: info.clone(),
        child: Some(child),
    };

    // Store in active tunnel manager
    {
        let mut lock = state.tunnels.lock().await;
        lock.insert(tunnel_id.clone(), active);
    }

    // Spawn background task to monitor stderr (where trycloudflare URL is printed)
    let manager_clone = state.inner().clone();
    let tid = tunnel_id.clone();
    let sid = service_id.clone();
    let app_handle = app.clone();

    tokio::spawn(async move {
        if let Some(stderr) = stderr {
            let mut reader = BufReader::new(stderr).lines();
            let mut url_found = false;

            while let Ok(Some(line)) = reader.next_line().await {
                // Emit log event
                let _ = app_handle.emit(
                    "tunnel-log",
                    TunnelLogPayload {
                        id: tid.clone(),
                        line: line.clone(),
                    },
                );

                if !url_found {
                    if let Some(mat) = url_regex.find(&line) {
                        let tunnel_url = mat.as_str().to_string();
                        url_found = true;

                        // Update in manager
                        {
                            let mut lock = manager_clone.tunnels.lock().await;
                            if let Some(t) = lock.get_mut(&tid) {
                                t.info.url = Some(tunnel_url.clone());
                                t.info.status = "running".into();
                            }
                        }

                        // Emit tunnel-ready event to Svelte UI
                        let _ = app_handle.emit(
                            "tunnel-ready",
                            TunnelReadyPayload {
                                id: tid.clone(),
                                service_id: sid.clone(),
                                port,
                                url: tunnel_url,
                            },
                        );
                    }
                }
            }
        }

        // When stderr finishes, mark tunnel stopped if still registered
        let mut lock = manager_clone.tunnels.lock().await;
        if let Some(t) = lock.get_mut(&tid) {
            t.info.status = "stopped".into();
        }
        let _ = app_handle.emit("tunnel-closed", tid.clone());
    });

    // Also monitor stdout for auxiliary logs
    if let Some(stdout) = stdout {
        let app_handle2 = app.clone();
        let tid2 = tunnel_id.clone();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                let _ = app_handle2.emit(
                    "tunnel-log",
                    TunnelLogPayload {
                        id: tid2.clone(),
                        line,
                    },
                );
            }
        });
    }

    Ok(info)
}

#[tauri::command]
pub async fn stop_tunnel(
    state: tauri::State<'_, TunnelManager>,
    tunnel_id: String,
) -> Result<bool, String> {
    state.stop(&tunnel_id).await
}

#[tauri::command]
pub async fn list_active_tunnels(
    state: tauri::State<'_, TunnelManager>,
) -> Result<Vec<ActiveTunnelInfo>, String> {
    Ok(state.list().await)
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    format!("{:x}", now)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_cloudflared() {
        let binary = find_cloudflared_binary();
        assert!(binary.is_some(), "Expected cloudflared binary to be detected");
    }

    #[test]
    fn test_url_extraction_regex() {
        let regex = Regex::new(r"https://[a-zA-Z0-9-]+\.trycloudflare\.com").unwrap();
        let log_sample = "2026-10-25T11:50:00Z INF Your quick Tunnel has been created! Visit it at (it may take some time to be reachable):\n2026-10-25T11:50:00Z INF https://bold-pine-482.trycloudflare.com\n";
        let mat = regex.find(log_sample);
        assert!(mat.is_some());
        assert_eq!(mat.unwrap().as_str(), "https://bold-pine-482.trycloudflare.com");
    }
}

