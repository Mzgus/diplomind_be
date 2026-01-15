use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a complete user sheet record from the database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserSheet {
    pub id: i32,
    pub last_name: String,
    pub first_name: String,
    pub type_user: String,
    pub profile_picture: Option<String>,
}

/// Data required to create a new user sheet
#[derive(Debug, Deserialize)]
pub struct CreateUserSheet {
    pub last_name: String,
    pub first_name: String,
    pub type_user: String,
    pub profile_picture: Option<String>,
}

/// Data for updating an existing user sheet (all fields optional)
#[derive(Debug, Deserialize)]
pub struct UpdateUserSheet {
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub type_user: Option<String>,
    pub profile_picture: Option<String>,
}
