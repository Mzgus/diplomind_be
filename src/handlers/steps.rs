use crate::{
    db,
    errors::MyError,
    middleware::jwt_auth::AuthUser,
    models::steps::*,
};
use poem::web::{Data, Json, Path};
use sqlx::{Pool, Postgres};

/// Create a new step (admin/teacher only)
#[poem::handler]
pub async fn create_step(
    Data(pool): Data<&Pool<Postgres>>,
    Json(data): Json<CreateStep>,
    auth_user: AuthUser,
) -> Result<Json<Step>, MyError> {
    let role = &auth_user.0.user_role;
    let user_id = auth_user.0.user_id;

    if role != "admin" && role != "teacher" {
        return Err(MyError::Unauthorized);
    }

    if role == "teacher" {
        let project = db::projects::get_project_by_id(pool, data.project_id).await?;
        let is_assigned = db::user_courses::check_user_course(pool, user_id, project.course_id).await?;
        if !is_assigned {
            return Err(MyError::Unauthorized);
        }
    }

    let step = db::steps::create_step(pool, data).await?;
    Ok(Json(step))
}

/// Get a step by ID
#[poem::handler]
pub async fn get_step(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    _auth_user: AuthUser,
) -> Result<Json<Step>, MyError> {
    let step = db::steps::get_step_by_id(pool, id).await?;
    Ok(Json(step))
}

/// Get all steps
#[poem::handler]
pub async fn get_all_steps(
    Data(pool): Data<&Pool<Postgres>>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<Step>>, MyError> {
    let steps = db::steps::get_all_steps(pool).await?;
    Ok(Json(steps))
}

/// Get steps by project ID
#[poem::handler]
pub async fn get_steps_by_project(
    Data(pool): Data<&Pool<Postgres>>,
    Path(project_id): Path<i32>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<Step>>, MyError> {
    let steps = db::steps::get_steps_by_project_id(pool, project_id).await?;
    Ok(Json(steps))
}

/// Update a step (admin/teacher only)
#[poem::handler]
pub async fn update_step(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(data): Json<UpdateStep>,
    auth_user: AuthUser,
) -> Result<Json<Step>, MyError> {
    let role = &auth_user.0.user_role;
    let user_id = auth_user.0.user_id;

    if role != "admin" && role != "teacher" {
        return Err(MyError::Unauthorized);
    }

    let step = db::steps::get_step_by_id(pool, id).await?;
    let project = db::projects::get_project_by_id(pool, step.project_id).await?;

    if role == "teacher" {
        let is_assigned = db::user_courses::check_user_course(pool, user_id, project.course_id).await?;
        if !is_assigned {
            return Err(MyError::Unauthorized);
        }

        if let Some(new_project_id) = data.project_id {
            if new_project_id != step.project_id {
                let new_project = db::projects::get_project_by_id(pool, new_project_id).await?;
                let is_assigned_new = db::user_courses::check_user_course(pool, user_id, new_project.course_id).await?;
                if !is_assigned_new {
                    return Err(MyError::Unauthorized);
                }
            }
        }
    }

    let step = db::steps::update_step(pool, id, data).await?;
    Ok(Json(step))
}

/// Delete a step (admin/teacher only)
#[poem::handler]
pub async fn delete_step(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    auth_user: AuthUser,
) -> Result<Json<Step>, MyError> {
    let role = &auth_user.0.user_role;
    let user_id = auth_user.0.user_id;

    if role != "admin" && role != "teacher" {
        return Err(MyError::Unauthorized);
    }

    let step = db::steps::get_step_by_id(pool, id).await?;
    let project = db::projects::get_project_by_id(pool, step.project_id).await?;

    if role == "teacher" {
        let is_assigned = db::user_courses::check_user_course(pool, user_id, project.course_id).await?;
        if !is_assigned {
            return Err(MyError::Unauthorized);
        }
    }

    let step = db::steps::delete_step(pool, id).await?;
    Ok(Json(step))
}
