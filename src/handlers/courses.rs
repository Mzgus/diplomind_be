use crate::{db, errors::MyError, middleware::{self, jwt_auth::AuthUser}, models::courses::*};
use poem::web::{Data, Json, Path};
use sqlx::{Pool, Postgres};

/// Create a new course (admin/teacher only)
#[poem::handler]
pub async fn create_course(
    Data(pool): Data<&Pool<Postgres>>,
    Json(data): Json<CreateCourse>,
    auth_user: AuthUser,
) -> Result<Json<Course>, MyError> {
    if auth_user.0.user_role != "admin" && auth_user.0.user_role != "teacher" {
        return Err(MyError::Unauthorized);
    }
    
    let course = db::courses::create_course(pool, data).await?;
    Ok(Json(course))
}

/// Get a course by ID
#[poem::handler]
pub async fn get_course(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    _auth_user: AuthUser,
) -> Result<Json<Course>, MyError> {
    let course = db::courses::get_course_by_id(pool, id).await?;
    Ok(Json(course))
}

/// Get all courses
#[poem::handler]
pub async fn get_all_courses(
    Data(pool): Data<&Pool<Postgres>>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<Course>>, MyError> {
    let courses = db::courses::get_all_courses(pool).await?;
    Ok(Json(courses))
}

/// Update a course (admin/teacher only)
#[poem::handler]
pub async fn update_course(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(data): Json<UpdateCourse>,
    auth_user: AuthUser,
) -> Result<Json<Course>, MyError> {
    if auth_user.0.user_role != "admin" && auth_user.0.user_role != "teacher" {
        return Err(MyError::Unauthorized);
    }
    
    let course = db::courses::update_course(pool, id, data).await?;
    Ok(Json(course))
}

/// Delete a course (admin only)
#[poem::handler]
pub async fn delete_course(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    auth_user: AuthUser,
) -> Result<Json<Course>, MyError> {
    middleware::rbac::require_admin(&auth_user.0)?;
    
    let course = db::courses::delete_course(pool, id).await?;
    Ok(Json(course))
}
