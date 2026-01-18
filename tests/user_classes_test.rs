mod common;
use common::*;

#[tokio::test]
async fn test_assign_user_to_class_as_admin() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .post("http://localhost:3000/user-classes")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({"user_id": 6, "class_id": 3}))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["user_id"], 6);
    assert_eq!(body["class_id"], 3);
}

#[tokio::test]
async fn test_assign_user_to_class_as_student_fails() {
    let (_, student_token) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .post("http://localhost:3000/user-classes")
        .header("Authorization", format!("Bearer {}", student_token))
        .json(&serde_json::json!({"user_id": 6, "class_id": 1}))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_get_user_classes() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .get("http://localhost:3000/users/6/classes")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_get_class_users() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .get("http://localhost:3000/classes/1/users")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_remove_user_from_class_as_admin() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    // Create link first (8,3) - not in seed
    client.post("http://localhost:3000/user-classes")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({"user_id": 8, "class_id": 3}))
        .send().await.unwrap();
    
    let response = client
        .delete("http://localhost:3000/users/8/classes/3")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_assign_duplicate_user_class() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    // Create first link (9,3) - not in seed
    client.post("http://localhost:3000/user-classes")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({"user_id": 9, "class_id": 3}))
        .send().await.unwrap();
    
    // Try duplicate - should return 409
    let response = client
        .post("http://localhost:3000/user-classes")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({"user_id": 9, "class_id": 3}))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 409); // CONFLICT
}
