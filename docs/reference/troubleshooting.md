# Troubleshooting Guide

This guide helps you diagnose and fix common issues with `protofolio`.

## Quick Reference

| Error | Section | Solution |
|-------|---------|----------|
| `no associated item named 'CHANNEL' found` | [Missing Derive](#error-no-associated-item-named-channel-found) | Add `#[derive(AsyncApiMessage)]` or `#[derive(AsyncApiOperation)]` |
| Channel not declared | [Invalid Channel Reference](#error-message-type-x-references-channel-y-which-is-not-declared) | Add channel to `channels(...)` list |
| Schema generation failed | [Missing JsonSchema](#error-failed-to-generate-schema-for-message-type) | Add `#[derive(JsonSchema)]` |
| Operation missing messages | [Missing Messages](#error-asyncapioperation-requires-at-least-one-message) | Add `messages(...)` attribute |
| Runtime panic | [Runtime Errors](#runtime-errors) | Use `try_asyncapi()` for error handling |

## Common Compilation Errors

### Error: "no associated item named `CHANNEL` found"

**Problem**: A message or operation type is referenced in `messages(...)` or `operations(...)` but hasn't been processed by its derive macro.

**Solution**: Ensure the type has the appropriate derive macro:

```rust
// ❌ Wrong - missing derive
pub struct MyMessage { /* ... */ }

// ✅ Correct
#[derive(AsyncApiMessage)]
#[asyncapi(channel = "events")]
pub struct MyMessage { /* ... */ }
```

### Error: "Message type 'X' references channel 'Y' which is not declared"

**Problem**: A message references a channel that isn't listed in the `channels(...)` attribute.

**Solution**: Add the channel to the `channels(...)` list:

```rust
// ❌ Wrong - channel missing
#[derive(AsyncApi)]
#[asyncapi(
    channels("other.channel"),  // Missing "events"
    messages(MyMessage)  // MyMessage uses "events" channel
)]
pub struct MyApi;

// ✅ Correct
#[derive(AsyncApi)]
#[asyncapi(
    channels("events", "other.channel"),  // Includes "events"
    messages(MyMessage)
)]
pub struct MyApi;
```

### Error: "Failed to generate schema for message type"

**Problem**: The message type doesn't implement `JsonSchema`.

**Solution**: Add `JsonSchema` derive (usually via `schemars`):

```rust
// ❌ Wrong - missing JsonSchema
#[derive(Serialize, Deserialize, AsyncApiMessage)]
pub struct MyMessage { /* ... */ }

// ✅ Correct
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
pub struct MyMessage { /* ... */ }
```

### Error: "AsyncApiOperation requires at least one message"

**Problem**: An operation is defined without any messages.

**Solution**: Add at least one message to the `messages(...)` attribute:

```rust
// ❌ Wrong - no messages
#[derive(AsyncApiOperation)]
#[asyncapi(
    id = "my-op",
    action = "send",
    channel = "events"
    // Missing messages(...)
)]
pub struct MyOp;

// ✅ Correct
#[derive(AsyncApiOperation)]
#[asyncapi(
    id = "my-op",
    action = "send",
    channel = "events",
    messages(MyMessage)  // At least one message required
)]
pub struct MyOp;
```

## Runtime Errors

### Panic: "Message 'X' references channel 'Y' which is not declared"

**Problem**: Channel validation failed at runtime (should be caught at compile time, but can happen with macro ordering issues).

**Solution**: 
1. Ensure the channel is listed in `channels(...)`
2. Check that message types are defined before the `AsyncApi` struct
3. Verify the channel name matches exactly (case-sensitive)

### Panic: "Failed to generate schema for message type"

**Problem**: The type doesn't implement `JsonSchema` or has unsupported types.

**Solution**:
1. Ensure `#[derive(JsonSchema)]` is present
2. Check that all nested types also implement `JsonSchema`
3. For complex types, you may need to implement `JsonSchema` manually

## Common Issues

### Schema generation fails for generic types

**Problem**: Generic types can't be automatically converted to JSON Schema.

**Solution**: Use concrete types or implement `JsonSchema` manually:

```rust
// ❌ Problematic - generic type
pub struct GenericMessage<T> {
    pub data: T,
}

// ✅ Better - use concrete types or type aliases
pub struct StringMessage {
    pub data: String,
}
```

### Circular references in types

**Problem**: Types that reference each other can cause issues.

**Solution**: Use `Option` or references to break cycles:

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

### Large specifications are slow to generate

**Problem**: Many messages or complex schemas can slow down compilation.

**Solution**:
1. Consider splitting into multiple `AsyncApi` structs
2. Use simpler types where possible
3. Cache generated specs if generating frequently

### Macro ordering issues

**Problem**: Message and operation types must be defined before the `AsyncApi` struct that references them.

**Solution**: Define message types before the API struct:

```rust
// ✅ Good - messages defined first
#[derive(AsyncApiMessage)]
#[asyncapi(channel = "events")]
pub struct MyMessage { /* ... */ }

#[derive(AsyncApi)]
#[asyncapi(messages(MyMessage))]
pub struct MyApi;
```

## Getting Help

If you're still experiencing issues:

1. Check the [Limitations](limitations.md) guide for known limitations
2. Review the [Macro Expansion](macro-expansion.md) guide to understand how macros work
3. Check the error message carefully - it often includes hints and suggestions
4. Ensure all dependencies are up to date

## See Also

- [Limitations](limitations.md) - Known limitations and workarounds
- [Macro Expansion](macro-expansion.md) - How macros work internally
- [Validation Guide](../guides/validation.md) - Validation details

