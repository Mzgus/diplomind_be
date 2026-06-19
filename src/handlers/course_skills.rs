use crate::{db, errors::MyError, middleware::jwt_auth::AuthUser, models::course_skills::*};
use poem::web::{Data, Json, Path};
use sqlx::{Pool, Postgres};

/// Link a skill to a course (admin/teacher)
#[poem::handler]
pub async fn link_skill_to_course(
    Data(pool): Data<&Pool<Postgres>>,
    Json(data): Json<LinkSkillToCourse>,
    auth_user: AuthUser,
) -> Result<Json<CourseSkill>, MyError> {
    if auth_user.0.user_role != "admin" && auth_user.0.user_role != "teacher" {
        return Err(MyError::Unauthorized);
    }
    let course_skill = db::course_skills::link_skill_to_course(pool, data).await?;
    Ok(Json(course_skill))
}

/// Get all skills for a course
#[poem::handler]
pub async fn get_course_skills(
    Data(pool): Data<&Pool<Postgres>>,
    Path(course_id): Path<i32>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<crate::models::skills::Skill>>, MyError> {
    let skills = db::course_skills::get_course_skills(pool, course_id).await?;
    Ok(Json(skills))
}

/// Get all courses for a skill
#[poem::handler]
pub async fn get_skill_courses(
    Data(pool): Data<&Pool<Postgres>>,
    Path(skill_id): Path<i32>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<crate::models::courses::Course>>, MyError> {
    let courses = db::course_skills::get_skill_courses(pool, skill_id).await?;
    Ok(Json(courses))
}

/// Unlink a skill from a course (admin/teacher)
#[poem::handler]
pub async fn unlink_skill_from_course(
    Data(pool): Data<&Pool<Postgres>>,
    Path((course_id, skill_id)): Path<(i32, i32)>,
    auth_user: AuthUser,
) -> Result<Json<CourseSkill>, MyError> {
    if auth_user.0.user_role != "admin" && auth_user.0.user_role != "teacher" {
        return Err(MyError::Unauthorized);
    }
    let course_skill =
        db::course_skills::unlink_skill_from_course(pool, course_id, skill_id).await?;
    Ok(Json(course_skill))
}
