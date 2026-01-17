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

// ============================================
// TESTS RBAC
// ============================================

#[tokio::test]
async fn test_admin_can_deactivate_user() {
    // Login as admin (Sophie Martin - user_id 1)
    let token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    
    // Deactivate a student (Emma Moreau - user_id 6)
    let client = reqwest::Client::new();
    let response = client
        .patch("http://localhost:3000/admin/users/6/deactivate")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    
    // Reactivate for next tests
    client
        .patch("http://localhost:3000/admin/users/6/activate")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_student_cannot_deactivate_user() {
    // Login as student (Emma Moreau - user_id 6)
    let token = login_and_get_token("emma.moreau@student.diplomind.fr", "Password123").await;
    
    // Try to deactivate admin (should fail)
    let client = reqwest::Client::new();
    let response = client
        .patch("http://localhost:3000/admin/users/1/deactivate")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 401); // Unauthorize
}

// ============================================
// TESTS ADMIN SECURITY
// ============================================

#[tokio::test]
async fn test_deactivated_user_cannot_login() {
    // Login as admin
    let admin_token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    
    // Deactivate Louis (user_id 7)
    let client = reqwest::Client::new();
    client
        .patch("http://localhost:3000/admin/users/7/deactivate")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();
    
    // Try to login as Louis (should fail)
    let response = client
        .post("http://localhost:3000/login")
        .json(&serde_json::json!({
            "email": "louis.simon@student.diplomind.fr",
            "pwd": "Password123"
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 401);
    
    // Reactivate Louis for next tests
    client
        .patch("http://localhost:3000/admin/users/7/activate")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_revoke_all_sessions() {
    // Login as admin
    let token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3000/admin/security/revoke-all-sessions")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["success"], true);
}

// ============================================
// TESTS VALIDATION
// ============================================

#[test]
fn test_email_validation() {
    use diplomind::validators::validate_email;
    
    assert!(validate_email("test@example.com").is_ok());
    assert!(validate_email("user+tag@domain.co.uk").is_ok());
    assert!(validate_email("invalid").is_err());
    assert!(validate_email("@example.com").is_err());
}

#[test]
fn test_password_validation() {
    use diplomind::validators::validate_password;
    
    assert!(validate_password("Password123").is_ok());
    assert!(validate_password("Secure1Pass").is_ok());
    assert!(validate_password("short").is_err());
    assert!(validate_password("nouppercase1").is_err());
}

// ============================================
// TESTS PASSWORD HASHING
// ============================================

#[tokio::test]
async fn test_password_is_hashed_on_update() {
    let pool = get_test_pool().await;
    
    // Login as admin
    let token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    
    // Get auth ID for Sophie
    let auth_id: i32 = sqlx::query_scalar(
        "SELECT id FROM users_auth WHERE email = 'sophie.martin@diplomind.fr'"
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    
    // Update password
    let client = reqwest::Client::new();
    let response = client
        .patch(&format!("http://localhost:3000/users_auth/{}/password", auth_id))
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "pwd": "NewPassword456"
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    
    // Verify password is hashed in DB
    let hashed_pwd: String = sqlx::query_scalar(
        "SELECT pwd FROM users_auth WHERE id = $1"
    )
    .bind(auth_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    
    assert!(hashed_pwd.starts_with("$argon2"));
    assert_ne!(hashed_pwd, "NewPassword456");
    
    // Verify can login with new password
    let login_response = client
        .post("http://localhost:3000/login")
        .json(&serde_json::json!({
            "email": "sophie.martin@diplomind.fr",
            "pwd": "NewPassword456"
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(login_response.status(), 200);
    
    // Restore original password
    sqlx::query("UPDATE users_auth SET pwd = $1 WHERE id = $2")
        .bind("$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI")
        .bind(auth_id)
        .execute(&pool)
        .await
        .unwrap();
}
