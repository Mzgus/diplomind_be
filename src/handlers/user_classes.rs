use crate::{
    db,
    errors::MyError,
    middleware::{self, jwt_auth::AuthUser},
    models::user_classes::*,
};
use poem::web::{Data, Json, Path};
use sqlx::{Pool, Postgres};

/// Assign a user to a class (admin only)
#[poem::handler]
pub async fn assign_user_to_class(
    Data(pool): Data<&Pool<Postgres>>,
    Json(data): Json<AssignUserToClass>,
    auth_user: AuthUser,
) -> Result<Json<UserClass>, MyError> {
    middleware::rbac::require_admin(&auth_user.0)?;
    let user_class = db::user_classes::assign_user_to_class(pool, data).await?;
    Ok(Json(user_class))
}

/// Get all classes for a user
#[poem::handler]
pub async fn get_user_classes(
    Data(pool): Data<&Pool<Postgres>>,
    Path(user_id): Path<i32>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<crate::models::classes::Class>>, MyError> {
    let classes = db::user_classes::get_user_classes(pool, user_id).await?;
    Ok(Json(classes))
}

/// Get all users in a class
#[poem::handler]
pub async fn get_class_users(
    Data(pool): Data<&Pool<Postgres>>,
    Path(class_id): Path<i32>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<crate::models::users_sheets::UserSheet>>, MyError> {
    let users = db::user_classes::get_class_users(pool, class_id).await?;
    Ok(Json(users))
}

/// Remove a user from a class (admin only)
#[poem::handler]
pub async fn remove_user_from_class(
    Data(pool): Data<&Pool<Postgres>>,
    Path((user_id, class_id)): Path<(i32, i32)>,
    auth_user: AuthUser,
) -> Result<Json<UserClass>, MyError> {
    middleware::rbac::require_admin(&auth_user.0)?;
    let user_class = db::user_classes::remove_user_from_class(pool, user_id, class_id).await?;
    Ok(Json(user_class))
}
