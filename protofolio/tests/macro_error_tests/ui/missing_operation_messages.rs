// Test that missing messages in operation produces a helpful error

use protofolio_derive::AsyncApiOperation;

#[derive(AsyncApiOperation)]
#[asyncapi(
    id = "op-1",
    action = "send",
    channel = "events"
)]
pub struct MyOperation;

