// Test that invalid action value produces a helpful error

use protofolio_derive::AsyncApiOperation;

#[derive(AsyncApiOperation)]
#[asyncapi(
    id = "op-1",
    action = "invalid",
    channel = "events",
    messages()
)]
pub struct MyOperation;

