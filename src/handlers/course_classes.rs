use crate::{db, errors::MyError, middleware::jwt_auth::AuthUser, models::course_classes::*};
use poem::web::{Data, Json, Path};
use sqlx::{Pool, Postgres};

/// Link a course to a class (admin/teacher)
#[poem::handler]
pub async fn link_course_to_class(
    Data(pool): Data<&Pool<Postgres>>,
    Json(data): Json<LinkCourseToClass>,
    auth_user: AuthUser,
) -> Result<Json<CourseClass>, MyError> {
    if auth_user.0.user_role != "admin" && auth_user.0.user_role != "teacher" {
        return Err(MyError::Unauthorized);
    }
    let course_class = db::course_classes::link_course_to_class(pool, data).await?;
    Ok(Json(course_class))
}

/// Get all classes for a course
#[poem::handler]
pub async fn get_course_classes(
    Data(pool): Data<&Pool<Postgres>>,
    Path(course_id): Path<i32>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<CourseClass>>, MyError> {
    let course_classes = db::course_classes::get_course_classes(pool, course_id).await?;
    Ok(Json(course_classes))
}

/// Get all courses for a class
#[poem::handler]
pub async fn get_class_courses(
    Data(pool): Data<&Pool<Postgres>>,
    Path(class_id): Path<i32>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<CourseClass>>, MyError> {
    let class_courses = db::course_classes::get_class_courses(pool, class_id).await?;
    Ok(Json(class_courses))
}

/// Unlink a course from a class (admin/teacher)
#[poem::handler]
pub async fn unlink_course_from_class(
    Data(pool): Data<&Pool<Postgres>>,
    Path((course_id, class_id)): Path<(i32, i32)>,
    auth_user: AuthUser,
) -> Result<Json<CourseClass>, MyError> {
    if auth_user.0.user_role != "admin" && auth_user.0.user_role != "teacher" {
        return Err(MyError::Unauthorized);
    }
    let course_class =
        db::course_classes::unlink_course_from_class(pool, course_id, class_id).await?;
    Ok(Json(course_class))
}
