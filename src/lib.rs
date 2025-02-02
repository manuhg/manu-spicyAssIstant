// define all the server endpoints
use axum::{
    routing::{get, post},
    Router, Json,
    extract::Path};
use crate::assistant::assist_user;
use crate::models::AssistantResponse;

pub mod models;
pub mod constants;
pub mod clients;
pub mod llm_connect;
pub mod assistant;
//Set up all the api routes


pub fn setup_router() -> Router {
    Router::new()
        .route("/", get(api_root))
        .route("/greet/:name", get(greet_user))
        .route("/assist_user", post(assist_user_api))
}

// Handler for the root endpoint
pub async fn api_root() -> &'static str {
    "Welcome to the Manu's SpicyAI Assistant API server!"
}

// Handler for greeting a user
pub async fn greet_user(Path(name): Path<String>) -> Json<models::Greeting> {
    let message = format!("Hello, {}!", name);
    Json(models::Greeting { message })
}

// Handler for echoing back a JSON message
pub async fn assist_user_api(Json(payload): Json<models::UserInput>) -> Json<AssistantResponse> {
    Json(assist_user(payload.message).await.expect("Exception fetching response from user assistant"))
}
