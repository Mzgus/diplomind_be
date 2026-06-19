mod common;
use common::*;
use chrono::{Duration, Utc};
use diplomind::models::{JWTClaims, User};
use diplomind::services::TokenManager;

#[tokio::test]
async fn test_expired_token_rejected() {
    dotenv::dotenv().ok();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_manager = TokenManager::new(secret, "auth_cookie_diplomind".to_string());

    let user = User {
        account_id: 6,
        user_id: 6,
        user_lastname: "Moreau".to_string(),
        user_firstname: "Emma".to_string(),
        user_role: "student".to_string(),
        user_profilepicture: None,
        user_email: "emma.moreau@student.diplomind.fr".to_string(),
        user_active: true,
    };

    // Expired 1 hour ago
    let claims = JWTClaims {
        user,
        exp: (Utc::now() - Duration::hours(1)).timestamp() as usize,
    };

    let expired_token = token_manager.generate_access_token(claims).unwrap();
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:3000/verify_token")
        .header("Authorization", format!("Bearer {}", expired_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_tampered_token_rejected() {
    // Get a valid token first
    let valid_token = login_and_get_token("emma.moreau@student.diplomind.fr", "Password123").await;
    
    // Modify one character of the token (e.g. append a character or change the last one)
    let tampered_token = format!("{}a", valid_token);
    
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3000/verify_token")
        .header("Authorization", format!("Bearer {}", tampered_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_invalid_signature_token_rejected() {
    // Sign token with a different/fake secret key
    let wrong_token_manager = TokenManager::new(
        "completely_wrong_secret_key_which_should_fail_signature_checks".to_string(),
        "auth_cookie_diplomind".to_string(),
    );

    let user = User {
        account_id: 6,
        user_id: 6,
        user_lastname: "Moreau".to_string(),
        user_firstname: "Emma".to_string(),
        user_role: "student".to_string(),
        user_profilepicture: None,
        user_email: "emma.moreau@student.diplomind.fr".to_string(),
        user_active: true,
    };

    let claims = JWTClaims {
        user,
        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
    };

    let fake_token = wrong_token_manager.generate_access_token(claims).unwrap();
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:3000/verify_token")
        .header("Authorization", format!("Bearer {}", fake_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
}
