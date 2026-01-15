use crate::{errors::MyError, models::{User, PaginationParams, PaginatedResponse}};
use sqlx::PgExecutor;
use sqlx::Row;

/// Get a complete user by ID (joins users_sheets and users_auth)
pub async fn get_user_by_id<'e>(
    executor: impl PgExecutor<'e>,
    id: i32,
) -> Result<User, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT 
            us.id AS user_id, 
            us.last_name AS user_lastname, 
            us.first_name AS user_firstname, 
            us.type_user AS user_role, 
            us.profile_picture AS user_profilepicture, 
            ua.email AS user_email, 
            ua.pwd AS user_pwd
        FROM "users_sheets" as us 
        JOIN "users_auth" as ua ON us.id = ua.id_user_sheet
        WHERE us.id = $1
        "#,
    )
    .bind(id);

    let user: User = query.fetch_one(executor).await.map_err(|_| MyError::DBErrors {
        entity: "User not found",
    })?;

    Ok(user)
}

/// Get a complete user by email (joins users_sheets and users_auth)
pub async fn get_user_by_email<'e>(
    executor: impl PgExecutor<'e>,
    email: &str,
) -> Result<User, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT 
            us.id AS user_id, 
            us.last_name AS user_lastname, 
            us.first_name AS user_firstname, 
            us.type_user AS user_role, 
            us.profile_picture AS user_profilepicture, 
            ua.email AS user_email, 
            ua.pwd AS user_pwd
        FROM "users_sheets" as us 
        JOIN "users_auth" as ua ON us.id = ua.id_user_sheet
        WHERE ua.email = $1
        "#,
    )
    .bind(email);

    let user: User = query.fetch_one(executor).await.map_err(|_| MyError::DBErrors {
        entity: "User not found",
    })?;

    Ok(user)
}

/// Get all complete users (joins users_sheets and users_auth)
pub async fn get_all_users<'e>(
    executor: impl PgExecutor<'e>,
) -> Result<Vec<User>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT 
            us.id AS user_id, 
            us.last_name AS user_lastname, 
            us.first_name AS user_firstname, 
            us.type_user AS user_role, 
            us.profile_picture AS user_profilepicture, 
            ua.email AS user_email, 
            ua.pwd AS user_pwd
        FROM "users_sheets" as us
        JOIN "users_auth" as ua ON us.id = ua.id_user_sheet
        ORDER BY us.last_name, us.first_name
        "#,
    );

    let users: Vec<User> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching users: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch users",
        }
    })?;

    Ok(users)
}

/// Get paginated users
pub async fn get_all_users_paginated<'e>(
    executor: impl PgExecutor<'e> + Copy,
    params: PaginationParams,
) -> Result<PaginatedResponse<User>, MyError> {
    // Get total count
    let count_query = sqlx::query(
        r#"
        SELECT COUNT(*) 
        FROM "users_sheets" as us
        JOIN "users_auth" as ua ON us.id = ua.id_user_sheet
        "#,
    );
    
    let total: i64 = count_query.fetch_one(executor).await
        .map_err(|err| {
            eprintln!("Error fetching count: {:?}", err);
            MyError::DBErrors { entity: "Failed to count users" }
        })?
        .get(0);

    // Get paginated data
    let query = sqlx::query_as(
        r#"
        SELECT 
            us.id AS user_id, 
            us.last_name AS user_lastname, 
            us.first_name AS user_firstname, 
            us.type_user AS user_role, 
            us.profile_picture AS user_profilepicture, 
            ua.email AS user_email, 
            ua.pwd AS user_pwd
        FROM "users_sheets" as us
        JOIN "users_auth" as ua ON us.id = ua.id_user_sheet
        ORDER BY us.last_name, us.first_name
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(params.limit())
    .bind(params.offset());

    let users: Vec<User> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching users: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch users",
        }
    })?;
    
    let total_pages = (total as f64 / params.per_page as f64).ceil() as u32;

    Ok(PaginatedResponse {
        data: users,
        page: params.page,
        per_page: params.per_page,
        total,
        total_pages,
    })
}
