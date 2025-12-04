use std::result;
use crate::{models, errors, db};
use chrono::DateTime;
use chrono::prelude::*;
use poem::web::{Data, Json};
use sqlx::*;
use base64::prelude::*;

pub async fn generate_access_token<'e>(
    executor: impl sqlx::PgExecutor<'e>,
    user_auth_id: i32,
) -> Result<models::JWTClaims, errors::MyError> {
    let user_info: models::JWTClaims = match db::get_access_token_claims(executor, user_auth_id).await {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    
    Ok(user_info)
    // TODO : créer le JWT à partir de user_info
}
// Un JWT contenant la user_sheet, enregistré dans le local storage

// pub async fn create_access_token<'e>(user_claims: Claims) {
//     let secret = std::env::var("JWT_SECRET").expect("eeee"); // Est-ce que l'on a le droit de partir du principe que le JWT_SECRET est forcément défini ?
//     let token = match encode(
//         &Header::default(), 
//         &user_claims, 
//         &EncodingKey::from_secret(secret.as_ref())
//     ) {
//         Ok(res) => res,
//         Err(err) => {
//             return Err(AuthError::GenerationFailed);
//         }
        
//     };
// }

// Chaine de caractère aléatoire cryptographique enregistré dans un cookie
pub fn get_random_u128() -> Result<[u8; 32], getrandom::Error> {
    let mut buf = [0u8; 32];
    getrandom::fill(&mut buf)?;
    Ok(buf)
}

pub fn generate_expiration_date(duration: chrono::Duration) -> DateTime<Utc> { 
    let mut utc: DateTime<Utc> = Utc::now();
    let validity_duration = duration; 
    utc += validity_duration;
    return utc
}

#[poem::handler]
pub async fn create_refresh_token(Data(executor): Data<&Pool<Postgres>>,Json(refresh_token) : Json<models::RefreshToken>) -> Result<Json<models::RefreshToken>, errors::MyError> {
    
    let token_bytes: [u8; 32] = match get_random_u128() {
        Ok(res) => res,
        Err(_) => {
            return Err(errors::MyError::GenerationFailed);
        }
    };
    
    let expiration_date = generate_expiration_date(chrono::Duration::weeks(1));
    let token: String = BASE64_STANDARD.encode(token_bytes);
    let result = match db::create_refresh_token(executor, user_auth_id, token, expiration_date).await {
        Ok(result) => result,
        Err(error) => return Err(error),
    };
    Ok(Json(result))
}


// Récupère la variable enregistrée dans le local storage (le JWT) et verifie les informations contenues à l'intérieur
// pour valider l'autohrisation de faire la commande

//fn use_refresh_token()
// Verifie le refresh token pour enclencher la création d'un nouveau access token et d'un noouveau refresh token à
// donner au user
