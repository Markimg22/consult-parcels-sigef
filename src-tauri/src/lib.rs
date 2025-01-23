
mod services {
    pub mod cookies_services;
    pub mod consult_parcels_services;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(services::consult_parcels_services::AppState::new())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            services::cookies_services::save_text_cookies,
            services::cookies_services::save_json_cookies,
            services::consult_parcels_services::consult_parcels,
            services::consult_parcels_services::pause_consult,
            services::consult_parcels_services::resume_consult,
            services::consult_parcels_services::reset_consult,
            services::consult_parcels_services::cancel_consult
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
