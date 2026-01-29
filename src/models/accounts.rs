
#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct Account {
    pub id: i32,
    pub email: String,
}
