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
    auth_user: AuthUser, // Requires authentication
) -> Result<Json<UserSheet>, MyError> {
    // Only admin can create user sheets (or maybe anyone if public registration? typically self or admin)
    // For now, let's restrict creation to admin or maybe allow it for registration flow?
    // Assuming admin creates users or users register themselves. If users register themselves,
    // the endpoint might need to be public or we assume they are already auth?
    // Let's assume for now admin only for explicit creation via this API.
    middleware::rbac::require_admin(&auth_user.0)?;

    let user_sheet = db::users_sheets::create_user_sheet(pool, data).await?;
    Ok(Json(user_sheet))
}

/// Get a user sheet by ID
#[poem::handler]
pub async fn get_user_sheet(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    auth_user: AuthUser, // Requires authentication
) -> Result<Json<UserSheet>, MyError> {
    // Admin or self can view
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

/// Update a user sheet
#[poem::handler]
pub async fn update_user_sheet(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(data): Json<UpdateUserSheet>,
    auth_user: AuthUser, // Requires authentication
) -> Result<Json<UserSheet>, MyError> {
    println!("Update request for ID {}: {:?}", id, data); // Debug logging
    // Admin or self can update
    middleware::rbac::can_modify_user(&auth_user.0, id)?;

    // Extract account_id before the update (since data will be moved)
    let account_id_to_link = data.account_id;

    let user_sheet = db::users_sheets::update_user_sheet(pool, id, data).await?;

    // If account_id was provided, link the sheet to the account
    if let Some(acc_id) = account_id_to_link {
        db::users_sheets::link_account_to_sheet(pool, acc_id, id).await?;
    }

    Ok(Json(user_sheet))
}

/// Delete a user sheet
#[poem::handler]
pub async fn delete_user_sheet(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    auth_user: AuthUser, // Requires authentication
) -> Result<Json<UserSheet>, MyError> {
    // Only admin can delete
    middleware::rbac::require_admin(&auth_user.0)?;

    let user_sheet = db::users_sheets::delete_user_sheet(pool, id).await?;
    Ok(Json(user_sheet))
}
