use poem::web::cookie::CookieJar;

use crate::{models, handlers, errors};

pub fn get_token_pair(token_manager: handlers::auth::TokenManager, user_info: models::auth::User, cookie_jar: &CookieJar) -> Result<models::AccessToken, errors::MyError> {
    let exp = token_manager.generate_expiration_date(chrono::Duration::minutes(5)).timestamp() as usize;
    let claims = models::JWTClaims{user: user_info, exp};
    
    let token_pair: (String, String) = match token_manager.generate_token_pair(claims) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    // toker_manager.add_token_to_cookie
    let cookie = token_manager.create_cookie(token_pair.1);
    cookie_jar.add(cookie);
    Ok(models::AccessToken{token: token_pair.0})
}