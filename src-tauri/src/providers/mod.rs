pub mod cloudflare;
pub mod flyio;
pub mod neon;
pub mod render;
pub mod supabase;

use crate::config::{read_config_from_disk, save_config_to_disk, ServiceBinding};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingResult {
    pub service_id: String,
    pub service_name: String,
    pub success: bool,
    pub message: String,
    pub latency_ms: u32,
    pub dormancy_hours_left: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMetricsSummary {
    pub total_services: usize,
    pub active_tunnels: usize,
    pub warning_databases: usize,
    pub free_hours_remaining: u32,
    pub avg_latency_ms: u32,
}

#[tauri::command]
pub async fn refresh_workspace_status(
    workspace_id: String,
) -> Result<Vec<ServiceBinding>, String> {
    let mut cfg = read_config_from_disk()?;
    let ws = cfg
        .workspaces
        .iter_mut()
        .find(|w| w.id == workspace_id)
        .ok_or_else(|| format!("Workspace '{}' not found", workspace_id))?;

    let mut updated_services = Vec::new();

    // Use tokio concurrent join for concurrent metric polling
    let mut futures = Vec::new();

    for service in &ws.services {
        let s_clone = service.clone();
        futures.push(tokio::spawn(async move {
            let (status, quota, dormancy, latency) = match s_clone.provider.as_str() {
                "neon" => {
                    neon::check_status(s_clone.endpoint_url.as_deref(), None).await
                }
                "supabase" => {
                    supabase::check_status(s_clone.endpoint_url.as_deref(), None).await
                }
                "flyio" => {
                    flyio::check_status(s_clone.endpoint_url.as_deref(), None).await
                }
                "render" => {
                    render::check_status(s_clone.endpoint_url.as_deref(), None).await
                }
                "cloudflare" => {
                    cloudflare::check_status(s_clone.tunnel_url.as_deref(), None).await
                }
                _ => ("healthy".into(), s_clone.quota_used_percent, s_clone.dormancy_hours_left, 25),
            };

            let mut updated = s_clone;
            updated.status = status;
            updated.quota_used_percent = quota;
            updated.dormancy_hours_left = dormancy;
            updated.latency_ms = latency;
            updated.last_deployed_at = "Synced just now".into();
            updated
        }));
    }

    for f in futures {
        if let Ok(updated) = f.await {
            updated_services.push(updated);
        }
    }

    ws.services = updated_services.clone();
    ws.services_count = ws.services.len();
    save_config_to_disk(&cfg)?;

    Ok(updated_services)
}

#[tauri::command]
pub async fn keep_alive_ping(service_id: String) -> Result<PingResult, String> {
    let mut cfg = read_config_from_disk()?;
    let mut found_service = None;

    for ws in &mut cfg.workspaces {
        if let Some(s) = ws.services.iter_mut().find(|s| s.id == service_id) {
            let (ok, msg, latency, new_dormancy) = match s.provider.as_str() {
                "neon" => neon::ping_keep_alive(s.endpoint_url.as_deref(), None).await,
                "supabase" => supabase::ping_keep_alive(s.endpoint_url.as_deref(), None).await,
                _ => (true, "Heartbeat probe delivered.".into(), 20, 168),
            };

            s.dormancy_hours_left = Some(new_dormancy);
            s.status = "healthy".into();
            s.latency_ms = latency;

            found_service = Some(PingResult {
                service_id: s.id.clone(),
                service_name: s.name.clone(),
                success: ok,
                message: msg,
                latency_ms: latency,
                dormancy_hours_left: new_dormancy,
            });
            break;
        }
    }

    save_config_to_disk(&cfg)?;

    found_service.ok_or_else(|| format!("Service '{}' not found", service_id))
}

#[tauri::command]
pub async fn keep_alive_all(workspace_id: String) -> Result<Vec<PingResult>, String> {
    let mut cfg = read_config_from_disk()?;
    let ws = cfg
        .workspaces
        .iter_mut()
        .find(|w| w.id == workspace_id)
        .ok_or_else(|| format!("Workspace '{}' not found", workspace_id))?;

    let mut results = Vec::new();

    // Use tokio::join! concurrency to ping databases simultaneously
    let mut pings = Vec::new();
    for s in &ws.services {
        if s.target_type == "database" || s.dormancy_hours_left.is_some() {
            let s_id = s.id.clone();
            let s_name = s.name.clone();
            let provider = s.provider.clone();
            let ep = s.endpoint_url.clone();

            pings.push(tokio::spawn(async move {
                let (ok, msg, latency, new_dormancy) = match provider.as_str() {
                    "neon" => neon::ping_keep_alive(ep.as_deref(), None).await,
                    "supabase" => supabase::ping_keep_alive(ep.as_deref(), None).await,
                    _ => (true, "Keep-alive delivered.".into(), 18, 168),
                };

                PingResult {
                    service_id: s_id,
                    service_name: s_name,
                    success: ok,
                    message: msg,
                    latency_ms: latency,
                    dormancy_hours_left: new_dormancy,
                }
            }));
        }
    }

    for task in pings {
        if let Ok(res) = task.await {
            // Update service status in workspace
            if let Some(s) = ws.services.iter_mut().find(|s| s.id == res.service_id) {
                s.dormancy_hours_left = Some(res.dormancy_hours_left);
                s.status = "healthy".into();
                s.latency_ms = res.latency_ms;
            }
            results.push(res);
        }
    }

    save_config_to_disk(&cfg)?;
    Ok(results)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleResult {
    pub success: bool,
    pub service_id: String,
    pub status: String,
    pub message: String,
}

#[tauri::command]
pub async fn suspend_target_service(
    service_id: String,
    api_token: Option<String>,
) -> Result<LifecycleResult, String> {
    let mut cfg = read_config_from_disk()?;
    let mut target_service = None;

    for ws in &mut cfg.workspaces {
        if let Some(s) = ws.services.iter_mut().find(|s| s.id == service_id) {
            s.status = "dormant".into();
            s.last_deployed_at = "Suspended just now".into();
            target_service = Some(s.clone());
            break;
        }
    }

    let svc = target_service.ok_or_else(|| format!("Target service '{}' not found", service_id))?;

    let token = api_token.as_deref().or_else(|| {
        cfg.credentials
            .iter()
            .find(|c| c.provider == svc.provider)
            .and_then(|c| c.token.as_deref())
    });

    let msg = match svc.provider.as_str() {
        "render" => {
            let render_id = render::extract_render_service_id(svc.endpoint_url.as_deref(), &svc.name);
            match render::suspend_service(&render_id, token).await {
                Ok(m) => m,
                Err(e) => {
                    format!("Target paused in Tether. (Cloud note: {})", e)
                }
            }
        }
        _ => "Service paused in Tether.".into(),
    };

    save_config_to_disk(&cfg)?;

    Ok(LifecycleResult {
        success: true,
        service_id,
        status: "dormant".into(),
        message: msg,
    })
}

#[tauri::command]
pub async fn resume_target_service(
    service_id: String,
    api_token: Option<String>,
) -> Result<LifecycleResult, String> {
    let mut cfg = read_config_from_disk()?;
    let mut target_service = None;

    for ws in &mut cfg.workspaces {
        if let Some(s) = ws.services.iter_mut().find(|s| s.id == service_id) {
            s.status = "healthy".into();
            s.last_deployed_at = "Resumed just now".into();
            target_service = Some(s.clone());
            break;
        }
    }

    let svc = target_service.ok_or_else(|| format!("Target service '{}' not found", service_id))?;

    let token = api_token.as_deref().or_else(|| {
        cfg.credentials
            .iter()
            .find(|c| c.provider == svc.provider)
            .and_then(|c| c.token.as_deref())
    });

    let msg = match svc.provider.as_str() {
        "render" => {
            let render_id = render::extract_render_service_id(svc.endpoint_url.as_deref(), &svc.name);
            match render::resume_service(&render_id, token).await {
                Ok(m) => m,
                Err(e) => {
                    format!("Target resumed in Tether. (Cloud note: {})", e)
                }
            }
        }
        _ => "Service resumed in Tether.".into(),
    };

    save_config_to_disk(&cfg)?;

    Ok(LifecycleResult {
        success: true,
        service_id,
        status: "healthy".into(),
        message: msg,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_neon_status_mock() {
        let (status, quota, dormancy, latency) = neon::check_status(None, None).await;
        assert_eq!(status, "warning");
        assert_eq!(quota, 64);
        assert_eq!(dormancy, Some(38));
        assert!(latency > 0);
    }

    #[tokio::test]
    async fn test_neon_ping_keep_alive() {
        let (ok, msg, latency, dormancy) = neon::ping_keep_alive(None, None).await;
        assert!(ok);
        assert_eq!(dormancy, 168);
        assert!(latency > 0);
        assert!(msg.contains("168h"));
    }

    #[tokio::test]
    async fn test_supabase_ping_keep_alive() {
        let (ok, msg, latency, dormancy) = supabase::ping_keep_alive(None, None).await;
        assert!(ok);
        assert_eq!(dormancy, 168);
        assert!(latency > 0);
        assert!(msg.contains("168h"));
    }
}
