use sqlx::types::chrono;

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct UserAuth {
    pub email: String,
    pub pwd: String,
}

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct AccessToken {
    pub token: String,
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct User {
    pub account_id: i32,
    pub user_id: i32, // Represents user_sheet_id
    pub user_lastname: String,
    pub user_firstname: String,
    pub user_role: String,
    pub user_profilepicture: String,
    pub user_email: String,
    // Removed direct pwd mapping as it's not needed in claims usually, but if used elsewhere handle with care.
    // Removed user_active from claims to reduce size, check db if critical.
}

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct JWTClaims {
    #[serde(flatten)]
    // #[sqlx(flatten)] // sqlx flatten might not work if we are not querying this directly as a single row in the same way
    pub user: User,
    pub exp: usize,
}

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct RefreshToken {
    pub token: String,
    pub account_id: i32,
    pub expiration_date: chrono::DateTime<chrono::Utc>,
}
