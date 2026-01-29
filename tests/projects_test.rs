mod common;
use common::*;

#[tokio::test]
async fn test_create_project_as_admin() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let response = client
        .post("http://localhost:3000/projects")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({
            "name": "Test Project",
            "description": "A test project",
            "course_id": 1
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["name"], "Test Project");
    assert_eq!(body["course_id"], 1);
}

#[tokio::test]
async fn test_create_project_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    let response = client
        .post("http://localhost:3000/projects")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "name": "Teacher Project",
            "course_id": 1
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_get_project_by_id() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:3000/projects/1")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["id"], 1);
}

#[tokio::test]
async fn test_get_all_projects() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:3000/projects")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_get_projects_by_course() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:3000/courses/1/projects")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_update_project_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    let response = client
        .put("http://localhost:3000/projects/1")
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
async fn test_delete_project_as_admin() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let create_response = client
        .post("http://localhost:3000/projects")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({
            "name": "Project To Delete",
            "course_id": 1
        }))
        .send()
        .await
        .unwrap();

    let created: serde_json::Value = create_response.json().await.unwrap();
    let project_id = created["id"].as_i64().unwrap();

    let response = client
        .delete(format!("http://localhost:3000/projects/{}", project_id))
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_create_project_with_invalid_course() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let response = client
        .post("http://localhost:3000/projects")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({
            "name": "Invalid Project",
            "course_id": 99999
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 500);
}

#[tokio::test]
async fn test_get_student_projects() {
    let (_, student_token) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    // Student 7 is linked to Course 1 (Dev Web)
    // Project 1 is linked to Course 1
    // So Student 7 should see Project 1
    let response = client
        .get("http://localhost:3000/users/7/projects")
        .header("Authorization", format!("Bearer {}", student_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());

    // Should contain at least one project
    let projects = body.as_array().unwrap();
    assert!(!projects.is_empty());
}
