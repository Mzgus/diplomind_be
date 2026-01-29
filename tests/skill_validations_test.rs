mod common;
use common::*;

#[tokio::test]
async fn test_create_validation_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    // Create validation with comment - User 6 has no validations in seed
    let response = client
        .post("http://localhost:3000/skill-validations")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "user_id": 6,
            "skill_id": 4,
            "status": "pending",
            "comment": "À valider après le projet"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["user_id"], 6);
    assert_eq!(body["skill_id"], 4);
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
            "skill_id": 5,
            "status": "pending"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_create_duplicate_validation() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    // Create first validation - User 11 has no validations in seed
    let first_response = client
        .post("http://localhost:3000/skill-validations")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "user_id": 11,
            "skill_id": 2,
            "status": "pending"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(first_response.status(), 200);

    // Try to create duplicate - should fail with 409
    let response = client
        .post("http://localhost:3000/skill-validations")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "user_id": 11,
            "skill_id": 2,
            "status": "validated"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 409); // CONFLICT
}

#[tokio::test]
async fn test_get_user_validations() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    // Get validations for user 7 (has validations in seed data)
    let response = client
        .get("http://localhost:3000/skill-validations/user/7")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
    assert!(body.as_array().unwrap().len() > 0);
}

#[tokio::test]
async fn test_get_pending_validations_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:3000/skill-validations/pending")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .send()
        .await
        .unwrap();

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
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_validate_skill_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    // Create a pending validation first - User 12 has no validations in seed
    let create_response = client
        .post("http://localhost:3000/skill-validations")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "user_id": 12,
            "skill_id": 3,
            "status": "pending"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(create_response.status(), 200);

    // Validate it with comment
    let response = client
        .patch("http://localhost:3000/skill-validations/12/3")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "status": "validated",
            "comment": "Excellent travail !"
        }))
        .send()
        .await
        .unwrap();

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

    // Create a pending validation first - User 13 has no validations in seed
    let create_response = client
        .post("http://localhost:3000/skill-validations")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "user_id": 13,
            "skill_id": 6,
            "status": "pending"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(create_response.status(), 200);

    // Reject it with comment
    let response = client
        .patch("http://localhost:3000/skill-validations/13/6")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "status": "rejected",
            "comment": "Nécessite plus de pratique"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["status"], "rejected");
    assert_eq!(body["comment"], "Nécessite plus de pratique");
}

#[tokio::test]
async fn test_student_cannot_validate() {
    let (_, student_token) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    // Try to update a validation from seed (user 7, skill 6 is pending)
    let response = client
        .patch("http://localhost:3000/skill-validations/7/6")
        .header("Authorization", format!("Bearer {}", student_token))
        .json(&serde_json::json!({
            "status": "validated"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_get_validation_details() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    // Get details of an existing validation from seed (user 7, skill 1)
    let response = client
        .get("http://localhost:3000/skill-validations/7/1")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["user_id"], 7);
    assert_eq!(body["skill_id"], 1);
    assert_eq!(body["status"], "validated");
}

#[tokio::test]
async fn test_delete_validation_as_admin() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    // Create a validation first - User 14 has no validations in seed
    let create_response = client
        .post("http://localhost:3000/skill-validations")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "user_id": 14,
            "skill_id": 7,
            "status": "pending"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(create_response.status(), 200);

    // Delete as admin
    let response = client
        .delete("http://localhost:3000/skill-validations/14/7")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_teacher_cannot_delete() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    // Create a validation first - User 15 has no validations in seed
    let create_response = client
        .post("http://localhost:3000/skill-validations")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({
            "user_id": 15,
            "skill_id": 8,
            "status": "pending"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(create_response.status(), 200);

    // Try to delete as teacher - should fail
    let response = client
        .delete("http://localhost:3000/skill-validations/15/8")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_get_student_course_validations() {
    let (_, student_token) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    // Get validations for student 7 in course 1
    // Based on seed data, student 7 is in course 1
    let response = client
        .get("http://localhost:3000/users/7/courses/1/validations")
        .header("Authorization", format!("Bearer {}", student_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());

    let validations = body.as_array().unwrap();
    if !validations.is_empty() {
        let v = &validations[0];
        // Check for enriched fields
        assert!(v.get("skill_name").is_some());
        assert!(v.get("skill_description").is_some());
    }
}
