mod types;
mod agents;
mod orchestrator;

use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use http::{header, Method};
use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use types::{AnalyzeRequest, FinalReport, HistoryEntry};

// Geçmişi tutmak için paylaşılan state
type HistoryStore = Arc<Mutex<Vec<HistoryEntry>>>;

#[derive(Deserialize)]
struct HistoryQuery {
    client_id: Option<String>,
}

async fn handle_analyze(
    State(history): State<HistoryStore>,
    Json(payload): Json<AnalyzeRequest>,
) -> Result<Json<FinalReport>, (axum::http::StatusCode, String)> {
    let client_id = payload
        .client_id
        .unwrap_or_else(|| "anonim".to_string());
    let text = payload.text.clone();

    match orchestrator::run_orchestrator(&text).await {
        Ok(report) => {
            // Geçmiş kaydı oluştur
            let entry = HistoryEntry {
                timestamp: Utc::now().to_rfc3339(),
                client_id: client_id.clone(),
                text_preview: text.chars().take(120).collect::<String>(),
                is_manipulated: report.is_manipulated,
                dominant_manipulation: report.dominant_manipulation.clone(),
                genel_sonuc: report.genel_sonuc.clone(),
            };

            // Belleğe ekle
            {
                let mut store = history.lock().await;
                store.push(entry.clone());

                // Son 500 kaydı tut
                if store.len() > 500 {
                    let excess = store.len() - 500;
                    store.drain(0..excess);
                }
            }

            // JSONL geçmiş dosyasına yaz
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open("history.jsonl")
            {
                if let Ok(line) = serde_json::to_string(&entry) {
                    let _ = writeln!(file, "{}", line);
                }
            }

            // ===== OKUNABİLİR LOG DOSYASI =====
            if let Ok(mut log_file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open("analiz_log.txt")
            {
                let now = Utc::now().format("%Y-%m-%d %H:%M:%S");
                let log_line = format!(
                    "[{}] | Client: {} | Baskın: {} | Manipülasyon: {}\nMetin: {}\n{}\n",
                    now,
                    client_id,
                    report.dominant_manipulation,
                    if report.is_manipulated { "EVET" } else { "HAYIR" },
                    text,
                    "-".repeat(80)
                );
                let _ = write!(log_file, "{}", log_line);
            }

            Ok(Json(report))
        }
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}

async fn handle_history(
    State(history): State<HistoryStore>,
    Query(_query): Query<HistoryQuery>,
) -> Json<Vec<HistoryEntry>> {
    let store = history.lock().await;

    // Geçici olarak tüm geçmişi döndürüyoruz (client_id filtresi kapalı)
    let mut filtered: Vec<HistoryEntry> = store.clone();

    // En yeniden eskiye
    filtered.reverse();
    Json(filtered)
}

#[tokio::main]
async fn main() {
    // Daha güçlü CORS ayarı (Chrome eklentisi + ngrok için)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::OPTIONS,
            Method::PUT,
            Method::DELETE,
        ])
        .allow_headers(Any)
        .expose_headers(Any);

    let history: HistoryStore = Arc::new(Mutex::new(Vec::new()));

    // Varsa eski geçmişi yükle
    if let Ok(content) = std::fs::read_to_string("history.jsonl") {
        let mut store = history.lock().await;
        for line in content.lines() {
            if let Ok(entry) = serde_json::from_str::<HistoryEntry>(line) {
                store.push(entry);
            }
        }
        println!("Geçmiş yüklendi: {} kayıt", store.len());
    }

    let app = Router::new()
        .route("/v1/analyze", post(handle_analyze))
        .route("/v1/history", get(handle_history))
        .layer(cors)
        .with_state(history);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Manipülasyon Tespit Backend Servisi {} adresinde çalışıyor...", addr);
    println!("Geçmiş endpoint hazır → GET /v1/history");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}