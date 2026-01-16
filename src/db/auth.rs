use crate::{errors::*, models::*};
use chrono::{DateTime, Utc};
use sqlx::Row;

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
    query = query.bind(user_auth_id);
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

/// Revoke all refresh tokens for a specific user (admin function)
pub async fn revoke_user_refresh_tokens<'e>(
    executor: impl sqlx::PgExecutor<'e>,
    user_id: i32,
) -> Result<u64, MyError> {
    let result = sqlx::query(
        r#"
        DELETE FROM "refresh_tokens"
        WHERE "id_user_auth" IN (
            SELECT id FROM "users_auth" WHERE "id_user_sheet" = $1
        )
        "#,
    )
    .bind(user_id)
    .execute(executor)
    .await
    .map_err(|err| {
        eprintln!("Error revoking user refresh tokens: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to revoke user refresh tokens",
        }
    })?;

    Ok(result.rows_affected())
}

/// Revoke ALL refresh tokens (emergency function for security incidents)
pub async fn revoke_all_refresh_tokens<'e>(
    executor: impl sqlx::PgExecutor<'e>,
) -> Result<u64, MyError> {
    let result = sqlx::query(
        r#"
        DELETE FROM "refresh_tokens"
        "#,
    )
    .execute(executor)
    .await
    .map_err(|err| {
        eprintln!("Error revoking all refresh tokens: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to revoke all refresh tokens",
        }
    })?;

    Ok(result.rows_affected())
}

pub async fn login<'e>(
    executor: impl sqlx::PgExecutor<'e>,
    user_email: &String,
) -> Result<User, MyError> {
    let mut query = sqlx::query_as(
        r#"
        SELECT us.id AS user_id, us.last_name AS user_lastname, us.first_name AS user_firstname, us.type_user AS user_role, us.profile_picture AS user_profilepicture, us.active AS user_active, ua.email AS user_email, ua.pwd AS user_pwd
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
        SELECT us.id AS user_id, us.last_name AS user_lastname, us.first_name AS user_firstname, us.type_user AS user_role, us.profile_picture AS user_profilepicture, us.active AS user_active, ua.email AS user_email, ua.pwd AS user_pwd
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
