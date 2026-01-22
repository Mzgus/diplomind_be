use crate::{errors::MyError, models::step_skills::*};
use sqlx::PgExecutor;

/// Link a skill to a step
pub async fn link_skill_to_step<'e>(
    executor: impl PgExecutor<'e>,
    data: LinkSkillToStep,
) -> Result<StepSkill, MyError> {
    let query = sqlx::query_as(
        r#"
        INSERT INTO "step_skills" ("step_id", "skill_id")
        VALUES ($1, $2)
        RETURNING *
        "#,
    )
    .bind(data.step_id)
    .bind(data.skill_id);

    let step_skill: StepSkill = query.fetch_one(executor).await.map_err(|err| {
        eprintln!("Error linking skill to step: {:?}", err);

        if let sqlx::Error::Database(db_err) = &err
            && db_err.is_unique_violation()
        {
            return MyError::AlreadyExists {
                entity: "Step-Skill association",
            };
        }

        MyError::DBErrors {
            entity: "Failed to link skill to step",
        }
    })?;

    Ok(step_skill)
}

/// Get all skills for a step
pub async fn get_step_skills<'e>(
    executor: impl PgExecutor<'e>,
    step_id: i32,
) -> Result<Vec<StepSkill>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "step_skills"
        WHERE "step_id" = $1
        "#,
    )
    .bind(step_id);

    let step_skills: Vec<StepSkill> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching step skills: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch step skills",
        }
    })?;

    Ok(step_skills)
}

/// Get all steps for a skill
pub async fn get_skill_steps<'e>(
    executor: impl PgExecutor<'e>,
    skill_id: i32,
) -> Result<Vec<StepSkill>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "step_skills"
        WHERE "skill_id" = $1
        "#,
    )
    .bind(skill_id);

    let skill_steps: Vec<StepSkill> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching skill steps: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch skill steps",
        }
    })?;

    Ok(skill_steps)
}

/// Unlink a skill from a step
pub async fn unlink_skill_from_step<'e>(
    executor: impl PgExecutor<'e>,
    step_id: i32,
    skill_id: i32,
) -> Result<StepSkill, MyError> {
    let query = sqlx::query_as(
        r#"
        DELETE FROM "step_skills"
        WHERE "step_id" = $1 AND "skill_id" = $2
        RETURNING *
        "#,
    )
    .bind(step_id)
    .bind(skill_id);

    let step_skill: StepSkill = query
        .fetch_one(executor)
        .await
        .map_err(|_| MyError::NotFound {
            entity: "Step-Skill association",
        })?;

    Ok(step_skill)
}
