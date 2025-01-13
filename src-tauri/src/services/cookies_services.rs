use serde::{Deserialize, Serialize};
use std::{fs::{read_to_string, File}, io::Write};

const FILE_NAME: &str = "consult-parcels-sigef-cookies.json";

#[derive(Serialize, Deserialize)]
struct Cookie {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct CookiesFile {
    cookies: Vec<Cookie>,
}

#[tauri::command]
pub fn save_text_cookies(text: String) -> Result<String, String> {
    let mut dir = std::env::temp_dir();
    dir.push(FILE_NAME);

    let mut file = File::create(&dir).map_err(|e| e.to_string())?;
    file.write_all(text.as_bytes()).map_err(|e| e.to_string())?;

    Ok(dir.to_string_lossy().to_string())
}

#[tauri::command]
pub fn save_json_cookies(file_path: String) -> Result<String, String> {
    let content = read_to_string(file_path).map_err(|e| e.to_string())?;

    let mut dir = std::env::temp_dir();
    dir.push(FILE_NAME);

    let mut file = File::create(&dir).map_err(|e| e.to_string())?;
    file.write_all(content.as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(dir.to_string_lossy().to_string())
}

pub fn get_cookies() -> Result<String, String> {
    let mut dir = std::env::temp_dir();
    dir.push(FILE_NAME);

    let content = std::fs::read_to_string(&dir).map_err(|e| e.to_string())?;
    let cookies_file: CookiesFile = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    let cookies = cookies_file.cookies
        .iter()
        .map(|cookie| format!("{}={}", cookie.name, cookie.value))
        .collect::<Vec<_>>()
        .join(";");

    Ok(cookies)
}
