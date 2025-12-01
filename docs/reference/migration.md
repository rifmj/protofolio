# Migration Guide

This guide helps you migrate from other approaches to `protofolio`.

## Migrating from Handwritten AsyncAPI Specs

If you're currently maintaining AsyncAPI specifications manually in JSON or YAML:

### Step 1: Identify Your Message Types

Map your existing message schemas to Rust structs:

```rust
// Your existing JSON schema:
// {
//   "type": "object",
//   "properties": {
//     "id": {"type": "string"},
//     "data": {"type": "string"}
//   }
// }

// Convert to Rust:
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(channel = "events", messageId = "event-v1")]
pub struct Event {
    pub id: String,
    pub data: String,
}
```

### Step 2: Add Derives

Add `Serialize`, `Deserialize`, `JsonSchema`, and `AsyncApiMessage` to each message type:

```rust
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(channel = "events")]
pub struct MyMessage {
    // ... fields
}
```

### Step 3: Define Channels

Create an `AsyncApi` struct with your channels and messages:

```rust
#[derive(AsyncApi)]
#[asyncapi(
    info(title = "My API", version = "1.0.0"),
    channels("events", "notifications"),
    messages(Event, Notification)
)]
pub struct MyApi;
```

### Step 4: Generate and Compare

Use `asyncapi_json()` to generate your spec and compare with your existing one:

```rust
let generated = MyApi::asyncapi_json()?;
// Compare with your existing spec
```

### Benefits of Migration

- **Type safety**: Your documentation matches your code automatically
- **Single source of truth**: No manual sync between code and docs
- **Compile-time validation**: Catch errors before runtime
- **Version control**: Changes to messages are tracked in git diffs

## Comparison with Alternatives

### vs. Manual Spec Writing

**Advantages**:

- ✅ Type-safe and validated at compile time
- ✅ Automatic schema generation from Rust types
- ✅ No manual synchronization needed

### vs. OpenAPI/utoipa

**Similarities**:

- Both use code-first, macro-based generation
- Both provide type-safe documentation generation

**Differences**:

- `protofolio` is for async messaging (AsyncAPI) instead of REST APIs (OpenAPI)
- Different specification format and use cases
- Different protocol support (NATS, Kafka, MQTT vs HTTP)

### vs. Other AsyncAPI Tools

**Advantages**:

- ✅ Native Rust integration
- ✅ Compile-time validation
- ✅ Type safety
- ✅ No external tooling required

## Workarounds for Common Scenarios

### For Generic Types

Use type aliases for common cases:

```rust
type UserId = String;
type Timestamp = i64;

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(channel = "events")]
pub struct Event {
    pub user_id: UserId,  // Instead of generic
    pub timestamp: Timestamp,
}
```

### For Cross-Crate Types

In the crate where the type is defined:

```rust
#[derive(AsyncApiMessage)]
#[asyncapi(channel = "events")]
pub struct ExternalMessage { /* ... */ }
```

In your crate, re-export or use the type directly. The `CHANNEL` const will be accessible.

### For Complex Validation

Use runtime validation as a fallback:

```rust
let spec = MyApi::asyncapi();
if let Err(e) = validate_spec(&spec) {
    eprintln!("Validation failed: {}", e);
    // Handle error appropriately
}
```

## Migration Checklist

- [ ] Map existing message schemas to Rust structs
- [ ] Add required derives (`Serialize`, `Deserialize`, `JsonSchema`, `AsyncApiMessage`)
- [ ] Define channels in `AsyncApi` struct
- [ ] Register all messages
- [ ] Generate spec and compare with existing
- [ ] Update any tooling that consumes the spec
- [ ] Test the generated spec with your AsyncAPI tooling
- [ ] Remove old manual spec files (after verification)

## See Also

- [Getting Started](../guides/getting-started.md) - Installation and basic usage
- [Considerations](limitations.md) - Design decisions and recommended approaches
- [Troubleshooting](troubleshooting.md) - Common issues and solutions
