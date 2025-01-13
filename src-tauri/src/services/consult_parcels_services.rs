use futures::lock::Mutex;
use serde::Serialize;
use std::{sync::Arc, time::Duration};
use tauri::Emitter;
use tauri_plugin_http::reqwest::Client;
use tokio::{task, time::sleep};

use super::cookies_services;

#[derive(Debug, Serialize, Default, Clone)]
pub struct ParcelData {
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

#[derive(Clone, Debug)]
enum ExecutionState {
    Running,
    Paused,
    Stopped
}

#[derive(Clone)]
struct SharedState {
    execution_state: ExecutionState,
    error_message: Option<String>,
    current_indices: Vec<usize>
}

impl SharedState {
    fn new(num_threads: usize) -> Self {
        Self {
            execution_state: ExecutionState::Running,
            error_message: None,
            current_indices: vec![0; num_threads],
        }
    }
}

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

async fn consult_single_parcel(parcel_code_consult: String, shared_state: Arc<Mutex<SharedState>>) -> Result<ParcelData, String> {
    {
        let state = shared_state.lock().await;
        match state.execution_state {
            ExecutionState::Stopped => {
                return Err("Operation cancelled due to error or user request".to_string());
            }
            ExecutionState::Paused => {
                drop(state);
                while shared_state.lock().await.execution_state == ExecutionState::Paused {
                    sleep(Duration::from_millis(100)).await;
                }
            }
            ExecutionState::Running => {}
        }
    }

    match cookies_services::get_cookies() {
        Ok(cookies) => {
            let url = format!("https://sigef.incra.gov.br/geo/parcela/detalhe/{}/", parcel_code_consult);
            let client = Client::new();

            let response = client.get(&url)
                .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
                .header("Cookie", &cookies)
                .send()
                .await
                .map_err(|e| format!("Erro ao enviar a requisição: {}", e))?;

            if !response.status().is_success() {
                return Err(format!("Código da parcela não encontrado!: {}", response.status()));
            }

            let html_in_text = response.text()
                .await
                .map_err(|e| format!("Erro ao ler o corpo da resposta: {}", e))?;

            if html_in_text.contains("Request Rejected") {
                return Err("Cookies vencidos ou inválidos! Renove os cookies e tente novamente.".to_string());
            }

            // Searching for HTML data
            let mut result = ParcelData::default();

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
            result.registration_situation = extract_value(&html_in_text, "Situação do Registro", None)
                .contains("<br />")
                .then(|| "Confirmado".to_string())
                .unwrap_or_else(|| "Não confirmado".to_string());

            Ok(result)
        }
        Err(e) => Err(format!("Os cookies não foram encontrados ou por algum motivo foram apagados, tente salvá-los novamente. {}", e)),
    }
}

#[tauri::command]
pub async fn consult_parcels(window: tauri::Window, parcels: Vec<String>) -> Result<(), String> {
    let chunk_size = (parcels.len() + 9) / 10;
    let chunks: Vec<_> = parcels.chunks(chunk_size).collect();
    let shared_state = Arc::new(Mutex::new(SharedState::new(chunks.len())));
    let mut tasks = Vec::new();

    for (i, chunk) in chunks.into_iter().enumerate() {
        let window_clone = window.clone();
        let chunk = chunk.to_vec();
        let shared_state = Arc::clone(&shared_state);

        let task = task::spawn(async move {
            let mut current_index = shared_state.lock().await.current_indices[i];

            while current_index < chunk.len() {
                let parcel = &chunk[current_index];

                {
                    let mut state = shared_state.lock().await;
                    state.current_indices[i] = current_index;
                }

                let result = consult_single_parcel(parcel.clone(), Arc::clone(&shared_state)).await;

                match result {
                    Ok(parcel_data) => {
                        let state = shared_state.lock().await;
                        if matches!(state.execution_state, ExecutionState::Stopped) {
                            break;
                        }

                        if let Err(e) = window_clone.emit("consult_parcel_result", &parcel_data) {
                            eprintln!("Thread {} - erro ao enviar resultado: {}", i, e);
                        }
                    }
                    Err(e) => {
                        if let Err(emit_err) = window_clone.emit("consult_parcel_result", &e) {
                            eprintln!("Thread {} - erro ao enviar erro: {}", i, emit_err);
                        }
                        break;
                    }
                }

                current_index += 1;
                sleep(Duration::from_secs(2)).await;
            }
        });

        tasks.push(task);
    }

    for task in tasks {
        if let Err(e) = task.await {
            eprintln!("Erro em uma das Threads: {}", e);
        }
    }

    let final_state = shared_state.lock().await;
    if matches!(final_state.execution_state, ExecutionState::Stopped) {
        if let Some(error_msg) = &final_state.error_message {
            return Err(error_msg.clone());
        }
    }

    Ok(())

    // for (i, chunk) in parcels.chunks(chunk_size).enumerate() {
    //     let window_clone = window.clone();
    //     let chunk = chunk.to_vec();

    //     let task = task::spawn(async move {
    //         for parcel in chunk {
    //             let result = consult_single_parcel(parcel).await;

    //             match result {
    //                 Ok(parcel_data) => {
    //                     if let Err(e) = window_clone.emit("consult_parcel_result", &parcel_data) {
    //                         eprintln!("Thread {} - erro ao enviar resultado: {}", i, e);
    //                     }
    //                 }
    //                 Err(e) => {
    //                     if let Err(e) = window_clone.emit("consult_parcel_result", &e.to_string()) {
    //                         eprintln!("Thread {} - erro ao enviar erro: {}", i, e);
    //                     }
    //                 }
    //             }

    //             sleep(Duration::from_secs(2)).await;
    //         }
    //     });

    //     tasks.push(task);
    // }

    // for task in tasks {
    //     if let Err(e) = task.await {
    //         eprintln!("Erro em uma das threads: {}", e);
    //     }
    // }

    // Ok(())
}
