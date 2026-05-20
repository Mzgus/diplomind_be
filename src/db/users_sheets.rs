use crate::{errors::MyError, models::*};
use sqlx::PgExecutor;

/// Create a new user sheet
pub async fn create_user_sheet<'e>(
    executor: impl PgExecutor<'e>,
    data: CreateUserSheet,
) -> Result<UserSheet, MyError> {
    let query = sqlx::query_as(
        r#"
        WITH inserted_sheet AS (
            INSERT INTO "users_sheets" ("last_name", "first_name", "type_user", "profile_picture")
            VALUES ($1, $2, $3, $4)
            RETURNING *
        ),
        inserted_link AS (
            INSERT INTO "accounts_users_sheets" ("account_id", "user_sheet_id")
            SELECT $5, id FROM inserted_sheet
            WHERE $5 IS NOT NULL
        )
        SELECT * FROM inserted_sheet
        "#,
    )
    .bind(&data.last_name)
    .bind(&data.first_name)
    .bind(&data.type_user)
    .bind(&data.profile_picture)
    .bind(data.account_id);

    let user_sheet: UserSheet = query.fetch_one(executor).await.map_err(|err| {
        eprintln!("Error creating user sheet: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to create user sheet",
        }
    })?;

    Ok(user_sheet)
}

/// Get a user sheet by ID
pub async fn get_user_sheet_by_id<'e>(
    executor: impl PgExecutor<'e>,
    id: i32,
) -> Result<UserSheet, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "users_sheets"
        WHERE "id" = $1
        "#,
    )
    .bind(id);

    let user_sheet: UserSheet = query
        .fetch_one(executor)
        .await
        .map_err(|_| MyError::NotFound {
            entity: "User sheet",
        })?;

    Ok(user_sheet)
}

/// Get all user sheets
pub async fn get_all_user_sheets<'e>(
    executor: impl PgExecutor<'e>,
) -> Result<Vec<UserSheet>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "users_sheets"
        ORDER BY "last_name", "first_name"
        "#,
    );

    let user_sheets: Vec<UserSheet> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching user sheets: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch user sheets",
        }
    })?;

    Ok(user_sheets)
}

/// Update a user sheet
pub async fn update_user_sheet<'e>(
    executor: impl PgExecutor<'e>,
    id: i32,
    data: UpdateUserSheet,
) -> Result<UserSheet, MyError> {
    // Build dynamic query based on which fields are provided
    let mut query_parts = Vec::new();
    let mut param_count = 1;

    if data.last_name.is_some() {
        query_parts.push(format!("\"last_name\" = ${}", param_count));
        param_count += 1;
    }
    if data.first_name.is_some() {
        query_parts.push(format!("\"first_name\" = ${}", param_count));
        param_count += 1;
    }
    if data.type_user.is_some() {
        query_parts.push(format!("\"type_user\" = ${}", param_count));
        param_count += 1;
    }
    if data.profile_picture.is_some() {
        query_parts.push(format!("\"profile_picture\" = ${}", param_count));
        param_count += 1;
    }

    if data.active.is_some() {
        query_parts.push(format!("\"active\" = ${}", param_count));
        param_count += 1;
    }

    if query_parts.is_empty() {
        return Err(MyError::DBErrors {
            entity: "No fields to update",
        });
    }

    let query_str = format!(
        r#"UPDATE "users_sheets" SET {} WHERE "id" = ${} RETURNING *"#,
        query_parts.join(", "),
        param_count
    );

    let mut query = sqlx::query_as(&query_str);

    if let Some(last_name) = data.last_name {
        query = query.bind(last_name);
    }
    if let Some(first_name) = data.first_name {
        query = query.bind(first_name);
    }
    if let Some(type_user) = data.type_user {
        query = query.bind(type_user);
    }
    if let Some(profile_picture) = data.profile_picture {
        query = query.bind(profile_picture);
    }
    if let Some(active) = data.active {
        query = query.bind(active);
    }
    query = query.bind(id);

    let user_sheet: UserSheet = query
        .fetch_one(executor)
        .await
        .map_err(|err| {
            eprintln!("Error updating user sheet: {:?}", err);
            MyError::DBErrors {
                entity: "Failed to update user sheet",
            }
        })?;

    Ok(user_sheet)
}

/// Delete a user sheet by ID
pub async fn delete_user_sheet<'e>(
    executor: impl PgExecutor<'e>,
    id: i32,
) -> Result<UserSheet, MyError> {
    let query = sqlx::query_as(
        r#"
        DELETE FROM "users_sheets"
        WHERE "id" = $1
        RETURNING *
        "#,
    )
    .bind(id);

    let user_sheet: UserSheet = query
        .fetch_one(executor)
        .await
        .map_err(|_| MyError::DBErrors {
            entity: "Failed to delete user sheet",
        })?;

    Ok(user_sheet)
}

/// Set user active status (for admin deactivation/activation)
pub async fn set_user_active_status<'e>(
    executor: impl PgExecutor<'e>,
    user_id: i32,
    active: bool,
) -> Result<UserSheet, MyError> {
    let query = sqlx::query_as(
        r#"
        UPDATE "users_sheets"
        SET "active" = $1
        WHERE "id" = $2
        RETURNING *
        "#,
    )
    .bind(active)
    .bind(user_id);

    let user_sheet: UserSheet = query
        .fetch_one(executor)
        .await
        .map_err(|_| MyError::DBErrors {
            entity: "Failed to update user active status",
        })?;

    Ok(user_sheet)
}

/// Link an account to a user sheet (for associating existing sheets with accounts)
pub async fn link_account_to_sheet<'e>(
    executor: impl PgExecutor<'e>,
    account_id: i32,
    user_sheet_id: i32,
) -> Result<(), MyError> {
    // Use INSERT ... ON CONFLICT DO NOTHING to avoid duplicates
    let query = sqlx::query(
        r#"
        INSERT INTO "accounts_users_sheets" ("account_id", "user_sheet_id")
        VALUES ($1, $2)
        ON CONFLICT DO NOTHING
        "#,
    )
    .bind(account_id)
    .bind(user_sheet_id);

    query.execute(executor).await.map_err(|err| {
        eprintln!("Error linking account to sheet: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to link account to sheet",
        }
    })?;

    Ok(())
}

/// Dissociate a user sheet from its account
pub async fn unlink_account_from_sheet<'e>(
    executor: impl PgExecutor<'e>,
    user_sheet_id: i32,
) -> Result<(), MyError> {
    let query = sqlx::query(
        r#"
        DELETE FROM "accounts_users_sheets"
        WHERE "user_sheet_id" = $1
        "#
    )
    .bind(user_sheet_id);

    query.execute(executor).await.map_err(|err| {
        eprintln!("Error unlinking sheet from account: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to unlink sheet from account",
        }
    })?;

    Ok(())
}
