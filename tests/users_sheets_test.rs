mod common;

use common::*;

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
    let student_token =
        login_and_get_token("emma.moreau@student.diplomind.fr", "Password123").await;

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
