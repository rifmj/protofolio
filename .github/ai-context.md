# AI Context for protofolio

Rust crate for AsyncAPI 3.0 spec generation from code annotations. Workspace: `protofolio` (runtime) + `protofolio-derive` (macros). MSRV: 1.80+, Edition: 2021.

## Patterns
- Procedural macros for compile-time code generation
- Builder pattern for specs
- Schema caching: `LazyLock<Mutex<HashMap<TypeId, Arc<Value>>>>`
- Dual API: `asyncapi()` (panic) and `try_asyncapi()` (Result)
- Type-safe enums, centralized errors in `error.rs`

## Structure
```
protofolio/          # Runtime
  src/spec/          # AsyncAPI structures
  src/schema/        # JSON Schema (cached)
  src/validation/    # Validation
  src/protocol/      # NATS, Kafka, MQTT

protofolio-derive/   # Macros
  src/asyncapi/      # AsyncApi derive
  src/message/       # AsyncApiMessage derive
  src/operation/     # AsyncApiOperation derive
```

## Conventions
- Channels: `order.created` (dot-separated, lowercase)
- Message IDs: `order-created-v1` (include version)
- `pub(crate)` for internals
- `thiserror` for errors

## Docs
- User: `docs/` (guides, examples, reference)
- Architecture: `ARCHITECTURE.md`
- Protocols: `PROTOCOLS.md`
