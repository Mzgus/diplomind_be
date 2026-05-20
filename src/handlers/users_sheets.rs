use crate::{
    db,
    errors::MyError,
    middleware::{self, jwt_auth::AuthUser},
    models::*,
};
use poem::web::{Data, Json, Path};
use sqlx::{Pool, Postgres};

/// Create a new user sheet
#[poem::handler]
pub async fn create_user_sheet(
    Data(pool): Data<&Pool<Postgres>>,
    Json(data): Json<CreateUserSheet>,
    auth_user: AuthUser,
) -> Result<Json<UserSheet>, MyError> {
    middleware::rbac::require_admin(&auth_user.0)?;
    let user_sheet = db::users_sheets::create_user_sheet(pool, data).await?;
    Ok(Json(user_sheet))
}

/// Get a user sheet by ID
#[poem::handler]
pub async fn get_user_sheet(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    auth_user: AuthUser,
) -> Result<Json<UserSheet>, MyError> {
    middleware::rbac::require_admin_or_self(&auth_user.0, id)?;
    let user_sheet = db::users_sheets::get_user_sheet_by_id(pool, id).await?;
    Ok(Json(user_sheet))
}

/// Get all user sheets (admin and teacher: read-only)
#[poem::handler]
pub async fn get_all_user_sheets(
    Data(pool): Data<&Pool<Postgres>>,
    auth_user: AuthUser,
) -> Result<Json<Vec<UserSheet>>, MyError> {
    middleware::rbac::require_admin_or_teacher(&auth_user.0)?;
    let user_sheets = db::users_sheets::get_all_user_sheets(pool).await?;
    Ok(Json(user_sheets))
}

/// Update a user sheet (fields only — no account linking)
#[poem::handler]
pub async fn update_user_sheet(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(data): Json<UpdateUserSheet>,
    auth_user: AuthUser,
) -> Result<Json<UserSheet>, MyError> {
    middleware::rbac::require_admin(&auth_user.0)?;
    let user_sheet = db::users_sheets::update_user_sheet(pool, id, data).await?;
    Ok(Json(user_sheet))
}

/// Delete a user sheet
#[poem::handler]
pub async fn delete_user_sheet(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    auth_user: AuthUser,
) -> Result<Json<UserSheet>, MyError> {
    middleware::rbac::require_admin(&auth_user.0)?;
    let user_sheet = db::users_sheets::delete_user_sheet(pool, id).await?;
    Ok(Json(user_sheet))
}

/// Link a sheet to an account (POST /users_sheets/:id/account)
/// Body: { "account_id": <i32> }
#[poem::handler]
pub async fn link_sheet_to_account(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(body): Json<LinkSheetBody>,
    auth_user: AuthUser,
) -> Result<Json<()>, MyError> {
    middleware::rbac::require_admin(&auth_user.0)?;
    db::users_sheets::link_account_to_sheet(pool, body.account_id, id).await?;
    Ok(Json(()))
}

/// Dissociate a user sheet from its account (DELETE /users_sheets/:id/account)
#[poem::handler]
pub async fn unlink_account_from_sheet(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    auth_user: AuthUser,
) -> Result<Json<()>, MyError> {
    middleware::rbac::require_admin(&auth_user.0)?;
    db::users_sheets::unlink_account_from_sheet(pool, id).await?;
    Ok(Json(()))
}
