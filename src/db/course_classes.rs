use crate::{errors::MyError, models::course_classes::*};
use sqlx::PgExecutor;

/// Link a course to a class
pub async fn link_course_to_class<'e>(
    executor: impl PgExecutor<'e>,
    data: LinkCourseToClass,
) -> Result<CourseClass, MyError> {
    let query = sqlx::query_as(
        r#"
        INSERT INTO "course_classes" ("course_id", "class_id")
        VALUES ($1, $2)
        RETURNING *
        "#,
    )
    .bind(data.course_id)
    .bind(data.class_id);

    let course_class: CourseClass = query.fetch_one(executor).await.map_err(|err| {
        eprintln!("Error linking course to class: {:?}", err);

        // Check if it's a unique constraint violation
        if let sqlx::Error::Database(db_err) = &err
            && db_err.is_unique_violation()
        {
            return MyError::AlreadyExists {
                entity: "Course-Class association",
            };
        }

        MyError::DBErrors {
            entity: "Failed to link course to class",
        }
    })?;

    Ok(course_class)
}

/// Get all classes for a course
pub async fn get_course_classes<'e>(
    executor: impl PgExecutor<'e>,
    course_id: i32,
) -> Result<Vec<CourseClass>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "course_classes"
        WHERE "course_id" = $1
        "#,
    )
    .bind(course_id);

    let course_classes: Vec<CourseClass> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching course classes: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch course classes",
        }
    })?;

    Ok(course_classes)
}

/// Get all courses for a class
pub async fn get_class_courses<'e>(
    executor: impl PgExecutor<'e>,
    class_id: i32,
) -> Result<Vec<CourseClass>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "course_classes"
        WHERE "class_id" = $1
        "#,
    )
    .bind(class_id);

    let class_courses: Vec<CourseClass> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching class courses: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch class courses",
        }
    })?;

    Ok(class_courses)
}

/// Unlink a course from a class
pub async fn unlink_course_from_class<'e>(
    executor: impl PgExecutor<'e>,
    course_id: i32,
    class_id: i32,
) -> Result<CourseClass, MyError> {
    let query = sqlx::query_as(
        r#"
        DELETE FROM "course_classes"
        WHERE "course_id" = $1 AND "class_id" = $2
        RETURNING *
        "#,
    )
    .bind(course_id)
    .bind(class_id);

    let course_class: CourseClass =
        query
            .fetch_one(executor)
            .await
            .map_err(|_| MyError::NotFound {
                entity: "Course-Class association",
            })?;

    Ok(course_class)
}
