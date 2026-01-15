use crate::{db, errors::MyError, middleware::jwt_auth::AuthUser, models::User};
use poem::web::{Data, Path, Json};
use sqlx::{Pool, Postgres};

/// Get all users (complete information with JOIN)
#[poem::handler]
pub async fn get_all_users(
    Data(pool): Data<&Pool<Postgres>>,
    _auth_user: AuthUser, // Requires authentication
) -> Result<Json<Vec<User>>, MyError> {
    let users = db::users::get_all_users(pool).await?;
    Ok(Json(users))
}

/// Get a user by ID (complete information with JOIN)
#[poem::handler]
pub async fn get_user_by_id(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    _auth_user: AuthUser, // Requires authentication
) -> Result<Json<User>, MyError> {
    let user = db::users::get_user_by_id(pool, id).await?;
    Ok(Json(user))
}

/// Get a user by email (complete information with JOIN)
#[poem::handler]
pub async fn get_user_by_email(
    Data(pool): Data<&Pool<Postgres>>,
    Path(email): Path<String>,
    _auth_user: AuthUser, // Requires authentication
) -> Result<Json<User>, MyError> {
    let user = db::users::get_user_by_email(pool, &email).await?;
    Ok(Json(user))
}
