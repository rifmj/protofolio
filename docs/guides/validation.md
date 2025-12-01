# Validation Guide

This guide explains how validation works in `protofolio` and how to handle validation errors.

## Overview

`protofolio` performs validation at two levels:
1. **Compile-time validation** - Checks performed during macro expansion
2. **Runtime validation** - Checks performed when generating the spec

## Compile-Time Validation

The macros validate the following at compile time:

- `CHANNEL` consts exist for all message/operation types (ensures they have the appropriate derive macro)
- Attribute syntax is correct
- Required fields are present (e.g., `info(title, version)`, `channel` for messages)

### Example Compile-Time Errors

If a message type is missing the `AsyncApiMessage` derive:

```rust
// ❌ Wrong - missing derive
pub struct MyMessage { /* ... */ }

// ✅ Correct
#[derive(AsyncApiMessage)]
#[asyncapi(channel = "events")]
pub struct MyMessage { /* ... */ }
```

## Runtime Validation

Runtime validation occurs when you call `asyncapi()` or `try_asyncapi()`. It checks:

- Channel references match declared channels
- Message references exist in their channels
- JSON Schema generation succeeds
- AsyncAPI spec validation passes

## Using Validation

### Panic-on-Error API

The `asyncapi()` method panics if validation fails:

```rust
let spec = MyApi::asyncapi();  // Panics on error
```

Use this when you're certain the spec is valid (e.g., in tests or during development).

### Result-Based API

The `try_asyncapi()` method returns a `Result`:

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

Use this in production code to handle errors gracefully.

### Additional Validation

You can also validate a spec after generation:

```rust
use protofolio::{AsyncApi, validate_spec};

let spec = ECommerceApi::asyncapi();
match validate_spec(&spec) {
    Ok(()) => println!("Spec is valid!"),
    Err(e) => eprintln!("Validation error: {}", e),
}
```

## Validation Flow

The recommended validation flow for production:

```rust
let spec = MyApi::try_asyncapi()?;  // Returns Result
protofolio::validate_spec(&spec)?;  // Additional validation
```

## Common Validation Errors

### Channel Not Declared

**Error**: "Message type 'X' references channel 'Y' which is not declared"

**Solution**: Add the channel to the `channels(...)` list:

```rust
#[derive(AsyncApi)]
#[asyncapi(
    channels("events", "other.channel"),  // Include all channels
    messages(MyMessage)
)]
pub struct MyApi;
```

### Message Not Found in Channel

**Error**: "Operation references message 'X' in channel 'Y' which does not exist"

**Solution**: Ensure the message is registered in the `messages(...)` list and uses the correct channel.

### Schema Generation Failed

**Error**: "Failed to generate schema for message type"

**Solution**: Ensure the message type implements `JsonSchema`:

```rust
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]  // Include JsonSchema
#[asyncapi(channel = "events")]
pub struct MyMessage { /* ... */ }
```

## See Also

- [Troubleshooting](../reference/troubleshooting.md) - Detailed troubleshooting guide
- [Best Practices](best-practices.md) - Validation best practices

