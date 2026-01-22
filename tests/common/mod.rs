use sqlx::PgPool;

#[allow(dead_code)]
pub async fn get_test_pool() -> PgPool {
    // Charger le fichier .env
    dotenv::dotenv().ok();

    // Utiliser DATABASE_URL du .env
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    PgPool::connect(&url)
        .await
        .expect("Failed to connect to test DB")
}

// Plus de cleanup - on utilise les données du seed
// Plus de create_admin/create_student - on utilise les données existantes

#[allow(dead_code)]
pub async fn login_and_get_token(email: &str, pwd: &str) -> String {
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3000/login")
        .json(&serde_json::json!({
            "email": email,
            "pwd": pwd
        }))
        .send()
        .await
        .unwrap();

    let body: serde_json::Value = response.json().await.unwrap();
    body["token"].as_str().unwrap().to_string()
}

/// Get admin and student tokens for testing
#[allow(dead_code)]
pub async fn get_admin_and_student_tokens() -> (String, String) {
    let admin_token = login_and_get_token("sophie.martin@diplomind.fr", "Password123").await;
    let student_token =
        login_and_get_token("louis.simon@student.diplomind.fr", "Password123").await;
    (admin_token, student_token)
}

/// Get teacher token for testing
#[allow(dead_code)]
pub async fn get_teacher_token() -> String {
    login_and_get_token("marie.dubois@diplomind.fr", "Password123").await
}
