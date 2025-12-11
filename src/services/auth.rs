use poem::web::cookie::CookieJar;
use base64::prelude::*;
use jsonwebtoken::*;
use poem::web::cookie;
use chrono::DateTime;
use chrono::Duration;
use chrono::prelude::*;
use crate::{models, errors};
use crate::MyError;


#[derive(Clone)]
pub struct TokenManager {
    pub jwt_secret: String,
    pub cookie_name: String,
}

impl TokenManager {
    pub fn new(jwt_secret: String, cookie_name: String) -> TokenManager
    {
     TokenManager
     { jwt_secret, cookie_name }
    }

    pub fn generate_access_token(
        &self,
        claims: models::JWTClaims,
    ) -> Result<String, errors::MyError> {
        let encoded_token = match encode(
            &Header::default(), 
            &claims,    
            &EncodingKey::from_secret(self.jwt_secret.as_ref())
        ) {
            Ok(res) => res,
            Err(_) => {
                return Err(errors::MyError::GenerationFailed { entity: "acces token" });
            }    
        };
        Ok(encoded_token)
    }

    // Chaine de caractère aléatoire cryptographique enregistré dans un cookie
    pub fn get_random_u128() -> Result<[u8; 32], getrandom::Error> {
        let mut buf = [0u8; 32];
        getrandom::fill(&mut buf)?;
        Ok(buf)
    }
    
    pub fn generate_expiration_date(&self, duration: chrono::Duration) -> DateTime<Utc> { 
        let mut utc: DateTime<Utc> = Utc::now();
        let validity_duration = duration; 
        utc += validity_duration;
        return utc
    }

    pub fn generate_refresh_token() -> Result<String, errors::MyError> {
        let token_bytes: [u8; 32] = match TokenManager::get_random_u128() {
            Ok(res) => res,
            Err(_) => {
                return Err(errors::MyError::GenerationFailed { entity: "refresh token" });
            }
        };
        let token: String = BASE64_STANDARD.encode(token_bytes);
        Ok(token)
    }

    pub fn verify_token_validity(date: DateTime<Utc>) -> bool {
        date > Utc::now()
    }

    pub fn generate_token_pair(&self, claims: models::JWTClaims) -> Result<(String, String), errors::MyError> {
        let access_token: String = match TokenManager::generate_access_token(self, claims) {
            Ok(res) => res,
            Err(err) => return Err(err)
        };
        let refresh_token: String = match TokenManager::generate_refresh_token() {
            Ok(res) => res,
            Err(err) => return Err(err)
        };
        Ok((access_token, refresh_token))
    }

    pub fn create_cookie(&self, token: String) -> cookie::Cookie {
        let cookie_name = &self.cookie_name;
        let cookie_value = token;

        let mut cookie = cookie::Cookie::new(cookie_name, cookie_value);

        cookie.set_path("/refresh_tokens"); // Make the cookie available to all paths on the domain
        cookie.set_http_only(true); // Prevent JavaScript access (good for security)
        cookie.set_secure(true); // Only send over HTTPS (essential for production)
        cookie.set_same_site(poem::web::cookie::SameSite::Lax); // Protection against CSRF
    
        let expiration_date = self.generate_expiration_date(chrono::Duration::weeks(1));
        cookie.set_expires(expiration_date);
    
        cookie
    }

    pub fn get_cookie_value(&self, cookie_jar: &CookieJar) -> Result<String, errors::MyError> {
        let cookie_name = &self.cookie_name;
        let refresh_token = match cookie_jar.get(&cookie_name) {
            Some(res) => { 
                match res.value::<String>() {
                    Ok(res) => res,
                    Err(err) => return Err(MyError::CookieError(err))
                }
            },
            None => "".to_string()
        };
        Ok(refresh_token)
    }

    pub fn clear_cookie(&self, cookie_jar: &CookieJar) {
        let cookie_name = &self.cookie_name;
        
        let mut cookie = cookie::Cookie::new(cookie_name, ""); // Value doesn't matter much for clearing
        cookie.set_path("/refresh_tokens");
        cookie.set_http_only(true);
        cookie.set_secure(true);
        cookie.set_same_site(poem::web::cookie::SameSite::Lax);
        
        // Set expiration to a past date
        cookie.set_expires(Utc::now() - Duration::days(7)); // Expire 7 days ago
        cookie_jar.add(cookie);
    }

    pub fn get_token_pair(&self, token_manager: TokenManager, user_info: models::auth::User, cookie_jar: &CookieJar) -> Result<models::AccessToken, errors::MyError> {
        let exp = token_manager.generate_expiration_date(chrono::Duration::minutes(5)).timestamp() as usize;
        let claims = models::JWTClaims{user: user_info, exp};
        
        let token_pair: (String, String) = match token_manager.generate_token_pair(claims) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        // ajouter le cookie à la bdd
        let cookie = token_manager.create_cookie(token_pair.1);
        cookie_jar.add(cookie);
        Ok(models::AccessToken{token: token_pair.0})
    }

    pub fn verify_refresh_validity(&self, token: String) -> Option<models::User> {
        //Check if token is valid
        None
    }
}

