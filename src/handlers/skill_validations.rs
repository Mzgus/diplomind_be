use crate::{db, errors::MyError, middleware::jwt_auth::AuthUser, models::skill_validations::*};
use poem::web::{Data, Json, Path, Query};
use sqlx::{Pool, Postgres};

/// Create a new skill validation (teacher/admin only)
#[poem::handler]
pub async fn create_validation(
    Data(pool): Data<&Pool<Postgres>>,
    Json(data): Json<CreateValidationRequest>,
    auth_user: AuthUser,
) -> Result<Json<SkillValidation>, MyError> {
    // RBAC: Only teacher or admin can create validations
    if auth_user.0.user_role != "teacher" && auth_user.0.user_role != "admin" {
        return Err(MyError::Unauthorized);
    }

    let validation = db::skill_validations::create_validation(pool, data).await?;
    Ok(Json(validation))
}

/// Get all validations for a specific user
#[poem::handler]
pub async fn get_user_validations(
    Data(pool): Data<&Pool<Postgres>>,
    Path(user_id): Path<i32>,
    Query(filter): Query<StatusFilter>,
    auth_user: AuthUser,
) -> Result<Json<Vec<SkillValidation>>, MyError> {
    // RBAC: Students can only view their own validations
    if auth_user.0.user_role == "student" && auth_user.0.user_id != user_id {
        return Err(MyError::Unauthorized);
    }

    let validations = db::skill_validations::get_user_validations(pool, user_id, Some(filter)).await?;
    Ok(Json(validations))
}

/// Get all pending validations (teacher/admin only)
#[poem::handler]
pub async fn get_pending_validations(
    Data(pool): Data<&Pool<Postgres>>,
    auth_user: AuthUser,
) -> Result<Json<Vec<SkillValidation>>, MyError> {
    // RBAC: Only teacher or admin can view pending validations
    if auth_user.0.user_role != "teacher" && auth_user.0.user_role != "admin" {
        return Err(MyError::Unauthorized);
    }

    let validations = db::skill_validations::get_pending_validations(pool).await?;
    Ok(Json(validations))
}

/// Update validation status (teacher/admin only)
#[poem::handler]
pub async fn update_validation_status(
    Data(pool): Data<&Pool<Postgres>>,
    Path((user_id, skill_id)): Path<(i32, i32)>,
    Json(data): Json<UpdateValidationStatus>,
    auth_user: AuthUser,
) -> Result<Json<SkillValidation>, MyError> {
    // RBAC: Only teacher or admin can update validation status
    if auth_user.0.user_role != "teacher" && auth_user.0.user_role != "admin" {
        return Err(MyError::Unauthorized);
    }

    let validation =
        db::skill_validations::update_validation_status(pool, user_id, skill_id, data, auth_user.0.user_id)
            .await?;
    Ok(Json(validation))
}

/// Get a specific validation
#[poem::handler]
pub async fn get_validation(
    Data(pool): Data<&Pool<Postgres>>,
    Path((user_id, skill_id)): Path<(i32, i32)>,
    auth_user: AuthUser,
) -> Result<Json<SkillValidation>, MyError> {
    // RBAC: Students can only view their own validations
    if auth_user.0.user_role == "student" && auth_user.0.user_id != user_id {
        return Err(MyError::Unauthorized);
    }

    let validation = db::skill_validations::get_validation(pool, user_id, skill_id).await?;
    Ok(Json(validation))
}

/// Delete a validation (admin only)
#[poem::handler]
pub async fn delete_validation(
    Data(pool): Data<&Pool<Postgres>>,
    Path((user_id, skill_id)): Path<(i32, i32)>,
    auth_user: AuthUser,
) -> Result<Json<SkillValidation>, MyError> {
    // RBAC: Only admin can delete validations
    if auth_user.0.user_role != "admin" {
        return Err(MyError::Unauthorized);
    }

    let validation = db::skill_validations::delete_validation(pool, user_id, skill_id).await?;
    Ok(Json(validation))
}
