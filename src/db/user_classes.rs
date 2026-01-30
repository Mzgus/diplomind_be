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
) -> Result<Vec<crate::models::classes::Class>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT c.* FROM "classes" c
        JOIN "user_classes" uc ON c.id = uc.class_id
        WHERE uc.user_id = $1
        "#,
    )
    .bind(user_id);

    let classes: Vec<crate::models::classes::Class> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching user classes: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch user classes",
        }
    })?;

    Ok(classes)
}

/// Get all users in a class
pub async fn get_class_users<'e>(
    executor: impl PgExecutor<'e>,
    class_id: i32,
) -> Result<Vec<crate::models::users_sheets::UserSheet>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT us.* FROM "users_sheets" us
        JOIN "user_classes" uc ON us.id = uc.user_id
        WHERE uc.class_id = $1
        "#,
    )
    .bind(class_id);

    let users: Vec<crate::models::users_sheets::UserSheet> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching class users: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch class users",
        }
    })?;

    Ok(users)
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

    let user_class: UserClass = query
        .fetch_one(executor)
        .await
        .map_err(|_| MyError::NotFound {
            entity: "User-Class association",
        })?;

    Ok(user_class)
}
