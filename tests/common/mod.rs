use sqlx::PgPool;

pub async fn get_test_pool() -> PgPool {
    // Charger le fichier .env
    dotenv::dotenv().ok();
    
    // Utiliser DATABASE_URL du .env
    let url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    PgPool::connect(&url).await.expect("Failed to connect to test DB")
}

// Plus de cleanup - on utilise les données du seed
// Plus de create_admin/create_student - on utilise les données existantes

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
