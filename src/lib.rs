// define all the server endpoints
use axum::{
    routing::{get, post},
    Router, Json,
    extract::Path};

pub mod models;
pub mod constants;
pub mod clients;
pub mod llm_connect;

//Set up all the api routes
pub fn setup_router() -> Router {
    Router::new()
        .route("/", get(api_root))
        .route("/greet/:name", get(greet_user))
        .route("/echo", post(echo))
}

// Handler for the root endpoint
pub async fn api_root() -> &'static str {
    "Welcome to the Manu's Spicy Assistant API server!"
}

// Handler for greeting a user
pub async fn greet_user(Path(name): Path<String>) -> Json<models::Greeting> {
    let message = format!("Hello, {}!", name);
    Json(models::Greeting { message })
}

// Handler for echoing back a JSON message
pub async fn echo(Json(payload): Json<models::Greeting>) -> Json<models::Greeting> {
    Json(models::Greeting {
        message: format!("Echo: {}", payload.message),
    })
}
