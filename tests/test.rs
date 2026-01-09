use poem::test::TestClient;

#[tokio::test]
mod tests {

    async fn base_test() {
        let cli = TestClient::new(routes());
        let resp = cli.get("/").send().await;
        resp.assert_status_is_ok();
        resp.assert_text("hello").await;
    }
}
