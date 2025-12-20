use crate::{errors::*, models::*, refresh_tokens};
use chrono::{DateTime, Utc};
use sqlx::Row;
use sqlx::query;

// Comment because not used
// pub async fn get_access_token_claims<'e>(executor: impl sqlx::PgExecutor<'e>, user_auth_id: i32, exp : usize) -> Result<JWTClaims, MyError> {
//     let mut query = sqlx::query_as(
//         r#"
//         SELECT (us.id, us.last_name, us.first_name, us.type_user, us.profile_picture, ua.email) from "users_sheets" as us
//         JOIN "users_auth" as ua ON us.id = ua.id_user_sheet
//         WHERE ua.id = ($1)
//     "#,
//     );
//     query = query.bind(user_auth_id);
//     let user: User = match query.fetch_one(executor).await {
//         Ok(res) => res,
//         Err(_) => {
//             return Err(MyError::DBErrors { entity: "acces token not created" });
//         }
//     };
//     // let exp = TokenManager::generate_expiration_date(Duration::minutes(5)).timestamp() as usize;
//     let claims = JWTClaims{user, exp};
//     Ok(claims)
// }

// Comment because not used
pub async fn get_refresh_token<'e>(
    executor: impl sqlx::PgExecutor<'e>,
    token: String,
) -> Result<RefreshToken, MyError> {
    let mut query = sqlx::query_as(
        r#"
        SELECT * FROM "refresh_tokens"
        WHERE "token" = ($1)
    "#,
    );
    query = query.bind(&token);
    let refresh_token: RefreshToken = match query.fetch_one(executor).await {
        Ok(res) => res,
        Err(_err) => {
            return Err(MyError::DBErrors {
                entity: "Refresh token not found",
            });
        }
    };

    Ok(refresh_token)
}

pub async fn create_refresh_token<'e>(
    executor: impl sqlx::PgExecutor<'e>,
    user_auth_id: i32,
    refresh_token: String,
    expiration_date: DateTime<Utc>,
) -> Result<RefreshToken, MyError> {
    let mut query = sqlx::query_as(
        r#"
        INSERT INTO "refresh_tokens" ("token", "id_user_auth", "expiration_date")
        VALUES ($1, $2, $3)
        RETURNING *
    "#,
    );
    query = query
        .bind(&refresh_token)
        .bind(user_auth_id)
        .bind(expiration_date);
    let refresh_token: RefreshToken = match query.fetch_one(executor).await {
        Ok(res) => res,
        Err(err) => {
            println!("{:?}", err);
            return Err(MyError::DBErrors {
                entity: "Refresh token not created",
            });
        }
    };

    Ok(refresh_token)
}

pub async fn delete_refresh_token<'e>(
    executor: impl sqlx::PgExecutor<'e>,
    token: String,
) -> Result<RefreshToken, MyError> {
    let mut query = sqlx::query_as(
        r#"
        DELETE FROM "refresh_tokens"
        WHERE "token" = ($1)
        RETURNING *
    "#,
    );
    query = query.bind(&token);
    let refresh_token: RefreshToken = match query.fetch_one(executor).await {
        Ok(res) => res,
        Err(_err) => {
            return Err(MyError::DBErrors {
                entity: "Refresh token not deleted",
            });
        }
    };

    Ok(refresh_token)
}

pub async fn delete_refresh_token_by_auth_id<'e>(
    executor: impl sqlx::PgExecutor<'e>,
    user_auth_id: i32,
) -> Result<RefreshToken, MyError> {
    let mut query = sqlx::query_as(
        r#"
        DELETE FROM "refresh_tokens" 
        WHERE "id_user_auth" = ($1) 
        RETURNING *
    "#,
    );
    query = query.bind(&user_auth_id);
    let refresh_token: RefreshToken = match query.fetch_one(executor).await {
        Ok(res) => res,
        Err(_err) => {
            return Err(MyError::DBErrors {
                entity: "Refresh token not deleted",
            });
        }
    };
    Ok(refresh_token)
}

pub async fn login<'e>(
    executor: impl sqlx::PgExecutor<'e>,
    user_email: &String,
) -> Result<User, MyError> {
    let mut query = sqlx::query_as(
        r#"
        SELECT us.id AS user_id, us.last_name AS user_lastname, us.first_name AS user_firstname, us.type_user AS user_role, us.profile_picture AS user_profilepicture, ua.email AS user_email, ua.pwd AS user_pwd
        FROM "users_sheets" as us 
        JOIN "users_auth" as ua
        ON us.id = ua.id_user_sheet
        WHERE ua.email = ($1)
        "#,
    );
    query = query.bind(user_email);
    let row = match query.fetch_one(executor).await {
        Ok(res) => res,
        Err(_err) => {
            return Err(MyError::DBErrors {
                entity: "User not found",
            });
        }
    };
    Ok(row)
}

pub async fn get_user_info_by_token<'e>(
    executor: impl sqlx::PgExecutor<'e>,
    refresh_token: String,
) -> Result<User, MyError> {
    let mut query = sqlx::query_as(
        r#"
        SELECT us.id AS user_id, us.last_name AS user_lastname, us.first_name AS user_firstname, us.type_user AS user_role, us.profile_picture AS user_profilepicture, ua.email AS user_email, ua.pwd AS user_pwd
        FROM "users_sheets" as us
        JOIN "users_auth" as ua ON us.id = ua.id_user_sheet
        JOIN "refresh_tokens" as rt ON ua.id = rt.id_user_auth
        WHERE rt.token = ($1)
        "#,
    );
    query = query.bind(refresh_token);
    let user_info: User = match query.fetch_one(executor).await {
        Ok(res) => res,
        Err(_) => {
            return Err(MyError::DBErrors {
                entity: "Refresh token not found",
            });
        }
    };
    Ok(user_info)
}

pub async fn get_auth_id_by_email<'e>(
    executor: impl sqlx::PgExecutor<'e>,
    user_email: &String,
) -> Result<i32, MyError> {
    let mut query = sqlx::query(
        r#"
        SELECT ua.id from "users_auth" as ua
        WHERE ua.email = ($1)
        "#,
    );
    query = query.bind(user_email);
    let user_auth_id: i32 = match query.fetch_one(executor).await {
        Ok(row) => row.get("id"),
        Err(_) => {
            return Err(MyError::DBErrors {
                entity: "User auth not found",
            });
        }
    };
    Ok(user_auth_id)
}
