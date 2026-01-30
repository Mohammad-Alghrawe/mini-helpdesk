use crate::state::AppState;
use axum::middleware::from_fn;
use axum::{
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use sqlx::SqlitePool;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
mod auth;
mod middleware;
mod models;
mod repositories;
mod routes;
mod state;

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
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

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Router
    let app = Router::<AppState>::new()
        .route("/health", get(health))
        .route("/api/auth/register", post(routes::auth::register))
        .route("/api/auth/login", post(routes::auth::login))
        .nest(
            "/api/tickets",
            routes::tickets::router().layer(from_fn(middleware::auth::jwt_auth)),
        )
        .with_state(app_state)
        .layer(cors);

    let addr: SocketAddr = format!("{host}:{port}").parse().expect("invalid host/port");
    println!("API listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
