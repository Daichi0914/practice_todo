use axum::Router;
use axum::routing::get;
use crate::routes::user_routes;

pub fn app() -> Router {
    let app = Router::new()
        .route("/api/v1/health_check", get(|| async { "Hello, world!" }))
        .nest("/api/v1/users", user_routes::routes());

    app
}
