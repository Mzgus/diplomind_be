use crate::{errors::MyError, models::user_courses::*};
use sqlx::PgExecutor;

/// Assign a user to a course
pub async fn assign_user_to_course<'e>(
    executor: impl PgExecutor<'e>,
    data: AssignUserToCourse,
) -> Result<UserCourse, MyError> {
    let query = sqlx::query_as(
        r#"
        INSERT INTO "user_courses" ("user_id", "course_id")
        VALUES ($1, $2)
        RETURNING *
        "#,
    )
    .bind(data.user_id)
    .bind(data.course_id);

    let user_course: UserCourse = query.fetch_one(executor).await.map_err(|err| {
        eprintln!("Error assigning user to course: {:?}", err);

        if let sqlx::Error::Database(db_err) = &err
            && db_err.is_unique_violation()
        {
            return MyError::AlreadyExists {
                entity: "User-Course association",
            };
        }

        MyError::DBErrors {
            entity: "Failed to assign user to course",
        }
    })?;

    Ok(user_course)
}

/// Get all courses for a user
pub async fn get_user_courses<'e>(
    executor: impl PgExecutor<'e>,
    user_id: i32,
) -> Result<Vec<UserCourse>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "user_courses"
        WHERE "user_id" = $1
        "#,
    )
    .bind(user_id);

    let user_courses: Vec<UserCourse> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching user courses: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch user courses",
        }
    })?;

    Ok(user_courses)
}

/// Get all users in a course
pub async fn get_course_users<'e>(
    executor: impl PgExecutor<'e>,
    course_id: i32,
) -> Result<Vec<UserCourse>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "user_courses"
        WHERE "course_id" = $1
        "#,
    )
    .bind(course_id);

    let course_users: Vec<UserCourse> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching course users: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch course users",
        }
    })?;

    Ok(course_users)
}

/// Remove a user from a course
pub async fn remove_user_from_course<'e>(
    executor: impl PgExecutor<'e>,
    user_id: i32,
    course_id: i32,
) -> Result<UserCourse, MyError> {
    let query = sqlx::query_as(
        r#"
        DELETE FROM "user_courses"
        WHERE "user_id" = $1 AND "course_id" = $2
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(course_id);

    let user_course: UserCourse =
        query
            .fetch_one(executor)
            .await
            .map_err(|_| MyError::NotFound {
                entity: "User-Course association",
            })?;

    Ok(user_course)
}
