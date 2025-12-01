//! Axum integration example
//!
//! This example demonstrates how to integrate protofolio with Axum
//! to serve AsyncAPI specifications via HTTP endpoints.
//!
//! To run this example, you'll need to add axum to your dependencies:
//! ```toml
//! [dependencies]
//! axum = "0.7"
//! tokio = { version = "1", features = ["full"] }
//! ```
//!
//! Note: This example is provided for reference. You may need to install
//! axum and tokio as dev-dependencies to run it.

use protofolio::AsyncApi;
use protofolio_derive::{AsyncApi, AsyncApiMessage};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Define message types
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "api.events",
    messageId = "api-event-v1",
    name = "ApiEvent",
    title = "API Event"
)]
pub struct ApiEvent {
    pub id: String,
    pub message: String,
}

// Define the AsyncAPI specification
#[derive(AsyncApi)]
#[asyncapi(
    info(
        title = "Axum Integration Example API",
        version = "1.0.0",
        description = "Example API for Axum integration"
    ),
    servers(
        (name = "nats", url = "nats://localhost:4222", protocol = "nats")
    ),
    channels("api.events"),
    messages(ApiEvent)
)]
pub struct AxumExampleApi;

// Example Axum handlers (commented out since axum may not be available)
//
// use axum::{
//     response::{Json, Response},
//     routing::get,
//     Router,
// };
// use std::net::SocketAddr;
//
// async fn asyncapi_json() -> Json<protofolio::AsyncApiSpec> {
//     Json(AxumExampleApi::asyncapi())
// }
//
// async fn asyncapi_yaml() -> Response<String> {
//     let yaml = AxumExampleApi::asyncapi_yaml()
//         .expect("Failed to generate YAML");
//     Response::builder()
//         .header("content-type", "application/yaml")
//         .body(yaml)
//         .unwrap()
// }
//
// #[tokio::main]
// async fn main() {
//     let app = Router::new()
//         .route("/asyncapi.json", get(asyncapi_json))
//         .route("/asyncapi.yaml", get(asyncapi_yaml));
//
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     println!("Serving AsyncAPI spec at http://{}/asyncapi.json", addr);
//     println!("Serving AsyncAPI spec at http://{}/asyncapi.yaml", addr);
//
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

fn main() {
    println!("=== Axum Integration Example ===\n");
    println!("This example demonstrates how to integrate protofolio with Axum.");
    println!("To use this example, uncomment the code and add axum to your dependencies.\n");

    // Generate and display the specification
    let json = AxumExampleApi::asyncapi_json().expect("Failed to generate JSON");

    println!("Generated AsyncAPI specification:");
    println!("{}", json);

    println!("\nTo serve this via HTTP, use the Axum handlers shown in the source code.");
    println!("The handlers would be available at:");
    println!("  - GET /asyncapi.json");
    println!("  - GET /asyncapi.yaml");
}
