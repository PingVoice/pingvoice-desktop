mod config;
mod tray;

use config::AppConfig;
use tauri::Manager;
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config = AppConfig::load();
    let url = config.url.clone();
    let start_minimized = config.start_minimized;

    // Check if --minimized flag was passed (for autostart)
    let args: Vec<String> = std::env::args().collect();
    let minimized_flag = args.iter().any(|arg| arg == "--minimized");
    let should_start_minimized = start_minimized || minimized_flag;

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .setup(move |app| {
            // Create system tray
            tray::create_tray(app.handle())?;

            // Get the main window and configure it
            if let Some(window) = app.get_webview_window("main") {
                // Navigate to configured URL
                let url_clone = url.clone();
                window.eval(&format!(
                    "window.location.replace('{}')",
                    url_clone.replace('\'', "\\'")
                )).ok();

                // Show window if not starting minimized
                if !should_start_minimized {
                    window.show().ok();
                    window.set_focus().ok();
                }

                // Handle window close - hide to tray instead of quitting
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        window_clone.hide().ok();
                    }
                });
            }

            // Enable autostart if configured
            if config.autostart {
                let autostart = app.autolaunch();
                autostart.enable().ok();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
