mod build_info;
mod commands;
mod state;

use state::AppState;
use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir().expect("app data dir");
            app.manage(Arc::new(AppState::new(data_dir)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::scan::pick_scan_folders,
            commands::scan::start_scan,
            commands::scan::cancel_scan,
            commands::scan::get_scan_snapshot,
            commands::cleanup::build_cleanup_plan,
            commands::cleanup::execute_cleanup,
            commands::settings::get_settings,
            commands::settings::save_settings,
            commands::license_cmd::get_license_status,
            commands::license_cmd::activate_license,
            commands::license_cmd::generate_local_license,
            commands::license_cmd::get_cleanup_history,
            commands::build_info_cmd::get_build_info_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
