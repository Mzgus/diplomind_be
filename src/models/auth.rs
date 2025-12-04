use sqlx::types::chrono;


#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct LoginInfo {
    pub email: String,
    pub pwd: String,
}

// #[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
// pub struct LoginResponse {
//     pub token: String,
// }

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct User {
    pub user_id: i32,
    pub user_lastname: String,
    pub user_firstname: String,
    pub user_role: String,
    pub user_profilepicture: String,
    pub user_email: String,
}

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct JWTClaims {
    #[serde(flatten)]
    #[sqlx(flatten)]
    pub user: User,
    pub exp: usize,
}

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct RefreshToken {
    pub token: String,
    pub id_user_auth: i32,
    pub expiration_date: chrono::DateTime<chrono::Utc>,
}

