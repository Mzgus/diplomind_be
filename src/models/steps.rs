use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a step (étape) from the database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Step {
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub project_id: i32,
    pub step_order: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Data required to create a new step
#[derive(Debug, Deserialize)]
pub struct CreateStep {
    pub name: String,
    pub description: Option<String>,
    pub project_id: i32,
    pub step_order: Option<i32>,
}

/// Data for updating a step
#[derive(Debug, Deserialize)]
pub struct UpdateStep {
    pub name: Option<String>,
    pub description: Option<String>,
    pub project_id: Option<i32>,
    pub step_order: Option<i32>,
}
