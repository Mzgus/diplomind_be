// Unit tests for token management

use chrono::Utc;
use diplomind::models::{JWTClaims, User};
use diplomind::services::TokenManager;

#[test]
fn test_generate_access_token_creates_valid_jwt() {
    let token_manager = TokenManager::new(
        "test_secret_key_for_unit_tests".to_string(),
        "test_cookie".to_string(),
    );

    let user = User {
        user_id: 1,
        user_lastname: "Test".to_string(),
        user_firstname: "User".to_string(),
        user_role: "admin".to_string(),
        user_profilepicture: "https://example.com/pic.jpg".to_string(),
        user_email: "test@example.com".to_string(),
        user_pwd: "hashed_password".to_string(),
        user_active: Some(true),
    };

    let claims = JWTClaims {
        user,
        exp: (Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
    };

    let token = token_manager.generate_access_token(claims).unwrap();

    // JWT should have 3 parts separated by dots
    let parts: Vec<&str> = token.split('.').collect();
    assert_eq!(parts.len(), 3);

    // Token should not be empty
    assert!(!token.is_empty());
}

#[test]
fn test_generate_refresh_token_creates_unique_tokens() {
    let token1 = TokenManager::generate_refresh_token().unwrap();
    let token2 = TokenManager::generate_refresh_token().unwrap();

    // Each refresh token should be unique
    assert_ne!(token1, token2);

    // Refresh tokens should be base64 encoded (44 chars for 32 bytes)
    assert_eq!(token1.len(), 44);
    assert_eq!(token2.len(), 44);
}

#[test]
fn test_generate_expiration_date_future() {
    let token_manager = TokenManager::new("test_secret".to_string(), "test_cookie".to_string());

    let duration = chrono::Duration::hours(1);
    let expiration = token_manager.generate_expiration_date(duration);

    // Expiration should be in the future
    assert!(expiration > Utc::now());
}

#[test]
fn test_verify_token_validity_not_expired() {
    let token_manager = TokenManager::new("test_secret".to_string(), "test_cookie".to_string());

    // Token expires in the future
    let future_date = Utc::now() + chrono::Duration::hours(1);
    assert!(token_manager.verify_token_validity(future_date));
}

#[test]
fn test_verify_token_validity_expired() {
    let token_manager = TokenManager::new("test_secret".to_string(), "test_cookie".to_string());

    // Token expired in the past
    let past_date = Utc::now() - chrono::Duration::hours(1);
    assert!(!token_manager.verify_token_validity(past_date));
}

#[test]
fn test_access_token_contains_claims() {
    let token_manager = TokenManager::new(
        "test_secret_key_for_unit_tests".to_string(),
        "test_cookie".to_string(),
    );

    let user = User {
        user_id: 42,
        user_lastname: "Doe".to_string(),
        user_firstname: "John".to_string(),
        user_role: "student".to_string(),
        user_profilepicture: "https://example.com/pic.jpg".to_string(),
        user_email: "john.doe@example.com".to_string(),
        user_pwd: "hashed".to_string(),
        user_active: Some(true),
    };

    let claims = JWTClaims {
        user,
        exp: (Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
    };

    let token = token_manager.generate_access_token(claims).unwrap();

    // Token should be a valid JWT format
    assert!(token.contains('.'));
    assert!(!token.is_empty());
}

#[test]
fn test_token_manager_new() {
    let secret = "my_secret_key".to_string();
    let cookie = "my_cookie".to_string();

    let token_manager = TokenManager::new(secret.clone(), cookie.clone());

    assert_eq!(token_manager.jwt_secret, secret);
    assert_eq!(token_manager.cookie_name, cookie);
}
