use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a course (cours) from the database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Course {
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Data required to create a new course
#[derive(Debug, Deserialize)]
pub struct CreateCourse {
    pub name: String,
    pub description: Option<String>,
}

/// Data for updating a course
#[derive(Debug, Deserialize)]
pub struct UpdateCourse {
    pub name: Option<String>,
    pub description: Option<String>,
}
