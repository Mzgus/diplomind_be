use crate::{db, errors, models, services::*};
use poem::web::cookie::CookieJar;
use poem::web::{Data, Json};
use sqlx::*;

#[poem::handler]
pub async fn login(
    Data(executor): Data<&Pool<Postgres>>,
    Data(token_manager): Data<&TokenManager>,
    Json(user_auth): Json<models::UserAuth>,
    cookie_jar: &CookieJar,
) -> Result<Json<models::AccessToken>, errors::MyError> {
    // 1. Fetch Credentials (including Pwd Hash)
    let credentials = match db::auth::get_credentials_by_email(executor, &user_auth.email).await {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    // 2. Verify password using Argon2
    let password_valid = match auth::verify_password(&user_auth.pwd, &credentials.pwd) {
        Ok(valid) => valid,
        Err(err) => return Err(err),
    };

    if !password_valid {
        return Err(errors::MyError::InvalidCredentials);
    }

    // 3. Get Session User (Account + Default Profile)
    let user_info: models::User = match db::auth::login(executor, &user_auth.email).await {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    
    if !user_info.user_active {
        return Err(errors::MyError::Unauthorized);
    }
    
    let (access_token, refresh_token) =
        match token_manager.generate_token_pair(token_manager.clone(), user_info.clone()) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

    token_manager
        .manage_token(executor, cookie_jar, &refresh_token, credentials.account_id)
        .await?;

    Ok(Json(crate::AccessToken {
        token: access_token,
    }))
}

#[poem::handler]
pub async fn refresh_tokens(
    Data(executor): Data<&Pool<Postgres>>,
    Data(token_manager): Data<&TokenManager>,
    cookie_jar: &CookieJar,
) -> Result<Json<models::AccessToken>, errors::MyError> {
    let token = match token_manager.get_cookie_value(cookie_jar) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    let refresh_token: models::RefreshToken =
        match db::auth::get_refresh_token(executor, token.clone()).await {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

    if !token_manager.verify_token_validity(refresh_token.expiration_date) {
        return Err(errors::MyError::TokenExpired);
    }

    let user_info: models::User =
        match db::auth::get_user_info_by_token(executor, token.clone()).await {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

    if !user_info.user_active {
        return Err(errors::MyError::Unauthorized);
    }

    let user_auth_id: i32 = user_info.account_id;

    // Delete the old refresh token (Rotation)
    let _ = db::auth::delete_refresh_token(executor, token.clone()).await;

    let (access_token, refresh_token) =
        match token_manager.generate_token_pair(token_manager.clone(), user_info) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

    token_manager
        .manage_token(executor, cookie_jar, &refresh_token, user_auth_id)
        .await?;

    Ok(Json(crate::AccessToken {
        token: access_token,
    }))
}

//logout
#[poem::handler]
pub async fn logout(
    Data(executor): Data<&Pool<Postgres>>,
    Data(token_manager): Data<&TokenManager>,
    cookie_jar: &CookieJar,
) -> Result<(), errors::MyError> {
    let token = match token_manager.get_cookie_value(cookie_jar) {
        Ok(res) => res,
        Err(_) => return Ok(()),
    };
    // If no cookie, just return ok (already logged out conceptually)
    println!("token: {}", token);
    if !token.is_empty() {
        let _ = db::auth::delete_refresh_token(executor, token).await;
    }

    token_manager.clear_cookie(cookie_jar);
    Ok(())
}

// Verify token endpoint - validates JWT and returns user info
// Now simplified using AuthUser extractor
#[poem::handler]
pub async fn verify_token(
    auth_user: crate::middleware::jwt_auth::AuthUser,
) -> Result<Json<models::User>, errors::MyError> {
    // The middleware already validated the token
    // Just return the authenticated user
    Ok(Json(auth_user.0))
}

/// Get all profiles available for the current account
#[poem::handler]
pub async fn get_my_profiles(
    Data(executor): Data<&Pool<Postgres>>,
    auth_user: crate::middleware::jwt_auth::AuthUser,
) -> Result<Json<Vec<models::UserSheet>>, errors::MyError> {
    let profiles = db::auth::get_account_profiles(executor, auth_user.0.account_id).await?;
    Ok(Json(profiles))
}

#[derive(serde::Deserialize)]
pub struct SwitchProfileRequest {
    pub user_sheet_id: i32,
}

/// Switch the current active profile (UserSheet) within the same Account
#[poem::handler]
pub async fn switch_profile(
    Data(executor): Data<&Pool<Postgres>>,
    Data(token_manager): Data<&TokenManager>,
    Json(req): Json<SwitchProfileRequest>,
    auth_user: crate::middleware::jwt_auth::AuthUser,
    cookie_jar: &CookieJar,
) -> Result<Json<models::AccessToken>, errors::MyError> {
    // 1. Verify that the requested profile belongs to the current account
    // This call will fail if the link doesn't exist
    let new_user_info = db::auth::get_user_info_by_profile(
        executor,
        auth_user.0.account_id,
        req.user_sheet_id,
    )
    .await?;

    if !new_user_info.user_active {
        return Err(errors::MyError::Unauthorized);
    }

    // 2. Generate new token pair for the new profile
    let (access_token, refresh_token) =
        match token_manager.generate_token_pair(token_manager.clone(), new_user_info) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

    // 3. Update the refresh token in the database (Account level) and cookie
    // Reuse existing manage_token logic which handles rotation/insertion
    token_manager
        .manage_token(executor, cookie_jar, &refresh_token, auth_user.0.account_id)
        .await?;

    Ok(Json(crate::AccessToken {
        token: access_token,
    }))
}
