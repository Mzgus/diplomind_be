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

        if let sqlx::Error::Database(db_err) = &err {
            if db_err.is_unique_violation() {
                return MyError::AlreadyExists {
                    entity: "User-Course association",
                };
            }
        }

        MyError::DBErrors {
            entity: "Failed to assign user to course",
        }
    })?;

    Ok(user_course)
}

/// Get all courses for a user (returns full Course objects)
pub async fn get_user_courses<'e>(
    executor: impl PgExecutor<'e>,
    user_id: i32,
) -> Result<Vec<crate::models::courses::Course>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT DISTINCT c.*
        FROM courses c
        LEFT JOIN user_courses uc ON c.id = uc.course_id
        LEFT JOIN course_classes cc ON c.id = cc.course_id
        LEFT JOIN user_classes ucls ON cc.class_id = ucls.class_id
        WHERE uc.user_id = $1 OR ucls.user_id = $1
        ORDER BY c.name
        "#,
    )
    .bind(user_id);

    let courses: Vec<crate::models::courses::Course> =
        query.fetch_all(executor).await.map_err(|err| {
            eprintln!("Error fetching user courses: {:?}", err);
            MyError::DBErrors {
                entity: "Failed to fetch user courses",
            }
        })?;

    Ok(courses)
}

/// Get all steps accessible to a user (via their enrolled courses → projects → steps)
pub async fn get_user_steps<'e>(
    executor: impl PgExecutor<'e>,
    user_id: i32,
) -> Result<Vec<crate::models::steps::Step>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT DISTINCT s.*
        FROM steps s
        JOIN projects p ON s.project_id = p.id
        JOIN courses c ON p.course_id = c.id
        LEFT JOIN user_courses uc ON c.id = uc.course_id
        LEFT JOIN course_classes cc ON c.id = cc.course_id
        LEFT JOIN user_classes ucls ON cc.class_id = ucls.class_id
        WHERE uc.user_id = $1 OR ucls.user_id = $1
        ORDER BY s.name
        "#,
    )
    .bind(user_id);

    let steps: Vec<crate::models::steps::Step> =
        query.fetch_all(executor).await.map_err(|err| {
            eprintln!("Error fetching user steps: {:?}", err);
            MyError::DBErrors {
                entity: "Failed to fetch user steps",
            }
        })?;

    Ok(steps)
}

/// Get all skills accessible to a user (via their enrolled courses → course_skills)
pub async fn get_user_skills<'e>(
    executor: impl PgExecutor<'e>,
    user_id: i32,
) -> Result<Vec<crate::models::skills::Skill>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT DISTINCT sk.*
        FROM skills sk
        JOIN course_skills cs ON sk.id = cs.skill_id
        LEFT JOIN user_courses uc ON cs.course_id = uc.course_id
        LEFT JOIN course_classes cc ON cs.course_id = cc.course_id
        LEFT JOIN user_classes ucls ON cc.class_id = ucls.class_id
        WHERE uc.user_id = $1 OR ucls.user_id = $1
        ORDER BY sk.name
        "#,
    )
    .bind(user_id);

    let skills: Vec<crate::models::skills::Skill> =
        query.fetch_all(executor).await.map_err(|err| {
            eprintln!("Error fetching user skills: {:?}", err);
            MyError::DBErrors {
                entity: "Failed to fetch user skills",
            }
        })?;

    Ok(skills)
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

/// Check if a user-course association exists
pub async fn check_user_course<'e>(
    executor: impl PgExecutor<'e>,
    user_id: i32,
    course_id: i32,
) -> Result<bool, MyError> {
    let query = sqlx::query_as::<_, UserCourse>(
        r#"
        SELECT * FROM "user_courses"
        WHERE "user_id" = $1 AND "course_id" = $2
        "#,
    )
    .bind(user_id)
    .bind(course_id);

    match query.fetch_one(executor).await {
        Ok(_) => Ok(true),
        Err(sqlx::Error::RowNotFound) => Ok(false),
        Err(err) => {
            eprintln!("Error checking user course: {:?}", err);
            Err(MyError::DBErrors {
                entity: "Failed to check user course",
            })
        }
    }
}
