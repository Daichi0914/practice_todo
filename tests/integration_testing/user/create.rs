use axum::http::StatusCode;
use crate::integration_testing::test_helper::response::body_to_string;
use crate::integration_testing::test_helper::response::get_method::call_api_with_body;

#[tokio::test]
async fn test_create_user() {
    // given
    let uri = format!("/api/v1/users");
    let body = format!(r#"{{ "username": "田中太郎" }}"#);

    // when
    let response = call_api_with_body("POST", &uri, body).await;

    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(body_to_string(response).await, r#"{"id":1234,"username":"田中太郎"}"#);
}
