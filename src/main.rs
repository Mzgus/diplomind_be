use diplomind::{MyError, handlers, services};
// use dotenvy::dotenv;
use dotenv::dotenv;
use poem::{
    EndpointExt, Route, Server, delete, get, listener::TcpListener, middleware::CookieJarManager,
    patch, post,
};
use sqlx::PgPool;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    dotenv().unwrap();
    let db_url = match std::env::var("DATABASE_URL") {
        Ok(res) => res,
        Err(_) => {
            return Err(std::io::Error::other(
                MyError::EnvVarNotSet {
                    entity: "DATABASE_URL",
                }
                .to_string(),
            ));
        }
    };
    let secret = match std::env::var("JWT_SECRET") {
        Ok(res) => res,
        Err(_) => {
            return Err(std::io::Error::other(
                MyError::EnvVarNotSet {
                    entity: "JWT_SECRET",
                }
                .to_string(),
            ));
        }
    };
    let cookie_name = match std::env::var("COOKIE_NAME") {
        Ok(res) => res,
        Err(_) => {
            return Err(std::io::Error::other(
                MyError::EnvVarNotSet {
                    entity: "COOKIE_NAME",
                }
                .to_string(),
            ));
        }
    };
    let token_manager = services::auth::TokenManager::new(secret, cookie_name);
    match PgPool::connect(&db_url).await {
        Ok(pool) => {
            println!("Database connection pool created successfully");
            use diplomind::middleware::jwt_auth::JwtAuth;

            // Create the main route with all endpoints
            let routes = Route::new()
                // Public routes (no authentication required)
                .at("/", get(test))
                .at("/login", post(handlers::auth::login))
                .at("/refresh_tokens", get(handlers::auth::refresh_tokens))
                // Protected routes (authentication required) - add middleware per route
                .at(
                    "/logout",
                    get(handlers::auth::logout).with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/verify_token",
                    get(handlers::auth::verify_token).with(JwtAuth::new(token_manager.clone())),
                )
                // users routes (complete user information with JOIN)
                .at(
                    "/users",
                    get(handlers::users::get_all_users).with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/users/:id",
                    get(handlers::users::get_user_by_id).with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/users/email/:email",
                    get(handlers::users::get_user_by_email)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                // users_sheets CRUD routes
                .at(
                    "/users_sheets",
                    get(handlers::users_sheets::get_all_user_sheets)
                        .post(handlers::users_sheets::create_user_sheet)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/users_sheets/:id",
                    get(handlers::users_sheets::get_user_sheet)
                        .put(handlers::users_sheets::update_user_sheet)
                        .delete(handlers::users_sheets::delete_user_sheet)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                // users_auth CRUD routes
                .at(
                    "/users_auth",
                    post(handlers::users_auth::create_user_auth)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/users_auth/:id",
                    get(handlers::users_auth::get_user_auth)
                        .delete(handlers::users_auth::delete_user_auth)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/users_auth/:id/email",
                    patch(handlers::users_auth::update_user_auth_email)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/users_auth/:id/password",
                    patch(handlers::users_auth::update_user_auth_password)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                // Admin security routes
                .at(
                    "/admin/users/:id/deactivate",
                    patch(handlers::admin::deactivate_user)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/admin/users/:id/activate",
                    patch(handlers::admin::activate_user).with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/admin/security/revoke-all-sessions",
                    post(handlers::admin::revoke_all_sessions)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                // Classes routes
                .at(
                    "/classes",
                    get(handlers::classes::get_all_classes)
                        .post(handlers::classes::create_class)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/classes/:id",
                    get(handlers::classes::get_class)
                        .put(handlers::classes::update_class)
                        .delete(handlers::classes::delete_class)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                // Skills routes
                .at(
                    "/skills",
                    get(handlers::skills::get_all_skills)
                        .post(handlers::skills::create_skill)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/skills/:id",
                    get(handlers::skills::get_skill)
                        .put(handlers::skills::update_skill)
                        .delete(handlers::skills::delete_skill)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                // Courses routes
                .at(
                    "/courses",
                    get(handlers::courses::get_all_courses)
                        .post(handlers::courses::create_course)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/courses/:id",
                    get(handlers::courses::get_course)
                        .put(handlers::courses::update_course)
                        .delete(handlers::courses::delete_course)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                // Projects routes
                .at(
                    "/projects",
                    get(handlers::projects::get_all_projects)
                        .post(handlers::projects::create_project)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/projects/:id",
                    get(handlers::projects::get_project)
                        .put(handlers::projects::update_project)
                        .delete(handlers::projects::delete_project)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/courses/:id/projects",
                    get(handlers::projects::get_projects_by_course)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                // Student Dashboard: Get my projects
                .at(
                    "/users/:id/projects",
                    get(handlers::projects::get_student_projects)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                // Steps routes
                .at(
                    "/steps",
                    get(handlers::steps::get_all_steps)
                        .post(handlers::steps::create_step)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/steps/:id",
                    get(handlers::steps::get_step)
                        .put(handlers::steps::update_step)
                        .delete(handlers::steps::delete_step)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/projects/:id/steps",
                    get(handlers::steps::get_steps_by_project)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                // User-Classes association routes
                .at(
                    "/user-classes",
                    post(handlers::user_classes::assign_user_to_class)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/users/:id/classes",
                    get(handlers::user_classes::get_user_classes)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                // Teacher Dashboard: Get my classes
                .at(
                    "/teachers/:id/classes",
                    get(handlers::classes::get_teacher_classes)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/classes/:id/users",
                    get(handlers::user_classes::get_class_users)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/users/:user_id/classes/:class_id",
                    delete(handlers::user_classes::remove_user_from_class)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                // User-Courses association routes
                .at(
                    "/user-courses",
                    post(handlers::user_courses::assign_user_to_course)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/users/:id/courses",
                    get(handlers::user_courses::get_user_courses)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/courses/:id/users",
                    get(handlers::user_courses::get_course_users)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/users/:user_id/courses/:course_id",
                    delete(handlers::user_courses::remove_user_from_course)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                // Student Dashboard: Get validations for a course
                .at(
                    "/users/:user_id/courses/:course_id/validations",
                    get(handlers::skill_validations::get_student_course_validations)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                // Course-Classes association routes
                .at(
                    "/course-classes",
                    post(handlers::course_classes::link_course_to_class)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/courses/:id/classes",
                    get(handlers::course_classes::get_course_classes)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/classes/:id/courses",
                    get(handlers::course_classes::get_class_courses)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/courses/:course_id/classes/:class_id",
                    delete(handlers::course_classes::unlink_course_from_class)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                // Course-Skills association routes
                .at(
                    "/course-skills",
                    post(handlers::course_skills::link_skill_to_course)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/courses/:id/skills",
                    get(handlers::course_skills::get_course_skills)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/skills/:id/courses",
                    get(handlers::course_skills::get_skill_courses)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/courses/:course_id/skills/:skill_id",
                    delete(handlers::course_skills::unlink_skill_from_course)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                // Step-Skills association routes
                .at(
                    "/step-skills",
                    post(handlers::step_skills::link_skill_to_step)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/steps/:id/skills",
                    get(handlers::step_skills::get_step_skills)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/skills/:id/steps",
                    get(handlers::step_skills::get_skill_steps)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/steps/:step_id/skills/:skill_id",
                    delete(handlers::step_skills::unlink_skill_from_step)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                // Skill Validations routes
                .at(
                    "/skill-validations",
                    post(handlers::skill_validations::create_validation)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/skill-validations/user/:user_id",
                    get(handlers::skill_validations::get_user_validations)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/skill-validations/pending",
                    get(handlers::skill_validations::get_pending_validations)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .at(
                    "/skill-validations/:user_id/:skill_id",
                    get(handlers::skill_validations::get_validation)
                        .patch(handlers::skill_validations::update_validation_status)
                        .delete(handlers::skill_validations::delete_validation)
                        .with(JwtAuth::new(token_manager.clone())),
                )
                .data(pool)
                .data(token_manager)
                .with(CookieJarManager::new());

            Server::new(TcpListener::bind("0.0.0.0:3000"))
                .run(routes)
                .await?;
            return Ok(());
        }
        Err(e) => {
            eprintln!("Failed to create database connection pool: {}", e);
            return Err(std::io::Error::other(
                "Database connection pool creation failed",
            ));
        }
    }
}

#[poem::handler]
fn test() -> &'static str {
    "hello"
}
