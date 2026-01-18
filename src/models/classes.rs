use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a class from the database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Class {
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Data required to create a new class
#[derive(Debug, Deserialize)]
pub struct CreateClass {
    pub name: String,
}

/// Data for updating a class
#[derive(Debug, Deserialize)]
pub struct UpdateClass {
    pub name: Option<String>,
}
