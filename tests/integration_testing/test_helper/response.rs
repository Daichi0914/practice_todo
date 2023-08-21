use axum::response::Response;

pub mod get_method {
    use axum::body::Body;
    use axum::http::Request;
    use axum::response::Response;
    use hyper::header;
    use tower::ServiceExt;
    use todo::app;

    pub async fn call_api_without_body(method: &str, uri: &str) -> Response {
        app()
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(uri)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap()
    }

    pub async fn call_api_with_body(method: &str, uri: &str, body: String) -> Response {
        app()
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(uri)
                    .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap()
    }
}

pub async fn body_to_string(response: Response) -> String {
    String::from_utf8(
        hyper::body::to_bytes(
            response.into_body()
        ).await.unwrap().to_vec()
    ).unwrap()
}
