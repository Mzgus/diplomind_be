use crate::{errors::MyError, models::user_classes::*};
use sqlx::PgExecutor;

/// Assign a user to a class
pub async fn assign_user_to_class<'e>(
    executor: impl PgExecutor<'e>,
    data: AssignUserToClass,
) -> Result<UserClass, MyError> {
    let query = sqlx::query_as(
        r#"
        INSERT INTO "user_classes" ("user_id", "class_id")
        VALUES ($1, $2)
        RETURNING *
        "#,
    )
    .bind(data.user_id)
    .bind(data.class_id);

    let user_class: UserClass = query.fetch_one(executor).await.map_err(|err| {
        eprintln!("Error assigning user to class: {:?}", err);
        
        if let sqlx::Error::Database(db_err) = &err {
            if db_err.is_unique_violation() {
                return MyError::AlreadyExists {
                    entity: "User-Class association",
                };
            }
        }
        
        MyError::DBErrors {
            entity: "Failed to assign user to class",
        }
    })?;

    Ok(user_class)
}

/// Get all classes for a user
pub async fn get_user_classes<'e>(
    executor: impl PgExecutor<'e>,
    user_id: i32,
) -> Result<Vec<UserClass>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "user_classes"
        WHERE "user_id" = $1
        "#,
    )
    .bind(user_id);

    let user_classes: Vec<UserClass> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching user classes: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch user classes",
        }
    })?;

    Ok(user_classes)
}

/// Get all users in a class
pub async fn get_class_users<'e>(
    executor: impl PgExecutor<'e>,
    class_id: i32,
) -> Result<Vec<UserClass>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "user_classes"
        WHERE "class_id" = $1
        "#,
    )
    .bind(class_id);

    let class_users: Vec<UserClass> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching class users: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch class users",
        }
    })?;

    Ok(class_users)
}

/// Remove a user from a class
pub async fn remove_user_from_class<'e>(
    executor: impl PgExecutor<'e>,
    user_id: i32,
    class_id: i32,
) -> Result<UserClass, MyError> {
    let query = sqlx::query_as(
        r#"
        DELETE FROM "user_classes"
        WHERE "user_id" = $1 AND "class_id" = $2
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(class_id);

    let user_class: UserClass = query.fetch_one(executor).await.map_err(|_| MyError::NotFound {
        entity: "User-Class association",
    })?;

    Ok(user_class)
}
