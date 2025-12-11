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
    let user_info: models::User = match db::auth::login(executor, user_auth.email).await {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    if user_info.user_pwd != user_auth.pwd {
        return Err(errors::MyError::InvalidInput { input_type: "password" });
    }
    
    let access_token: models::AccessToken = match token_manager.get_token_pair(token_manager.clone(), user_info, cookie_jar) {
        Ok(res) => res,
        Err(err) => return Err(err)
    };
    
    Ok(Json(access_token))   
}

#[poem::handler]
pub async fn refresh_tokens(
    Data(executor): Data<&Pool<Postgres>>, 
    Data(token_manager): Data<&TokenManager>, 
    cookie_jar: &CookieJar
) -> Result<Json<models::AccessToken>, errors::MyError> {
    let refresh_token = match token_manager.get_cookie_value(cookie_jar) {
        Ok(res) => res,
        Err(err) => return Err(err)
    };
    let user_info: models::User = match token_manager.verify_refresh_validity(refresh_token) {
        Some(res) => res,
        None => return Err(errors::MyError::InvalidInput { input_type: "Refresh token" }),
    };

    let access_token: models::AccessToken = match token_manager.get_token_pair(token_manager.clone(), user_info, cookie_jar) {
        Ok(res) => res,
        Err(err) => return Err(err)
    };
    
    Ok(Json(access_token)) 
}

// Récupère la variable enregistrée dans le local storage (le JWT) et verifie les informations contenues à l'intérieur
// pour valider l'autohrisation de faire la commande

//fn use_refresh_token()
// Verifie le refresh token pour enclencher la création d'un nouveau access token et d'un noouveau refresh token à
// donner au user

