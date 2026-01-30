#[derive(Debug, serde::Serialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub role: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
