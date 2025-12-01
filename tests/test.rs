use diplomind::routes;
use poem::test::TestClient;

#[tokio::test]
mod tests {

    async fn base_test() {
        let cli = TestClient::new(routes());
        let resp = cli.get("/").send().await;
        resp.assert_status_is_ok();
        resp.assert_text("hello").await;
    }

    async fn test_generate_refresh_token() {
        let cli = TestClient::new(routes());
        let resp = cli.get("/generate-refresh-token").send().await;
        resp.assert_status_is_ok();
    }
}
