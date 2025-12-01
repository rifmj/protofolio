//! Helper example to generate AsyncAPI spec JSON file for testing

use protofolio::AsyncApi;
use protofolio_derive::{AsyncApi, AsyncApiMessage};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(channel = "events", messageId = "event-v1")]
pub struct Event {
    pub id: String,
    pub data: String,
}

#[derive(AsyncApi)]
#[asyncapi(
    info(title = "Basic Example API", version = "1.0.0"),
    channels("events"),
    messages(Event)
)]
pub struct BasicApi;

fn main() {
    let json = BasicApi::asyncapi_json().expect("Failed to generate JSON");
    print!("{}", json);
}
