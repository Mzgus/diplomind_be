use crate::{models::*, errors::*};
use chrono::{DateTime, Utc};

pub async fn get_access_token_claims<'e>(executor: impl sqlx::PgExecutor<'e>, user_auth_id: i32, exp : usize) -> Result<JWTClaims, MyError> {
    let mut query = sqlx::query_as(
        r#"
        SELECT (us.id, us.last_name, us.first_name, us.type_user, us.profile_picture, ua.email) from "users_sheets" as us 
        JOIN "users_auth" as ua ON us.id = ua.id_user_sheet
        WHERE ua.id = ($1)
    "#,
    );
    query = query.bind(user_auth_id);
    let user: User = match query.fetch_one(executor).await {
        Ok(res) => res,
        Err(_) => {
            return Err(MyError::DBErrors { entity: "acces token not created" });
        } 
    };
    // let exp = TokenManager::generate_expiration_date(Duration::minutes(5)).timestamp() as usize;
    let claims = JWTClaims{user, exp};
    Ok(claims)
}

pub async fn create_refresh_token<'e>(executor: impl sqlx::PgExecutor<'e>, user_auth_id: i32, token: String, expiration_date : DateTime<Utc>) -> Result<RefreshToken, MyError> {

    let mut query = sqlx::query_as(
        r#"
        INSERT INTO "refresh_token" ("token", "id_user_auth", "expiration_date")
        VALUES ($1, $2, $3)
        RETURNNING *
    "#,
    );
    query = query.bind(&token).bind(user_auth_id).bind(expiration_date);
    let refresh_token : RefreshToken = match query.fetch_one(executor).await {
        Ok(res) => res,
        Err(_err) => {
            return Err(MyError::DBErrors { entity: "refresh token not created" });
        }
    };

    Ok(refresh_token)
}

pub async fn get_refresh_token<'e>(executor: impl sqlx::PgExecutor<'e>, token: String) -> Result<RefreshToken, MyError> {
    let mut query = sqlx::query_as(
        r#"
        SELECT * FROM "refresh_token"
        WHERE "token" = ($1)
    "#,
    );
    query = query.bind(&token);
    let refresh_token: RefreshToken = match query.fetch_one(executor).await {
        Ok(res) => res,
        Err(_err) => {
            return Err(MyError::DBErrors { entity: "refresh token not found" });
        }
    };

    Ok(refresh_token)
}

pub async fn delete_refresh_token<'e>(executor: impl sqlx::PgExecutor<'e>, token: String) -> Result<RefreshToken, MyError> {
    let mut query = sqlx::query_as(
        r#"
        DELETE FROM "refresh_token"
        WHERE "token" = ($1)
        RETURNING *
    "#,
    );
    query = query.bind(&token);
    let refresh_token: RefreshToken = match query.fetch_one(executor).await {
        Ok(res) => res,
        Err(_err) => {
            return Err(MyError::DBErrors { entity: "refresh token not deleted" });
        }
    };

    Ok(refresh_token)
}

pub async fn login<'e>(executor: impl sqlx::PgExecutor<'e>, user_email: String) -> Result<User, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * from "users_sheets" as us 
        JOIN "users_auth" as ua ON us.id = ua.id_user_sheet
        WHERE email = ($1)
        "#,
    );
    let row = match query.bind(user_email).fetch_one(executor).await {
        Ok(res) => res,
        Err(_) => {
            return Err(MyError::DBErrors { entity: "user not found" });
        }
    };
    Ok(row)
    
}