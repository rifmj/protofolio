// Test that missing info(title) produces a helpful error

use protofolio_derive::AsyncApi;

#[derive(AsyncApi)]
#[asyncapi(
    info(version = "1.0.0"),
    channels("events"),
    messages()
)]
pub struct MyApi;

