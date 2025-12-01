// Test that missing channel attribute produces a helpful error

use protofolio_derive::AsyncApiMessage;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(messageId = "msg-1")]
pub struct MyMessage {
    pub id: String,
}

