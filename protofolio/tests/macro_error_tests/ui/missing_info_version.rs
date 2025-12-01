// Test that missing info(version) produces a helpful error

use protofolio_derive::AsyncApi;

#[derive(AsyncApi)]
#[asyncapi(
    info(title = "My API"),
    channels("events"),
    messages()
)]
pub struct MyApi;

