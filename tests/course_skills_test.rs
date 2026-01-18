mod common;
use common::*;

#[tokio::test]
async fn test_link_skill_to_course_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();
    
    let response = client
        .post("http://localhost:3000/course-skills")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({"course_id": 1, "skill_id": 10}))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["course_id"], 1);
    assert_eq!(body["skill_id"], 10);
}

#[tokio::test]
async fn test_get_course_skills() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .get("http://localhost:3000/courses/1/skills")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_get_skill_courses() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .get("http://localhost:3000/skills/1/courses")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_unlink_skill_from_course_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();
    
    // Create link first (2,1) - not in seed
    client.post("http://localhost:3000/course-skills")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({"course_id": 2, "skill_id": 1}))
        .send().await.unwrap();
    
    let response = client
        .delete("http://localhost:3000/courses/2/skills/1")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_link_duplicate_course_skill() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();
    
    // Create first link (3,1) - not in seed
    client.post("http://localhost:3000/course-skills")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({"course_id": 3, "skill_id": 1}))
        .send().await.unwrap();
    
    // Try duplicate - should return 409
    let response = client
        .post("http://localhost:3000/course-skills")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({"course_id": 3, "skill_id": 1}))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 409); // CONFLICT
}
