use crate::MyError;
use crate::db;
use crate::{errors, models};
use base64::prelude::*;
use chrono::DateTime;
use chrono::Duration;
use chrono::prelude::*;
use jsonwebtoken::*;
use poem::web::cookie;
use poem::web::cookie::CookieJar;
use sqlx::*;

#[derive(Clone)]
pub struct TokenManager {
    pub jwt_secret: String,
    pub cookie_name: String,
}

impl TokenManager {
    pub fn new(jwt_secret: String, cookie_name: String) -> TokenManager {
        TokenManager {
            jwt_secret,
            cookie_name,
        }
    }

    pub fn generate_access_token(
        &self,
        claims: models::JWTClaims,
    ) -> Result<String, errors::MyError> {
        let encoded_token = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        ) {
            Ok(res) => res,
            Err(_) => {
                return Err(errors::MyError::GenerationFailed {
                    entity: "acces token",
                });
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
        return utc;
    }

    pub fn generate_refresh_token() -> Result<String, errors::MyError> {
        let token_bytes: [u8; 32] = match TokenManager::get_random_u128() {
            Ok(res) => res,
            Err(_) => {
                return Err(errors::MyError::GenerationFailed {
                    entity: "refresh token",
                });
            }
        };
        let token: String = BASE64_STANDARD.encode(token_bytes);
        Ok(token)
    }

    pub fn verify_token_validity(&self, date: DateTime<Utc>) -> bool {
        date > Utc::now()
    }

    pub fn create_cookie(&self, token: String, expiration_date: DateTime<Utc>) -> cookie::Cookie {
        let cookie_name = &self.cookie_name;
        let cookie_value = token;

        let mut cookie = cookie::Cookie::new(cookie_name, cookie_value);

        cookie.set_path("/api/refresh_tokens"); // Make the cookie available to all paths on the domain
        cookie.set_http_only(true);
        cookie.set_secure(true);
        cookie.set_same_site(poem::web::cookie::SameSite::Lax);

        // let expiration_date = self.generate_expiration_date(chrono::Duration::weeks(1));
        cookie.set_expires(expiration_date);

        cookie
    }

    pub fn get_cookie_value(&self, cookie_jar: &CookieJar) -> Result<String, errors::MyError> {
        let cookie_name = &self.cookie_name;
        let refresh_token = match cookie_jar.get(&cookie_name) {
            Some(res) => match res.value::<String>() {
                Ok(res) => res,
                Err(err) => return Err(MyError::CookieError(err)),
            },
            None => "".to_string(),
        };
        Ok(refresh_token)
    }

    pub fn clear_cookie(&self, cookie_jar: &CookieJar) {
        let mut cookie = cookie::Cookie::named(self.cookie_name.clone());
        cookie.set_path("/api/refresh_tokens");
        cookie.set_http_only(true);
        cookie.set_secure(true);
        cookie.set_same_site(poem::web::cookie::SameSite::Lax);
        cookie.set_expires(Utc::now() - chrono::Duration::days(1)); // Expire immediately
        cookie_jar.add(cookie); // Overwrite the existing cookie with the expired one
    }

    pub fn generate_token_pair(
        &self,
        token_manager: TokenManager,
        user_info: models::auth::User,
    ) -> Result<(String, String), errors::MyError> {
        let exp = token_manager
            .generate_expiration_date(chrono::Duration::minutes(5))
            .timestamp() as usize;
        let claims = models::JWTClaims {
            user: user_info,
            exp,
        };

        let access_token: String = match TokenManager::generate_access_token(self, claims) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        let refresh_token: String = match TokenManager::generate_refresh_token() {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        Ok((access_token, refresh_token))
    }

    pub async fn manage_token<'e>(
        &self,
        executor: &Pool<Postgres>,
        cookie_jar: &CookieJar,
        refresh_token: &String,
        user_auth_id: i32,
    ) -> Result<(), MyError> {
        // enregistrer le refresh d ans la db
        let exp: DateTime<Utc> = self.generate_expiration_date(chrono::Duration::weeks(2));

        match db::create_refresh_token(executor, user_auth_id, refresh_token.clone(), exp).await {
            Ok(_) => {}
            Err(err) => return Err(err),
        };
        let cookie = self.create_cookie(refresh_token.clone(), exp);
        cookie_jar.add(cookie);
        Ok(())
    }
}
