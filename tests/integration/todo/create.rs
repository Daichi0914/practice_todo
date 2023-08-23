use axum::http::StatusCode;
use crate::integration::test_helper::response::body_to_string;
use crate::integration::test_helper::response::get_method::call_api_with_body;

#[tokio::test]
async fn todoを作成する() {
    // given
    let uri = format!("/api/v1/todo");
    let body = format!(r#"{{ "todo": "運動する" }}"#);

    // when
    let response = call_api_with_body("POST", &uri, body).await;

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    assert_eq!(body_to_string(response).await, "");
    // TODO: DBの値を取り出してアサートする
}
