use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a skill (compétence) from the database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Skill {
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Data required to create a new skill
#[derive(Debug, Deserialize)]
pub struct CreateSkill {
    pub name: String,
    pub description: Option<String>,
}

/// Data for updating a skill
#[derive(Debug, Deserialize)]
pub struct UpdateSkill {
    pub name: Option<String>,
    pub description: Option<String>,
}
