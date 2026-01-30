use crate::state::AppState;
use axum::{extract::State, http::StatusCode, Json};

use crate::{
    auth::{jwt::generate_jwt, password::verify_password},
    models::user::{LoginRequest, User},
};

use crate::auth::password::hash_password;
use crate::models::user::RegisterRequest;

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}
#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let record = sqlx::query!(
        r#"
    SELECT id as "id!: i64", email, password_hash, role
    FROM users
    WHERE email = ?
    "#,
        payload.email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let record = match record {
        Some(user) => user,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let valid = verify_password(&payload.password, &record.password_hash)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let jwt_secret = std::env::var("JWT_SECRET").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let token = generate_jwt(record.id, &record.email, &record.role, &jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = User {
        id: record.id,
        email: record.email,
        role: record.role,
    };

    Ok(Json(LoginResponse { token, user }))
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    let email = payload.email.trim().to_lowercase();

    if email.is_empty() || !email.contains('@') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "invalid email" })),
        ));
    }

    if payload.password.len() < 6 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "password must be at least 6 chars" })),
        ));
    }

    let password_hash = hash_password(&payload.password).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "hash error", "details": e.to_string() })),
        )
    })?;

    let res = sqlx::query!(
        r#"
        INSERT INTO users (email, password_hash, role)
        VALUES (?, ?, 'user')
        "#,
        email,
        password_hash
    )
    .execute(&state.db)
    .await;

    match res {
        Ok(_) => Ok((
            StatusCode::CREATED,
            Json(serde_json::json!({ "email": email })),
        )),
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("UNIQUE") || msg.contains("constraint") {
                return Err((
                    StatusCode::CONFLICT,
                    Json(serde_json::json!({ "error": "email already exists" })),
                ));
            }
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "db error", "details": msg })),
            ))
        }
    }
}
