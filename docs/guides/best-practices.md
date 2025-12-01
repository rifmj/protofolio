# Best Practices ⭐

This guide covers recommended patterns and conventions for using `protofolio`. Let's make your code shine! ✨

## Error Handling 🎭

### Use `try_asyncapi()` in Production 🚀

Use `try_asyncapi()` for production code to handle errors gracefully:

```rust
match MyApi::try_asyncapi() {
    Ok(spec) => {
        // Use the spec
        let json = protofolio::to_json(&spec)?;
    }
    Err(e) => {
        // Handle validation errors
        eprintln!("Failed to generate spec: {}", e);
    }
}
```

Use `asyncapi()` only when you're certain the spec is valid (e.g., in tests or during development).

## Channel Naming 🏷️

Follow consistent naming conventions - consistency is key! 🔑

- Use dot-separated hierarchical names: `order.created`, `order.status.changed`
- Use lowercase with dots or hyphens: `user.events`, `payment-processed`
- Include version in message IDs, not channel names: `order-created-v1` (messageId) vs `order.created` (channel)

### Good Examples ✨

```rust
// Hierarchical naming - clean and organized! 🎯
channels("order.created", "order.status.changed", "order.payment.processed")

// Consistent patterns - easy to understand! 📚
channels("user.events", "user.notifications", "user.profile.updated")
```

### Alternative Approaches

For consistency, prefer the patterns shown above. Alternative naming styles can work but may reduce clarity:

```rust
// Alternative - mixed naming styles
channels("OrderCreated", "order_status_changed", "order-payment-processed")

// Alternative - versioning in channel names (prefer messageId for versions)
channels("order.created.v1", "order.created.v2")  // Consider using messageId for versions instead
```

## Message Versioning 🔢

Version your messages using `messageId` - keep track of changes! 📝

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

**Benefits** 🎁:

- ✨ Clear versioning strategy
- 🔄 Multiple versions can coexist on the same channel
- 📊 Easy to track breaking changes

## Root-Level Tags 🏷️

Use root-level tags to organize and document your API at the specification level. Tags provide a centralized way to categorize operations and messages - super handy! 🎯

```rust
#[derive(AsyncApi)]
#[asyncapi(
    info(title = "E-Commerce API", version = "1.0.0"),
    tags(
        (name = "orders", description = "Order management operations and events"),
        (name = "payments", description = "Payment processing"),
        (name = "inventory", description = "Inventory management")
    ),
    channels("order.created", "payment.processed", "inventory.updated"),
    messages(OrderCreated, PaymentProcessed, InventoryUpdated)
)]
pub struct ECommerceApi;
```

**Best Practices** 💡:

- 🎯 Use descriptive tag names that reflect your domain (e.g., `orders`, `payments`, `users`)
- 📝 Include descriptions to help documentation tools and API consumers
- 🔄 Keep tag names consistent across your API
- 🎨 Use tags to group related operations and messages

**Benefits** 🎁:

- 📊 Centralized organization of API elements
- 🔍 Better documentation and discoverability
- 🧭 Easier filtering and navigation in API tools
- ✨ Consistent categorization across the specification

## Validation Flow ✅

Always validate your specs, especially in production - safety first! 🛡️

```rust
let spec = MyApi::try_asyncapi()?;  // Returns Result
protofolio::validate_spec(&spec)?;  // Additional validation
```

## Schema Generation ⚡

Schema generation is automatically cached by type, so repeated calls are fast! 🚀 However, for large specifications:

- Consider splitting into multiple `AsyncApi` structs
- Use simpler types where possible
- Cache the generated spec if you need to serialize it frequently

### Example: Splitting Large Specs

```rust
// Instead of one large spec
#[derive(AsyncApi)]
#[asyncapi(/* 100+ messages */)]
pub struct HugeApi;

// Split into domain-specific specs
#[derive(AsyncApi)]
#[asyncapi(/* order-related messages */)]
pub struct OrderApi;

#[derive(AsyncApi)]
#[asyncapi(/* user-related messages */)]
pub struct UserApi;
```

## Type Design 🎨

### Use Concrete Types 💎

Prefer concrete types over generics - clarity wins! ✨

```rust
// Recommended - concrete type
pub struct UserMessage {
    pub user_id: String,
    pub timestamp: i64,
}

// Alternative - generic type (requires manual JsonSchema implementation)
pub struct GenericMessage<T> {
    pub data: T,
}
```

### Use Type Aliases 🔤

For common types, use type aliases - keep it DRY! 💧

