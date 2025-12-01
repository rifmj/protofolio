# Macro Expansion Process

Understanding how the macros work can help with debugging and advanced usage.

## Overview

`protofolio` uses procedural macros to generate AsyncAPI specifications at compile time. The macros perform validation and generate code that builds the specification at runtime.

## AsyncApiMessage Derive Macro

When you use `#[derive(AsyncApiMessage)]`, the macro:

1. **Parses attributes**: Extracts `channel`, `messageId`, `name`, `title`, `summary`, `description`, `contentType`, and `tags` from the `#[asyncapi(...)]` attribute
2. **Generates methods**: Creates static methods like `channel()`, `message_id()`, `name()`, etc.
3. **Creates constants**: Generates a `CHANNEL` const for compile-time validation
4. **No runtime code**: All metadata is stored as static values

### Example Expansion

**Your code**:

```rust
#[derive(AsyncApiMessage)]
#[asyncapi(channel = "events", messageId = "event-v1")]
pub struct Event {
    pub id: String,
}
```

**Generated code** (conceptual):

```rust
impl Event {
    pub const CHANNEL: &'static str = "events";

    pub fn channel() -> &'static str { "events" }
    pub fn message_id() -> Option<&'static str> { Some("event-v1") }
    pub fn name() -> Option<&'static str> { None }
    // ... other methods
}
```

## AsyncApiOperation Derive Macro

The `AsyncApiOperation` macro:

1. **Parses attributes**: Extracts `id`, `action`, `channel`, `messages`, `summary`, `description`, and `tags`
2. **Validates action**: Ensures `action` is either "send" or "receive"
3. **Generates trait impl**: Implements `AsyncApiOperation` trait
4. **Creates constants**: Generates `CHANNEL` and `MESSAGE_TYPES` consts

### Example Expansion

**Your code**:

```rust
#[derive(AsyncApiOperation)]
#[asyncapi(
    id = "publish-event",
    action = "send",
    channel = "events",
    messages(Event)
)]
pub struct PublishEvent;
```

**Generated code** (conceptual):

```rust
impl PublishEvent {
    pub const CHANNEL: &'static str = "events";
    pub const MESSAGE_TYPES: &[&str] = &["Event"];
}

impl AsyncApiOperation for PublishEvent {
    fn operation_id() -> &'static str { "publish-event" }
    fn channel() -> &'static str { "events" }
    fn action() -> OperationAction { OperationAction::Send }
    // ... other methods
}
```

## AsyncApi Derive Macro

The main `AsyncApi` macro performs the most complex expansion:

1. **Parses specification**: Extracts `info`, `servers`, `channels`, `messages`, and `operations`
2. **Compile-time validation**:
   - Checks that `CHANNEL` consts exist for all message/operation types
   - Validates required fields are present
3. **Generates implementation**: Creates the `asyncapi()` method that:
   - Builds the AsyncAPI spec structure
   - Generates JSON Schema for each message type
   - Validates channel references
   - Assembles the complete specification

### Expansion Flow

```
#[derive(AsyncApi)]
  ↓
Parse #[asyncapi(...)] attributes
  ↓
Extract channels, messages, operations
  ↓
Validate CHANNEL consts exist (compile-time)
  ↓
Generate asyncapi() method implementation
  ↓
Runtime: Build spec, generate schemas, validate
```

### Example Expansion

**Your code**:

```rust
#[derive(AsyncApi)]
#[asyncapi(
    info(title = "My API", version = "1.0.0"),
    channels("events"),
    messages(Event)
)]
pub struct MyApi;
```

**Generated code** (simplified):

```rust
impl AsyncApi for MyApi {
    fn asyncapi() -> AsyncApiSpec {
        // Build info
        let info = Info {
            title: "My API".to_string(),
            version: "1.0.0".to_string(),
            description: None,
        };

        // Build channels
        let mut channels = HashMap::new();
        channels.insert("events".to_string(), Channel { /* ... */ });

        // Add messages to channels
        // Generate schema for Event
        let schema = schema_for_type::<Event>()?;
        // Add message to channel
        // ...

        // Build spec
        AsyncApiSpec {
            asyncapi: "3.0.0".to_string(),
            info,
            channels,
            // ...
        }
    }
}
```

## Compile-Time vs Runtime Validation

### Compile-Time Validation

The macros check at compile time:

- `CHANNEL` consts exist for all message/operation types
- Attribute syntax is correct
- Required fields are present

**Why compile-time?** These checks can be performed during macro expansion and provide immediate feedback.

### Runtime Validation

Runtime validation occurs when generating the spec:

- Channel references match declared channels
- Message references exist in their channels
- JSON Schema generation succeeds
- AsyncAPI spec validation passes

**Why runtime?** Full compile-time validation of channel names against declared channels isn't possible in stable Rust due to limitations with string matching in const contexts.

## Error Messages

The macros provide detailed error messages with:

- Available channels/messages
- Hints on how to fix the issue
- Exact location of the problem

Example error:

```
error: Message type 'MyMessage' references channel 'events' which is not declared.
Available channels: ["other.channel"].
Add 'events' to channels(...) in your #[asyncapi] attribute on MyApi

Hint: Update your #[derive(AsyncApi)] on MyApi to include: channels("events", ...)
```

## See Also

- [Troubleshooting](troubleshooting.md) - Common issues and solutions
- [Limitations](limitations.md) - Known limitations
- [Validation Guide](../guides/validation.md) - Validation details
