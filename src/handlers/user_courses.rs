use crate::{db, errors::MyError, middleware::jwt_auth::AuthUser, models::user_courses::*};
use poem::web::{Data, Json, Path};
use sqlx::{Pool, Postgres};

/// Assign a user to a course (admin/teacher)
#[poem::handler]
pub async fn assign_user_to_course(
    Data(pool): Data<&Pool<Postgres>>,
    Json(data): Json<AssignUserToCourse>,
    auth_user: AuthUser,
) -> Result<Json<UserCourse>, MyError> {
    if auth_user.0.user_role != "admin" && auth_user.0.user_role != "teacher" {
        return Err(MyError::Unauthorized);
    }
    let user_course = db::user_courses::assign_user_to_course(pool, data).await?;
    Ok(Json(user_course))
}

/// Get all courses for a user
#[poem::handler]
pub async fn get_user_courses(
    Data(pool): Data<&Pool<Postgres>>,
    Path(user_id): Path<i32>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<UserCourse>>, MyError> {
    let user_courses = db::user_courses::get_user_courses(pool, user_id).await?;
    Ok(Json(user_courses))
}

/// Get all users in a course
#[poem::handler]
pub async fn get_course_users(
    Data(pool): Data<&Pool<Postgres>>,
    Path(course_id): Path<i32>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<UserCourse>>, MyError> {
    let course_users = db::user_courses::get_course_users(pool, course_id).await?;
    Ok(Json(course_users))
}

/// Remove a user from a course (admin/teacher)
#[poem::handler]
pub async fn remove_user_from_course(
    Data(pool): Data<&Pool<Postgres>>,
    Path((user_id, course_id)): Path<(i32, i32)>,
    auth_user: AuthUser,
) -> Result<Json<UserCourse>, MyError> {
    if auth_user.0.user_role != "admin" && auth_user.0.user_role != "teacher" {
        return Err(MyError::Unauthorized);
    }
    let user_course = db::user_courses::remove_user_from_course(pool, user_id, course_id).await?;
    Ok(Json(user_course))
}
