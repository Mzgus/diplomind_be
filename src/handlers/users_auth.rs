use crate::{db, errors::MyError, middleware::jwt_auth::AuthUser, models::*};
use poem::web::{Data, Json, Path};
use sqlx::{Pool, Postgres};

/// Create a new user auth record (admin only - TODO: add role check)
#[poem::handler]
pub async fn create_user_auth(
    Data(pool): Data<&Pool<Postgres>>,
    Json(data): Json<CreateUserAuth>,
    _auth_user: AuthUser, // Requires authentication
) -> Result<Json<UserAuthRecord>, MyError> {
    // TODO: Check if user has admin role
    let user_auth = db::users_auth::create_user_auth(pool, data).await?;
    Ok(Json(user_auth))
}

/// Get a user auth record by ID (admin only - TODO: add role check)
#[poem::handler]
pub async fn get_user_auth(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    _auth_user: AuthUser, // Requires authentication
) -> Result<Json<UserAuthRecord>, MyError> {
    // TODO: Check if user has admin role
    let user_auth = db::users_auth::get_user_auth_by_id(pool, id).await?;
    Ok(Json(user_auth))
}

/// Update user auth email
#[poem::handler]
pub async fn update_user_auth_email(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(data): Json<UpdateUserAuthEmail>,
    auth_user: AuthUser, // Requires authentication
) -> Result<Json<UserAuthRecord>, MyError> {
    // Users can only update their own email unless they're admin
    // TODO: Add proper authorization check
    if auth_user.0.user_id != id && auth_user.0.user_role != "admin" {
        return Err(MyError::Unauthorized);
    }
    
    let user_auth = db::users_auth::update_user_auth_email(pool, id, data).await?;
    Ok(Json(user_auth))
}

/// Update user auth password
#[poem::handler]
pub async fn update_user_auth_password(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(data): Json<UpdateUserAuthPassword>,
    auth_user: AuthUser, // Requires authentication
) -> Result<Json<UserAuthRecord>, MyError> {
    // Users can only update their own password unless they're admin
    // TODO: Add proper authorization check and password hashing
    if auth_user.0.user_id != id && auth_user.0.user_role != "admin" {
        return Err(MyError::Unauthorized);
    }
    
    let user_auth = db::users_auth::update_user_auth_password(pool, id, data).await?;
    Ok(Json(user_auth))
}

/// Delete a user auth record (admin only - TODO: add role check)
#[poem::handler]
pub async fn delete_user_auth(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    _auth_user: AuthUser, // Requires authentication
) -> Result<Json<UserAuthRecord>, MyError> {
    // TODO: Check if user has admin role
    let user_auth = db::users_auth::delete_user_auth(pool, id).await?;
    Ok(Json(user_auth))
}
