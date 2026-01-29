mod common;
use common::*;

#[tokio::test]
async fn test_create_skill_as_admin() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let response = client
        .post("http://localhost:3000/skills")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({
            "name": "Test Skill",
            "description": "A test skill description"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["name"], "Test Skill");
    assert_eq!(body["description"], "A test skill description");
}

#[tokio::test]
async fn test_create_skill_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    let response = client
        .post("http://localhost:3000/skills")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "name": "Teacher Created Skill",
            "description": "Created by teacher"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_create_skill_as_student_fails() {
    let (_, student_token) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let response = client
        .post("http://localhost:3000/skills")
        .header("Authorization", format!("Bearer {}", student_token))
        .json(&serde_json::json!({
            "name": "Unauthorized Skill"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_get_skill_by_id() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:3000/skills/1")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["id"], 1);
}

#[tokio::test]
async fn test_get_all_skills() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:3000/skills")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
    assert!(body.as_array().unwrap().len() > 0);
}

#[tokio::test]
async fn test_update_skill_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    let response = client
        .put("http://localhost:3000/skills/1")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "description": "Updated description by teacher"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_delete_skill_as_admin() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let create_response = client
        .post("http://localhost:3000/skills")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({
            "name": "Skill To Delete"
        }))
        .send()
        .await
        .unwrap();

    let created: serde_json::Value = create_response.json().await.unwrap();
    let skill_id = created["id"].as_i64().unwrap();

    let response = client
        .delete(format!("http://localhost:3000/skills/{}", skill_id))
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}
