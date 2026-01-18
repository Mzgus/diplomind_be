mod common;

use common::*;

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
