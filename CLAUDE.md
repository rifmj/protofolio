# protofolio - AI Context

Rust crate for generating AsyncAPI 3.0 specifications from code annotations using procedural macros. Code-first approach: Rust types become AsyncAPI docs automatically.

## Structure

**Workspace**: Two crates

- `protofolio/` - Runtime library (spec structures, schema generation, validation, protocols)
- `protofolio-derive/` - Procedural macros (`AsyncApi`, `AsyncApiMessage`, `AsyncApiOperation`)

**Key modules**:

- `src/spec/` - AsyncAPI data structures
- `src/schema/` - JSON Schema generation (cached via `LazyLock<Mutex<HashMap<TypeId, Arc<Value>>>>`)
- `src/validation/` - Spec validation
- `src/protocol/` - NATS, Kafka, MQTT support
- `protofolio-derive/src/asyncapi/` - Main derive macro

## Conventions

**Error handling**: `try_asyncapi()` (Result) for production, `asyncapi()` (panic) for dev. Errors in `error.rs` via `thiserror`.

**Naming**: Channels use dot-separated hierarchy (`order.created`). Message IDs include version (`order-created-v1`). Channel names never include versions.

**Code style**: Rust 1.80+ (MSRV), Edition 2021. Use `pub(crate)` for internals. Type-safe enums over strings. Centralize errors. Follow Rust API guidelines.

## Macros

1. `AsyncApiMessage` - Parses attributes, generates `CHANNEL` const and static methods
2. `AsyncApiOperation` - Parses attributes, validates action, implements trait
3. `AsyncApi` - Parses spec, validates at compile-time, generates `asyncapi()` method

**Validation**: Compile-time checks `CHANNEL` consts exist. Runtime validates references, generates schemas.

## Commands

```bash
cargo build              # Build
cargo test              # Test
cargo clippy            # Lint
cargo doc --open        # Docs
cargo run --example basic
```

## Documentation

- `README.md` - Quick start
- `ARCHITECTURE.md` - Design decisions
- `PROTOCOLS.md` - Protocol docs
- `docs/` - User guides, examples, reference

## Status

✅ Implemented: AsyncAPI 3.0 generation, multi-protocol (NATS/Kafka/MQTT), schema caching, validation, operations

🚧 Planned: Components/`$ref`, security schemes, external docs, server variables, message examples/headers

## Patterns

**New protocol**: Add module in `src/protocol/`, define constants/bindings, export, add example, document.

**New validation**: Add to `src/validation/validator.rs`, add error variant in `error.rs`, test, document.

**Modify macros**: Update derive in `protofolio-derive/src/`, update trybuild tests if errors change, test attributes.
