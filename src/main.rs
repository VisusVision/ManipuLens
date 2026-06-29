mod types;
mod agents;
mod orchestrator;

use axum::{routing::post, Json, Router};
use tower_http::cors::{Any, CorsLayer};
use types::{AnalyzeRequest, FinalReport};
use std::net::SocketAddr;

async fn handle_analyze(Json(payload): Json<AnalyzeRequest>) -> Result<Json<FinalReport>, (axum::http::StatusCode, String)> {
    match orchestrator::run_orchestrator(&payload.text).await {
        Ok(report) => Ok(Json(report)),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}

#[tokio::main]
async fn main() {
    // Eklentiden (chrome-extension://) gelecek istekleri kabul etmek için CORS ayarı
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let app = Router::new()
        .route("/v1/analyze", post(handle_analyze))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Manipülasyon Tespit Backend Servisi {} adresinde çalışıyor...", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}