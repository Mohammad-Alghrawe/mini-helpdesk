use axum::{extract::State, http::StatusCode, routing::post, Json, Router};

use crate::{
    models::ticket::CreateTicketRequest, repositories::ticket_repository, state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new().route("/tickets", post(create_ticket).get(list_tickets))
}

async fn create_ticket(
    State(state): State<AppState>,
    Json(payload): Json<CreateTicketRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    // Basic validation (Sprint 1 level)
    if payload.title.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "title is required" })),
        ));
    }

    let created = ticket_repository::create_ticket(&state.db, payload)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "db error", "details": e.to_string() })),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({ "ticket": created })),
    ))
}

async fn list_tickets(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let tickets = ticket_repository::list_tickets(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "db error", "details": e.to_string() })),
            )
        })?;

    Ok(Json(serde_json::json!({ "tickets": tickets })))
}
