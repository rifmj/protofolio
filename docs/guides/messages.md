# Messages Guide

Messages are the core building blocks of your AsyncAPI specification. This guide covers how to define and configure message types.

## Basic Message Definition

A message type must implement `Serialize`, `Deserialize`, and `JsonSchema`:

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

## Message Attributes

The `AsyncApiMessage` derive macro supports the following attributes:

### Required Attributes

- `channel` - The channel name for this message (required)

### Optional Attributes

- `messageId` - Unique message identifier
- `name` - Message name
- `title` - Message title
- `summary` - Brief summary of the message
- `description` - Detailed description
- `contentType` - Content type (default: "application/json")
- `tags` - Array of tag names (e.g., `tags = ["order", "status"]`)

## Complete Example

```rust
use protofolio_derive::AsyncApiMessage;
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
    tags = ["order", "status"]
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

## See Also

- [Operations Guide](operations.md) - How to define operations that use messages
- [Examples](../examples/basic.md) - More message examples
- [Best Practices](best-practices.md) - Message naming and versioning conventions
