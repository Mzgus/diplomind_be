use crate::{
    db,
    errors::MyError,
    middleware::{self, jwt_auth::AuthUser},
    models::*,
};
use poem::web::{Data, Json, Path};
use sqlx::{Pool, Postgres};

/// Create a new user auth record (admin only)
#[poem::handler]
pub async fn create_user_auth(
    Data(pool): Data<&Pool<Postgres>>,
    Json(mut data): Json<CreateUserAuth>,
    auth_user: AuthUser, // Requires authentication
) -> Result<Json<UserAuthRecord>, MyError> {
    middleware::rbac::require_admin(&auth_user.0)?;

    // Validate input
    crate::validators::validate_email(&data.email)?;
    crate::validators::validate_password(&data.pwd)?;

    // Hash the password before storing
    use argon2::{
        Argon2,
        password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
    };

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(data.pwd.as_bytes(), &salt)
        .map_err(|e| MyError::PasswordHashError(e.to_string()))?
        .to_string();

    // Replace plain password with hashed version
    data.pwd = password_hash;

    let user_auth = db::users_auth::create_user_auth(pool, data).await?;
    Ok(Json(user_auth))
}

/// Get a user auth record by ID (admin only)
#[poem::handler]
pub async fn get_user_auth(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    auth_user: AuthUser, // Requires authentication
) -> Result<Json<UserAuthRecord>, MyError> {
    // Auth records (email/pwd) are sensitive, maybe only admin or self?
    // Let's restrict to admin or self
    middleware::rbac::require_admin_or_self(&auth_user.0, id)?;

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
    middleware::rbac::can_modify_user(&auth_user.0, id)?;

    // Validate input
    crate::validators::validate_email(&data.email)?;

    let user_auth = db::users_auth::update_user_auth_email(pool, id, data).await?;
    Ok(Json(user_auth))
}

/// Update user auth password
#[poem::handler]
pub async fn update_user_auth_password(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(mut data): Json<UpdateUserAuthPassword>,
    auth_user: AuthUser, // Requires authentication
) -> Result<Json<UserAuthRecord>, MyError> {
    middleware::rbac::can_modify_user(&auth_user.0, id)?;

    // Validate input
    crate::validators::validate_password(&data.pwd)?;

    // Hash the password before updating
    use argon2::{
        Argon2,
        password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
    };

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(data.pwd.as_bytes(), &salt)
        .map_err(|e| MyError::PasswordHashError(e.to_string()))?
        .to_string();

    // Replace plain password with hashed version
    data.pwd = password_hash;

    let user_auth = db::users_auth::update_user_auth_password(pool, id, data).await?;
    Ok(Json(user_auth))
}

/// Delete a user auth record (admin only)
#[poem::handler]
pub async fn delete_user_auth(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    auth_user: AuthUser, // Requires authentication
) -> Result<Json<UserAuthRecord>, MyError> {
    middleware::rbac::require_admin(&auth_user.0)?;

    let user_auth = db::users_auth::delete_user_auth(pool, id).await?;
    Ok(Json(user_auth))
}