```rust
type UserId = String;
type Timestamp = i64;

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(channel = "events")]
pub struct Event {
    pub user_id: UserId,
    pub timestamp: Timestamp,
}
```

### Handling Circular References 🔄

For types with circular references, use `Option` to break the cycle - smart move! 🧠

```rust
// Direct circular reference
pub struct Node {
    pub children: Vec<Node>,
}

// Recommended - use Option to break the cycle
pub struct Node {
    pub children: Option<Vec<Node>>,
}
```

## Documentation 📚

### Add Descriptions ✍️

Always add descriptions to your messages and operations - your future self will thank you! 🙏

```rust
#[derive(AsyncApiMessage)]
#[asyncapi(
    channel = "order.status.changed",
    description = "Published when order status changes",  // Clear description
    summary = "Order Status Changed Event"  // Brief summary
)]
pub struct OrderStatusChanged { /* ... */ }
```

### Use Meaningful Names 🏷️

Choose clear, descriptive names - make it obvious what things do! 💡

```rust
// Recommended - clear and descriptive
pub struct OrderStatusChanged { /* ... */ }
pub struct UserProfileUpdated { /* ... */ }

// Alternative - less descriptive names
pub struct Event1 { /* ... */ }
pub struct Update { /* ... */ }
```

## Organization 📁

### Group Related Messages 🗂️

Group related messages in the same module or file - stay organized! 🎯

```rust
// order.rs
pub mod order {
    #[derive(AsyncApiMessage)]
    #[asyncapi(channel = "order.created")]
    pub struct OrderCreated { /* ... */ }

    #[derive(AsyncApiMessage)]
    #[asyncapi(channel = "order.updated")]
    pub struct OrderUpdated { /* ... */ }
}
```

### Define Messages Before API 📝

Define message and operation types before the `AsyncApi` struct that references them - order matters! ⚡

```rust
// Recommended - messages defined first
#[derive(AsyncApiMessage)]
#[asyncapi(channel = "events")]
pub struct MyMessage { /* ... */ }

#[derive(AsyncApi)]
#[asyncapi(messages(MyMessage))]
pub struct MyApi;

// Alternative - messages defined after (define messages first for best results)
#[derive(AsyncApi)]
#[asyncapi(messages(MyMessage))]
pub struct MyApi;

#[derive(AsyncApiMessage)]
#[asyncapi(channel = "events")]
pub struct MyMessage { /* ... */ }
```

### Use Components for Reusable Definitions 🔗

When the same structure is used across multiple places, define it as a component - DRY principle! 💧

#### Reusable Messages

```rust
use protofolio::{AsyncApiBuilder, Message, MessageOrRef};
use std::collections::HashMap;

// ✅ Good - define once, reuse multiple times
let spec = AsyncApiBuilder::new()
    .component_message(
        "CommonEvent".to_string(),
        Message { /* ... */ }
    )
    .channel("events.user".to_string(), Channel {
        messages: {
            let mut m = HashMap::new();
            m.insert("CommonEvent".to_string(), MessageOrRef::component_ref("CommonEvent"));
            m
        },
        // ...
    })
    .channel("events.system".to_string(), Channel {
        messages: {
            let mut m = HashMap::new();
            m.insert("CommonEvent".to_string(), MessageOrRef::component_ref("CommonEvent"));
            m
        },
        // ...
    })
    .build();
```

#### Reusable Bindings and Traits

```rust
use protofolio::{AsyncApiBuilder, OperationTrait, MessageTrait, ChannelBindingsOrRef};

// ✅ Good - define bindings once, reuse across channels
let spec = AsyncApiBuilder::new()
    .component_channel_bindings(
        "KafkaBinding".to_string(),
        serde_json::json!({
            "kafka": {
                "topic": "events",
                "partitions": 3
            }
        }),
    )
    .component_operation_trait(
        "CommonTrait".to_string(),
        OperationTrait { /* ... */ }
    )
    .component_message_trait(
        "CommonMessageTrait".to_string(),
        MessageTrait { /* ... */ }
    )
    .channel("events".to_string(), Channel {
        address: "events".to_string(),
        // ...
        bindings: Some(ChannelBindingsOrRef::component_ref("KafkaBinding")),
        // ...
    })
    .build();
```

Benefits 🎁:

- 🎯 Single source of truth for reusable definitions
- 🔧 Easier maintenance (update once, applies everywhere)
- ✨ Consistent definitions across your API
- 📉 Reduced specification size
- 🎨 Better organization with component references

## See Also

- [Messages Guide](messages.md) - Message definition best practices
- [Operations Guide](operations.md) - Operation definition best practices
- [Validation Guide](validation.md) - Validation best practices
