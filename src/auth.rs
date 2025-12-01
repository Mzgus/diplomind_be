use crate::models::*;
use crate::queries::*;
use base64::prelude::*;
use sqlx::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("generation failed")]
    GenerationFailed,
    #[error("fetch failed")]
    FetchFailed,
}

pub async fn generate_access_token<'e>(
    executor: impl sqlx::PgExecutor<'e>,
    user_auth_id: i32,
) -> Result<UserInfos, Error> {
    let user_info = match get_user_sheet_and_email(executor, user_auth_id).await {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    Ok(user_info)
    // TODO : créer le JWT à partir de user_info
}
// Un JWT contenant la user_sheet, enregistré dans le local storage

// Chaine de caractère aléatoire cryptographique enregistré dans un cookie
fn get_random_u128() -> Result<[u8; 32], getrandom::Error> {
    let mut buf = [0u8; 32];
    getrandom::fill(&mut buf)?;
    Ok(buf)
}

// pub async fn generate_refresh_token<'e>(user_auth_id: i32) -> Result<String, AuthError> {
//     let token_bytes = match get_random_u128() {
//         Ok(res) => res,
//         Err(_) => {
//             return Err(AuthError::GenerationFailed);
//         }
//     };

//     let token = BASE64_STANDARD.encode(token_bytes);

//     let _query_result = sqlx::query(
//         r#"
//         INSERT INTO "refresh_token" ("token", "id_user_auth")
//         VALUES ($1, $2)
//     "#,
//     )
//     .bind(&token)
//     .bind(user_auth_id);

//     Ok(token)
// }

pub async fn generate_refresh_token(
    executor: impl sqlx::PgExecutor<'_>, // 1. On ajoute l'executor (pool ou connection)
    user_auth_id: i32,
) -> Result<String, AuthError> {
    let token_bytes = match get_random_u128() {
        Ok(res) => res,
        Err(_) => {
            return Err(AuthError::GenerationFailed);
        }
    };

    let token = BASE64_STANDARD.encode(token_bytes);

    // 2. On exécute la requête directement
    sqlx::query(
        r#"
        INSERT INTO "refresh_token" ("token", "id_user_auth")
        VALUES ($1, $2)
        "#,
    )
    .bind(&token)
    .bind(user_auth_id)
    .execute(executor) // <--- C'est ici que la magie opère (définit le type Postgres)
    .await // <--- C'est de l'async, il faut attendre
    .map_err(|_| AuthError::GenerationFailed)?; // <--- On gère l'erreur SQL éventuelle

    Ok(token)
}

// Récupère la variable enregistrée dans le local storage (le JWT) et verifie les informations contenues à l'intérieur
// pour valider l'autohrisation de faire la commande

//fn use_refresh_token()
// Verifie le refresh token pour enclencher la création d'un nouveau access token et d'un noouveau refresh token à
// donner au user
