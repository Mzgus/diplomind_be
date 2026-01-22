use crate::errors::MyError;
use regex::Regex;

pub fn validate_email(email: &str) -> Result<(), MyError> {
    if email.len() > 255 {
        return Err(MyError::InvalidInput {
            input_type: "email too long (max 255 chars)",
        });
    }

    // Simple but effective email regex
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

    if !email_regex.is_match(email) {
        return Err(MyError::InvalidInput {
            input_type: "invalid email format",
        });
    }

    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), MyError> {
    if password.len() < 8 {
        return Err(MyError::InvalidInput {
            input_type: "password must be at least 8 characters",
        });
    }

    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());

    if !has_uppercase || !has_lowercase || !has_digit {
        return Err(MyError::InvalidInput {
            input_type: "password must contain at least 1 uppercase, 1 lowercase, and 1 digit",
        });
    }

    Ok(())
}
