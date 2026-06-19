use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a complete user auth record from the database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserAuthRecord {
    pub id: i32,
    pub email: String,
    pub pwd: String,
    pub account_id: i32,
}

/// Data required to create a new user auth record
#[derive(Debug, Deserialize)]
pub struct CreateUserAuth {
    pub email: String,
    pub pwd: String,
    pub account_id: Option<i32>,
}

/// Data for updating user auth email
#[derive(Debug, Deserialize)]
pub struct UpdateUserAuthEmail {
    pub email: String,
}

/// Data for updating user auth password
#[derive(Debug, Deserialize)]
pub struct UpdateUserAuthPassword {
    pub pwd: String,
}
