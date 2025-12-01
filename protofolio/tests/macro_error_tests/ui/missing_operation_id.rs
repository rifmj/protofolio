// Test that missing operation id produces a helpful error

use protofolio_derive::AsyncApiOperation;

#[derive(AsyncApiOperation)]
#[asyncapi(
    action = "send",
    channel = "events",
    messages()
)]
pub struct MyOperation;

