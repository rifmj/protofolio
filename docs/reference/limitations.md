# Limitations

This document describes known limitations of `protofolio` and potential workarounds.

## Compile-Time vs Runtime Validation

**Compile-time validation** (what the macros check):
- `CHANNEL` consts exist for all message/operation types
- Attribute syntax is correct
- Required fields are present

**Runtime validation** (what happens when generating the spec):
- Channel references match declared channels
- Message references exist in their channels
- JSON Schema generation succeeds
- AsyncAPI spec validation passes

**Why this split?** Full compile-time validation of channel names against declared channels isn't possible in stable Rust due to limitations with string matching in const contexts. The macro validates that `CHANNEL` consts exist, but channel name validation happens at runtime. Use `try_asyncapi()` to handle runtime validation errors gracefully.

## Type System Limitations

### Generic Types

Generic message types require manual `JsonSchema` implementation or concrete type aliases.

```rust
// ❌ Problematic - generic type
pub struct GenericMessage<T> {
    pub data: T,
}

// ✅ Better - use concrete types or type aliases
type UserId = String;
pub struct UserMessage {
    pub user_id: UserId,
}
```

**Workaround**: Use type aliases for common cases:

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

### Cross-Crate Types

Types defined in other crates may not have their `CHANNEL` consts accessible during macro expansion.

**Solution**: Define message types in the same crate as your `AsyncApi` struct, or re-export types from other crates.

```rust
// In the crate where the type is defined:
#[derive(AsyncApiMessage)]
#[asyncapi(channel = "events")]
pub struct ExternalMessage { /* ... */ }

// In your crate, re-export or use the type directly
// The CHANNEL const will be accessible
```

### Macro Ordering

Message and operation types must be defined before the `AsyncApi` struct that references them, or the `CHANNEL` consts may not be accessible.

**Solution**: Always define message types before the API struct:

```rust
// ✅ Good
#[derive(AsyncApiMessage)]
#[asyncapi(channel = "events")]
pub struct MyMessage { /* ... */ }

#[derive(AsyncApi)]
#[asyncapi(messages(MyMessage))]
pub struct MyApi;
```

## AsyncAPI 3.0 Feature Coverage

Not all AsyncAPI 3.0 features are supported yet:

- ❌ Components and `$ref` references (planned)
- ❌ Security schemes (planned)
- ❌ External documentation (planned)
- ❌ Server variables (planned)
- ❌ Message examples (planned)
- ❌ Message headers (planned)
- ❌ Correlation IDs (planned)

See the [Status](../README.md#status) section in the README for current feature support.

## Schema Customization

Limited control over JSON Schema generation - relies on `schemars` defaults. For advanced schema customization, you may need to implement `JsonSchema` manually for specific types.

**Workaround**: Implement `JsonSchema` manually for types that need custom schemas:

```rust
use schemars::{JsonSchema, gen::SchemaGenerator, schema::Schema};

impl JsonSchema for MyCustomType {
    fn schema_name() -> String {
        "MyCustomType".to_string()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        // Custom schema generation
        // ...
    }
}
```

## Performance Considerations

### Large Specifications

For large specifications with many messages:

- Consider splitting into multiple `AsyncApi` structs
- Use simpler types where possible
- Cache generated specs if generating frequently

### Schema Generation

Schema generation is automatically cached by type, but the first generation can be slow for complex types. Subsequent calls are fast due to caching.

## Future Improvements

Potential future enhancements:

- Full compile-time channel validation using const generics or other Rust features
- Support for AsyncAPI 3.0 components and references
- Additional protocol support (Kafka, MQTT, AMQP)
- Enhanced schema customization options
- Better error messages with suggestions
- IDE support and autocompletion
- CLI tool for validation and generation

## See Also

- [Troubleshooting](troubleshooting.md) - Solutions to common issues
- [Macro Expansion](macro-expansion.md) - How macros work internally

