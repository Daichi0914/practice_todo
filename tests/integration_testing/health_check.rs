mod success {
    use axum::http::StatusCode;
    use crate::integration_testing::test_helper::response::get_method::call_api_without_body;

    #[tokio::test]
    async fn 応答が返ってくる() {
        // given
        let uri = "/api/v1/health_check";

        // when
        let response = call_api_without_body("GET", &uri).await;

        // then
        assert_eq!(response.status(), StatusCode::OK);
    }
}
