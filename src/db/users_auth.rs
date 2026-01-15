use crate::{errors::MyError, models::*};
use sqlx::PgExecutor;

/// Create a new user auth record
pub async fn create_user_auth<'e>(
    executor: impl PgExecutor<'e>,
    data: CreateUserAuth,
) -> Result<UserAuthRecord, MyError> {
    let query = sqlx::query_as(
        r#"
        INSERT INTO "users_auth" ("email", "pwd", "id_user_sheet")
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(&data.email)
    .bind(&data.pwd)
    .bind(data.id_user_sheet);

    let user_auth: UserAuthRecord = query.fetch_one(executor).await.map_err(|err| {
        eprintln!("Error creating user auth: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to create user auth",
        }
    })?;

    Ok(user_auth)
}

/// Get a user auth record by ID
pub async fn get_user_auth_by_id<'e>(
    executor: impl PgExecutor<'e>,
    id: i32,
) -> Result<UserAuthRecord, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "users_auth"
        WHERE "id" = $1
        "#,
    )
    .bind(id);

    let user_auth: UserAuthRecord = query.fetch_one(executor).await.map_err(|_| MyError::DBErrors {
        entity: "User auth not found",
    })?;

    Ok(user_auth)
}

/// Get a user auth record by email
pub async fn get_user_auth_by_email<'e>(
    executor: impl PgExecutor<'e>,
    email: &str,
) -> Result<UserAuthRecord, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "users_auth"
        WHERE "email" = $1
        "#,
    )
    .bind(email);

    let user_auth: UserAuthRecord = query.fetch_one(executor).await.map_err(|_| MyError::DBErrors {
        entity: "User auth not found",
    })?;

    Ok(user_auth)
}

/// Update user auth email
pub async fn update_user_auth_email<'e>(
    executor: impl PgExecutor<'e>,
    id: i32,
    data: UpdateUserAuthEmail,
) -> Result<UserAuthRecord, MyError> {
    let query = sqlx::query_as(
        r#"
        UPDATE "users_auth"
        SET "email" = $1
        WHERE "id" = $2
        RETURNING *
        "#,
    )
    .bind(&data.email)
    .bind(id);

    let user_auth: UserAuthRecord = query.fetch_one(executor).await.map_err(|err| {
        eprintln!("Error updating user auth email: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to update user auth email",
        }
    })?;

    Ok(user_auth)
}

/// Update user auth password
pub async fn update_user_auth_password<'e>(
    executor: impl PgExecutor<'e>,
    id: i32,
    data: UpdateUserAuthPassword,
) -> Result<UserAuthRecord, MyError> {
    let query = sqlx::query_as(
        r#"
        UPDATE "users_auth"
        SET "pwd" = $1
        WHERE "id" = $2
        RETURNING *
        "#,
    )
    .bind(&data.pwd)
    .bind(id);

    let user_auth: UserAuthRecord = query.fetch_one(executor).await.map_err(|err| {
        eprintln!("Error updating user auth password: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to update user auth password",
        }
    })?;

    Ok(user_auth)
}

/// Delete a user auth record by ID
pub async fn delete_user_auth<'e>(
    executor: impl PgExecutor<'e>,
    id: i32,
) -> Result<UserAuthRecord, MyError> {
    let query = sqlx::query_as(
        r#"
        DELETE FROM "users_auth"
        WHERE "id" = $1
        RETURNING *
        "#,
    )
    .bind(id);

    let user_auth: UserAuthRecord = query.fetch_one(executor).await.map_err(|_| MyError::DBErrors {
        entity: "Failed to delete user auth",
    })?;

    Ok(user_auth)
}
