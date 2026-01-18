// Unit tests for validators

use diplomind::validators::{validate_email, validate_password};

// ============================================
// EMAIL VALIDATION TESTS
// ============================================

#[test]
fn test_validate_email_valid_simple() {
    assert!(validate_email("test@example.com").is_ok());
}

#[test]
fn test_validate_email_valid_with_subdomain() {
    assert!(validate_email("user@mail.example.com").is_ok());
}

#[test]
fn test_validate_email_valid_with_plus() {
    assert!(validate_email("user+tag@example.com").is_ok());
}

#[test]
fn test_validate_email_valid_with_dots() {
    assert!(validate_email("first.last@example.com").is_ok());
}

#[test]
fn test_validate_email_valid_with_numbers() {
    assert!(validate_email("user123@example.com").is_ok());
}

#[test]
fn test_validate_email_invalid_no_at() {
    assert!(validate_email("userexample.com").is_err());
}

#[test]
fn test_validate_email_invalid_no_domain() {
    assert!(validate_email("user@").is_err());
}

#[test]
fn test_validate_email_invalid_no_local() {
    assert!(validate_email("@example.com").is_err());
}

#[test]
fn test_validate_email_invalid_multiple_at() {
    assert!(validate_email("user@@example.com").is_err());
}

#[test]
fn test_validate_email_invalid_spaces() {
    assert!(validate_email("user @example.com").is_err());
}

#[test]
fn test_validate_email_invalid_empty() {
    assert!(validate_email("").is_err());
}

#[test]
fn test_validate_email_invalid_only_at() {
    assert!(validate_email("@").is_err());
}

// ============================================
// PASSWORD VALIDATION TESTS
// ============================================

#[test]
fn test_validate_password_valid_simple() {
    assert!(validate_password("Password123").is_ok());
}

#[test]
fn test_validate_password_valid_with_special_chars() {
    assert!(validate_password("P@ssw0rd!").is_ok());
}

#[test]
fn test_validate_password_valid_long() {
    assert!(validate_password("VeryLongPassword123WithManyCharacters").is_ok());
}

#[test]
fn test_validate_password_invalid_too_short() {
    assert!(validate_password("Pass1").is_err());
}

#[test]
fn test_validate_password_invalid_no_uppercase() {
    assert!(validate_password("password123").is_err());
}

#[test]
fn test_validate_password_invalid_no_lowercase() {
    assert!(validate_password("PASSWORD123").is_err());
}

#[test]
fn test_validate_password_invalid_no_digit() {
    assert!(validate_password("PasswordOnly").is_err());
}

#[test]
fn test_validate_password_invalid_empty() {
    assert!(validate_password("").is_err());
}

#[test]
fn test_validate_password_invalid_only_letters() {
    assert!(validate_password("OnlyLetters").is_err());
}

#[test]
fn test_validate_password_invalid_only_numbers() {
    assert!(validate_password("12345678").is_err());
}

#[test]
fn test_validate_password_valid_min_length() {
    // Exactly 8 characters with all requirements
    assert!(validate_password("Pass1234").is_ok());
}

#[test]
fn test_validate_password_valid_with_unicode() {
    assert!(validate_password("Pässwörd1").is_ok());
}
