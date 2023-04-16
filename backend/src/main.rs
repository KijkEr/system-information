use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use num_format::{Locale, ToFormattedString};
use serde::Serialize;
use std::{net::SocketAddr, time::Duration};
use sysinfo::{CpuExt, System, SystemExt};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/system_information", get(get_system_information))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 6969));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_system_information() -> impl IntoResponse {
    let mut sys = System::new_all();
    sys.refresh_all();

    let total_memory = sys.total_memory();
    let free_memory = sys.available_memory();
    let cpu_count = sys.cpus().len();
    let mut cpu_usage: Vec<f32> = Vec::new();

    for cpu in sys.cpus() {
        cpu_usage.push(cpu.cpu_usage());
    }

    let output = SystemInformation {
        total_memory: total_memory,
        free_memory: free_memory,
        cpu_count: cpu_count,
        cpu_usage: cpu_usage,
    };
    (StatusCode::OK, Json(output))
}
#[derive(Serialize)]
struct SystemInformation {
    total_memory: u64,
    free_memory: u64,
    cpu_count: usize,
    cpu_usage: Vec<f32>,
}
