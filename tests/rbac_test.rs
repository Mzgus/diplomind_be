mod common;

use common::*;

// ============================================
// TESTS RBAC (Role-Based Access Control)
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
    
    assert_eq!(response.status(), 401); // Unauthorized
}

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
