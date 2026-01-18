use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a user-course association from the database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserCourse {
    pub user_id: i32,
    pub course_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrolled_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Data required to assign a user to a course
#[derive(Debug, Deserialize)]
pub struct AssignUserToCourse {
    pub user_id: i32,
    pub course_id: i32,
}
