use todo::init_server;

#[tokio::main]
async fn main() {
    init_server().await;
}
