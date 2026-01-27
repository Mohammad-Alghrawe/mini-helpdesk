use axum::{routing::get, Json, Router};
use serde_json::json;
use sqlx::SqlitePool;
use std::net::SocketAddr;

mod models;
mod repositories;
mod routes;
mod state;

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

#[tokio::main]
async fn main() {
    println!("Starting Mini-Helpdesk API...");

    // Host/Port
    let host = std::env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("APP_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8080);

    // SQLite DB URL (file path)
    // Example: sqlite:backend/data/helpdesk.db (relative path works fine)
    let db_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:data/helpdesk.db".to_string());

    // Create pool
    let pool = SqlitePool::connect(&db_url)
        .await
        .expect("failed to connect to sqlite database");

    // Run migration (Sprint 1 simple approach)
    // This reads your SQL file and executes it on startup.
    let migration_sql = include_str!("../migrations/001_create_tickets.sql");
    sqlx::query(migration_sql)
        .execute(&pool)
        .await
        .expect("failed to run migrations");

    // App state
    let app_state = state::AppState { db: pool.clone() };

    // Router
    let app = Router::new()
        .route("/health", get(health))
        .nest("/api", routes::tickets::router())
        .with_state(app_state);

    let addr: SocketAddr = format!("{host}:{port}").parse().expect("invalid host/port");
    println!("API listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
