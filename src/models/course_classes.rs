use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a course-class association from the database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CourseClass {
    pub course_id: i32,
    pub class_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Data required to link a course to a class
#[derive(Debug, Deserialize)]
pub struct LinkCourseToClass {
    pub course_id: i32,
    pub class_id: i32,
}
