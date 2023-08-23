use axum::Router;
use axum::routing::post;
use crate::handlers::user_handlers;

pub fn routes() -> Router {
    let routes = Router::new()
        .route("/", post(user_handlers::create_user));

    routes
}
