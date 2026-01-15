use crate::{db, errors::MyError, middleware::jwt_auth::AuthUser, models::*};
use poem::web::{Data, Json, Path};
use sqlx::{Pool, Postgres};

/// Create a new user sheet
#[poem::handler]
pub async fn create_user_sheet(
    Data(pool): Data<&Pool<Postgres>>,
    Json(data): Json<CreateUserSheet>,
    _auth_user: AuthUser, // Requires authentication
) -> Result<Json<UserSheet>, MyError> {
    let user_sheet = db::users_sheets::create_user_sheet(pool, data).await?;
    Ok(Json(user_sheet))
}

/// Get a user sheet by ID
#[poem::handler]
pub async fn get_user_sheet(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    _auth_user: AuthUser, // Requires authentication
) -> Result<Json<UserSheet>, MyError> {
    let user_sheet = db::users_sheets::get_user_sheet_by_id(pool, id).await?;
    Ok(Json(user_sheet))
}

/// Get all user sheets
#[poem::handler]
pub async fn get_all_user_sheets(
    Data(pool): Data<&Pool<Postgres>>,
    _auth_user: AuthUser, // Requires authentication
) -> Result<Json<Vec<UserSheet>>, MyError> {
    let user_sheets = db::users_sheets::get_all_user_sheets(pool).await?;
    Ok(Json(user_sheets))
}

/// Update a user sheet
#[poem::handler]
pub async fn update_user_sheet(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(data): Json<UpdateUserSheet>,
    _auth_user: AuthUser, // Requires authentication
) -> Result<Json<UserSheet>, MyError> {
    let user_sheet = db::users_sheets::update_user_sheet(pool, id, data).await?;
    Ok(Json(user_sheet))
}

/// Delete a user sheet
#[poem::handler]
pub async fn delete_user_sheet(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    _auth_user: AuthUser, // Requires authentication
) -> Result<Json<UserSheet>, MyError> {
    let user_sheet = db::users_sheets::delete_user_sheet(pool, id).await?;
    Ok(Json(user_sheet))
}
