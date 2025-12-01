# Troubleshooting Guide 🛠️

This guide helps you resolve common scenarios and questions when using `protofolio`. We've got your back! 💪

## Quick Reference 📋

| Error | Section | Solution |
|-------|---------|----------|
| `no associated item named 'CHANNEL' found` | [Missing Derive](#error-no-associated-item-named-channel-found) | Add `#[derive(AsyncApiMessage)]` or `#[derive(AsyncApiOperation)]` |
| Channel not declared | [Invalid Channel Reference](#error-message-type-x-references-channel-y-which-is-not-declared) | Add channel to `channels(...)` list |
| Schema generation failed | [Missing JsonSchema](#error-failed-to-generate-schema-for-message-type) | Add `#[derive(JsonSchema)]` |
| Operation missing messages | [Missing Messages](#error-asyncapioperation-requires-at-least-one-message) | Add `messages(...)` attribute |
| Runtime panic | [Runtime Errors](#runtime-errors) | Use `try_asyncapi()` for error handling |

## Common Compilation Errors 🔧

### Error: "no associated item named `CHANNEL` found"

**Scenario**: A message or operation type is referenced in `messages(...)` or `operations(...)` but hasn't been processed by its derive macro.

**Solution**: Ensure the type has the appropriate derive macro:

```rust
// Missing derive
pub struct MyMessage { /* ... */ }

// With derive macro
#[derive(AsyncApiMessage)]
#[asyncapi(channel = "events")]
pub struct MyMessage { /* ... */ }
```

### Error: "Message type 'X' references channel 'Y' which is not declared"

**Scenario**: A message references a channel that isn't listed in the `channels(...)` attribute.

**Solution**: Add the channel to the `channels(...)` list:

```rust
// Channel missing from list
#[derive(AsyncApi)]
#[asyncapi(
    channels("other.channel"),  // Missing "events"
    messages(MyMessage)  // MyMessage uses "events" channel
)]
pub struct MyApi;

// With all required channels
#[derive(AsyncApi)]
#[asyncapi(
    channels("events", "other.channel"),  // Includes "events"
    messages(MyMessage)
)]
pub struct MyApi;
```

### Error: "Failed to generate schema for message type"

**Scenario**: The message type doesn't implement `JsonSchema`.

**Solution**: Add `JsonSchema` derive (usually via `schemars`):

```rust
// Missing JsonSchema derive
#[derive(Serialize, Deserialize, AsyncApiMessage)]
pub struct MyMessage { /* ... */ }

// With JsonSchema derive
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
pub struct MyMessage { /* ... */ }
```

### Error: "AsyncApiOperation requires at least one message"

**Scenario**: An operation is defined without any messages.

**Solution**: Add at least one message to the `messages(...)` attribute:

```rust
// Missing messages attribute
#[derive(AsyncApiOperation)]
#[asyncapi(
    id = "my-op",
    action = "send",
    channel = "events"
    // Missing messages(...)
)]
pub struct MyOp;

// With messages specified
#[derive(AsyncApiOperation)]
#[asyncapi(
    id = "my-op",
    action = "send",
    channel = "events",
    messages(MyMessage)  // At least one message required
)]
pub struct MyOp;
```

## Runtime Scenarios ⚡

### Panic: "Message 'X' references channel 'Y' which is not declared"

**Scenario**: Channel validation occurred at runtime (typically caught at compile time, but can occur with macro ordering).

**Solution**: 
1. Ensure the channel is listed in `channels(...)`
2. Check that message types are defined before the `AsyncApi` struct
3. Verify the channel name matches exactly (case-sensitive)

### Panic: "Failed to generate schema for message type"

**Scenario**: The type doesn't implement `JsonSchema` or has unsupported types.

**Solution**:
1. Ensure `#[derive(JsonSchema)]` is present
2. Check that all nested types also implement `JsonSchema`
3. For complex types, you may need to implement `JsonSchema` manually

## Common Scenarios 💡

### Schema generation for generic types

**Scenario**: Generic types require manual `JsonSchema` implementation.

**Solution**: Use concrete types or implement `JsonSchema` manually:

```rust
// Generic type requires manual implementation
pub struct GenericMessage<T> {
    pub data: T,
}

// Recommended - use concrete types or type aliases
pub struct StringMessage {
    pub data: String,
}
```

### Circular references in types

**Scenario**: Types that reference each other may need special handling.

**Solution**: Use `Option` or references to break cycles:

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

### Large specifications performance

**Scenario**: Many messages or complex schemas may take longer to generate.

**Solution**:
1. Consider splitting into multiple `AsyncApi` structs
2. Use simpler types where possible
3. Cache generated specs if generating frequently

### Macro ordering

**Scenario**: Message and operation types should be defined before the `AsyncApi` struct that references them.

**Solution**: Define message types before the API struct:

```rust
// Recommended - messages defined first
#[derive(AsyncApiMessage)]
#[asyncapi(channel = "events")]
pub struct MyMessage { /* ... */ }

#[derive(AsyncApi)]
#[asyncapi(messages(MyMessage))]
pub struct MyApi;
```

## Getting Help 💬

If you need additional assistance, we're here to help! 🤝

1. 📖 Check the [Considerations](limitations.md) guide for design decisions and approaches
2. ⚙️ Review the [Macro Expansion](macro-expansion.md) guide to understand how macros work
3. 🔍 Check the error message carefully - it often includes hints and suggestions
4. 📦 Ensure all dependencies are up to date

## See Also

- [Considerations](limitations.md) - Design decisions and recommended approaches
- [Macro Expansion](macro-expansion.md) - How macros work internally
- [Validation Guide](../guides/validation.md) - Validation details

