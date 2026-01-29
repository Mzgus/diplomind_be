use crate::{errors::MyError, models::steps::*};
use sqlx::PgExecutor;

/// Create a new step
pub async fn create_step<'e>(
    executor: impl PgExecutor<'e>,
    data: CreateStep,
) -> Result<Step, MyError> {
    let step_order = data.step_order.unwrap_or(0);

    let query = sqlx::query_as(
        r#"
        INSERT INTO "steps" ("name", "description", "project_id", "step_order")
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(&data.name)
    .bind(&data.description)
    .bind(data.project_id)
    .bind(step_order);

    let step: Step = query.fetch_one(executor).await.map_err(|err| {
        eprintln!("Error creating step: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to create step",
        }
    })?;

    Ok(step)
}

/// Get a step by ID
pub async fn get_step_by_id<'e>(executor: impl PgExecutor<'e>, id: i32) -> Result<Step, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "steps"
        WHERE "id" = $1
        "#,
    )
    .bind(id);

    let step: Step = query
        .fetch_one(executor)
        .await
        .map_err(|_| MyError::NotFound { entity: "Step" })?;

    Ok(step)
}

/// Get all steps
pub async fn get_all_steps<'e>(executor: impl PgExecutor<'e>) -> Result<Vec<Step>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "steps"
        ORDER BY "project_id", "step_order"
        "#,
    );

    let steps: Vec<Step> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching steps: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch steps",
        }
    })?;

    Ok(steps)
}

/// Get steps by project ID
pub async fn get_steps_by_project_id<'e>(
    executor: impl PgExecutor<'e>,
    project_id: i32,
) -> Result<Vec<Step>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "steps"
        WHERE "project_id" = $1
        ORDER BY "step_order"
        "#,
    )
    .bind(project_id);

    let steps: Vec<Step> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching steps by project: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch steps",
        }
    })?;

    Ok(steps)
}

/// Update a step
pub async fn update_step<'e>(
    executor: impl PgExecutor<'e>,
    id: i32,
    data: UpdateStep,
) -> Result<Step, MyError> {
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
    if data.project_id.is_some() {
        query_parts.push(format!("\"project_id\" = ${}", param_count));
        param_count += 1;
    }
    if data.step_order.is_some() {
        query_parts.push(format!("\"step_order\" = ${}", param_count));
        param_count += 1;
    }

    if query_parts.is_empty() {
        return Err(MyError::DBErrors {
            entity: "No fields to update",
        });
    }

    query_parts.push("\"updated_at\" = NOW()".to_string());

    let query_str = format!(
        r#"UPDATE "steps" SET {} WHERE "id" = ${} RETURNING *"#,
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
    if let Some(project_id) = data.project_id {
        query = query.bind(project_id);
    }
    if let Some(step_order) = data.step_order {
        query = query.bind(step_order);
    }
    query = query.bind(id);

    let step: Step = query
        .fetch_one(executor)
        .await
        .map_err(|_| MyError::NotFound { entity: "Step" })?;

    Ok(step)
}

/// Delete a step by ID
pub async fn delete_step<'e>(executor: impl PgExecutor<'e>, id: i32) -> Result<Step, MyError> {
    let query = sqlx::query_as(
        r#"
        DELETE FROM "steps"
        WHERE "id" = $1
        RETURNING *
        "#,
    )
    .bind(id);

    let step: Step = query
        .fetch_one(executor)
        .await
        .map_err(|_| MyError::NotFound { entity: "Step" })?;

    Ok(step)
}
