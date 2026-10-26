use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use crate::config::{read_config_from_disk, save_config_to_disk, UserProfile};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleOAuthInstalled {
    pub client_id: String,
    pub project_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    #[serde(default)]
    pub auth_provider_x509_cert_url: Option<String>,
    pub client_secret: String,
    pub redirect_uris: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleOAuthFile {
    pub installed: GoogleOAuthInstalled,
}

#[derive(Debug, Deserialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    #[serde(default)]
    pub id_token: Option<String>,
    #[serde(default)]
    pub token_type: Option<String>,
    #[serde(default)]
    pub expires_in: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct GoogleUserInfo {
    pub sub: String,
    pub email: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub picture: Option<String>,
}

pub fn is_admin_email(email: &str) -> bool {
    let lower = email.trim().to_lowercase();
    lower.contains("sivanandham5528") || lower.contains("528cva")
}

pub fn load_oauth_credentials() -> GoogleOAuthInstalled {
    let candidate_paths = vec![
        PathBuf::from("/home/siva/Projects/Tether/oauth.json"),
        PathBuf::from("oauth.json"),
        PathBuf::from("../oauth.json"),
        dirs::home_dir()
            .map(|h| h.join(".config").join("tether").join("oauth.json"))
            .unwrap_or_else(|| PathBuf::from("oauth.json")),
    ];

    for path in candidate_paths {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(parsed) = serde_json::from_str::<GoogleOAuthFile>(&content) {
                    return parsed.installed;
                }
            }
        }
    }

    // Fallback if oauth.json is not yet configured on disk
    GoogleOAuthInstalled {
        client_id: std::env::var("GOOGLE_CLIENT_ID").unwrap_or_default(),
        project_id: std::env::var("GOOGLE_PROJECT_ID").unwrap_or_else(|_| "tether-app".into()),
        auth_uri: "https://accounts.google.com/o/oauth2/auth".into(),
        token_uri: "https://oauth2.googleapis.com/token".into(),
        auth_provider_x509_cert_url: Some("https://www.googleapis.com/oauth2/v1/certs".into()),
        client_secret: std::env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default(),
        redirect_uris: vec!["http://localhost".into()],
    }
}

pub fn open_browser_url(url: &str) {
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(url).spawn();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd").args(["/C", "start", url]).spawn();
    }
}

fn urlencode(input: &str) -> String {
    let mut out = String::new();
    for b in input.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            _ => {
                out.push_str(&format!("%{:02X}", b));
            }
        }
    }
    out
}

fn urlencode_decode(input: &str) -> String {
    let mut out = Vec::new();
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(h) = u8::from_str_radix(std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or(""), 16) {
                out.push(h);
                i += 3;
                continue;
            }
        }
        if bytes[i] == b'+' {
            out.push(b' ');
        } else {
            out.push(bytes[i]);
        }
        i += 1;
    }
    String::from_utf8_lossy(&out).to_string()
}

fn extract_code_from_request(request_line: &str) -> Option<String> {
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }
    let path_and_query = parts[1];
    let query = path_and_query.split('?').nth(1)?;

    for pair in query.split('&') {
        let mut kv = pair.split('=');
        if let (Some(key), Some(val)) = (kv.next(), kv.next()) {
            if key == "code" {
                return Some(urlencode_decode(val));
            }
        }
    }
    None
}

