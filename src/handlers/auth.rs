use crate::{models, errors, db, services};
use chrono::DateTime;
use chrono::Duration;
use chrono::prelude::*;
use poem::Response;
use poem::web::{Data, Json, cookie};
use sqlx::*;
use base64::prelude::*;
use jsonwebtoken::*;
use poem::http::header::{SET_COOKIE};

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

    fn generate_access_token(
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
    fn get_random_u128() -> Result<[u8; 32], getrandom::Error> {
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

    async fn create_cookie(&self, token: String) -> cookie::Cookie {
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

    async fn clear_cookie(&self) -> cookie::Cookie {
        let cookie_name = &self.cookie_name;
        

        let mut cookie = cookie::Cookie::new(cookie_name, ""); // Value doesn't matter much for clearing
        cookie.set_path("/refresh_tokens");
        cookie.set_http_only(true);
        cookie.set_secure(true);
        cookie.set_same_site(poem::web::cookie::SameSite::Lax);
        
        // Set expiration to a past date
        cookie.set_expires(Utc::now() - Duration::days(7)); // Expire 7 days ago
        // Or set max_age to 0
        // cookie.set_max_age(Duration::zero()); 
        cookie
    }
}



#[poem::handler]
pub async fn login(Data(executor): Data<&Pool<Postgres>>, Data(token_manager): Data<&TokenManager>, Json(user_auth): Json<models::UserAuth>) -> Result<Json<models::AccessToken>, errors::MyError> {
    let user_info: models::User = match db::auth::login(executor, user_auth.email).await {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    if user_info.user_pwd != user_auth.pwd {
        return Err(errors::MyError::InvalidInput { input_type: "password" });
    }
    
    let token: models::AccessToken = match services::auth::get_token_pair(token_manager.clone(), user_info) {
        Ok(res) => res,
        Err(err) => return Err(err)
    };
    
    Ok(Json(token))   
}


// Récupère la variable enregistrée dans le local storage (le JWT) et verifie les informations contenues à l'intérieur
// pour valider l'autohrisation de faire la commande

//fn use_refresh_token()
// Verifie le refresh token pour enclencher la création d'un nouveau access token et d'un noouveau refresh token à
// donner au user

