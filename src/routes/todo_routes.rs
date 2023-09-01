use axum::Router;
use axum::routing::post;
use crate::handlers::todo_handlers;

pub fn routes() -> Router {
    todo!("handlerに何かしらのトレイトの実装が足りていないっぽい。多分 impl IntoResponse")
    // let routes = Router::new()
        // .route("/", post(todo_handlers::create_todo));

    // routes
}
