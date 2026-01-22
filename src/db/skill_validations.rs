use crate::errors::MyError;
use crate::models::skill_validations::{
    CreateValidationRequest, SkillValidation, StatusFilter, UpdateValidationStatus,
};
use sqlx::{PgExecutor, Postgres};

/// Create a new skill validation (teacher/admin only)
pub async fn create_validation<'e>(
    executor: impl PgExecutor<'e>,
    data: CreateValidationRequest,
) -> Result<SkillValidation, MyError> {
    let query = sqlx::query_as::<Postgres, SkillValidation>(
        r#"
        INSERT INTO skill_validations (user_id, skill_id, status, comment)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(data.user_id)
    .bind(data.skill_id)
    .bind(&data.status)
    .bind(&data.comment);

    match query.fetch_one(executor).await {
        Ok(validation) => Ok(validation),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            Err(MyError::AlreadyExists {
                entity: "Skill validation",
            })
        }
        Err(_) => Err(MyError::DBErrors {
            entity: "skill_validations",
        }),
    }
}

/// Get all validations for a specific user
pub async fn get_user_validations<'e>(
    executor: impl PgExecutor<'e>,
    user_id: i32,
    filter: Option<StatusFilter>,
) -> Result<Vec<SkillValidation>, MyError> {
    let mut query_str = String::from(
        r#"
        SELECT * FROM skill_validations
        WHERE user_id = $1
        "#,
    );

    if let Some(f) = &filter
        && f.status.is_some()
    {
        query_str.push_str(" AND status = $2");
    }

    query_str.push_str(" ORDER BY created_at DESC");

    let mut query = sqlx::query_as::<Postgres, SkillValidation>(&query_str).bind(user_id);

    if let Some(f) = filter
        && let Some(status) = f.status
    {
        query = query.bind(status);
    }

    query
        .fetch_all(executor)
        .await
        .map_err(|_| MyError::DBErrors {
            entity: "skill_validations",
        })
}

/// Get all pending validations (teacher/admin only)
pub async fn get_pending_validations<'e>(
    executor: impl PgExecutor<'e>,
) -> Result<Vec<SkillValidation>, MyError> {
    sqlx::query_as::<Postgres, SkillValidation>(
        r#"
        SELECT * FROM skill_validations
        WHERE status = 'pending'
        ORDER BY created_at ASC
        "#,
    )
    .fetch_all(executor)
    .await
    .map_err(|_| MyError::DBErrors {
        entity: "skill_validations",
    })
}

/// Update validation status (validate or reject)
pub async fn update_validation_status<'e>(
    executor: impl PgExecutor<'e>,
    user_id: i32,
    skill_id: i32,
    data: UpdateValidationStatus,
    validated_by: i32,
) -> Result<SkillValidation, MyError> {
    sqlx::query_as::<Postgres, SkillValidation>(
        r#"
        UPDATE skill_validations
        SET status = $1,
            comment = $2,
            validated_at = NOW(),
            validated_by = $3,
            updated_at = NOW()
        WHERE user_id = $4 AND skill_id = $5
        RETURNING *
        "#,
    )
    .bind(&data.status)
    .bind(&data.comment)
    .bind(validated_by)
    .bind(user_id)
    .bind(skill_id)
    .fetch_one(executor)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => MyError::NotFound {
            entity: "Skill validation",
        },
        _ => MyError::DBErrors {
            entity: "skill_validations",
        },
    })
}

/// Get a specific validation
pub async fn get_validation<'e>(
    executor: impl PgExecutor<'e>,
    user_id: i32,
    skill_id: i32,
) -> Result<SkillValidation, MyError> {
    sqlx::query_as::<Postgres, SkillValidation>(
        r#"
        SELECT * FROM skill_validations
        WHERE user_id = $1 AND skill_id = $2
        "#,
    )
    .bind(user_id)
    .bind(skill_id)
    .fetch_one(executor)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => MyError::NotFound {
            entity: "Skill validation",
        },
        _ => MyError::DBErrors {
            entity: "skill_validations",
        },
    })
}

/// Delete a validation (admin only)
pub async fn delete_validation<'e>(
    executor: impl PgExecutor<'e>,
    user_id: i32,
    skill_id: i32,
) -> Result<SkillValidation, MyError> {
    sqlx::query_as::<Postgres, SkillValidation>(
        r#"
        DELETE FROM skill_validations
        WHERE user_id = $1 AND skill_id = $2
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(skill_id)
    .fetch_one(executor)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => MyError::NotFound {
            entity: "Skill validation",
        },
        _ => MyError::DBErrors {
            entity: "skill_validations",
        },
    })
}

/// Get validations for a student in a specific course
pub async fn get_student_course_validations<'e>(
    executor: impl PgExecutor<'e>,
    user_id: i32,
    course_id: i32,
) -> Result<Vec<crate::models::skill_validations::SkillValidationWithDetails>, MyError> {
    sqlx::query_as::<Postgres, crate::models::skill_validations::SkillValidationWithDetails>(
        r#"
        SELECT sv.*, s.name as skill_name, s.description as skill_description
        FROM skill_validations sv
        JOIN skills s ON sv.skill_id = s.id
        JOIN course_skills cs ON s.id = cs.skill_id
        WHERE sv.user_id = $1 AND cs.course_id = $2
        "#,
    )
    .bind(user_id)
    .bind(course_id)
    .fetch_all(executor)
    .await
    .map_err(|_| MyError::DBErrors {
        entity: "skill_validations",
    })
}
