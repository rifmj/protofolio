# Considerations 💭

This document describes design decisions and considerations when using `protofolio`, along with recommended approaches. Let's explore! 🔍

## Compile-Time vs Runtime Validation 🔍

**Compile-time validation** (what the macros check) - catch issues early! ⚡

- `CHANNEL` consts exist for all message/operation types
- Attribute syntax is correct
- Required fields are present

**Runtime validation** (what happens when generating the spec) - final safety check! ✅

- Channel references match declared channels
- Message references exist in their channels
- JSON Schema generation succeeds
- AsyncAPI spec validation passes

**Why this split?** Channel name validation occurs at runtime to provide flexibility and support dynamic channel configurations. The macro validates that `CHANNEL` consts exist at compile time, while full channel name matching happens at runtime. Use `try_asyncapi()` to handle validation results gracefully.

## Type System Considerations 🎨

### Generic Types 🔤

For generic message types, use concrete type aliases or implement `JsonSchema` manually - flexibility with structure! 💪

```rust
// Alternative approach - generic type requires manual JsonSchema
pub struct GenericMessage<T> {
    pub data: T,
}

// Recommended - use concrete types or type aliases
type UserId = String;
pub struct UserMessage {
    pub user_id: UserId,
}
```

**Recommended approach**: Use type aliases for common cases:

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

### Cross-Crate Types 📦

Types defined in other crates may not have their `CHANNEL` consts accessible during macro expansion.

**Solution** 💡: Define message types in the same crate as your `AsyncApi` struct, or re-export types from other crates.

```rust
// In the crate where the type is defined:
#[derive(AsyncApiMessage)]
#[asyncapi(channel = "events")]
pub struct ExternalMessage { /* ... */ }

// In your crate, re-export or use the type directly
// The CHANNEL const will be accessible
```

### Macro Ordering 📝

Message and operation types must be defined before the `AsyncApi` struct that references them, or the `CHANNEL` consts may not be accessible.

**Solution** 💡: Always define message types before the API struct - order matters! ⚡

```rust
// ✅ Good
#[derive(AsyncApiMessage)]
#[asyncapi(channel = "events")]
pub struct MyMessage { /* ... */ }

#[derive(AsyncApi)]
#[asyncapi(messages(MyMessage))]
pub struct MyApi;
```

## AsyncAPI 3.0 Feature Coverage 🎯

Here's what's currently supported - and we're always adding more! 🚀

- ✅ Components and `$ref` references (messages, schemas, parameters, bindings, traits)
- ✅ Security schemes
- ✅ External documentation
- ✅ Server variables
- ✅ Message examples
- ✅ Message headers
- ✅ Correlation IDs
- ✅ Operation traits
- ✅ Message traits
- ✅ Component bindings (channel, message, server)
- ✅ Channel address field
- ✅ Operation ID field
- ✅ Root-level tags

See the [Status](https://github.com/rifmj/protofolio#status) section in the README for current feature support.

## Schema Customization 🎨

JSON Schema generation uses `schemars` defaults, which work well for most cases. For advanced schema customization, you can implement `JsonSchema` manually for specific types - full control when you need it! 💪

**Custom implementation** 🛠️: Implement `JsonSchema` manually for types that need custom schemas:

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

## Performance Considerations ⚡

### Large Specifications 📊

For large specifications with many messages - scale smartly! 🚀

- Consider splitting into multiple `AsyncApi` structs
- Use simpler types where possible
- Cache generated specs if generating frequently

### Schema Generation 🔄

Schema generation is automatically cached by type, but the first generation can be slow for complex types. Subsequent calls are fast due to caching - smart caching! 🧠

## Future Improvements 🔮

Potential future enhancements - exciting stuff coming! 🚀

- Full compile-time channel validation using const generics or other Rust features
- Additional protocol support (AMQP, WebSocket, etc.)
- Enhanced schema customization options
- Better error messages with suggestions
- IDE support and autocompletion
- CLI tool for validation and generation
- Macro support for defining components (currently only via builder API)
- Component traits and bindings can be referenced but must be defined using the builder API

## See Also

- [Troubleshooting](troubleshooting.md) - Solutions to common scenarios
- [Macro Expansion](macro-expansion.md) - How macros work internally
