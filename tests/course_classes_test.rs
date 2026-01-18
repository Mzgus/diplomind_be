mod common;
use common::*;

#[tokio::test]
async fn test_link_course_to_class_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();
    
    // Use course_id=1, class_id=4 (not in seed: course_classes has (1,1) but not (1,4))
    let response = client
        .post("http://localhost:3000/course-classes")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({"course_id": 1, "class_id": 4}))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["course_id"], 1);
    assert_eq!(body["class_id"], 4);
}

#[tokio::test]
async fn test_get_course_classes() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .get("http://localhost:3000/courses/1/classes")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_get_class_courses() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .get("http://localhost:3000/classes/1/courses")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_unlink_course_from_class_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();
    
    // Create a new link first (3,2) - not in seed
    client.post("http://localhost:3000/course-classes")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({"course_id": 3, "class_id": 2}))
        .send().await.unwrap();
    
    // Then unlink it
    let response = client
        .delete("http://localhost:3000/courses/3/classes/2")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_link_duplicate_course_class() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();
    
    // Create first link (4,2) - not in seed
    client.post("http://localhost:3000/course-classes")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({"course_id": 4, "class_id": 2}))
        .send().await.unwrap();
    
    // Try to create duplicate - should return 409 CONFLICT
    let response = client
        .post("http://localhost:3000/course-classes")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({"course_id": 4, "class_id": 2}))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 409); // CONFLICT
}
