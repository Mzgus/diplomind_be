mod common;
use common::*;

#[tokio::test]
async fn test_assign_user_to_course_as_admin() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    // Use user_id=13, course_id=4 (not in seed)
    let response = client
        .post("http://localhost:3000/user-courses")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({"user_id": 13, "course_id": 4}))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["user_id"], 13);
    assert_eq!(body["course_id"], 4);
}

#[tokio::test]
async fn test_assign_user_to_course_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    // Use user_id=6, course_id=1 (user 6 not in course 1 in seed)
    let response = client
        .post("http://localhost:3000/user-courses")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({"user_id": 6, "course_id": 1}))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_get_user_courses() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:3000/users/6/courses")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_get_course_users() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:3000/courses/1/users")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_remove_user_from_course_as_teacher() {
    let teacher_token = get_teacher_token().await;
    let client = reqwest::Client::new();

    // Create link first (6,2) - not in seed
    client
        .post("http://localhost:3000/user-courses")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .json(&serde_json::json!({"user_id": 6, "course_id": 2}))
        .send()
        .await
        .unwrap();

    let response = client
        .delete("http://localhost:3000/users/6/courses/2")
        .header("Authorization", format!("Bearer {}", teacher_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_assign_duplicate_user_course() {
    let (admin_token, _) = get_admin_and_student_tokens().await;
    let client = reqwest::Client::new();

    // Create first link (6,4) - not in seed
    client
        .post("http://localhost:3000/user-courses")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({"user_id": 6, "course_id": 4}))
        .send()
        .await
        .unwrap();

    // Try duplicate - should return 409
    let response = client
        .post("http://localhost:3000/user-courses")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&serde_json::json!({"user_id": 6, "course_id": 4}))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 409); // CONFLICT
}
