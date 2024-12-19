mod commands;

use commands::file_operations::{save_json_cookies, save_text_cookies};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            save_text_cookies,
            save_json_cookies
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
