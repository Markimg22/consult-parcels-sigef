use std::{fs::{read_to_string, File}, io::Write};
use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest::Client;

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

#[derive(Debug, Serialize, Default)]
struct ParcelData {
    parcel_code: String,
    owner_name: String,
    owner_cpf_or_cnpj: String,
    denomination: String,
    area: String,
    situation_parcel: String,
    technical_manager: String,
    situation_area: String,
    city_uf: String,
    registry_office: String,
    cns: String,
    registration: String,
    registration_situation: String,
    code_incra: String,
    property_type: String,
    date_of_entry: String,
    rt_document: String,
}

#[tauri::command]
fn save_text_cookies(text: String) -> Result<String, String> {
    let mut dir = std::env::temp_dir();
    dir.push(FILE_NAME);

    let mut file = File::create(&dir).map_err(|e| e.to_string())?;
    file.write_all(text.as_bytes()).map_err(|e| e.to_string())?;

    Ok(dir.to_string_lossy().to_string())
}

#[tauri::command]
fn save_json_cookies(file_path: String) -> Result<String, String> {
    let content = read_to_string(file_path).map_err(|e| e.to_string())?;

    let mut dir = std::env::temp_dir();
    dir.push(FILE_NAME);

    let mut file = File::create(&dir).map_err(|e| e.to_string())?;
    file.write_all(content.as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(dir.to_string_lossy().to_string())
}

fn get_cookies() -> Result<String, String> {
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

#[tauri::command]
async fn consult_parcels(parcels: Vec<String>) -> Result<ParcelData, String> {
    match get_cookies() {
        Ok(cookies) => {
            let parcel_code_consult = parcels[0].clone();

            let url = format!("https://sigef.incra.gov.br/geo/parcela/detalhe/{}/", parcel_code_consult);
            let client = Client::new();

            let response = client.get(&url)
                .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
                .header("Cookie", &cookies)
                .send()
                .await
                .map_err(|e| format!("Erro ao enviar a requisição: {}", e))?;

            if !response.status().is_success() {
                return Err(format!("Erro na requisição: {}", response.status()));
            }

            let html_in_text = response.text()
                .await
                .map_err(|e| format!("Erro ao ler o corpo da resposta: {}", e))?;

            // Searching for HTML data
            let mut result = ParcelData::default();

            fn split_and_get<'a>(text: &'a str, split_by: &str, index: usize) -> Option<&'a str> {
                text.split(split_by).nth(index)
            }

            fn extract_value(html: &str, start_pattern: &str, clean_pattern: Option<&str>) -> String {
                split_and_get(html, start_pattern, 1)
                    .and_then(|s| split_and_get(s, "<td>", 1))
                    .and_then(|s| split_and_get(s, "</td>", 0))
                    .map(|s| {
                        if let Some(pattern) = clean_pattern {
                            s.split(pattern).next().unwrap_or("").trim()
                        } else {
                            s.trim()
                        }
                    })
                    .unwrap_or_default()
                    .to_string()
            }

            // Código Parcela
            result.parcel_code = parcel_code_consult;

            // Denominação
            result.denomination = extract_value(&html_in_text, "<th>Denominação</th>", None)
                .chars()
                .take(60)
                .collect();

            // Área
            result.area = extract_value(&html_in_text, "<th>Área</th>", Some("ha"));

            // Data de Entrada
            result.date_of_entry = extract_value(&html_in_text, "<th>Data de Entrada</th>", None);

            // Situação Parcela
            result.situation_parcel = split_and_get(&html_in_text, "<th>Situação</th>", 1)
                .and_then(|s| split_and_get(s, "<td>", 1))
                .and_then(|s| split_and_get(s, "<br />", 0))
                .map(|s| s.trim().to_string())
                .unwrap_or_default();

            // Responsável Técnico
            result.technical_manager = extract_value(&html_in_text, "<th>Responsável Técnico(a)</th>", None);

            // Documento de RT
            result.rt_document = extract_value(&html_in_text, "<th>Documento de RT</th>", Some(" - "));

            // Situação Área
            if let Some(after_first_situation) = split_and_get(&html_in_text, &result.situation_parcel, 1) {
                result.situation_area = extract_value(after_first_situation, "<th>Situação</th>", None);
            }

            // Tipo da Propriedade
            result.property_type = extract_value(&html_in_text, "<th>Natureza</th>", None);

            // Código INCRA
            result.code_incra = extract_value(&html_in_text, "<th>Código do Imóvel (SNCR/INCRA)</th>", None);

            // Cidade e UF
            result.city_uf = split_and_get(&html_in_text, "<th colspan=\"2\">Municípios</th>", 1)
                .and_then(|s| split_and_get(s, "<td colspan=\"2\">", 1))
                .and_then(|s| split_and_get(s, "</td>", 0))
                .map(|s| s.trim())
                .unwrap_or_default()
                .to_string();

            // Nome do Proprietário
            result.owner_name = split_and_get(&html_in_text, "<th>Nome</th>", 1)
                .and_then(|s| split_and_get(s, "<td>", 1))
                .and_then(|s| split_and_get(s, "</td>", 0))
                .map(|s| s.trim())
                .unwrap_or_default()
                .chars()
                .take(60)
                .collect();

            // CPF ou CNPJ do Proprietário
            result.owner_cpf_or_cnpj =  split_and_get(&html_in_text, &result.owner_name, 1)
                .and_then(|s| split_and_get(s, "<td>", 1))
                .and_then(|s| split_and_get(s, "</td>", 0))
                .map(|s| s.trim())
                .unwrap_or_default()
                .to_string();

            // Cartório
            result.registry_office = extract_value(&html_in_text, "<th>Cartório</th>", None);

            // CNS
            result.cns = extract_value(&html_in_text, "<th>Código Nacional de Serventia (CNS)</th>", None);

            // Matrícula
            result.registration = extract_value(&html_in_text, "Matrícula", None);

            // Situação Matrícula
            result.registration_situation = extract_value(&html_in_text, "Situação do Registro", None);

            Ok(result)
        }
        Err(e) => Err(format!("Os cookies não foram encontrados ou por algum motivo foram apagados, tente salvá-los novamente. {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            save_text_cookies,
            save_json_cookies,
            consult_parcels
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
