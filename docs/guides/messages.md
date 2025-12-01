# Messages Guide 💬

Messages are the core building blocks of your AsyncAPI specification. This guide covers how to define and configure message types. Let's dive in! 🚀

## Basic Message Definition 🎯

A message type must implement `Serialize`, `Deserialize`, and `JsonSchema` - easy peasy! ✨

```rust
use protofolio_derive::AsyncApiMessage;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(channel = "events")]
pub struct MyMessage {
    pub id: String,
    pub data: String,
}
```

## Message Attributes 🏷️

The `AsyncApiMessage` derive macro supports the following attributes - lots of options! 🎨

### Required Attributes ⚡

- `channel` - The channel name for this message (required)

### Optional Attributes ✨

- `messageId` - Unique message identifier
- `name` - Message name
- `title` - Message title
- `summary` - Brief summary of the message
- `description` - Detailed description
- `contentType` - Content type (default: "application/json")
- `tags` - Array of tag names (e.g., `tags = ["order", "status"]`)
- `external_docs` - External documentation reference (e.g., `external_docs(url = "https://example.com/docs", description = "Documentation")`)
- `example` - Single message example as JSON string (e.g., `example = r#"{"id": "123"}"#`)
- `examples` - Multiple message examples as array of JSON strings (e.g., `examples = [r#"{"id": "1"}"#, r#"{"id": "2"}"#]`)
- `headers` - Message headers schema type (e.g., `headers = MessageHeaders` where `MessageHeaders` implements `JsonSchema`)
- `correlation_id` - Correlation ID definition for message tracking (e.g., `correlation_id(location = "$message.header#/correlationId", description = "Correlation ID")`)

## Complete Example

```rust
use protofolio_derive::AsyncApiMessage;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// Define headers type
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct OrderHeaders {
    pub correlation_id: String,
    pub user_id: Option<String>,
}

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
    example = r#"{"order_id": "12345", "customer_id": "user-789", "new_status": "shipped"}"#,
    headers = OrderHeaders,
    correlation_id(location = "$message.header#/correlationId", description = "Correlation ID for tracking order events"),
    external_docs(url = "https://example.com/docs/order-status", description = "Order status documentation")
)]
pub struct OrderStatusChanged {
    pub order_id: String,
    pub customer_id: String,
    pub new_status: String,
}
```

## Registering Messages

After defining your message types, register them in your `AsyncApi` specification:

```rust
use protofolio::AsyncApi;
use protofolio_derive::AsyncApi;

#[derive(AsyncApi)]
#[asyncapi(
    info(title = "My API", version = "1.0.0"),
    channels("order.status.changed"),
    messages(OrderStatusChanged)  // Register your message
)]
pub struct MyApi;
```

## Multiple Messages on Same Channel

You can have multiple message types on the same channel:

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
    messages(ProductCreated, ProductUpdated)  // Multiple messages on same channel
)]
pub struct ProductEventsApi;
```

## Message Versioning

Version your messages using `messageId`:

```rust
#[asyncapi(
    channel = "order.events",
    messageId = "order-created-v1"  // Include version
)]
pub struct OrderCreated { /* ... */ }
```

When breaking changes occur, create a new message type with a new version:

```rust
#[asyncapi(
    channel = "order.events",
    messageId = "order-created-v2"  // New version
)]
pub struct OrderCreatedV2 { /* ... */ }
```

## Complex Message Types

### Optional Fields

Messages can have optional fields, which will be reflected in the JSON Schema:

```rust
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(channel = "user.notifications")]
pub struct UserNotification {
    pub user_id: String,
    pub title: String,
    pub message: String,
    pub priority: Option<String>,  // Optional field
    pub metadata: Option<serde_json::Value>,  // Optional nested object
}
```

### Nested Types

Complex nested types are automatically converted to JSON Schema:

```rust
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub location: Location,
}

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(channel = "warehouse.location")]
pub struct WarehouseLocationUpdate {
    pub warehouse_id: String,
    pub address: Address,  // Nested type
    pub timestamp: i64,
}
```

### Enums in Messages

Enum types are properly converted to JSON Schema:

```rust
#[derive(Serialize, Deserialize, JsonSchema)]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(channel = "order.status")]
pub struct OrderStatusUpdate {
    pub order_id: String,
    pub status: OrderStatus,  // Enum type
}
```

## Message Examples

You can provide example payloads for messages to help consumers understand the expected format:

### Single Example

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

### Multiple Examples

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

**Note:** Examples must be valid JSON strings. Use raw string literals (`r#"..."#`) to avoid escaping issues.

