use axum::Router;
use axum::routing::{get, post};
use crate::handlers::create_user::create_user;
use crate::handlers::health_check::health_check;

pub fn app() -> Router {
    let app = Router::new()
        .route("/api/v1/health_check", get(health_check))
        .route("/api/v1/users", post(create_user));

    app
}
