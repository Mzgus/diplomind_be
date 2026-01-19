use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SkillValidation {
    pub user_id: i32,
    pub skill_id: i32,
    pub status: String,
    pub comment: Option<String>,
    pub validated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub validated_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateValidationRequest {
    pub user_id: i32,
    pub skill_id: i32,
    pub status: String,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateValidationStatus {
    pub status: String,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StatusFilter {
    pub status: Option<String>,
}
