use std::env;
use std::net::SocketAddr;
pub use crate::routes::routes::app;

pub mod handlers {
    pub mod user_handlers;
}
pub mod routes {
    pub mod routes;
    pub mod user_routes;
}

pub async fn init_server() {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let app = app();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
