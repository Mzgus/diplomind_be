use crate::{db, errors::MyError, middleware::{self, jwt_auth::AuthUser}, models::skills::*};
use poem::web::{Data, Json, Path};
use sqlx::{Pool, Postgres};

/// Create a new skill (admin/teacher only)
#[poem::handler]
pub async fn create_skill(
    Data(pool): Data<&Pool<Postgres>>,
    Json(data): Json<CreateSkill>,
    auth_user: AuthUser,
) -> Result<Json<Skill>, MyError> {
    // Admin or teacher can create skills
    if auth_user.0.user_role != "admin" && auth_user.0.user_role != "teacher" {
        return Err(MyError::Unauthorized);
    }
    
    let skill = db::skills::create_skill(pool, data).await?;
    Ok(Json(skill))
}

/// Get a skill by ID
#[poem::handler]
pub async fn get_skill(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    _auth_user: AuthUser,
) -> Result<Json<Skill>, MyError> {
    let skill = db::skills::get_skill_by_id(pool, id).await?;
    Ok(Json(skill))
}

/// Get all skills
#[poem::handler]
pub async fn get_all_skills(
    Data(pool): Data<&Pool<Postgres>>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<Skill>>, MyError> {
    let skills = db::skills::get_all_skills(pool).await?;
    Ok(Json(skills))
}

/// Update a skill (admin/teacher only)
#[poem::handler]
pub async fn update_skill(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(data): Json<UpdateSkill>,
    auth_user: AuthUser,
) -> Result<Json<Skill>, MyError> {
    if auth_user.0.user_role != "admin" && auth_user.0.user_role != "teacher" {
        return Err(MyError::Unauthorized);
    }
    
    let skill = db::skills::update_skill(pool, id, data).await?;
    Ok(Json(skill))
}

/// Delete a skill (admin only)
#[poem::handler]
pub async fn delete_skill(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    auth_user: AuthUser,
) -> Result<Json<Skill>, MyError> {
    middleware::rbac::require_admin(&auth_user.0)?;
    
    let skill = db::skills::delete_skill(pool, id).await?;
    Ok(Json(skill))
}
