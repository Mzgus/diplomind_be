use crate::{errors::MyError, models::course_skills::*};
use sqlx::PgExecutor;

/// Link a skill to a course
pub async fn link_skill_to_course<'e>(
    executor: impl PgExecutor<'e>,
    data: LinkSkillToCourse,
) -> Result<CourseSkill, MyError> {
    let query = sqlx::query_as(
        r#"
        INSERT INTO "course_skills" ("course_id", "skill_id")
        VALUES ($1, $2)
        ON CONFLICT (course_id, skill_id) DO NOTHING
        RETURNING *
        "#,
    )
    .bind(data.course_id)
    .bind(data.skill_id);

    let course_skill: CourseSkill = query.fetch_one(executor).await.map_err(|err| {
        eprintln!("Error linking skill to course: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to link skill to course",
        }
    })?;

    Ok(course_skill)
}

/// Get all skills for a course
pub async fn get_course_skills<'e>(
    executor: impl PgExecutor<'e>,
    course_id: i32,
) -> Result<Vec<CourseSkill>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "course_skills"
        WHERE "course_id" = $1
        ORDER BY "created_at" DESC
        "#,
    )
    .bind(course_id);

    let course_skills: Vec<CourseSkill> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching course skills: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch course skills",
        }
    })?;

    Ok(course_skills)
}

/// Get all courses for a skill
pub async fn get_skill_courses<'e>(
    executor: impl PgExecutor<'e>,
    skill_id: i32,
) -> Result<Vec<CourseSkill>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT * FROM "course_skills"
        WHERE "skill_id" = $1
        ORDER BY "created_at" DESC
        "#,
    )
    .bind(skill_id);

    let skill_courses: Vec<CourseSkill> = query.fetch_all(executor).await.map_err(|err| {
        eprintln!("Error fetching skill courses: {:?}", err);
        MyError::DBErrors {
            entity: "Failed to fetch skill courses",
        }
    })?;

    Ok(skill_courses)
}

/// Unlink a skill from a course
pub async fn unlink_skill_from_course<'e>(
    executor: impl PgExecutor<'e>,
    course_id: i32,
    skill_id: i32,
) -> Result<CourseSkill, MyError> {
    let query = sqlx::query_as(
        r#"
        DELETE FROM "course_skills"
        WHERE "course_id" = $1 AND "skill_id" = $2
        RETURNING *
        "#,
    )
    .bind(course_id)
    .bind(skill_id);

    let course_skill: CourseSkill = query.fetch_one(executor).await.map_err(|_| MyError::NotFound {
        entity: "Course-Skill association",
    })?;

    Ok(course_skill)
}
