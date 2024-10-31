// test all the api endpoints
use axum::http::{Request, StatusCode};
use axum::Router;
use hyper::Body;
use serde_json::json;
use tower::ServiceExt;
use manu_spicy_assistant;

#[tokio::test]
async fn test_greet_user() {
    // Set up the app with the greet route only
    let app = Router::new().route("/greet/:name", axum::routing::get(manu_spicy_assistant::greet_user));

    // Create a request to the greet endpoint with a name parameter
    let response = app
        .oneshot(
            Request::builder()
                .uri("/greet/Manu")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Check that the response status is OK
    assert_eq!(response.status(), StatusCode::OK);

    // Verify that the response body matches the expected JSON
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json, json!({ "message": "Hello, Manu!" }));
}
#[tokio::test]
async fn test_echo() {
    // Set up the app with just the echo route
    let app = Router::new().route("/echo", axum::routing::post(manu_spicy_assistant::echo));

    // Define the payload we want to send to the echo endpoint
    let payload = manu_spicy_assistant::models::Greeting {
        message: "Hello, world!".to_string(),
    };

    // Create a request to the echo endpoint with the payload
    let request = Request::builder()
        .method("POST")
        .uri("/echo")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&payload).unwrap()))
        .unwrap();

    // Send the request and get the response
    let response = app.oneshot(request).await.unwrap();

    // Check that the status code is 200 OK
    assert_eq!(response.status(), StatusCode::OK);

    // Parse the response body to JSON and compare it with the expected output
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Validate that the response contains the echoed message
    assert_eq!(json, json!({ "message": "Echo: Hello, world!" }));
}

