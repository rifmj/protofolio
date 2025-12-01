# Operations Guide

Operations define publish/subscribe actions on channels. This guide covers how to define and use operations.

## What are Operations?

Operations describe how messages are sent or received on channels:
- **Send operations**: Define how messages are published to a channel
- **Receive operations**: Define how messages are consumed from a channel

## Basic Operation Definition

Use the `AsyncApiOperation` derive macro to define operations:

```rust
use protofolio_derive::AsyncApiOperation;

#[derive(AsyncApiOperation)]
#[asyncapi(
    id = "publish-order-created",
    action = "send",
    channel = "order.created",
    messages(OrderCreated),
    summary = "Publish order created event"
)]
pub struct PublishOrderCreated;
```

## Operation Attributes

The `AsyncApiOperation` derive macro supports the following attributes:

### Required Attributes

- `id` - Unique operation identifier (required)
- `action` - Either "send" or "receive" (required)
- `channel` - The channel name for this operation (required)
- `messages(...)` - List of message types (at least one required)

### Optional Attributes

- `summary` - Brief summary of the operation
- `description` - Detailed description
- `tags` - Array of tag names (e.g., `tags = ["orders", "events"]`)
- `external_docs` - External documentation reference (e.g., `external_docs(url = "https://example.com/docs", description = "Documentation")`)

## Send Operations

Send operations define how messages are published:

```rust
#[derive(AsyncApiOperation)]
#[asyncapi(
    id = "publish-order-created",
    action = "send",
    channel = "order.created",
    messages(OrderCreated),
    summary = "Publish order created event",
    description = "Published when a new order is created",
    tags = ["orders", "events"],
    external_docs(url = "https://example.com/docs/operations", description = "Operation documentation")
)]
pub struct PublishOrderCreated;
```

## Receive Operations

Receive operations define how messages are consumed:

```rust
#[derive(AsyncApiOperation)]
#[asyncapi(
    id = "subscribe-order-status",
    action = "receive",
    channel = "order.status.changed",
    messages(OrderStatusChanged),
    summary = "Subscribe to order status changes"
)]
pub struct SubscribeOrderStatus;
```

## Registering Operations

After defining your operations, register them in your `AsyncApi` specification:

```rust
use protofolio::AsyncApi;
use protofolio_derive::AsyncApi;

#[derive(AsyncApi)]
#[asyncapi(
    info(title = "E-Commerce Events API", version = "1.0.0"),
    servers((name = "nats", url = "nats://nats:4222", protocol = "nats")),
    channels("order.created", "order.status.changed"),
    messages(OrderCreated, OrderStatusChanged),
    operations(PublishOrderCreated, SubscribeOrderStatus)  // Register operations
)]
pub struct ECommerceApi;
```

## Operations with Multiple Messages

An operation can reference multiple messages:

```rust
#[derive(AsyncApiOperation)]
#[asyncapi(
    id = "handle-product-events",
    action = "receive",
    channel = "product.events",
    messages(ProductCreated, ProductUpdated),  // Multiple messages
    summary = "Handle all product events"
)]
pub struct HandleProductEvents;
```

## Validation

Operations are validated to ensure:
- The channel exists in the declared channels
- All referenced messages exist in the channel
- The action is either "send" or "receive"
- At least one message is specified

If validation fails, you'll get detailed error messages with suggestions.

## See Also

- [Messages Guide](messages.md) - How to define message types
- [Validation Guide](validation.md) - How validation works
- [Examples](../examples/basic.md) - Operation examples

