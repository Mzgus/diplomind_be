mod common;
use common::*;

#[tokio::test]
async fn test_create_validation_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();
    
    // Create validation with comment
    let response = client
        .post("http://localhost:3000/skill-validations")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "user_id": 10,
            "skill_id": 15,
            "status": "pending",
            "comment": "À valider après le projet"
        }))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["user_id"], 10);
    assert_eq!(body["skill_id"], 15);
    assert_eq!(body["status"], "pending");
    assert_eq!(body["comment"], "À valider après le projet");
}

#[tokio::test]
async fn test_student_cannot_create_validation() {
    let (_, student_token) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .post("http://localhost:3000/skill-validations")
        .header("Authorization", format!("Bearer {}", student_token))
        .json(&serde_json::json!({
            "user_id": 6,
            "skill_id": 1,
            "status": "pending"
        }))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_create_duplicate_validation() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();
    
    // Create first validation
    client
        .post("http://localhost:3000/skill-validations")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "user_id": 11,
            "skill_id": 16,
            "status": "pending"
        }))
        .send().await.unwrap();
    
    // Try to create duplicate
    let response = client
        .post("http://localhost:3000/skill-validations")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "user_id": 11,
            "skill_id": 16,
            "status": "validated"
        }))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 409); // CONFLICT
}

#[tokio::test]
async fn test_get_user_validations() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();
    
    // Get validations for user 6 (from seed data)
    let response = client
        .get("http://localhost:3000/skill-validations/user/6")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_get_pending_validations_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();
    
    let response = client
        .get("http://localhost:3000/skill-validations/pending")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_student_cannot_access_pending() {
    let (_, student_token) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .get("http://localhost:3000/skill-validations/pending")
        .header("Authorization", format!("Bearer {}", student_token))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_validate_skill_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();
    
    // Create a pending validation first
    client
        .post("http://localhost:3000/skill-validations")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "user_id": 12,
            "skill_id": 17,
            "status": "pending"
        }))
        .send().await.unwrap();
    
    // Validate it with comment
    let response = client
        .patch("http://localhost:3000/skill-validations/12/17")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "status": "validated",
            "comment": "Excellent travail !"
        }))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["status"], "validated");
    assert_eq!(body["comment"], "Excellent travail !");
    assert!(body["validated_at"].is_string());
    assert!(body["validated_by"].is_number());
}

#[tokio::test]
async fn test_reject_skill_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();
    
    // Create a pending validation first
    client
        .post("http://localhost:3000/skill-validations")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "user_id": 13,
            "skill_id": 18,
            "status": "pending"
        }))
        .send().await.unwrap();
    
    // Reject it with comment
    let response = client
        .patch("http://localhost:3000/skill-validations/13/18")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "status": "rejected",
            "comment": "Nécessite plus de pratique"
        }))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["status"], "rejected");
    assert_eq!(body["comment"], "Nécessite plus de pratique");
}

#[tokio::test]
async fn test_student_cannot_validate() {
    let (_, student_token) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();
    
    let response = client
        .patch("http://localhost:3000/skill-validations/6/1")
        .header("Authorization", format!("Bearer {}", student_token))
        .json(&serde_json::json!({
            "status": "validated"
        }))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_get_validation_details() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();
    
    // Create a validation first
    client
        .post("http://localhost:3000/skill-validations")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "user_id": 14,
            "skill_id": 19,
            "status": "pending"
        }))
        .send().await.unwrap();
    
    // Get details
    let response = client
        .get("http://localhost:3000/skill-validations/14/19")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["user_id"], 14);
    assert_eq!(body["skill_id"], 19);
}

#[tokio::test]
async fn test_delete_validation_as_admin() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();
    
    // Create a validation first
    client
        .post("http://localhost:3000/skill-validations")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "user_id": 15,
            "skill_id": 20,
            "status": "pending"
        }))
        .send().await.unwrap();
    
    // Delete as admin
    let response = client
        .delete("http://localhost:3000/skill-validations/15/20")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_teacher_cannot_delete() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();
    
    // Create a validation first
    client
        .post("http://localhost:3000/skill-validations")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "user_id": 16,
            "skill_id": 21,
            "status": "pending"
        }))
        .send().await.unwrap();
    
    // Try to delete as teacher
    let response = client
        .delete("http://localhost:3000/skill-validations/16/21")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .send().await.unwrap();
    
    assert_eq!(response.status(), 401);
}
