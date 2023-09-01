use axum::Router;
use axum::routing::get;
use crate::routes::{todo_routes, user_routes};

pub fn app() -> Router {
    let app = Router::new()
        .route("/api/v1/health_check", get(|| async { "Hello, world!" }))
        .nest("/api/v1/users", user_routes::routes())
        .nest("/api/v1/todo", todo_routes::routes());

    app
}
