# protofolio - Context for AI Assistants

## Project Overview

`protofolio` is a Rust crate for generating AsyncAPI 3.0 specifications from Rust code annotations. It uses procedural macros to generate specs at compile time, similar to how `utoipa` generates OpenAPI specs for REST APIs.

**Key Concept**: Code-first approach - your Rust types become AsyncAPI documentation automatically, ensuring documentation stays in sync with code.

## Project Structure

This is a Rust workspace containing two crates:

### `protofolio/` - Main Runtime Library

- **`src/lib.rs`** - Public API surface with carefully curated exports
- **`src/error.rs`** - Centralized error types (`SchemaError`, `ValidationError`)
- **`src/types.rs`** - Core types (`OperationAction` enum, constants)
- **`src/traits.rs`** - Public traits (`AsyncApi`, `AsyncApiOperation`)
- **`src/spec/`** - AsyncAPI specification data structures
  - `info.rs` - Info and Server types
  - `channel.rs` - Channel, Message, Tag, Parameter types
  - `operation.rs` - Operation, ChannelReference, MessageReference
  - `components.rs` - Components for reusable definitions
- **`src/builder/`** - Builder pattern for constructing specs programmatically
- **`src/schema/`** - JSON Schema generation with automatic caching
  - Uses `LazyLock<Mutex<HashMap<TypeId, Arc<Value>>>>` for thread-safe caching
- **`src/validation/`** - Specification validation logic
- **`src/protocol/`** - Protocol support (NATS, Kafka, MQTT)
  - Each protocol has its own module with constants and bindings
- **`src/internal/`** - Internal utilities (`pub(crate)` only)

### `protofolio-derive/` - Procedural Macros

- **`src/lib.rs`** - Macro entry points
- **`src/asyncapi/`** - Main `AsyncApi` derive macro implementation
  - `mod.rs` - Main entry point
  - `attrs.rs` - Attribute parsing structures
  - `codegen.rs` - Code generation logic
  - `messages.rs` - Message code generation
  - `operations.rs` - Operation code generation
- **`src/message/`** - `AsyncApiMessage` derive macro
  - `attrs.rs` - Message attribute parsing
  - `codegen.rs` - Message code generation
- **`src/operation/`** - `AsyncApiOperation` derive macro
  - `attrs.rs` - Operation attribute parsing
  - `codegen.rs` - Operation code generation
- **`src/parse_utils.rs`** - Shared parsing utilities

## Key Conventions and Patterns

### Error Handling

- **Production**: Use `try_asyncapi()` - returns `Result<AsyncApiSpec, ValidationError>`
- **Development/Testing**: Use `asyncapi()` - panics on error for convenience
- All errors are centralized in `error.rs` using `thiserror`

### Channel Naming

- Use dot-separated hierarchical names: `order.created`, `order.status.changed`
- Use lowercase with dots or hyphens: `user.events`, `payment-processed`
- **DO NOT** include version in channel names - use `messageId` for versions instead

### Message Versioning

- Include version in `messageId`: `order-created-v1`
- Channel names should NOT include versions: `order.created` (not `order.created.v1`)
- When breaking changes occur, create new message type with new version

### Code Organization

- Use `pub(crate)` for internal APIs
- Centralize error types in `error.rs`
- Use type-safe enums instead of strings where appropriate
- Follow Rust API guidelines
- Each module has a single, well-defined responsibility

## Coding Standards

- **Rust Version**: 1.80+ (MSRV)
- **Edition**: 2021
- **Unsafe Code**: Denied across workspace
- **Documentation**: Required for public APIs (warn on missing)
- **Linting**: Pedantic clippy lints enabled
- **Error Handling**: Use `thiserror` for error types
- **Type Safety**: Prefer enums over strings, use constants for magic values

## Common Commands

```bash
# Build
cargo build

# Test
cargo test

# Lint
cargo clippy

# Documentation
cargo doc --open

# Run examples
cargo run --example basic
cargo run --example full_featured
cargo run --example axum_integration
```

## Important Files and Documentation

- **`README.md`** - Main user documentation with quick start
- **`ARCHITECTURE.md`** - Detailed architecture and design decisions
- **`PROTOCOLS.md`** - Protocol-specific documentation (NATS, Kafka, MQTT)
- **`CONTRIBUTING.md`** - Contribution guidelines
- **`docs/`** - Comprehensive user documentation
  - `guides/` - Getting started, messages, operations, validation, best practices
  - `examples/` - Basic, advanced, and integration examples
  - `reference/` - Troubleshooting, limitations, migration, macro expansion

## Macro Expansion Process

1. **AsyncApiMessage**: Parses attributes, generates static methods and `CHANNEL` const
2. **AsyncApiOperation**: Parses attributes, validates action, implements trait
3. **AsyncApi**: Parses spec, validates at compile-time, generates `asyncapi()` method

### Compile-Time vs Runtime Validation

- **Compile-time**: Checks `CHANNEL` consts exist, validates attribute syntax, ensures required fields
- **Runtime**: Validates channel references, message references, generates JSON Schema, validates spec

## Schema Generation

- Automatic caching using `LazyLock<Mutex<HashMap<TypeId, Arc<Value>>>>`
- Thread-safe by `TypeId`
- Zero-copy access with `Arc`
- First generation can be slow, subsequent calls are fast

## Protocol Support

Currently supports:

- **NATS** - `protofolio::protocol::NatsProtocol`
- **Kafka** - `protofolio::protocol::KafkaProtocol`
- **MQTT** - `protofolio::protocol::MqttProtocol`

Each protocol has constants and bindings. Easy to extend for new protocols.

## Testing

- Unit tests co-located with modules
- Integration tests in `tests/` directory
- Trybuild tests for macro error cases in `protofolio-derive/tests/macro_error_tests/`
- Error handling tests verify both panic and Result-based APIs

## Current Status

✅ Implemented:

- Basic AsyncAPI 3.0 spec generation
- Multi-protocol support (NATS, Kafka, MQTT)
- Message and channel mapping
- JSON Schema generation with caching
- JSON and YAML output
- Enhanced message attributes
- Channel parameters and bindings
- Specification validation
- Error handling (both APIs)
- Operations support
- Protocol-specific bindings

🚧 Planned:

- Components and `$ref` references
- Security schemes
- External documentation
- Server variables
- Message examples and headers
- Correlation IDs
- Full AsyncAPI 3.0 feature set

## When Making Changes

1. **Documentation**: Update relevant docs in `docs/` directory
2. **Error Messages**: Provide helpful hints and suggestions
3. **Backwards Compatibility**: Consider impact on existing users
4. **Testing**: Add tests for new features, update trybuild tests for macro changes
5. **Architecture**: Follow separation of concerns, keep modules focused

## Common Patterns

### Adding a New Protocol

1. Add protocol module in `src/protocol/`
2. Define constants and bindings
3. Export in `src/protocol/mod.rs`
4. Add example in `examples/`
5. Document in `PROTOCOLS.md`

### Adding a New Validation Rule

1. Add to `src/validation/validator.rs`
2. Add corresponding error variant in `src/error.rs`
3. Update tests
4. Document in validation guide

### Modifying Macros

1. Update relevant derive macro in `protofolio-derive/src/`
2. Update trybuild tests if error messages change
3. Update macro expansion documentation
4. Test with various attribute combinations
