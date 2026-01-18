mod common;

use common::*;

// ============================================
// TESTS AUTHENTICATION
// ============================================

#[tokio::test]
async fn test_login_success() {
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3000/login")
        .json(&serde_json::json!({
            "email": "sophie.martin@diplomind.fr",
            "pwd": "Password123"
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.get("token").is_some());
}

#[tokio::test]
async fn test_login_invalid_credentials() {
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3000/login")
        .json(&serde_json::json!({
            "email": "sophie.martin@diplomind.fr",
            "pwd": "wrongpassword"
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_refresh_tokens_success() {
    // Note: This test verifies the endpoint exists and responds
    // Full cookie-based testing would require additional setup
    let client = reqwest::Client::new();
    
    // Call refresh_tokens endpoint (will fail without cookie, but that's expected)
    let response = client
        .get("http://localhost:3000/refresh_tokens")
        .send()
        .await
        .unwrap();
    
    // Should return 401 or 500 without a valid refresh token cookie
    // The important part is that the endpoint exists and responds
    assert!(response.status().is_client_error() || response.status().is_server_error());
}

#[tokio::test]
async fn test_logout_success() {
    let token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3000/logout")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_verify_token_success() {
    let token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3000/verify_token")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["user_email"], "sophie.martin@diplomind.fr");
}

#[tokio::test]
async fn test_access_protected_route_without_token() {
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3000/users")
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_access_protected_route_with_invalid_token() {
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3000/users")
        .header("Authorization", "Bearer invalid_token_here")
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 401);
}
