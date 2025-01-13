mod services {
    pub mod cookies_services;
    pub mod consult_parcels_services;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            services::cookies_services::save_text_cookies,
            services::cookies_services::save_json_cookies,
            services::consult_parcels_services::consult_parcels
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
