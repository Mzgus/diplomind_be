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
    let user_info: models::User = match db::auth::login(executor, &user_auth.email).await {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let user_auth_id: i32 = match db::auth::get_auth_id_by_email(executor, &user_auth.email).await {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    
    // Verify password using Argon2
    let password_valid = match auth::verify_password(&user_auth.pwd, &user_info.user_pwd) {
        Ok(valid) => valid,
        Err(err) => return Err(err),
    };
    
    if !password_valid {
        return Err(errors::MyError::InvalidCredentials);
    }

    let (access_token, refresh_token) =
        match token_manager.generate_token_pair(token_manager.clone(), user_info) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

    match token_manager
        .manage_token(executor, cookie_jar, &refresh_token, user_auth_id)
        .await
    {
        Err(err) => return Err(err),
        _ => {}
    }

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

    let user_auth_id: i32 =
        match db::auth::get_auth_id_by_email(executor, &user_info.user_email).await {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

    // Delete the old refresh token (Rotation)
    let _ = db::auth::delete_refresh_token(executor, token.clone()).await;

    let (access_token, refresh_token) =
        match token_manager.generate_token_pair(token_manager.clone(), user_info) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

    match token_manager
        .manage_token(executor, cookie_jar, &refresh_token, user_auth_id)
        .await
    {
        Err(err) => return Err(err),
        _ => {}
    }

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
#[poem::handler]
pub async fn verify_token(
    Data(token_manager): Data<&TokenManager>,
    req: &poem::Request,
) -> Result<Json<models::User>, errors::MyError> {
    use jsonwebtoken::{decode, DecodingKey, Validation};
    
    // Extract Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(errors::MyError::TokenExpired)?;
    
    // Extract token from "Bearer <token>"
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(errors::MyError::TokenExpired)?;
    
    // Decode and validate the JWT
    let token_data = decode::<models::JWTClaims>(
        token,
        &DecodingKey::from_secret(token_manager.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| errors::MyError::TokenExpired)?;
    
    // Return the user information from the token
    Ok(Json(token_data.claims.user))
}