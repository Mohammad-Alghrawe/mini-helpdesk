use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Extension, Json, Router,
};

use crate::{
    auth::jwt::Claims,
    models::ticket::{CreateTicketRequest, Ticket, UpdateTicketRequest},
    repositories::ticket_repository,
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_ticket).get(list_tickets))
        .route(
            "/:id",
            get(get_ticket_by_id)
                .patch(update_ticket)
                .delete(delete_ticket),
        )
}

async fn create_ticket(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateTicketRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    if payload.title.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "title is required" })),
        ));
    }

    let created = ticket_repository::create_ticket(&state.db, claims.sub, payload)
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
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Ticket>>, StatusCode> {
    let tickets = ticket_repository::list_tickets(&state.db, claims.sub)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(tickets))
}

async fn get_ticket_by_id(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let ticket = ticket_repository::get_ticket_by_id(&state.db, claims.sub, &id)
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

async fn update_ticket(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTicketRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if payload.status.is_none()
        && payload.priority.is_none()
        && payload.title.is_none()
        && payload.description.is_none()
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "provide at least one field" })),
        ));
    }

    let updated = ticket_repository::update_ticket(&state.db, claims.sub, &id, payload)
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

async fn delete_ticket(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let deleted = ticket_repository::delete_ticket(&state.db, claims.sub, &id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "db error", "details": e.to_string()})),
            )
        })?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "ticket not found"})),
        ))
    }
}
