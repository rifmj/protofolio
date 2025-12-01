// Example binary for size analysis
use protofolio::AsyncApi;
use protofolio_derive::{AsyncApi, AsyncApiMessage};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "test.events",
    messageId = "test-event-v1",
    name = "TestEvent",
    title = "Test Event"
)]
pub struct TestEvent {
    pub id: String,
    pub data: String,
}

#[derive(AsyncApi)]
#[asyncapi(
    info(title = "Test API", version = "1.0.0"),
    servers(
        (name = "nats", url = "nats://localhost:4222", protocol = "nats")
    ),
    channels("test.events"),
    messages(TestEvent)
)]
pub struct TestApi;

fn main() {
    let spec = TestApi::asyncapi();
    let json = protofolio::to_json(&spec).unwrap();
    println!("Generated spec size: {} bytes", json.len());
}
