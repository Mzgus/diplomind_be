use crate::{errors::MyError, models::skills::*};
use sqlx::PgExecutor;

/// Create a new skill
pub async fn create_skill<'e>(
    executor: impl PgExecutor<'e>,
    data: CreateSkill,
) -> Result<Skill, MyError> {
    let query = sqlx::query_as(
        r#"
        INSERT INTO "skills" ("name", "description")
        VALUES ($1, $2)
        RETURNING *
        "#,
    )
    .bind(&data.name)
    .bind(&data.description);

    let skill: Skill = query.fetch_one(executor).await.map_err(|err| {
        eprintln!("Error creating skill: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to create skill",
        }
    })?;

    Ok(skill)
}

/// Get a skill by ID
pub async fn get_skill_by_id<'e>(executor: impl PgExecutor<'e>, id: i32) -> Result<Skill, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "skills"
        WHERE "id" = $1
        "#,
    )
    .bind(id);

    let skill: Skill = query
        .fetch_one(executor)
        .await
        .map_err(|_| MyError::NotFound { entity: "Skill" })?;

    Ok(skill)
}

/// Get all skills
pub async fn get_all_skills<'e>(executor: impl PgExecutor<'e>) -> Result<Vec<Skill>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "skills"
        ORDER BY "name"
        "#,
    );

    let skills: Vec<Skill> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching skills: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch skills",
        }
    })?;

    Ok(skills)
}

/// Update a skill
pub async fn update_skill<'e>(
    executor: impl PgExecutor<'e>,
    id: i32,
    data: UpdateSkill,
) -> Result<Skill, MyError> {
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
        r#"UPDATE "skills" SET {} WHERE "id" = ${} RETURNING *"#,
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

    let skill: Skill = query
        .fetch_one(executor)
        .await
        .map_err(|_| MyError::NotFound { entity: "Skill" })?;

    Ok(skill)
}

/// Delete a skill by ID
pub async fn delete_skill<'e>(executor: impl PgExecutor<'e>, id: i32) -> Result<Skill, MyError> {
    let query = sqlx::query_as(
        r#"
        DELETE FROM "skills"
        WHERE "id" = $1
        RETURNING *
        "#,
    )
    .bind(id);

    let skill: Skill = query
        .fetch_one(executor)
        .await
        .map_err(|_| MyError::NotFound { entity: "Skill" })?;

    Ok(skill)
}
