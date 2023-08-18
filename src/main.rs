use std::net::SocketAddr;
use todo::app;

#[tokio::main]
async fn main() {
    let app = app();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
