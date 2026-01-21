use crate::{errors::MyError, models::projects::*};
use sqlx::PgExecutor;

/// Create a new project
pub async fn create_project<'e>(
    executor: impl PgExecutor<'e>,
    data: CreateProject,
) -> Result<Project, MyError> {
    let query = sqlx::query_as(
        r#"
        INSERT INTO "projects" ("name", "description", "course_id")
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(&data.name)
    .bind(&data.description)
    .bind(data.course_id);

    let project: Project = query.fetch_one(executor).await.map_err(|err| {
        eprintln!("Error creating project: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to create project",
        }
    })?;

    Ok(project)
}

/// Get a project by ID
pub async fn get_project_by_id<'e>(
    executor: impl PgExecutor<'e>,
    id: i32,
) -> Result<Project, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "projects"
        WHERE "id" = $1
        "#,
    )
    .bind(id);

    let project: Project = query.fetch_one(executor).await.map_err(|_| MyError::NotFound {
        entity: "Project",
    })?;

    Ok(project)
}

/// Get all projects
pub async fn get_all_projects<'e>(
    executor: impl PgExecutor<'e>,
) -> Result<Vec<Project>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "projects"
        ORDER BY "name"
        "#,
    );

    let projects: Vec<Project> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching projects: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch projects",
        }
    })?;

    Ok(projects)
}

/// Get projects by course ID
pub async fn get_projects_by_course_id<'e>(
    executor: impl PgExecutor<'e>,
    course_id: i32,
) -> Result<Vec<Project>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "projects"
        WHERE "course_id" = $1
        ORDER BY "name"
        "#,
    )
    .bind(course_id);

    let projects: Vec<Project> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching projects by course: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch projects",
        }
    })?;

    Ok(projects)
}

/// Update a project
pub async fn update_project<'e>(
    executor: impl PgExecutor<'e>,
    id: i32,
    data: UpdateProject,
) -> Result<Project, MyError> {
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
    if data.course_id.is_some() {
        query_parts.push(format!("\"course_id\" = ${}", param_count));
        param_count += 1;
    }

    if query_parts.is_empty() {
        return Err(MyError::DBErrors {
            entity: "No fields to update",
        });
    }

    query_parts.push(format!("\"updated_at\" = NOW()"));

    let query_str = format!(
        r#"UPDATE "projects" SET {} WHERE "id" = ${} RETURNING *"#,
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
    if let Some(course_id) = data.course_id {
        query = query.bind(course_id);
    }
    query = query.bind(id);

    let project: Project = query.fetch_one(executor).await.map_err(|_| MyError::NotFound {
        entity: "Project",
    })?;

    Ok(project)
}

/// Delete a project by ID
pub async fn delete_project<'e>(
    executor: impl PgExecutor<'e>,
    id: i32,
) -> Result<Project, MyError> {
    let query = sqlx::query_as(
        r#"
        DELETE FROM "projects"
        WHERE "id" = $1
        RETURNING *
        "#,
    )
    .bind(id);

    let project: Project = query.fetch_one(executor).await.map_err(|_| MyError::NotFound {
        entity: "Project",
    })?;

    Ok(project)
}

/// Get all projects for a student (where they are enrolled in the course)
pub async fn get_student_projects<'e>(
    executor: impl PgExecutor<'e>,
    user_id: i32,
) -> Result<Vec<Project>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT p.*
        FROM projects p
        JOIN courses c ON p.course_id = c.id
        JOIN user_courses uc ON c.id = uc.course_id
        WHERE uc.user_id = $1
        ORDER BY p.name ASC
        "#,
    )
    .bind(user_id);

    let projects: Vec<Project> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching student projects: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch student projects",
        }
    })?;

    Ok(projects)
}
