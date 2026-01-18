use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a user-class association from the database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserClass {
    pub user_id: i32,
    pub class_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrolled_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Data required to assign a user to a class
#[derive(Debug, Deserialize)]
pub struct AssignUserToClass {
    pub user_id: i32,
    pub class_id: i32,
}
