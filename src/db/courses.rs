use crate::{errors::MyError, models::courses::*};
use sqlx::PgExecutor;

/// Create a new course
pub async fn create_course<'e>(
    executor: impl PgExecutor<'e>,
    data: CreateCourse,
) -> Result<Course, MyError> {
    let query = sqlx::query_as(
        r#"
        INSERT INTO "courses" ("name", "description")
        VALUES ($1, $2)
        RETURNING *
        "#,
    )
    .bind(&data.name)
    .bind(&data.description);

    let course: Course = query.fetch_one(executor).await.map_err(|err| {
        eprintln!("Error creating course: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to create course",
        }
    })?;

    Ok(course)
}

/// Get a course by ID
pub async fn get_course_by_id<'e>(
    executor: impl PgExecutor<'e>,
    id: i32,
) -> Result<Course, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "courses"
        WHERE "id" = $1
        "#,
    )
    .bind(id);

    let course: Course = query
        .fetch_one(executor)
        .await
        .map_err(|_| MyError::NotFound { entity: "Course" })?;

    Ok(course)
}

/// Get all courses
pub async fn get_all_courses<'e>(executor: impl PgExecutor<'e>) -> Result<Vec<Course>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "courses"
        ORDER BY "name"
        "#,
    );

    let courses: Vec<Course> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching courses: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch courses",
        }
    })?;

    Ok(courses)
}

/// Update a course
pub async fn update_course<'e>(
    executor: impl PgExecutor<'e>,
    id: i32,
    data: UpdateCourse,
) -> Result<Course, MyError> {
    let mut query_parts = Vec::new();
    let mut param_count = 1;

    if data.name.is_some() {
        query_parts.push(format!("\"name\" = ${}", param_count));
        param_count += 1;
    }
    if data.description.is_some() {
        query_parts.push(format!("\"description\" = ${}", param_count));
        param_count += 1;
    }

    if query_parts.is_empty() {
        return Err(MyError::DBErrors {
            entity: "No fields to update",
        });
    }

    query_parts.push("\"updated_at\" = NOW()".to_string());

    let query_str = format!(
        r#"UPDATE "courses" SET {} WHERE "id" = ${} RETURNING *"#,
        query_parts.join(", "),
        param_count
    );

    let mut query = sqlx::query_as(&query_str);

    if let Some(name) = data.name {
        query = query.bind(name);
    }
    if let Some(description) = data.description {
        query = query.bind(description);
    }
    query = query.bind(id);

    let course: Course = query
        .fetch_one(executor)
        .await
        .map_err(|_| MyError::NotFound { entity: "Course" })?;

    Ok(course)
}

/// Delete a course by ID
pub async fn delete_course<'e>(executor: impl PgExecutor<'e>, id: i32) -> Result<Course, MyError> {
    let query = sqlx::query_as(
        r#"
        DELETE FROM "courses"
        WHERE "id" = $1
        RETURNING *
        "#,
    )
    .bind(id);

    let course: Course = query
        .fetch_one(executor)
        .await
        .map_err(|_| MyError::NotFound { entity: "Course" })?;

    Ok(course)
}