#[tauri::command]
pub async fn start_google_oauth() -> Result<UserProfile, String> {
    let credentials = load_oauth_credentials();

    // 1. Bind to a dynamic loopback port
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .map_err(|e| format!("Failed to bind loopback TCP listener for OAuth: {}", e))?;

    let port = listener
        .local_addr()
        .map_err(|e| format!("Failed to get listener local addr: {}", e))?
        .port();

    let redirect_uri = format!("http://localhost:{}", port);

    // 2. Build Google authorization URL
    let auth_url = format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope=openid%20email%20profile&access_type=offline&prompt=select_account",
        credentials.auth_uri,
        urlencode(&credentials.client_id),
        urlencode(&redirect_uri)
    );

    // 3. Open user default browser
    open_browser_url(&auth_url);

    // 4. Wait for redirect with a 120-second timeout
    let auth_code = tokio::time::timeout(std::time::Duration::from_secs(120), async {
        let (mut socket, _) = listener.accept().await.map_err(|e| e.to_string())?;

        let mut buf = [0u8; 4096];
        let n = socket.read(&mut buf).await.map_err(|e| e.to_string())?;
        let request = String::from_utf8_lossy(&buf[..n]);

        let first_line = request.lines().next().unwrap_or("");
        let code = extract_code_from_request(first_line)
            .ok_or_else(|| "Failed to extract code from redirect callback".to_string())?;

        let html_body = r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>Tether - Connected</title>
  <style>
    body { background: #1f1f1f; color: #ffffff; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; display: flex; align-items: center; justify-content: center; height: 100vh; margin: 0; }
    .card { background: #262626; border: 1px solid #383838; padding: 36px; border-radius: 16px; text-align: center; max-width: 420px; box-shadow: 0 12px 32px rgba(0,0,0,0.4); }
    .badge { display: inline-flex; align-items: center; gap: 6px; background: rgba(226,92,72,0.15); color: #e25c48; border: 1px solid rgba(226,92,72,0.3); border-radius: 999px; padding: 4px 14px; font-size: 11px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: 16px; }
    h1 { margin: 0 0 10px; font-size: 20px; font-weight: 600; color: #ffffff; }
    p { color: #9e9e9e; font-size: 13px; line-height: 1.5; margin: 0; }
  </style>
</head>
<body>
  <div class="card">
    <div class="badge">&#10003; Tether Connected</div>
    <h1>Authentication Successful</h1>
    <p>Your Google account has been linked to your local Tether profile. You can safely close this browser window and return to Tether.</p>
  </div>
</body>
</html>"#;

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            html_body.len(),
            html_body
        );

        let _ = socket.write_all(response.as_bytes()).await;
        let _ = socket.flush().await;

        Ok::<String, String>(code)
    })
    .await
    .map_err(|_| "OAuth sign-in timed out after 120 seconds. Please try again.".to_string())?
    .map_err(|e| format!("OAuth callback error: {}", e))?;

    // 5. Exchange code for access token
    let client = reqwest::Client::new();
    let token_params = [
        ("code", auth_code.as_str()),
        ("client_id", credentials.client_id.as_str()),
        ("client_secret", credentials.client_secret.as_str()),
        ("redirect_uri", redirect_uri.as_str()),
        ("grant_type", "authorization_code"),
    ];

    let token_res = client
        .post(&credentials.token_uri)
        .form(&token_params)
        .send()
        .await
        .map_err(|e| format!("Failed to reach Google token endpoint: {}", e))?;

    if !token_res.status().is_success() {
        let err_text = token_res.text().await.unwrap_or_default();
        return Err(format!("Google token exchange failed: {}", err_text));
    }

    let token_data: GoogleTokenResponse = token_res
        .json()
        .await
        .map_err(|e| format!("Failed to parse Google token JSON: {}", e))?;

    // 6. Fetch user profile from Google UserInfo endpoint
    let userinfo_res = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(&token_data.access_token)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch Google user profile: {}", e))?;

    let user_info: GoogleUserInfo = userinfo_res
        .json()
        .await
        .map_err(|e| format!("Failed to parse Google userinfo: {}", e))?;

    let role = if is_admin_email(&user_info.email) {
        "admin".to_string()
    } else {
        "member".to_string()
    };

    let profile = UserProfile {
        id: user_info.sub,
        email: user_info.email,
        name: user_info.name.unwrap_or_else(|| "Developer".into()),
        picture: user_info.picture,
        signed_in: true,
        provider: "google".into(),
        token: Some(token_data.access_token),
        role: Some(role),
    };

    // 7. Save to local-first config.json (POSIX 0600)
    let mut cfg = read_config_from_disk()?;
    cfg.profile = Some(profile.clone());
    save_config_to_disk(&cfg)?;

    Ok(profile)
}

#[tauri::command]
pub fn get_user_profile() -> Result<Option<UserProfile>, String> {
    let mut cfg = read_config_from_disk()?;
    if let Some(mut profile) = cfg.profile.clone() {
        let expected_role = if is_admin_email(&profile.email) {
            "admin".to_string()
        } else {
            "member".to_string()
        };
        if profile.role.as_deref() != Some(&expected_role) {
            profile.role = Some(expected_role.clone());
            cfg.profile = Some(profile.clone());
            let _ = save_config_to_disk(&cfg);
        }
        profile.role = Some(expected_role);
        Ok(Some(profile))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub fn sign_out() -> Result<(), String> {
    let mut cfg = read_config_from_disk()?;
    cfg.profile = None;
    save_config_to_disk(&cfg)?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthDisplayCredentials {
    pub client_id: String,
    pub client_secret: String,
    pub project_id: String,
    pub redirect_mode: String,
}

#[tauri::command]
pub fn get_oauth_credentials() -> Result<OAuthDisplayCredentials, String> {
    let cfg = read_config_from_disk()?;
    let is_admin = cfg
        .profile
        .as_ref()
        .map(|p| is_admin_email(&p.email))
        .unwrap_or(false);

    if !is_admin {
        return Err("Access denied: Administrator privileges required.".into());
    }

    let creds = load_oauth_credentials();
    Ok(OAuthDisplayCredentials {
        client_id: creds.client_id,
        client_secret: creds.client_secret,
        project_id: creds.project_id,
        redirect_mode: "Loopback Port".into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_code_from_request() {
        let request = "GET /?code=4%2F0AY0e-gtest123&scope=email+profile HTTP/1.1";
        let code = extract_code_from_request(request);
        assert_eq!(code, Some("4/0AY0e-gtest123".to_string()));
    }

    #[test]
    fn test_urlencode_and_decode() {
        let original = "http://localhost:8080/callback";
        let encoded = urlencode(original);
        assert!(encoded.contains("%3A"));
        let decoded = urlencode_decode(&encoded);
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_load_oauth_credentials() {
        let creds = load_oauth_credentials();
        assert_eq!(creds.token_uri, "https://oauth2.googleapis.com/token");
    }
}
