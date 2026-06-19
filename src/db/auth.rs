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
        INSERT INTO "refresh_tokens" ("token", "account_id", "expiration_date")
        VALUES ($1, $2, $3)
        RETURNING *
    "#,
    );
    query = query
        .bind(&refresh_token)
        .bind(user_auth_id) // This variable name might be confusing now, but it holds account_id
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

pub async fn delete_refresh_token_by_account_id<'e>(
    executor: impl sqlx::PgExecutor<'e>,
    account_id: i32,
) -> Result<RefreshToken, MyError> {
    let mut query = sqlx::query_as(
        r#"
        DELETE FROM "refresh_tokens" 
        WHERE "account_id" = ($1) 
        RETURNING *
    "#,
    );
    query = query.bind(account_id);
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

/// Revoke all refresh tokens for a specific account (was user_id/sheet link)
pub async fn revoke_account_refresh_tokens<'e>(
    executor: impl sqlx::PgExecutor<'e>,
    account_id: i32,
) -> Result<u64, MyError> {
    let result = sqlx::query(
        r#"
        DELETE FROM "refresh_tokens"
        WHERE "account_id" = $1
        "#,
    )
    .bind(account_id)
    .execute(executor)
    .await
    .map_err(|err| {
        eprintln!("Error revoking account refresh tokens: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to revoke account refresh tokens",
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
        SELECT 
            a.id AS account_id,
            us.id AS user_id,
            us.last_name AS user_lastname,
            us.first_name AS user_firstname,
            us.type_user AS user_role,
            us.profile_picture AS user_profilepicture,
            a.email AS user_email,
            us.active AS user_active
        FROM "users_auth" ua
        JOIN "accounts" a ON ua.account_id = a.id
        JOIN "accounts_users_sheets" aus ON aus.account_id = a.id
        JOIN "users_sheets" us ON us.id = aus.user_sheet_id
        WHERE ua.email = ($1)
        LIMIT 1
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
        SELECT 
            a.id AS account_id,
            us.id AS user_id,
            us.last_name AS user_lastname,
            us.first_name AS user_firstname,
            us.type_user AS user_role,
            us.profile_picture AS user_profilepicture,
            a.email AS user_email,
            us.active AS user_active
        FROM "refresh_tokens" rt
        JOIN "accounts" a ON rt.account_id = a.id
        JOIN "accounts_users_sheets" aus ON aus.account_id = a.id
        JOIN "users_sheets" us ON us.id = aus.user_sheet_id
        WHERE rt.token = ($1)
        LIMIT 1
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
        SELECT ua.account_id as id from "users_auth" as ua
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

pub async fn get_credentials_by_email<'e>(
    executor: impl sqlx::PgExecutor<'e>,
    email: &str,
) -> Result<crate::models::users_auth::UserAuthRecord, MyError> {
    let mut query = sqlx::query_as(
        r#"
        SELECT * FROM "users_auth"
        WHERE email = ($1)
        "#,
    );
    query = query.bind(email);
    let record: crate::models::users_auth::UserAuthRecord = match query.fetch_one(executor).await {
        Ok(res) => res,
        Err(_) => {
            return Err(MyError::DBErrors {
                entity: "User auth not found",
            });
        }
    };
    Ok(record)
}

/// Get all profiles (user sheets) for a specific account
pub async fn get_account_profiles<'e>(
    executor: impl sqlx::PgExecutor<'e>,
    account_id: i32,
) -> Result<Vec<UserSheet>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT us.*
        FROM "users_sheets" us
        JOIN "accounts_users_sheets" aus ON us.id = aus.user_sheet_id
        WHERE aus.account_id = $1
        ORDER BY us.type_user, us.last_name
        "#,
    )
    .bind(account_id);

    let profiles: Vec<UserSheet> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching account profiles: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch account profiles",
        }
    })?;

    Ok(profiles)
}

/// Get a specific user profile (sheet) ensuring it belongs to the given account
pub async fn get_user_info_by_profile<'e>(
    executor: impl sqlx::PgExecutor<'e>,
    account_id: i32,
    user_sheet_id: i32,
) -> Result<User, MyError> {
    let mut query = sqlx::query_as(
        r#"
        SELECT 
            a.id AS account_id,
            us.id AS user_id,
            us.last_name AS user_lastname,
            us.first_name AS user_firstname,
            us.type_user AS user_role,
            us.profile_picture AS user_profilepicture,
            a.email AS user_email,
            us.active AS user_active
        FROM "users_sheets" us
        JOIN "accounts_users_sheets" aus ON aus.user_sheet_id = us.id
        JOIN "accounts" a ON a.id = aus.account_id
        WHERE a.id = $1 AND us.id = $2
        LIMIT 1
        "#,
    );
    query = query.bind(account_id).bind(user_sheet_id);

    let user_info: User = match query.fetch_one(executor).await {
        Ok(res) => res,
        Err(_) => {
            return Err(MyError::NotFound {
                entity: "Profile not found for this account",
            });
        }
    };
    Ok(user_info)
}
