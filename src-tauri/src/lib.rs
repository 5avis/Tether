pub mod config;
pub mod deployer;
pub mod oauth;
pub mod providers;
pub mod tunnel;

use tauri::Window;
use tunnel::TunnelManager;

#[tauri::command]
fn app_minimize(window: Window) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
fn app_toggle_maximize(window: Window) -> Result<bool, String> {
    let is_max = window.is_maximized().map_err(|e| e.to_string())?;
    if is_max {
        window.unmaximize().map_err(|e| e.to_string())?;
        Ok(false)
    } else {
        window.maximize().map_err(|e| e.to_string())?;
        Ok(true)
    }
}

#[tauri::command]
fn app_close(window: Window) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}

#[tauri::command]
fn app_is_maximized(window: Window) -> Result<bool, String> {
    window.is_maximized().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let tunnel_manager = TunnelManager::new();
    let tunnel_manager_cleanup = tunnel_manager.clone();

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(tunnel_manager)
        .invoke_handler(tauri::generate_handler![
            app_minimize,
            app_toggle_maximize,
            app_close,
            app_is_maximized,
            config::get_config,
            config::save_project,
            config::delete_project,
            config::save_service,
            config::delete_service,
            config::save_provider_credential,
            config::verify_tokens,
            tunnel::check_cloudflared,
            tunnel::start_tunnel,
            tunnel::stop_tunnel,
            tunnel::list_active_tunnels,
            providers::refresh_workspace_status,
            providers::keep_alive_ping,
            providers::keep_alive_all,
            providers::suspend_target_service,
            providers::resume_target_service,
            deployer::trigger_parallel_deploy,
            deployer::sync_environment_secrets,
            deployer::stream_service_logs,
            oauth::start_google_oauth,
            oauth::get_user_profile,
            oauth::get_oauth_credentials,
            oauth::sign_out
        ])
        .build(tauri::generate_context!())
        .expect("error while building tether application");

    app.run(move |_app_handle, event| {
        if let tauri::RunEvent::Exit = event {
            // Guarantee subprocess hygiene: kill any active child tunnels on application exit
            let manager = tunnel_manager_cleanup.clone();
            tauri::async_runtime::block_on(async move {
                manager.stop_all().await;
            });
        }
    });
}
