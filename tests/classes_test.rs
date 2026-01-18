mod common;

use common::*;

#[tokio::test]
async fn test_create_class_as_admin() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .post("http://localhost:3000/classes")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({
            "name": "Test Class 2026"
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["name"], "Test Class 2026");
    assert!(body["id"].is_number());
}

#[tokio::test]
async fn test_create_class_as_student_fails() {
    let (_, student_token) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .post("http://localhost:3000/classes")
        .header("Authorization", format!("Bearer {}", student_token))
        .json(&serde_json::json!({
            "name": "Unauthorized Class"
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_get_class_by_id() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .get("http://localhost:3000/classes/1")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["id"], 1);
    assert_eq!(body["name"], "CDA 2024-2025");
}

#[tokio::test]
async fn test_get_all_classes() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .get("http://localhost:3000/classes")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
    assert!(body.as_array().unwrap().len() >= 2);
}

#[tokio::test]
async fn test_update_class_as_admin() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .put("http://localhost:3000/classes/1")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({
            "name": "CDA 2024-2025 Updated"
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["name"], "CDA 2024-2025 Updated");
}

#[tokio::test]
async fn test_delete_class_as_admin() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let create_response = client
        .post("http://localhost:3000/classes")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({
            "name": "Class To Delete"
        }))
        .send()
        .await
        .unwrap();
    
    let created: serde_json::Value = create_response.json().await.unwrap();
    let class_id = created["id"].as_i64().unwrap();
    
    let response = client
        .delete(format!("http://localhost:3000/classes/{}", class_id))
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_get_nonexistent_class() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .get("http://localhost:3000/classes/99999")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 404);
}
