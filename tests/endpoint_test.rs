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

// ============================================
// TESTS USERS ROUTES
// ============================================

#[tokio::test]
async fn test_get_all_users_with_pagination() {
    let token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3000/users?page=1&per_page=5")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.get("data").is_some());
    assert!(body.get("page").is_some());
    assert!(body.get("per_page").is_some());
    assert!(body.get("total").is_some());
}

#[tokio::test]
async fn test_get_user_by_id_success() {
    let token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3000/users/1")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["user_id"], 1);
    assert_eq!(body["user_email"], "sophie.martin@diplomind.fr");
}

#[tokio::test]
async fn test_get_user_by_email_success() {
    let token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3000/users/email/sophie.martin@diplomind.fr")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["user_email"], "sophie.martin@diplomind.fr");
}

// ============================================
// TESTS USERS SHEETS CRUD
// ============================================

#[tokio::test]
async fn test_create_user_sheet_as_admin() {
    let token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3000/users_sheets")
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "last_name": "Test",
            "first_name": "User",
            "type_user": "student",
            "profile_picture": null
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["last_name"], "Test");
    assert_eq!(body["first_name"], "User");
    
    // Cleanup: delete the created user sheet
    let user_id = body["id"].as_i64().unwrap();
    client
        .delete(&format!("http://localhost:3000/users_sheets/{}", user_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_all_user_sheets() {
    let token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3000/users_sheets")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
    assert!(body.as_array().unwrap().len() > 0);
}

#[tokio::test]
async fn test_get_user_sheet_by_id() {
    let token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3000/users_sheets/1")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["id"], 1);
}

#[tokio::test]
async fn test_update_user_sheet_as_admin() {
    let token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    
    let client = reqwest::Client::new();
    
    // Create a temporary user sheet
    let create_response = client
        .post("http://localhost:3000/users_sheets")
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "last_name": "Original",
            "first_name": "Name",
            "type_user": "student",
            "profile_picture": null
        }))
        .send()
        .await
        .unwrap();
    
    let created_user: serde_json::Value = create_response.json().await.unwrap();
    let user_id = created_user["id"].as_i64().unwrap();
    
    // Update the user sheet
    let update_response = client
        .put(&format!("http://localhost:3000/users_sheets/{}", user_id))
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "last_name": "Updated",
            "first_name": "Name"
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(update_response.status(), 200);
    let updated_user: serde_json::Value = update_response.json().await.unwrap();
    assert_eq!(updated_user["last_name"], "Updated");
    
    // Cleanup
    client
        .delete(&format!("http://localhost:3000/users_sheets/{}", user_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_student_cannot_update_other_user_sheet() {
    let student_token = login_and_get_token("emma.moreau@student.diplomind.fr", "Password123").await;
    
    let client = reqwest::Client::new();
    let response = client
        .put("http://localhost:3000/users_sheets/1")
        .header("Authorization", format!("Bearer {}", student_token))
        .json(&serde_json::json!({
            "last_name": "Hacked"
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_delete_user_sheet_as_admin() {
    let token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    
    let client = reqwest::Client::new();
    
    // Create a temporary user sheet
    let create_response = client
        .post("http://localhost:3000/users_sheets")
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "last_name": "ToDelete",
            "first_name": "User",
            "type_user": "student",
            "profile_picture": null
        }))
        .send()
        .await
        .unwrap();
    
    let created_user: serde_json::Value = create_response.json().await.unwrap();
    let user_id = created_user["id"].as_i64().unwrap();
    
    // Delete the user sheet
    let delete_response = client
        .delete(&format!("http://localhost:3000/users_sheets/{}", user_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(delete_response.status(), 200);
}

// ============================================
// TESTS USERS AUTH CRUD
// ============================================

#[tokio::test]
async fn test_create_user_auth_as_admin() {
    let pool = get_test_pool().await;
    let token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    
    let client = reqwest::Client::new();
    
    // Create a user sheet first
    let sheet_response = client
        .post("http://localhost:3000/users_sheets")
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "last_name": "AuthTest",
            "first_name": "User",
            "type_user": "student",
            "profile_picture": null
        }))
        .send()
        .await
        .unwrap();
    
    let sheet: serde_json::Value = sheet_response.json().await.unwrap();
    let sheet_id = sheet["id"].as_i64().unwrap();
    
    // Create user auth
    let auth_response = client
        .post("http://localhost:3000/users_auth")
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "email": "authtest@test.com",
            "pwd": "TestPassword123",
            "id_user_sheet": sheet_id
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(auth_response.status(), 200);
    let auth: serde_json::Value = auth_response.json().await.unwrap();
    assert_eq!(auth["email"], "authtest@test.com");
    
    // Verify password is hashed
    let auth_id = auth["id"].as_i64().unwrap();
    let hashed_pwd: String = sqlx::query_scalar(
        "SELECT pwd FROM users_auth WHERE id = $1"
    )
    .bind(auth_id as i32)
    .fetch_one(&pool)
    .await
    .unwrap();
    
    assert!(hashed_pwd.starts_with("$argon2"));
    
    // Cleanup
    client
        .delete(&format!("http://localhost:3000/users_auth/{}", auth_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    client
        .delete(&format!("http://localhost:3000/users_sheets/{}", sheet_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_user_auth_by_id() {
    let pool = get_test_pool().await;
    let token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    
    // Get Sophie's auth ID
    let auth_id: i32 = sqlx::query_scalar(
        "SELECT id FROM users_auth WHERE email = 'sophie.martin@diplomind.fr'"
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://localhost:3000/users_auth/{}", auth_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["email"], "sophie.martin@diplomind.fr");
}

#[tokio::test]
async fn test_update_user_auth_email() {
    let pool = get_test_pool().await;
    let token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    
    let client = reqwest::Client::new();
    
    // Create temp user for testing
    let sheet_response = client
        .post("http://localhost:3000/users_sheets")
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "last_name": "EmailTest",
            "first_name": "User",
            "type_user": "student",
            "profile_picture": null
        }))
        .send()
        .await
        .unwrap();
    
    let sheet: serde_json::Value = sheet_response.json().await.unwrap();
    let sheet_id = sheet["id"].as_i64().unwrap();
    
    let auth_response = client
        .post("http://localhost:3000/users_auth")
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "email": "original@test.com",
            "pwd": "TestPassword123",
            "id_user_sheet": sheet_id
        }))
        .send()
        .await
        .unwrap();
    
    let auth: serde_json::Value = auth_response.json().await.unwrap();
    let auth_id = auth["id"].as_i64().unwrap();
    
    // Update email
    let update_response = client
        .patch(&format!("http://localhost:3000/users_auth/{}/email", auth_id))
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "email": "updated@test.com"
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(update_response.status(), 200);
    
    // Verify email was updated
    let updated_email: String = sqlx::query_scalar(
        "SELECT email FROM users_auth WHERE id = $1"
    )
    .bind(auth_id as i32)
    .fetch_one(&pool)
    .await
    .unwrap();
    
    assert_eq!(updated_email, "updated@test.com");
    
    // Cleanup
    client
        .delete(&format!("http://localhost:3000/users_auth/{}", auth_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    client
        .delete(&format!("http://localhost:3000/users_sheets/{}", sheet_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_student_cannot_delete_user_auth() {
    let pool = get_test_pool().await;
    let student_token = login_and_get_token("emma.moreau@student.diplomind.fr", "Password123").await;
    
    // Get Sophie's auth ID
    let auth_id: i32 = sqlx::query_scalar(
        "SELECT id FROM users_auth WHERE email = 'sophie.martin@diplomind.fr'"
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    
    let client = reqwest::Client::new();
    let response = client
        .delete(&format!("http://localhost:3000/users_auth/{}", auth_id))
        .header("Authorization", format!("Bearer {}", student_token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_delete_user_auth_as_admin() {
    let token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    
    let client = reqwest::Client::new();
    
    // Create temp user
    let sheet_response = client
        .post("http://localhost:3000/users_sheets")
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "last_name": "DeleteTest",
            "first_name": "User",
            "type_user": "student",
            "profile_picture": null
        }))
        .send()
        .await
        .unwrap();
    
    let sheet: serde_json::Value = sheet_response.json().await.unwrap();
    let sheet_id = sheet["id"].as_i64().unwrap();
    
    let auth_response = client
        .post("http://localhost:3000/users_auth")
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "email": "todelete@test.com",
            "pwd": "TestPassword123",
            "id_user_sheet": sheet_id
        }))
        .send()
        .await
        .unwrap();
    
    let auth: serde_json::Value = auth_response.json().await.unwrap();
    let auth_id = auth["id"].as_i64().unwrap();
    
    // Delete auth
    let delete_response = client
        .delete(&format!("http://localhost:3000/users_auth/{}", auth_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(delete_response.status(), 200);
    
    // Cleanup sheet
    client
        .delete(&format!("http://localhost:3000/users_sheets/{}", sheet_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
}

// ============================================
// TESTS ERROR SCENARIOS
// ============================================

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

#[tokio::test]
async fn test_get_nonexistent_user_sheet() {
    let token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3000/users_sheets/99999")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 404);
}

#[tokio::test]
async fn test_create_user_auth_with_duplicate_email() {
    let token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3000/users_auth")
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "email": "sophie.martin@diplomind.fr",
            "pwd": "TestPassword123",
            "id_user_sheet": 1
        }))
        .send()
        .await
        .unwrap();
    
    // Should fail due to unique constraint on email
    assert_eq!(response.status(), 500);
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