## Message Headers

You can define a schema for message headers by specifying a separate type that implements `JsonSchema`:

```rust
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// Define your header type
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct MessageHeaders {
    pub correlation_id: String,
    pub user_id: Option<String>,
    pub request_id: Option<String>,
}

// Use it in your message
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

The header type will be automatically converted to a JSON Schema and included in the AsyncAPI specification. This helps document the structure of headers that accompany your messages.

## Correlation IDs

Correlation IDs allow you to track and correlate related messages across your system. They specify where the correlation ID can be found within the message using AsyncAPI runtime expressions.

### Basic Correlation ID

```rust
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "order.created",
    messageId = "order-created-v1",
    correlation_id(location = "$message.header#/correlationId")
)]
pub struct OrderCreated {
    pub order_id: String,
    pub customer_id: String,
    pub total: f64,
}
```

### Correlation ID with Description

```rust
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "order.created",
    messageId = "order-created-v1",
    correlation_id(
        location = "$message.header#/correlationId",
        description = "Correlation ID for tracking order events across services"
    )
)]
pub struct OrderCreated {
    pub order_id: String,
    pub customer_id: String,
    pub total: f64,
}
```

**Common correlation ID locations:**

- `"$message.header#/correlationId"` - In message headers
- `"$message.payload#/correlationId"` - In message payload
- `"$message.header#/x-correlation-id"` - Custom header field

### Combining Examples and Headers

You can use both examples and headers together:

```rust
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct OrderHeaders {
    pub correlation_id: String,
    pub trace_id: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "order.created",
    messageId = "order-created-v1",
    example = r#"{"order_id": "12345", "total": 99.99}"#,
    headers = OrderHeaders
)]
pub struct OrderCreated {
    pub order_id: String,
    pub total: f64,
}
```

## Components and `$ref` References

Components allow you to define reusable messages that can be referenced from multiple channels or operations using `$ref` references. This is useful when the same message structure is used across different channels.

### Using the Builder API

You can define component messages using the `AsyncApiBuilder`:

```rust
use protofolio::{AsyncApiBuilder, Message, MessagePayload, MessageOrRef};
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
        "CommonMessage".to_string(),
        Message {
            message_id: Some("common-v1".to_string()),
            name: Some("CommonMessage".to_string()),
            title: Some("Common Message".to_string()),
            summary: Some("A reusable message".to_string()),
            description: None,
            content_type: Some("application/json".to_string()),
            tags: None,
            external_docs: None,
            payload: MessagePayload {
                schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "id": {"type": "string"},
                        "data": {"type": "string"}
                    }
                }),
            },
            examples: None,
            headers: None,
        },
    )
    // Reference the component in a channel
    .channel(
        "events".to_string(),
        Channel {
            description: None,
            messages: {
                let mut m = HashMap::new();
                m.insert(
                    "CommonMessage".to_string(),
                    MessageOrRef::component_ref("CommonMessage"),
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

### Referencing Components in Operations

Operations can also reference component messages:

```rust
use protofolio::{MessageReference, Operation, ChannelReference};
use std::collections::HashMap;

// Add an operation that references a component message
let mut operations = HashMap::new();
operations.insert(
    "sendCommonMessage".to_string(),
    Operation {
        action: "send".to_string(),
        channel: ChannelReference {
            ref_path: "#/channels/events".to_string(),
        },
        messages: vec![
            MessageReference {
                ref_path: "#/components/messages/CommonMessage".to_string(),
            },
        ],
        summary: None,
        description: None,
        tags: None,
        external_docs: None,
    },
);
spec.operations = Some(operations);
```

### Component Schemas

You can also define reusable schema components:

```rust
let spec = AsyncApiBuilder::new()
    .info(/* ... */)
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

### Component Parameters

Define reusable parameter components for channels:

```rust
use protofolio::{AsyncApiBuilder, Parameter};

let spec = AsyncApiBuilder::new()
    .info(/* ... */)
    // Define a reusable parameter component
    .component_parameter(
        "UserIdParam".to_string(),
        Parameter {
            description: Some("User ID parameter".to_string()),
            schema: Some(serde_json::json!({
                "type": "string",
                "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
            })),
            location: Some("$message.header#/userId".to_string()),
        },
    )
    .build();
```

### Component Bindings

Define reusable protocol bindings for channels, messages, and servers:

```rust
use protofolio::AsyncApiBuilder;

// Channel bindings component
let spec = AsyncApiBuilder::new()
    .info(/* ... */)
    .component_channel_bindings(
        "KafkaTopicBinding".to_string(),
        serde_json::json!({
            "kafka": {
                "topic": "my-topic",
                "partitions": 3,
                "replicas": 2
            }
        }),
    )
    .build();

// Message bindings component
let spec = AsyncApiBuilder::new()
    .info(/* ... */)
    .component_message_bindings(
        "KafkaMessageBinding".to_string(),
        serde_json::json!({
            "kafka": {
                "key": {
                    "type": "string",
                    "description": "Message key for partitioning"
                }
            }
        }),
    )
    .build();

// Server bindings component
let spec = AsyncApiBuilder::new()
    .info(/* ... */)
    .component_server_bindings(
        "KafkaServerBinding".to_string(),
        serde_json::json!({
            "kafka": {
                "schemaRegistryUrl": "https://schema-registry.example.com"
            }
        }),
    )
    .build();
```

### Component Operation Traits

Define reusable operation traits that can be applied to multiple operations:

```rust
use protofolio::{AsyncApiBuilder, OperationTrait, Tag, ExternalDocumentation};

let spec = AsyncApiBuilder::new()
    .info(/* ... */)
    .component_operation_trait(
        "CommonOperationTrait".to_string(),
        OperationTrait {
            summary: Some("Common operation pattern".to_string()),
            description: Some("This trait applies common properties to operations".to_string()),
            tags: Some(vec![
                Tag {
                    name: "common".to_string(),
                    description: Some("Common operations".to_string()),
                },
            ]),
            external_docs: Some(ExternalDocumentation {
                url: "https://example.com/docs/operations".to_string(),
                description: Some("Operation documentation".to_string()),
            }),
            bindings: None,
        },
    )
    .build();
```

### Component Message Traits

Define reusable message traits that can be applied to multiple messages:

```rust
use protofolio::{AsyncApiBuilder, MessageTrait, MessagePayload, CorrelationId};

let spec = AsyncApiBuilder::new()
    .info(/* ... */)
    .component_message_trait(
        "CommonMessageTrait".to_string(),
        MessageTrait {
            headers: Some(MessagePayload {
                schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "correlation_id": {"type": "string"},
                        "trace_id": {"type": "string"}
                    }
                }),
            }),
            correlation_id: Some(CorrelationId {
                location: "$message.header#/correlationId".to_string(),
                description: Some("Correlation ID for tracking".to_string()),
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
    .build();
```

### Using Component References

Once you've defined components, you can reference them using `$ref` paths:

```rust
use protofolio::{Channel, ChannelBindingsOrRef, MessageOrRef};

// Reference channel bindings component
let channel = Channel {
    address: "events".to_string(),
    description: None,
    messages: HashMap::new(),
    servers: None,
    parameters: None,
    bindings: Some(ChannelBindingsOrRef::component_ref("KafkaTopicBinding")),
};

// Reference message component (shown earlier)
let message_ref = MessageOrRef::component_ref("CommonMessage");
```

### Benefits of Components

- **Reusability**: Define once and reference from multiple places
- **Consistency**: Ensure the same structures are used across your API
- **Maintainability**: Update definitions in one place
- **Reduced duplication**: Avoid repeating definitions
- **Organization**: Group related reusable definitions together

### Validation

The validator ensures that:

- Component references exist in their respective component sections
- Message IDs are unique across both inline messages and components
- Component schemas are valid JSON schemas
- `$ref` paths are correctly formatted

## See Also

- [Operations Guide](operations.md) - How to define operations that use messages
- [Examples](../examples/basic.md) - More message examples
- [Best Practices](best-practices.md) - Message naming and versioning conventions
