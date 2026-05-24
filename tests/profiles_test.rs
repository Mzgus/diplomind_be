mod common;
use common::*;

#[tokio::test]
async fn test_get_my_profiles_success() {
    // Login as Alex Poly (account_id 16, has 2 profiles: 16 (Teacher) and 17 (Student))
    let token = login_and_get_token("alex.poly@diplomind.fr", "Password123").await;
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:3000/me/profiles")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let profiles: serde_json::Value = response.json().await.unwrap();
    assert!(profiles.is_array());
    let arr = profiles.as_array().unwrap();
    
    // Should have exactly 2 profiles linked
    assert_eq!(arr.len(), 2);
    
    // Verify profiles contain correct sheet IDs (16 and 17)
    let ids: Vec<i64> = arr.iter().map(|p| p["id"].as_i64().unwrap()).collect();
    assert!(ids.contains(&16));
    assert!(ids.contains(&17));
}

#[tokio::test]
async fn test_switch_profile_success() {
    // Login as Alex Poly
    let token = login_and_get_token("alex.poly@diplomind.fr", "Password123").await;
    let client = reqwest::Client::new();

    // Verify current profile is teacher (user_id 16)
    let verify_response = client
        .get("http://localhost:3000/verify_token")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    let user_info: serde_json::Value = verify_response.json().await.unwrap();
    assert_eq!(user_info["user_id"], 16);
    assert_eq!(user_info["user_role"], "teacher");

    // Switch profile to student (user_id 17)
    let switch_response = client
        .post("http://localhost:3000/auth/switch-profile")
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "user_sheet_id": 17
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(switch_response.status(), 200);
    let body: serde_json::Value = switch_response.json().await.unwrap();
    let new_token = body["token"].as_str().unwrap();

    // Verify new token corresponds to the student profile (user_id 17)
    let new_verify_response = client
        .get("http://localhost:3000/verify_token")
        .header("Authorization", format!("Bearer {}", new_token))
        .send()
        .await
        .unwrap();
    assert_eq!(new_verify_response.status(), 200);
    let new_user_info: serde_json::Value = new_verify_response.json().await.unwrap();
    assert_eq!(new_user_info["user_id"], 17);
    assert_eq!(new_user_info["user_role"], "student");
}

#[tokio::test]
async fn test_switch_profile_fails_for_unlinked_profile() {
    // Login as Alex Poly
    let token = login_and_get_token("alex.poly@diplomind.fr", "Password123").await;
    let client = reqwest::Client::new();

    // Attempt to switch to profile 1 (Sophie Martin - Admin), which belongs to account 1
    let response = client
        .post("http://localhost:3000/auth/switch-profile")
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "user_sheet_id": 1
        }))
        .send()
        .await
        .unwrap();

    // Should return 404 (Not Found / Profile not found for this account)
    assert_eq!(response.status(), 404);
}

#[tokio::test]
async fn test_switch_profile_fails_for_deactivated_profile() {
    // Login as admin to deactivate profile 17 (Alex Poly - Student)
    let admin_token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    let client = reqwest::Client::new();

    // Deactivate profile 17
    let deactivate_res = client
        .patch("http://localhost:3000/admin/users/17/deactivate")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();
    assert_eq!(deactivate_res.status(), 200);

    // Login as Alex Poly
    let token = login_and_get_token("alex.poly@diplomind.fr", "Password123").await;

    // Attempt to switch to the deactivated profile 17 (should fail)
    let switch_response = client
        .post("http://localhost:3000/auth/switch-profile")
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "user_sheet_id": 17
        }))
        .send()
        .await
        .unwrap();

    // Should return 401 Unauthorized (since profile is inactive)
    assert_eq!(switch_response.status(), 401);

    // Reactivate profile 17 so subsequent test runs succeed
    let reactivate_res = client
        .patch("http://localhost:3000/admin/users/17/activate")
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .unwrap();
    assert_eq!(reactivate_res.status(), 200);
}
