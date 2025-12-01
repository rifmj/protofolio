//! Basic example demonstrating minimal AsyncAPI specification generation
//!
//! This example shows the simplest possible usage of protofolio:
//! - Define a message type with AsyncApiMessage derive
//! - Define an API specification with AsyncApi derive
//! - Generate and print the specification

use protofolio::AsyncApi;
use protofolio_derive::{AsyncApi, AsyncApiMessage};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Define a simple message type
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "events",
    messageId = "event-v1"
)]
pub struct Event {
    pub id: String,
    pub data: String,
}

// Define the AsyncAPI specification
#[derive(AsyncApi)]
#[asyncapi(
    info(title = "Basic Example API", version = "1.0.0"),
    channels("events"),
    messages(Event)
)]
pub struct BasicApi;

fn main() {
    println!("=== Basic AsyncAPI Example ===\n");

    // Generate JSON output
    let json = BasicApi::asyncapi_json().expect("Failed to generate JSON");
    println!("Generated AsyncAPI specification (JSON):");
    println!("{}", json);

    // Generate YAML output
    let yaml = BasicApi::asyncapi_yaml().expect("Failed to generate YAML");
    println!("\nGenerated AsyncAPI specification (YAML):");
    println!("{}", yaml);
}

