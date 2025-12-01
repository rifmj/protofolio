# AI Context for protofolio

## Quick Reference

**Project**: Rust crate for AsyncAPI 3.0 spec generation from code annotations  
**Workspace**: Two crates - `protofolio` (runtime) and `protofolio-derive` (procedural macros)  
**MSRV**: Rust 1.80+  
**Edition**: 2021

## Key Patterns

- **Procedural macros** for code generation at compile time
- **Builder pattern** for spec construction
- **Schema caching** with `LazyLock<Mutex<HashMap<TypeId, Arc<Value>>>>`
- **Dual API**: `asyncapi()` (panic) and `try_asyncapi()` (Result)
- **Type-safe enums** instead of strings
- **Centralized errors** in `error.rs`

## Structure

```
protofolio/          # Runtime library
  src/spec/          # AsyncAPI data structures
  src/schema/        # JSON Schema generation
  src/validation/    # Spec validation
  src/protocol/      # NATS, Kafka, MQTT support

protofolio-derive/   # Procedural macros
  src/asyncapi/      # Main AsyncApi derive
  src/message/       # AsyncApiMessage derive
  src/operation/     # AsyncApiOperation derive
```

## Conventions

- Channels: `order.created`, `order.status.changed` (dot-separated, lowercase)
- Message IDs: `order-created-v1` (include version)
- Use `pub(crate)` for internal APIs
- Error handling: `thiserror` for error types

## Documentation

- User docs: `docs/` (guides, examples, reference)
- Architecture: `ARCHITECTURE.md`
- Protocols: `PROTOCOLS.md`
- Contributing: `CONTRIBUTING.md`

