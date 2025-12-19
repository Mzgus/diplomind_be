use crate::{models, errors, db, services::*};
use poem::web::cookie::CookieJar;
use poem::web::{Data, Json};
use sqlx::*;


#[poem::handler]
pub async fn login(
    Data(executor): Data<&Pool<Postgres>>, 
    Data(token_manager): Data<&TokenManager>, 
    Json(user_auth): Json<models::UserAuth>, 
    cookie_jar: &CookieJar
) -> Result<Json<models::AccessToken>, errors::MyError> {
    let user_info: models::User = match db::auth::login(executor, &user_auth.email).await {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let user_auth_id: i32 = match db::auth::get_auth_id_by_email(executor, &user_auth.email).await {
        Ok(res) => res,
        Err(err) => return Err(err)
    };
    // Comme le mot de passe sera haché dans la BDD il faut aussi hacher le mdp donner pour pouvoir les comparer
    if user_info.user_pwd != user_auth.pwd {
        return Err(errors::MyError::InvalidInput { input_type: "password" });
    }
    
    let (access_token, refresh_token) = match token_manager.generate_token_pair(token_manager.clone(), user_info) {
        Ok(res) => res,
        Err(err) => return Err(err)
    };

    match token_manager.manage_token(executor, cookie_jar, &refresh_token, user_auth_id).await {
        Err(err) => return Err(err),
        _ => {},
    }

    Ok(Json(crate::AccessToken { token: access_token }))   
}

#[poem::handler]
pub async fn refresh_tokens(
    Data(executor): Data<&Pool<Postgres>>, 
    Data(token_manager): Data<&TokenManager>, 
    cookie_jar: &CookieJar
) -> Result<Json<models::AccessToken>, errors::MyError> {
    let token = match token_manager.get_cookie_value(cookie_jar) {
        Ok(res) => res,
        Err(err) => return Err(err)
    };
    
    let refresh_token: models::RefreshToken = match db::auth::get_refresh_token(executor, token.clone()).await {
        Ok(res) => res,
        Err(err) => return Err(err)
    };

    if !token_manager.verify_token_validity(refresh_token.expiration_date) {
        return Err(errors::MyError::TokenExpired);
    }

    let user_info: models::User = match db::auth::get_user_info_by_token(executor, token.clone()).await {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    let user_auth_id: i32 = match db::auth::get_auth_id_by_email(executor, &user_info.user_email).await {
        Ok(res) => res,
        Err(err) => return Err(err)
    };

    let (access_token, refresh_token) = match token_manager.generate_token_pair(token_manager.clone(), user_info) {
        Ok(res) => res,
        Err(err) => return Err(err)
    };

    match token_manager.manage_token(executor, cookie_jar, &refresh_token, user_auth_id).await {
        Err(err) => return Err(err),
        _ => {},
    }
        
    Ok(Json(crate::AccessToken { token: access_token })) 
}


//logout
#[poem::handler]
pub async fn logout(
    Data(token_manager): Data<&TokenManager>,
    cookie_jar: &CookieJar,
) -> Result<(), errors::MyError> {
    // cookie_jar.remove(token_manager.cookie_name.clone());
    // token_manager.clear_cookie(cookie_jar);
    Ok(())
}
