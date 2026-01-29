mod common;
use common::*;

#[tokio::test]
async fn test_link_skill_to_step_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    let response = client
        .post("http://localhost:3000/step-skills")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({"step_id": 13, "skill_id": 1}))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["step_id"], 13);
    assert_eq!(body["skill_id"], 1);
}

#[tokio::test]
async fn test_get_step_skills() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:3000/steps/1/skills")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_get_skill_steps() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:3000/skills/1/steps")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_unlink_skill_from_step_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    // Create link first (12,1) - not in seed
    client
        .post("http://localhost:3000/step-skills")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({"step_id": 12, "skill_id": 1}))
        .send()
        .await
        .unwrap();

    let response = client
        .delete("http://localhost:3000/steps/12/skills/1")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_link_duplicate_step_skill() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    // Create first link (14,1) - not in seed
    client
        .post("http://localhost:3000/step-skills")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({"step_id": 14, "skill_id": 1}))
        .send()
        .await
        .unwrap();

    // Try duplicate - should return 409
    let response = client
        .post("http://localhost:3000/step-skills")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({"step_id": 14, "skill_id": 1}))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 409); // CONFLICT
}
