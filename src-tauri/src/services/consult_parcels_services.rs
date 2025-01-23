use futures::lock::Mutex;
use serde::Serialize;
use tauri::{AppHandle, Emitter, State};
use std::{sync::{atomic::{AtomicUsize, Ordering}, Arc}, time::Duration};
use tauri_plugin_http::reqwest::{Client, Response};
use tokio::{sync::{mpsc, Notify}, time::sleep};

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

#[derive(Debug, Serialize, Default, Clone)]
pub struct ParcelResponse {
    data: ParcelData,
    total_count: usize,
    current_count: usize,
}

#[derive(Clone)]
pub struct AppState {
    is_paused: Arc<Mutex<bool>>,
    is_cancelled: Arc<Mutex<bool>>,
    notify: Arc<Notify>,
    has_error: Arc<Mutex<bool>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            is_paused: Arc::new(Mutex::new(false)),
            is_cancelled: Arc::new(Mutex::new(false)),
            notify: Arc::new(Notify::new()),
            has_error: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn wait_if_paused(&self) {
        loop {
            let paused = *self.is_paused.lock().await;
            let cancelled = *self.is_cancelled.lock().await;
            let has_error = *self.has_error.lock().await;

            if cancelled {
                break;
            }

            if !paused && !has_error {
                return;
            }

            self.notify.notified().await;
        }
    }

    pub async fn pause(&self) {
        *self.is_paused.lock().await = true;
    }

    pub async fn resume(&self) {
        *self.is_paused.lock().await = false;
        *self.has_error.lock().await = false;

        self.notify.notify_waiters();

        self.wait_if_paused().await;
    }

    pub async fn cancel(&self) {
        *self.is_cancelled.lock().await = true;
        self.notify.notify_waiters();
    }

    pub async fn reset(&self) {
        *self.is_paused.lock().await = false;
        *self.is_cancelled.lock().await = false;
        *self.has_error.lock().await = false;
        self.notify.notify_waiters();
    }

    pub async fn set_error(&self) {
        *self.has_error.lock().await = true;
        self.pause().await;
    }

    pub async fn get_has_error(&self) -> bool {
        *self.has_error.lock().await
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

async fn consult_single_parcel(parcel_code_consult: String) -> Result<ParcelData, String> {
    match cookies_services::get_cookies() {
        Ok(cookies) => {
            let url: String = format!("https://sigef.incra.gov.br/geo/parcela/detalhe/{}/", parcel_code_consult);
            let client: Client = Client::new();

            let response: Response = client.get(&url)
                .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
                .header("Cookie", &cookies)
                .send()
                .await
                .map_err(|e| format!("Erro ao enviar a requisição: {}", e))?;

            if !response.status().is_success() {
                return Err(format!("Código da parcela inválido! {}", parcel_code_consult));
            }

            let html_in_text: String = response.text()
                .await
                .map_err(|e| format!("Erro ao ler o corpo da resposta: {}", e))?;

            if html_in_text.contains("Request Rejected") {
                return Err("Cookies vencidos ou inválidos! Renove os cookies e tente novamente.".to_string());
            }

            sleep(Duration::from_secs(2)).await;

            // Searching for HTML data
            let mut result: ParcelData = ParcelData::default();

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
pub async fn consult_parcels(
    app_handler: AppHandle,
    parcels: Vec<String>,
    state: State<'_, AppState>
) -> Result<(), String> {
    let total_count = parcels.len();
    let thread_count = 10;
    let chunk_size = (parcels.len() + thread_count - 1) / thread_count;

    let (tx, mut rx) = mpsc::channel::<ParcelResponse>(100);

    let progress_counter = Arc::new(AtomicUsize::new(0));

    let state = Arc::new(state.inner().clone());

    for chunk in parcels.chunks(chunk_size) {
        let tx = tx.clone();
        let state = state.clone();
        let app_handler = app_handler.clone();
        let progress_counter = progress_counter.clone();
        let chunk = chunk.to_vec();

        tokio::spawn(async move {
            for parcel in chunk.into_iter() {
                if *state.is_cancelled.lock().await {
                    break;
                }

                state.wait_if_paused().await;

                match consult_single_parcel(parcel.clone()).await {
                    Ok(data) => {
                        let current_count = progress_counter.fetch_add(1, Ordering::SeqCst) + 1;

                        let response = ParcelResponse {
                            data,
                            total_count,
                            current_count,
                        };

                        if tx.send(response).await.is_err() {
                            break;
                        }
                    }
                    Err(error) => {
                        if !state.get_has_error().await {
                            state.set_error().await;
                            app_handler.emit("consult_parcels", error.clone()).unwrap();
                        }

                        break;
                    }
                }
            }
        });
    }

    drop(tx);

    while let Some(response) = rx.recv().await {
        if *state.is_cancelled.lock().await {
            break;
        }

        app_handler.emit("consult_parcels", response).unwrap();
    }

    Ok(())
}

#[tauri::command]
pub async fn pause_consult(state: State<'_, AppState>) -> Result<(), String> {
    state.pause().await;
    Ok(())
}

#[tauri::command]
pub async fn resume_consult(state: State<'_, AppState>) -> Result<(), String> {
    state.resume().await;
    Ok(())
}

#[tauri::command]
pub async fn reset_consult(state: State<'_, AppState>) -> Result<(), String> {
    state.reset().await;
    Ok(())
}

#[tauri::command]
pub async fn cancel_consult(state: State<'_, AppState>) -> Result<(), String> {
    state.cancel().await;
    Ok(())
}
