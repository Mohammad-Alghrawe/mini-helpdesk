use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::{
    models::ticket::CreateTicketRequest, repositories::ticket_repository, state::AppState,
};

use crate::models::ticket::UpdateTicketRequest;

async fn update_ticket(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTicketRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // must provide at least one field
    if payload.status.is_none() && payload.priority.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "provide status and/or priority" })),
        ));
    }

    let updated = ticket_repository::update_ticket(&state.db, &id, payload)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "db error", "details": e.to_string() })),
            )
        })?;

    match updated {
        Some(t) => Ok(Json(serde_json::json!({ "ticket": t }))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "ticket not found" })),
        )),
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route(
            "/tickets/:id",
            get(get_ticket_by_id)
                .patch(update_ticket)
                .delete(delete_ticket),
        )
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

async fn get_ticket_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let ticket = ticket_repository::get_ticket_by_id(&state.db, &id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "db error", "details": e.to_string() })),
            )
        })?;

    match ticket {
        Some(t) => Ok(Json(serde_json::json!({ "ticket": t }))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "ticket not found" })),
        )),
    }
}

async fn delete_ticket(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let deleted = ticket_repository::delete_ticket(&state.db, &id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "db error", "details": e.to_string()})),
            )
        })?;

    if deleted {
        Ok(StatusCode::NO_CONTENT) // 204
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "ticket not found"})),
        ))
    }
}
