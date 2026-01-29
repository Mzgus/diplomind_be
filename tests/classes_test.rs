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

    // Create a new class to verify isolation
    let create_response = client
        .post("http://localhost:3000/classes")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({
            "name": "Specific Class For GetById"
        }))
        .send()
        .await
        .unwrap();

    let created_body: serde_json::Value = create_response.json().await.unwrap();
    let new_id = created_body["id"].as_i64().unwrap();

    let response = client
        .get(format!("http://localhost:3000/classes/{}", new_id))
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["id"], new_id);
    assert_eq!(body["name"], "Specific Class For GetById");
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

    // Create a specific class to update to avoid race conditions
    let create_response = client
        .post("http://localhost:3000/classes")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({
            "name": "Class To Update"
        }))
        .send()
        .await
        .unwrap();

    let created_body: serde_json::Value = create_response.json().await.unwrap();
    let id_to_update = created_body["id"].as_i64().unwrap();

    let response = client
        .put(format!("http://localhost:3000/classes/{}", id_to_update))
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({
            "name": "Class Updated Successfully"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["name"], "Class Updated Successfully");
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

#[tokio::test]
async fn test_get_teacher_classes() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let teacher_token = get_teacher_token().await; // This is Marie (id 3 in seed)
    let client = reqwest::Client::new();

    // 1. Assign Teacher 3 to Course 1 (if not already)
    // We use admin token to do this assignment
    let _assign_response = client
        .post("http://localhost:3000/user-courses")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({
            "user_id": 3, // teacher Marie
            "course_id": 1 // Dev Web
        }))
        .send()
        .await
        .unwrap();

    // Note: Course 1 is linked to Class 1 (CDA) in seed data (via course_classes)

    // 2. Teacher requests their classes (using correct ID 3)
    let response = client
        .get("http://localhost:3000/teachers/3/classes")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());

    let classes = body.as_array().unwrap();
    // Teacher should see at least one class (Class 1)
    assert!(classes.iter().any(|c| c["id"] == 1));
}
