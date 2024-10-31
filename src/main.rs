use std::net::SocketAddr;
use manu_spicy_assistant::setup_router;
pub mod constants;

#[tokio::main]
async fn main() {
    // Define the routes
    let app = setup_router();

    // Define the address to bind to
    let addr = SocketAddr::from((constants::SERVER_HOST, constants::SERVER_PORT));
    println!("Server running on http://{}", addr);

    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
