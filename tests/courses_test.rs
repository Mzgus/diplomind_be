mod common;
use common::*;

#[tokio::test]
async fn test_create_course_as_admin() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .post("http://localhost:3000/courses")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({
            "name": "Test Course",
            "description": "A test course description"
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["name"], "Test Course");
}

#[tokio::test]
async fn test_create_course_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();
    
    let response = client
        .post("http://localhost:3000/courses")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "name": "Teacher Course"
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_create_course_as_student_fails() {
    let (_, student_token) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .post("http://localhost:3000/courses")
        .header("Authorization", format!("Bearer {}", student_token))
        .json(&serde_json::json!({
            "name": "Unauthorized Course"
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_get_course_by_id() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .get("http://localhost:3000/courses/1")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["id"], 1);
}

#[tokio::test]
async fn test_get_all_courses() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .get("http://localhost:3000/courses")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_update_course_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();
    
    let response = client
        .put("http://localhost:3000/courses/1")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "description": "Updated by teacher"
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_delete_course_as_admin() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let create_response = client
        .post("http://localhost:3000/courses")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({
            "name": "Course To Delete"
        }))
        .send()
        .await
        .unwrap();
    
    let created: serde_json::Value = create_response.json().await.unwrap();
    let course_id = created["id"].as_i64().unwrap();
    
    let response = client
        .delete(format!("http://localhost:3000/courses/{}", course_id))
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
}
