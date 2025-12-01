# Basic Examples 💡

This page contains basic examples of using `protofolio`. Perfect for getting started! 🚀

## Simple Message and API 🎯

The most basic example - let's start simple:

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

## Enhanced Message Attributes ✨

Example with all message attributes - let's see what's possible:

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
        external_docs(url = "https://example.com/api-docs", description = "Full API documentation"),
        contact(
            name = "API Support",
            email = "support@example.com",
            url = "https://example.com/contact"
        ),
        license(
            name = "Apache 2.0",
            url = "https://www.apache.org/licenses/LICENSE-2.0"
        ),
        terms_of_service = "https://example.com/terms"
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

## Multiple Messages on Same Channel 🔄

You can have multiple messages on the same channel:

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

## Multiple Servers 🌐

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

## Server Variables 🔧

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

## Security Schemes 🔐

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

## External Documentation 📚

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

## Info Fields (Contact, License, Terms of Service) ℹ️

The `info` section supports additional metadata about your API:

```rust
#[derive(AsyncApi)]
#[asyncapi(
    info(
        title = "My API",
        version = "1.0.0",
        description = "API description",
        // Contact information (all fields optional)
        contact(
            name = "API Support",
            email = "support@example.com",
            url = "https://example.com/contact"
        ),
        // License information (name required, url optional)
        license(
            name = "Apache 2.0",
            url = "https://www.apache.org/licenses/LICENSE-2.0"
        ),
        // Terms of Service URL
        terms_of_service = "https://example.com/terms"
    ),
    channels("events"),
    messages(Event)
)]
pub struct MyApi;
```

All Info fields are optional. You can use any combination:

```rust
// Minimal - only contact name
#[derive(AsyncApi)]
#[asyncapi(
    info(
        title = "My API",
        version = "1.0.0",
        contact(name = "Support")
    ),
    channels("events"),
    messages(Event)
)]
pub struct MinimalApi;

// Full example with all fields
#[derive(AsyncApi)]
#[asyncapi(
    info(
        title = "Production API",
        version = "2.0.0",
        description = "Production API for our service",
        contact(
            name = "Engineering Team",
            email = "eng@example.com",
            url = "https://example.com/contact"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        ),
        terms_of_service = "https://example.com/legal/terms",
        external_docs(url = "https://docs.example.com", description = "Full documentation")
    ),
    channels("events"),
    messages(Event)
)]
pub struct ProductionApi;
```

## Message Examples 💡

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

## Message Headers 📋

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

## Complete Example with Examples and Headers 🎯

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

## Correlation IDs 🔗

Correlation IDs help track related messages across your system - keep everything connected! 🎯

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

## Root-Level Tags 🏷️

Root-level tags provide reusable tag definitions at the specification level. These tags can be referenced by name in messages and operations, providing a centralized way to organize and document your API - super organized! 🎯

```rust
#[derive(AsyncApi)]
#[asyncapi(
    info(title = "E-Commerce API", version = "1.0.0"),
    tags(
        (name = "orders", description = "Order-related operations and events"),
        (name = "payments", description = "Payment processing events"),
        (name = "users", description = "User management operations")
    ),
    channels("order.created", "payment.processed"),
    messages(OrderCreated, PaymentProcessed)
)]
pub struct ECommerceApi;
```

Tags can have optional descriptions - add them for better docs! 📝

```rust
#[derive(AsyncApi)]
#[asyncapi(
    info(title = "My API", version = "1.0.0"),
    tags(
        (name = "orders", description = "Order operations"),
        (name = "events")  // Description is optional
    ),
    channels("order.created"),
    messages(OrderCreated)
)]
pub struct MyApi;
```

Root-level tags are serialized in the generated AsyncAPI specification and can be used by documentation tools to organize and filter operations and messages. Pretty neat! ✨

## Components and `$ref` References 🔗

Use components to define reusable messages that can be referenced from multiple channels - DRY at its finest! 💧

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
        contact: None,
        license: None,
        terms_of_service: None,
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
        contact: None,
        license: None,
        terms_of_service: None,
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

### Component Parameters, Bindings, and Traits

You can also define reusable parameters, bindings, and traits:

```rust
use protofolio::{AsyncApiBuilder, Parameter, OperationTrait, MessageTrait, Tag, MessagePayload, CorrelationId, ExternalDocumentation, ChannelBindingsOrRef};

let spec = AsyncApiBuilder::new()
    .info(Info {
        title: "My API".to_string(),
        version: "1.0.0".to_string(),
        description: None,
        external_docs: None,
        contact: None,
        license: None,
        terms_of_service: None,
    })
    // Reusable parameter component
    .component_parameter(
        "UserIdParam".to_string(),
        Parameter {
            description: Some("User ID parameter".to_string()),
            schema: Some(serde_json::json!({"type": "string"})),
            location: Some("$message.header#/userId".to_string()),
        },
    )
    // Reusable channel bindings component
    .component_channel_bindings(
        "KafkaBinding".to_string(),
        serde_json::json!({
            "kafka": {
                "topic": "events",
                "partitions": 3
            }
        }),
    )
    // Reusable message bindings component
    .component_message_bindings(
        "KafkaMessageBinding".to_string(),
        serde_json::json!({
            "kafka": {
                "key": {"type": "string"}
            }
        }),
    )
    // Reusable operation trait
    .component_operation_trait(
        "CommonTrait".to_string(),
        OperationTrait {
            summary: Some("Common operation".to_string()),
            description: None,
            tags: Some(vec![Tag {
                name: "common".to_string(),
                description: None,
            }]),
            external_docs: None,
            bindings: None,
        },
    )
    // Reusable message trait
    .component_message_trait(
        "CommonMessageTrait".to_string(),
        MessageTrait {
            headers: Some(MessagePayload {
                schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "correlation_id": {"type": "string"}
                    }
                }),
            }),
            correlation_id: Some(CorrelationId {
                location: "$message.header#/correlationId".to_string(),
                description: None,
            }),
            content_type: Some("application/json".to_string()),
            name: None,
            title: None,
            summary: None,
            description: None,
            tags: None,
            external_docs: None,
            examples: None,
            bindings: None,
        },
    )
    // Use component bindings reference
    .channel(
        "events".to_string(),
        Channel {
            address: "events".to_string(),
            description: None,
            messages: HashMap::new(),
            servers: None,
            parameters: None,
            bindings: Some(ChannelBindingsOrRef::component_ref("KafkaBinding")),
        },
    )
    .build();
```

## See Also

- [Advanced Examples](advanced.md) - More complex examples
- [Integration Examples](integration.md) - Framework integration examples
- [Messages Guide](../guides/messages.md#components-and-ref-references) - Detailed documentation on components
