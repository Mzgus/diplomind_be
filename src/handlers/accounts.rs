use crate::{
    db,
    errors::MyError,
    middleware::{self, jwt_auth::AuthUser},
    models::accounts::{Account, CreateAccount},
};
use poem::web::{Data, Json};
use sqlx::{Pool, Postgres};

/// Create a new account (Identity)
/// This is usually the first step in creating a user.
#[poem::handler]
pub async fn create_account(
    Data(pool): Data<&Pool<Postgres>>,
    Json(data): Json<CreateAccount>,
    auth_user: AuthUser, // Requires authentication (admin only?)
) -> Result<Json<Account>, MyError> {
    middleware::rbac::require_admin(&auth_user.0)?;

    // Check if account already exists
    if let Ok(_) = db::accounts::get_account_by_email(pool, &data.email).await {
         return Err(MyError::AlreadyExists { entity: "Account" });
    }

    crate::validators::validate_email(&data.email)?;

    let account = db::accounts::create_account(pool, data).await?;
    Ok(Json(account))
}
