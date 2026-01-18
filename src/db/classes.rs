use crate::{errors::MyError, models::classes::*};
use sqlx::PgExecutor;

/// Create a new class
pub async fn create_class<'e>(
    executor: impl PgExecutor<'e>,
    data: CreateClass,
) -> Result<Class, MyError> {
    let query = sqlx::query_as(
        r#"
        INSERT INTO "classes" ("name")
        VALUES ($1)
        RETURNING *
        "#,
    )
    .bind(&data.name);

    let class: Class = query.fetch_one(executor).await.map_err(|err| {
        eprintln!("Error creating class: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to create class",
        }
    })?;

    Ok(class)
}

/// Get a class by ID
pub async fn get_class_by_id<'e>(
    executor: impl PgExecutor<'e>,
    id: i32,
) -> Result<Class, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "classes"
        WHERE "id" = $1
        "#,
    )
    .bind(id);

    let class: Class = query.fetch_one(executor).await.map_err(|_| MyError::NotFound {
        entity: "Class",
    })?;

    Ok(class)
}

/// Get all classes
pub async fn get_all_classes<'e>(
    executor: impl PgExecutor<'e>,
) -> Result<Vec<Class>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "classes"
        ORDER BY "name"
        "#,
    );

    let classes: Vec<Class> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching classes: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch classes",
        }
    })?;

    Ok(classes)
}

/// Update a class
pub async fn update_class<'e>(
    executor: impl PgExecutor<'e>,
    id: i32,
    data: UpdateClass,
) -> Result<Class, MyError> {
    // Build dynamic query based on which fields are provided
    let mut query_parts = Vec::new();
    let mut param_count = 1;

    if data.name.is_some() {
        query_parts.push(format!("\"name\" = ${}", param_count));
        param_count += 1;
    }

    if query_parts.is_empty() {
        return Err(MyError::DBErrors {
            entity: "No fields to update",
        });
    }

    // Always update updated_at
    query_parts.push(format!("\"updated_at\" = NOW()"));

    let query_str = format!(
        r#"UPDATE "classes" SET {} WHERE "id" = ${} RETURNING *"#,
        query_parts.join(", "),
        param_count
    );

    let mut query = sqlx::query_as(&query_str);

    if let Some(name) = data.name {
        query = query.bind(name);
    }
    query = query.bind(id);

    let class: Class = query.fetch_one(executor).await.map_err(|_| MyError::NotFound {
        entity: "Class",
    })?;

    Ok(class)
}

/// Delete a class by ID
pub async fn delete_class<'e>(
    executor: impl PgExecutor<'e>,
    id: i32,
) -> Result<Class, MyError> {
    let query = sqlx::query_as(
        r#"
        DELETE FROM "classes"
        WHERE "id" = $1
        RETURNING *
        "#,
    )
    .bind(id);

    let class: Class = query.fetch_one(executor).await.map_err(|_| MyError::NotFound {
        entity: "Class",
    })?;

    Ok(class)
}
