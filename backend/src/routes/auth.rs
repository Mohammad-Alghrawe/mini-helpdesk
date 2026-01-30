use crate::state::AppState;
use axum::{extract::State, http::StatusCode, Json};

use crate::{
    auth::{jwt::generate_jwt, password::verify_password},
    models::user::{LoginRequest, User},
};

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
