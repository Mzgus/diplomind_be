use crate::{db, errors::MyError, middleware::jwt_auth::AuthUser, models::step_skills::*};
use poem::web::{Data, Json, Path};
use sqlx::{Pool, Postgres};

/// Link a skill to a step (admin/teacher)
#[poem::handler]
pub async fn link_skill_to_step(
    Data(pool): Data<&Pool<Postgres>>,
    Json(data): Json<LinkSkillToStep>,
    auth_user: AuthUser,
) -> Result<Json<StepSkill>, MyError> {
    if auth_user.0.user_role != "admin" && auth_user.0.user_role != "teacher" {
        return Err(MyError::Unauthorized);
    }
    let step_skill = db::step_skills::link_skill_to_step(pool, data).await?;
    Ok(Json(step_skill))
}

/// Get all skills for a step
#[poem::handler]
pub async fn get_step_skills(
    Data(pool): Data<&Pool<Postgres>>,
    Path(step_id): Path<i32>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<StepSkill>>, MyError> {
    let step_skills = db::step_skills::get_step_skills(pool, step_id).await?;
    Ok(Json(step_skills))
}

/// Get all steps for a skill
#[poem::handler]
pub async fn get_skill_steps(
    Data(pool): Data<&Pool<Postgres>>,
    Path(skill_id): Path<i32>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<StepSkill>>, MyError> {
    let skill_steps = db::step_skills::get_skill_steps(pool, skill_id).await?;
    Ok(Json(skill_steps))
}

/// Unlink a skill from a step (admin/teacher)
#[poem::handler]
pub async fn unlink_skill_from_step(
    Data(pool): Data<&Pool<Postgres>>,
    Path((step_id, skill_id)): Path<(i32, i32)>,
    auth_user: AuthUser,
) -> Result<Json<StepSkill>, MyError> {
    if auth_user.0.user_role != "admin" && auth_user.0.user_role != "teacher" {
        return Err(MyError::Unauthorized);
    }
    let step_skill = db::step_skills::unlink_skill_from_step(pool, step_id, skill_id).await?;
    Ok(Json(step_skill))
}
