use std::env;
use std::net::SocketAddr;
use axum::Router;
use crate::routes::routes;

pub mod handlers {
    pub mod health_check;
    pub mod create_user;
}

pub mod routes;

pub fn app() -> Router {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let app = routes();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    app
}
