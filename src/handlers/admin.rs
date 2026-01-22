use crate::{
    db,
    errors::MyError,
    middleware::{self, jwt_auth::AuthUser},
    models::*,
};
use chrono::Utc;
use poem::web::{Data, Path};
use sqlx::{Pool, Postgres};

/// Deactivate a user account (admin only)
#[poem::handler]
pub async fn deactivate_user(
    Data(pool): Data<&Pool<Postgres>>,
    Path(user_id): Path<i32>,
    auth_user: AuthUser,
) -> Result<poem::web::Json<UserSheet>, MyError> {
    // Only admin can deactivate users
    middleware::rbac::require_admin(&auth_user.0)?;

    // Prevent self-deactivation
    if auth_user.0.user_id == user_id {
        return Err(MyError::InvalidInput {
            input_type: "cannot deactivate your own account",
        });
    }

    // Set active status to false
    let user_sheet = db::users_sheets::set_user_active_status(pool, user_id, false).await?;

    // Revoke all user's refresh tokens
    let _ = db::auth::revoke_user_refresh_tokens(pool, user_id).await;

    println!(
        "ADMIN ACTION: User {} deactivated by admin {}",
        user_id, auth_user.0.user_email
    );

    Ok(poem::web::Json(user_sheet))
}

/// Activate a user account (admin only)
#[poem::handler]
pub async fn activate_user(
    Data(pool): Data<&Pool<Postgres>>,
    Path(user_id): Path<i32>,
    auth_user: AuthUser,
) -> Result<poem::web::Json<UserSheet>, MyError> {
    // Only admin can activate users
    middleware::rbac::require_admin(&auth_user.0)?;

    // Set active status to true
    let user_sheet = db::users_sheets::set_user_active_status(pool, user_id, true).await?;

    println!(
        "ADMIN ACTION: User {} activated by admin {}",
        user_id, auth_user.0.user_email
    );

    Ok(poem::web::Json(user_sheet))
}

/// Revoke all refresh tokens (emergency function for security incidents)
#[poem::handler]
pub async fn revoke_all_sessions(
    Data(pool): Data<&Pool<Postgres>>,
    auth_user: AuthUser,
) -> Result<poem::web::Json<RevocationResponse>, MyError> {
    // Only admin can perform mass revocation
    middleware::rbac::require_admin(&auth_user.0)?;

    // Revoke all refresh tokens
    let tokens_revoked = db::auth::revoke_all_refresh_tokens(pool).await?;

    println!(
        "SECURITY: All {} sessions revoked by admin {}",
        tokens_revoked, auth_user.0.user_email
    );

    Ok(poem::web::Json(RevocationResponse {
        success: true,
        tokens_revoked,
        revoked_at: Utc::now(),
    }))
}
