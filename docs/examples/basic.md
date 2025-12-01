# Basic Examples

This page contains basic examples of using `protofolio`.

## Simple Message and API

The most basic example:

```rust
use protofolio::AsyncApi;
use protofolio_derive::{AsyncApi, AsyncApiMessage};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// Define your message type
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(channel = "events", messageId = "event-v1")]
pub struct Event {
    pub id: String,
    pub data: String,
}

// Define your AsyncAPI specification
#[derive(AsyncApi)]
#[asyncapi(
    info(title = "My API", version = "1.0.0"),
    channels("events"),
    messages(Event)
)]
pub struct MyApi;

// Generate the spec
let spec = MyApi::asyncapi();
```

## Enhanced Message Attributes

Example with all message attributes:

```rust
use protofolio::AsyncApi;
use protofolio_derive::{AsyncApi, AsyncApiMessage};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "order.status.changed",
    messageId = "order-status-changed-v1",
    name = "OrderStatusChanged",
    title = "Order Status Changed",
    summary = "Order Status Changed Event",
    description = "Published when order status changes",
    contentType = "application/json",
    tags = ["order", "status"],
    external_docs(url = "https://example.com/docs/order-status", description = "Order status documentation")
)]
pub struct OrderStatusChanged {
    pub order_id: String,
    pub customer_id: String,
    pub new_status: String,
}

#[derive(AsyncApi)]
#[asyncapi(
    info(
        title = "E-Commerce Events API",
        version = "1.0.0",
        description = "Real-time e-commerce order events",
        external_docs(url = "https://example.com/api-docs", description = "Full API documentation")
    ),
    servers(
        (name = "nats", url = "nats://nats:4222", protocol = "nats")
    ),
    channels("order.status.changed"),
    messages(OrderStatusChanged)
)]
pub struct ECommerceApi;

// Generate JSON
let json = ECommerceApi::asyncapi_json()?;

// Generate YAML
let yaml = ECommerceApi::asyncapi_yaml()?;
```

## Multiple Messages on Same Channel

```rust
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(channel = "product.events", messageId = "product-created-v1")]
pub struct ProductCreated {
    pub product_id: String,
}

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(channel = "product.events", messageId = "product-updated-v1")]
pub struct ProductUpdated {
    pub product_id: String,
    pub changes: Vec<String>,
}

#[derive(AsyncApi)]
#[asyncapi(
    info(title = "Product Events", version = "1.0.0"),
    channels("product.events"),
    messages(ProductCreated, ProductUpdated)
)]
pub struct ProductEventsApi;
```

## Multiple Servers

```rust
#[derive(AsyncApi)]
#[asyncapi(
    info(title = "Multi-Server API", version = "1.0.0"),
    servers(
        (name = "production", url = "nats://prod:4222", protocol = "nats"),
        (name = "staging", url = "nats://staging:4222", protocol = "nats")
    ),
    channels("events"),
    messages(EventMessage)
)]
pub struct MultiServerApi;
```

## Server Variables

Use templated URLs with server variables for flexible server configurations:

```rust
#[derive(AsyncApi)]
#[asyncapi(
    info(title = "API with Variables", version = "1.0.0"),
    servers(
        (
            name = "nats",
            url = "nats://{host}:{port}",
            protocol = "nats",
            variables = [
                (name = "host", default = "localhost", description = "Server hostname"),
                (name = "port", default = "4222", enum_values = ["4222", "4223", "4224"], description = "Server port")
            ]
        )
    ),
    channels("events"),
    messages(Event)
)]
pub struct ApiWithVariables;
```

Server variables support:

- `name` - Variable name (required)
- `default` - Default value for the variable
- `description` - Description of the variable
- `enum_values` - List of allowed values (note: use `enum_values` instead of `enum` since `enum` is a Rust keyword)
- `examples` - Example values for the variable

## Security Schemes

Define security schemes and apply them to servers:

```rust
#[derive(AsyncApi)]
#[asyncapi(
    info(title = "Secure API", version = "1.0.0"),
    servers(
        (name = "nats", url = "nats://localhost:4222", protocol = "nats", security = ["userPassword"])
    ),
    security_schemes(
        (name = "userPassword", type = "userPassword", description = "User and password authentication"),
        (name = "apiKey", type = "apiKey", in = "header", description = "API key in header")
    ),
    channels("events"),
    messages(Event)
)]
pub struct SecureApi;
```

## External Documentation

Add external documentation references to Info, Messages, and Operations:

```rust
#[derive(AsyncApi)]
#[asyncapi(
    info(
        title = "My API",
        version = "1.0.0",
        external_docs(url = "https://example.com/api-docs", description = "Full API documentation")
    ),
    channels("events"),
    messages(Event)
)]
pub struct MyApi;

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "events",
    messageId = "event-v1",
    external_docs(url = "https://example.com/message-docs")
)]
pub struct Event {
    pub id: String,
}

#[derive(AsyncApiOperation)]
#[asyncapi(
    id = "publish-event",
    action = "send",
    channel = "events",
    messages(Event),
    external_docs(url = "https://example.com/operation-docs", description = "Operation documentation")
)]
pub struct PublishEvent;
```

## See Also

- [Advanced Examples](advanced.md) - More complex examples
- [Integration Examples](integration.md) - Framework integration examples
