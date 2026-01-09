
// #[derive(thiserror::Error, Debug)]
// pub enum MyError {
//     #[error("generation failed")]
//     GenerationFailed,
//     #[error("fetch failed")]
//     FetchFailed,
//     #[error("environment variable not set")]
//     EnvVarNotSet(String)
// }


pub use thiserror::Error;
use poem::error::ParseCookieError;

#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("{entity} not found")]
    NotFound { entity: &'static str },

    #[error("Not a valid {input_type}")]
    InvalidInput { input_type: &'static str },

    #[error("Error while using the database : {entity}")]
    DBErrors { entity: &'static str },

    #[error("Error when generating {entity}")]
    GenerationFailed { entity: &'static str },

    #[error("Environment variable {entity} not set")]
    EnvVarNotSet { entity: &'static str },

    #[error(transparent)]
    CookieError(#[from] ParseCookieError), // To discuss on ticket

    #[error("Expiration date is expired")]
    TokenExpired,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Password hashing error: {0}")]
    PasswordHashError(String),
}

impl poem::error::ResponseError for MyError {
    fn status(&self) -> poem::http::StatusCode {
        use poem::http::StatusCode;
        match self {
            Self::NotFound { entity: _ } => StatusCode::NOT_FOUND,
            Self::InvalidInput { input_type: _ } => StatusCode::BAD_REQUEST,
            Self::DBErrors { entity: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            Self::GenerationFailed { entity: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            Self::EnvVarNotSet{ entity: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            Self::CookieError(_) => StatusCode::BAD_REQUEST,
            Self::TokenExpired => StatusCode::UNAUTHORIZED,
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::PasswordHashError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
