use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a user-class association from the database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserClass {
    pub user_id: i32,
    pub class_id: i32,
}

/// Data required to assign a user to a class
#[derive(Debug, Deserialize)]
pub struct AssignUserToClass {
    pub user_id: i32,
    pub class_id: i32,
}
