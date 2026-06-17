mod common;
use common::*;

#[tokio::test]
async fn test_create_step_as_admin() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let response = client
        .post("http://localhost:3000/steps")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({
            "name": "Test Step",
            "description": "A test step",
            "project_id": 1,
            "step_order": 1
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["name"], "Test Step");
    assert_eq!(body["project_id"], 1);
    assert_eq!(body["step_order"], 1);
}

#[tokio::test]
async fn test_create_step_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    let response = client
        .post("http://localhost:3000/steps")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "name": "Teacher Step",
            "project_id": 1
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_get_step_by_id() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:3000/steps/1")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["id"], 1);
}

#[tokio::test]
async fn test_get_all_steps() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:3000/steps")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_get_steps_by_project() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:3000/projects/1/steps")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_update_step_order() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let response = client
        .put("http://localhost:3000/steps/1")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({
            "step_order": 99
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["step_order"], 99);
}

#[tokio::test]
async fn test_update_step_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    let response = client
        .put("http://localhost:3000/steps/1")
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
async fn test_delete_step_as_admin() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let create_response = client
        .post("http://localhost:3000/steps")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({
            "name": "Step To Delete",
            "project_id": 1
        }))
        .send()
        .await
        .unwrap();

    let created: serde_json::Value = create_response.json().await.unwrap();
    let step_id = created["id"].as_i64().unwrap();

    let response = client
        .delete(format!("http://localhost:3000/steps/{}", step_id))
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_create_step_with_invalid_project() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let response = client
        .post("http://localhost:3000/steps")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({
            "name": "Invalid Step",
            "project_id": 99999
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 500);
}

#[tokio::test]
async fn test_delete_step_as_assigned_teacher_success() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    // Create a step under Project 1 (Marie teaches Course 1, which Project 1 belongs to)
    let create_response = client
        .post("http://localhost:3000/steps")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({
            "name": "Teacher Owned Step",
            "project_id": 1
        }))
        .send()
        .await
        .unwrap();

    let created: serde_json::Value = create_response.json().await.unwrap();
    let step_id = created["id"].as_i64().unwrap();

    // Now delete it as teacher
    let response = client
        .delete(format!("http://localhost:3000/steps/{}", step_id))
        .header("Authorization", format!("Bearer {}", teacher_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_delete_step_as_unassigned_teacher_fails() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    // Create a step under Project 5 (Marie does NOT teach Course 5, which Project 5 belongs to)
    let create_response = client
        .post("http://localhost:3000/steps")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({
            "name": "Teacher Unowned Step",
            "project_id": 5
        }))
        .send()
        .await
        .unwrap();

    let created: serde_json::Value = create_response.json().await.unwrap();
    let step_id = created["id"].as_i64().unwrap();

    // Try to delete it as teacher (should fail)
    let response = client
        .delete(format!("http://localhost:3000/steps/{}", step_id))
        .header("Authorization", format!("Bearer {}", teacher_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401); // Unauthorized
}
