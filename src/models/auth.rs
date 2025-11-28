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
pub struct UserInfos {
    pub user_id: i32,
    pub user_lastname: String,
    pub user_firstname: String,
    pub user_role: String,
    pub user_profilepicture: String,
    pub user_email: String,
}

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct Claims {
    pub user_id: i32,
    pub user_lastname: String,
    pub user_firstname: String,
    pub user_role: String,
    pub user_profilepicture: String,
    pub user_email: String,
}