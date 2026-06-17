use crate::{
    db,
    errors::MyError,
    middleware::{self, jwt_auth::AuthUser},
    models::projects::*,
};
use poem::web::{Data, Json, Path};
use sqlx::{Pool, Postgres};

/// Create a new project (admin/teacher only)
#[poem::handler]
pub async fn create_project(
    Data(pool): Data<&Pool<Postgres>>,
    Json(data): Json<CreateProject>,
    auth_user: AuthUser,
) -> Result<Json<Project>, MyError> {
    let role = &auth_user.0.user_role;
    let user_id = auth_user.0.user_id;

    if role != "admin" && role != "teacher" {
        return Err(MyError::Unauthorized);
    }

    if role == "teacher" {
        let is_assigned = db::user_courses::check_user_course(pool, user_id, data.course_id).await?;
        if !is_assigned {
            return Err(MyError::Unauthorized);
        }
    }

    let project = db::projects::create_project(pool, data).await?;
    Ok(Json(project))
}

/// Get a project by ID
#[poem::handler]
pub async fn get_project(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    _auth_user: AuthUser,
) -> Result<Json<Project>, MyError> {
    let project = db::projects::get_project_by_id(pool, id).await?;
    Ok(Json(project))
}

/// Get all projects
#[poem::handler]
pub async fn get_all_projects(
    Data(pool): Data<&Pool<Postgres>>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<Project>>, MyError> {
    let projects = db::projects::get_all_projects(pool).await?;
    Ok(Json(projects))
}

/// Get projects by course ID
#[poem::handler]
pub async fn get_projects_by_course(
    Data(pool): Data<&Pool<Postgres>>,
    Path(course_id): Path<i32>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<Project>>, MyError> {
    let projects = db::projects::get_projects_by_course_id(pool, course_id).await?;
    Ok(Json(projects))
}

/// Update a project (admin/teacher only)
#[poem::handler]
pub async fn update_project(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(data): Json<UpdateProject>,
    auth_user: AuthUser,
) -> Result<Json<Project>, MyError> {
    let role = &auth_user.0.user_role;
    let user_id = auth_user.0.user_id;

    if role != "admin" && role != "teacher" {
        return Err(MyError::Unauthorized);
    }

    let project = db::projects::get_project_by_id(pool, id).await?;

    if role == "teacher" {
        let is_assigned = db::user_courses::check_user_course(pool, user_id, project.course_id).await?;
        if !is_assigned {
            return Err(MyError::Unauthorized);
        }

        if let Some(new_course_id) = data.course_id {
            if new_course_id != project.course_id {
                let is_assigned_new = db::user_courses::check_user_course(pool, user_id, new_course_id).await?;
                if !is_assigned_new {
                    return Err(MyError::Unauthorized);
                }
            }
        }
    }

    let project = db::projects::update_project(pool, id, data).await?;
    Ok(Json(project))
}

/// Delete a project (admin/teacher only)
#[poem::handler]
pub async fn delete_project(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    auth_user: AuthUser,
) -> Result<Json<Project>, MyError> {
    let role = &auth_user.0.user_role;
    let user_id = auth_user.0.user_id;

    if role != "admin" && role != "teacher" {
        return Err(MyError::Unauthorized);
    }

    let project = db::projects::get_project_by_id(pool, id).await?;

    if role == "teacher" {
        let is_assigned = db::user_courses::check_user_course(pool, user_id, project.course_id).await?;
        if !is_assigned {
            return Err(MyError::Unauthorized);
        }
    }

    let project = db::projects::delete_project(pool, id).await?;
    Ok(Json(project))
}

/// Get all projects for a student (where they are enrolled in the course)
#[poem::handler]
pub async fn get_student_projects(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    auth_user: AuthUser,
) -> Result<Json<Vec<Project>>, MyError> {
    // RBAC:
    // - Students can look up their own projects
    // - Teachers/Admins can look up any student's projects (or at least see the endpoint)
    let role = &auth_user.0.user_role;
    let auth_id = auth_user.0.user_id;

    if role == "student" && auth_id != id {
        return Err(MyError::Unauthorized);
    }

    let projects = db::projects::get_student_projects(pool, id).await?;
    Ok(Json(projects))
}
