# Best Practices

This guide covers recommended patterns and conventions for using `protofolio`.

## Error Handling

### Use `try_asyncapi()` in Production

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

## Channel Naming

Follow consistent naming conventions:

- Use dot-separated hierarchical names: `order.created`, `order.status.changed`
- Use lowercase with dots or hyphens: `user.events`, `payment-processed`
- Include version in message IDs, not channel names: `order-created-v1` (messageId) vs `order.created` (channel)

### Good Examples

```rust
// Hierarchical naming
channels("order.created", "order.status.changed", "order.payment.processed")

// Consistent patterns
channels("user.events", "user.notifications", "user.profile.updated")
```

### Avoid

```rust
// ❌ Inconsistent naming
channels("OrderCreated", "order_status_changed", "order-payment-processed")

// ❌ Including version in channel name
channels("order.created.v1", "order.created.v2")  // Use messageId for versions instead
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

**Benefits**:
- Clear versioning strategy
- Multiple versions can coexist on the same channel
- Easy to track breaking changes

## Validation Flow

Always validate your specs, especially in production:

```rust
let spec = MyApi::try_asyncapi()?;  // Returns Result
protofolio::validate_spec(&spec)?;  // Additional validation
```

## Schema Generation

Schema generation is automatically cached by type, so repeated calls are fast. However, for large specifications:

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

## Type Design

### Use Concrete Types

Prefer concrete types over generics:

```rust
// ✅ Good - concrete type
pub struct UserMessage {
    pub user_id: String,
    pub timestamp: i64,
}

// ❌ Avoid - generic type (requires manual JsonSchema impl)
pub struct GenericMessage<T> {
    pub data: T,
}
```

### Use Type Aliases

For common types, use type aliases:

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

### Avoid Circular References

Break circular references using `Option`:

```rust
// ❌ Problematic - circular reference
pub struct Node {
    pub children: Vec<Node>,  // Can cause issues
}

// ✅ Better - use Option or limit depth
pub struct Node {
    pub children: Option<Vec<Node>>,
}
```

## Documentation

### Add Descriptions

Always add descriptions to your messages and operations:

```rust
#[derive(AsyncApiMessage)]
#[asyncapi(
    channel = "order.status.changed",
    description = "Published when order status changes",  // Clear description
    summary = "Order Status Changed Event"  // Brief summary
)]
pub struct OrderStatusChanged { /* ... */ }
```

### Use Meaningful Names

Choose clear, descriptive names:

```rust
// ✅ Good - clear and descriptive
pub struct OrderStatusChanged { /* ... */ }
pub struct UserProfileUpdated { /* ... */ }

// ❌ Avoid - vague or unclear
pub struct Event1 { /* ... */ }
pub struct Update { /* ... */ }
```

## Organization

### Group Related Messages

Group related messages in the same module or file:

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

### Define Messages Before API

Define message and operation types before the `AsyncApi` struct that references them:

```rust
// ✅ Good - messages defined first
#[derive(AsyncApiMessage)]
#[asyncapi(channel = "events")]
pub struct MyMessage { /* ... */ }

#[derive(AsyncApi)]
#[asyncapi(messages(MyMessage))]
pub struct MyApi;

// ❌ Avoid - messages defined after (may cause macro ordering issues)
#[derive(AsyncApi)]
#[asyncapi(messages(MyMessage))]
pub struct MyApi;

#[derive(AsyncApiMessage)]
#[asyncapi(channel = "events")]
pub struct MyMessage { /* ... */ }
```

## See Also

- [Messages Guide](messages.md) - Message definition best practices
- [Operations Guide](operations.md) - Operation definition best practices
- [Validation Guide](validation.md) - Validation best practices

