# Validation Guide ✅

This guide explains how validation works in `protofolio` and how to handle validation results. Safety first! 🛡️

## Overview 📋

`protofolio` performs validation at two levels - double protection! 🎯
1. 🔍 **Compile-time validation** - Checks performed during macro expansion
2. ⚡ **Runtime validation** - Checks performed when generating the spec

## Compile-Time Validation 🔍

The macros validate the following at compile time - catch issues early! 🎯

- `CHANNEL` consts exist for all message/operation types (ensures they have the appropriate derive macro)
- Attribute syntax is correct
- Required fields are present (e.g., `info(title, version)`, `channel` for messages)

### Example Compile-Time Validation

When a message type needs the `AsyncApiMessage` derive:

```rust
// Missing derive
pub struct MyMessage { /* ... */ }

// With derive macro
#[derive(AsyncApiMessage)]
#[asyncapi(channel = "events")]
pub struct MyMessage { /* ... */ }
```

## Runtime Validation ⚡

Runtime validation occurs when you call `asyncapi()` or `try_asyncapi()`. It checks:

- ✅ Channel references match declared channels
- ✅ Message references exist in their channels
- ✅ JSON Schema generation succeeds
- ✅ AsyncAPI spec validation passes

## Using Validation 🎯

### Direct API 🚀

The `asyncapi()` method generates the spec directly - simple and fast! ⚡

```rust
let spec = MyApi::asyncapi();  // Generates the spec
```

Use this when you're certain the spec is valid (e.g., in tests or during development).

### Result-Based API 🎭

The `try_asyncapi()` method returns a `Result` for graceful handling - production-ready! 🚀

```rust
match MyApi::try_asyncapi() {
    Ok(spec) => {
        // Use the spec
        let json = protofolio::to_json(&spec)?;
    }
    Err(e) => {
        // Handle validation results
        eprintln!("Validation result: {}", e);
    }
}
```

Use this in production code to handle validation results gracefully.

### Additional Validation

You can also validate a spec after generation:

```rust
use protofolio::{AsyncApi, validate_spec};

let spec = ECommerceApi::asyncapi();
match validate_spec(&spec) {
    Ok(()) => println!("Spec is valid!"),
    Err(e) => eprintln!("Validation result: {}", e),
}
```

## Validation Flow 🔄

The recommended validation flow for production - follow this pattern! ✨

```rust
let spec = MyApi::try_asyncapi()?;  // Returns Result
protofolio::validate_spec(&spec)?;  // Additional validation
```

## Common Validation Scenarios 💡

### Channel Not Declared 🚫

**Message**: "Message type 'X' references channel 'Y' which is not declared"

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

**Message**: "Operation references message 'X' in channel 'Y' which does not exist"

**Solution**: Ensure the message is registered in the `messages(...)` list and uses the correct channel.

### Schema Generation

**Message**: "Failed to generate schema for message type"

**Solution**: Ensure the message type implements `JsonSchema`:

```rust
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]  // Include JsonSchema
#[asyncapi(channel = "events")]
pub struct MyMessage { /* ... */ }
```

## See Also

- [Troubleshooting](../reference/troubleshooting.md) - Detailed troubleshooting guide
- [Best Practices](best-practices.md) - Validation best practices

