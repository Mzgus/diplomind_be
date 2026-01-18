use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a step-skill association from the database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct StepSkill {
    pub step_id: i32,
    pub skill_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Data required to link a skill to a step
#[derive(Debug, Deserialize)]
pub struct LinkSkillToStep {
    pub step_id: i32,
    pub skill_id: i32,
}
