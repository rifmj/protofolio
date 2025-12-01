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

## Message Examples

Provide example payloads to help consumers understand message formats:

```rust
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "order.created",
    messageId = "order-created-v1",
    example = r#"{"order_id": "12345", "customer_id": "user-789", "total": 99.99}"#
)]
pub struct OrderCreated {
    pub order_id: String,
    pub customer_id: String,
    pub total: f64,
}
```

For multiple examples:

```rust
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "order.created",
    messageId = "order-created-v1",
    examples = [
        r#"{"order_id": "12345", "customer_id": "user-789", "total": 99.99}"#,
        r#"{"order_id": "67890", "customer_id": "user-456", "total": 149.50}"#
    ]
)]
pub struct OrderCreated {
    pub order_id: String,
    pub customer_id: String,
    pub total: f64,
}
```

## Message Headers

Define message headers using a separate type:

```rust
// Define header schema
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct MessageHeaders {
    pub correlation_id: String,
    pub user_id: Option<String>,
}

// Use in message
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "order.created",
    messageId = "order-created-v1",
    headers = MessageHeaders
)]
pub struct OrderCreated {
    pub order_id: String,
    pub customer_id: String,
    pub total: f64,
}
```

## Complete Example with Examples and Headers

```rust
use protofolio::AsyncApi;
use protofolio_derive::{AsyncApi, AsyncApiMessage};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// Define headers
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct OrderHeaders {
    pub correlation_id: String,
    pub trace_id: Option<String>,
}

// Define message with examples and headers
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "order.created",
    messageId = "order-created-v1",
    name = "OrderCreated",
    title = "Order Created Event",
    summary = "Published when a new order is created",
    example = r#"{"order_id": "12345", "customer_id": "user-789", "total": 99.99}"#,
    headers = OrderHeaders,
    correlation_id(location = "$message.header#/correlationId", description = "Correlation ID for tracking")
)]
pub struct OrderCreated {
    pub order_id: String,
    pub customer_id: String,
    pub total: f64,
}

#[derive(AsyncApi)]
#[asyncapi(
    info(title = "Order Service API", version = "1.0.0"),
    channels("order.created"),
    messages(OrderCreated)
)]
pub struct OrderApi;

// Generate spec
let spec = OrderApi::asyncapi();
```

## Correlation IDs

Correlation IDs help track related messages across your system:

```rust
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "order.status.changed",
    messageId = "order-status-changed-v1",
    correlation_id(location = "$message.header#/correlationId")
)]
pub struct OrderStatusChanged {
    pub order_id: String,
    pub new_status: String,
}
```

With description:

```rust
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "order.status.changed",
    messageId = "order-status-changed-v1",
    correlation_id(
        location = "$message.header#/correlationId",
        description = "Correlation ID for tracking order events"
    )
)]
pub struct OrderStatusChanged {
    pub order_id: String,
    pub new_status: String,
}
```

## Components and `$ref` References

Use components to define reusable messages that can be referenced from multiple channels:

```rust
use protofolio::{AsyncApiBuilder, Message, MessagePayload, MessageOrRef, Channel, Info};
use std::collections::HashMap;

// Define a reusable component message
let spec = AsyncApiBuilder::new()
    .info(Info {
        title: "My API".to_string(),
        version: "1.0.0".to_string(),
        description: None,
        external_docs: None,
    })
    // Define a component message
    .component_message(
        "CommonEvent".to_string(),
        Message {
            message_id: Some("common-event-v1".to_string()),
            name: Some("CommonEvent".to_string()),
            title: Some("Common Event".to_string()),
            summary: Some("A reusable event message".to_string()),
            description: None,
            content_type: Some("application/json".to_string()),
            tags: None,
            external_docs: None,
            payload: MessagePayload {
                schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "id": {"type": "string"},
                        "timestamp": {"type": "integer"},
                        "data": {"type": "string"}
                    },
                    "required": ["id", "timestamp"]
                }),
            },
            examples: None,
            headers: None,
        },
    )
    // Reference the component in multiple channels
    .channel(
        "events.user".to_string(),
        Channel {
            description: None,
            messages: {
                let mut m = HashMap::new();
                m.insert(
                    "CommonEvent".to_string(),
                    MessageOrRef::component_ref("CommonEvent"),
                );
                m
            },
            servers: None,
            parameters: None,
            bindings: None,
        },
    )
    .channel(
        "events.system".to_string(),
        Channel {
            description: None,
            messages: {
                let mut m = HashMap::new();
                m.insert(
                    "CommonEvent".to_string(),
                    MessageOrRef::component_ref("CommonEvent"),
                );
                m
            },
            servers: None,
            parameters: None,
            bindings: None,
        },
    )
    .build();
```

You can also define reusable schema components:

```rust
let spec = AsyncApiBuilder::new()
    .info(Info {
        title: "My API".to_string(),
        version: "1.0.0".to_string(),
        description: None,
        external_docs: None,
    })
    // Define a reusable schema component
    .component_schema(
        "UserSchema".to_string(),
        serde_json::json!({
            "type": "object",
            "properties": {
                "id": {"type": "string"},
                "name": {"type": "string"},
                "email": {"type": "string"}
            },
            "required": ["id", "name", "email"]
        }),
    )
    .build();
```

## See Also

- [Advanced Examples](advanced.md) - More complex examples
- [Integration Examples](integration.md) - Framework integration examples
- [Messages Guide](../guides/messages.md#components-and-ref-references) - Detailed documentation on components
