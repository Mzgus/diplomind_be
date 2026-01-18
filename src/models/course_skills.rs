use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a course-skill association from the database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CourseSkill {
    pub course_id: i32,
    pub skill_id: i32,
}

/// Data required to link a skill to a course
#[derive(Debug, Deserialize)]
pub struct LinkSkillToCourse {
    pub course_id: i32,
    pub skill_id: i32,
}
