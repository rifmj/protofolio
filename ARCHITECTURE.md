# protofolio Architecture

This document describes the architectural structure of the `protofolio` crate following Rust best practices.

## Module Structure

```
protofolio/
├── src/
│   ├── lib.rs              # Public API surface - carefully curated exports
│   ├── error.rs            # Centralized error types (SchemaError, ValidationError)
│   ├── types.rs            # Core types (OperationAction enum, constants)
│   ├── traits.rs           # Public traits (AsyncApi, AsyncApiOperation)
│   ├── spec/               # Specification data structures
│   │   ├── mod.rs          # Main spec types and re-exports
│   │   ├── info.rs         # Info and Server types
│   │   ├── channel.rs      # Channel, Message, Tag, Parameter types
│   │   ├── operation.rs    # Operation, ChannelReference, MessageReference
│   │   └── components.rs   # Components for reusable definitions
│   ├── builder/            # Builder pattern implementation
│   │   ├── mod.rs
│   │   └── builder.rs      # AsyncApiBuilder implementation
│   ├── schema/             # JSON Schema generation
│   │   ├── mod.rs
│   │   └── generator.rs   # Schema generation functions
│   ├── validation/         # Specification validation
│   │   ├── mod.rs
│   │   └── validator.rs    # Validation logic and rules
│   ├── protocol/           # Protocol support
│   │   ├── mod.rs
│   │   ├── traits.rs       # Protocol trait
│   │   └── nats.rs         # NATS protocol implementation
│   └── internal/           # Internal utilities (pub(crate) only)
│       └── mod.rs
└── protofolio-derive/   # Procedural macros (unchanged structure)
```

## Architectural Principles

### 1. **Separation of Concerns**

- Each module has a single, well-defined responsibility
- Related types are grouped together (e.g., all channel-related types in `spec/channel.rs`)
- Implementation details are separated from public API

### 2. **Clean Public API**

- `lib.rs` carefully curates what is exported publicly
- Internal implementation details are hidden using `pub(crate)`
- Only essential types and functions are exposed

### 3. **Centralized Error Handling**

- All error types are in `error.rs`
- Consistent error handling across the crate
- Clear error messages with context

### 4. **Type Safety**

- Use of enums instead of strings where appropriate (e.g., `OperationAction`)
- Constants for magic values (e.g., `ASYNCAPI_VERSION`)
- Strong typing throughout

### 5. **Modular Organization**

- Related functionality grouped into modules
- Each module can be understood independently
- Easy to extend (e.g., adding new protocols)

### 6. **Internal Utilities**

- `internal/` module for crate-internal utilities
- Prevents accidental exposure of implementation details
- Allows refactoring without breaking public API

## Key Improvements

### Before

- Flat module structure with all types in single files
- Error types scattered across modules
- String-based types where enums would be better
- No clear separation between public and internal API

### After

- Hierarchical module structure with clear organization
- Centralized error handling
- Type-safe enums and constants
- Clear public API boundary
- Easy to extend and maintain

## Module Responsibilities

### `error.rs`

- Defines all error types used throughout the crate
- Provides consistent error handling

### `types.rs`

- Core type definitions (enums, constants)
- Shared type utilities

### `spec/`

- All AsyncAPI specification data structures
- Organized by domain (info, channel, operation, components)

### `builder/`

- Builder pattern for constructing specs programmatically
- Fluent API for building AsyncAPI specifications

### `schema/`

- JSON Schema generation from Rust types
- Integration with `schemars` crate
- Automatic caching using `LazyLock` and `Arc` for performance
- Thread-safe schema cache by `TypeId`

### `validation/`

- Specification validation logic
- Ensures specs conform to AsyncAPI 3.0

### `protocol/`

- Protocol-specific implementations
- Currently supports NATS, Kafka, and MQTT
- Each protocol has its own module with constants and bindings
- Extensible for adding new protocols

### `internal/`

- Internal utilities not part of public API
- Allows refactoring without breaking changes

## Public API Design

The public API is intentionally minimal and focused:

```rust
// Main types
pub use spec::*;                    // All spec types
pub use traits::{AsyncApi, AsyncApiOperation};
pub use builder::AsyncApiBuilder;
pub use error::{SchemaError, ValidationError};
pub use types::OperationAction;

// Functions
pub use schema::{generate_schema, schema_for_type};
pub use validation::validate_spec;
pub fn to_yaml(spec: &AsyncApiSpec) -> Result<String, serde_yaml::Error>;
pub fn to_json(spec: &AsyncApiSpec) -> Result<String, serde_json::Error>;
```

This design:

- Makes it clear what users should use
- Prevents accidental use of internal APIs
- Allows internal refactoring without breaking changes
- Follows Rust's principle of "make invalid states unrepresentable"

## Extension Points

The architecture makes it easy to extend:

1. **New Protocols**: Add to `protocol/` module
2. **New Spec Types**: Add to appropriate `spec/` submodule
3. **New Validation Rules**: Add to `validation/validator.rs`
4. **New Builder Methods**: Add to `builder/builder.rs`

## Testing

- Unit tests are co-located with their modules
- Integration tests in `tests/` directory
- All tests compile and pass after restructuring

## Data Flow

The following diagram shows how an AsyncAPI specification is generated:

```
User Code
  ↓
#[derive(AsyncApi)] / #[derive(AsyncApiMessage)]
  ↓
Procedural Macros (protofolio-derive)
  ↓
Generated Code (impl AsyncApi for MyApi)
  ↓
Runtime Execution:
  ├─→ Schema Generation (schema/generator.rs)
  │   ├─→ Check cache (LazyLock<Mutex<HashMap<TypeId, Arc<Value>>>>)
  │   ├─→ Generate schema if not cached (schemars)
  │   └─→ Store in cache (Arc-wrapped for zero-copy access)
  │
  ├─→ Builder Pattern (builder/builder.rs)
  │   └─→ Construct AsyncApiSpec structure
  │
  ├─→ Validation (validation/validator.rs)
  │   ├─→ Version check
  │   ├─→ Required fields
  │   ├─→ Channel/message references
  │   └─→ Protocol validation
  │
  └─→ Serialization (to_json/to_yaml)
      └─→ Final AsyncAPI spec (JSON/YAML)
```

### Error Handling Flow

Two paths are available:

1. **`asyncapi()`** - Panic-on-error (convenience):
   ```
   User calls asyncapi()
   → Panics if validation fails
   → Returns AsyncApiSpec on success
   ```

2. **`try_asyncapi()`** - Result-based (production):
   ```
   User calls try_asyncapi()
   → Returns Result<AsyncApiSpec, ValidationError>
   → User handles errors gracefully
   ```

## Design Decisions

### Why Procedural Macros?

Procedural macros allow:
- **Compile-time validation**: Catch errors before runtime
- **Type safety**: Leverage Rust's type system
- **Zero runtime overhead**: All metadata is static
- **IDE integration**: Better autocomplete and error messages
- **Code generation**: Automatic implementation of traits

Alternative approaches (build scripts, runtime reflection) were considered but rejected for these benefits.

### Why Arc + Global Cache for Schemas?

**Problem**: Schema generation is expensive, and the same types are often used multiple times.

**Solution**: Global cache using `LazyLock<Mutex<HashMap<TypeId, Arc<serde_json::Value>>>>`

**Why Arc?**:
- Avoids cloning on cache hits (only clones when returning)
- Thread-safe reference counting
- Minimal memory overhead

**Why LazyLock?**:
- Standard library (Rust 1.80+)
- Thread-safe initialization
- No external dependencies

**Why Mutex?**:
- Thread-safe access to the cache
- Simple and sufficient for this use case
- Low contention expected (mostly reads)

### Error Handling: Panic vs Result

**Dual API Design**:

- **`asyncapi()`** - Panics on error:
  - Convenient for development and testing
  - Assumes spec is valid
  - Fast path (no error handling overhead)

- **`try_asyncapi()`** - Returns Result:
  - Required for production code
  - Allows graceful error handling
  - Validates spec before returning

**Rationale**: Provides both convenience and safety, following Rust's philosophy of "make invalid states unrepresentable" while also allowing ergonomic APIs.

### Protocol Support Architecture

Protocols are implemented as:
- **Constants**: Protocol identifiers (`NATS_PROTOCOL`, `KAFKA_PROTOCOL`, etc.)
- **Types**: Protocol-specific structs (`NatsProtocol`, `KafkaProtocol`, etc.)
- **Bindings**: Protocol-specific channel and message bindings

This design allows:
- Easy addition of new protocols
- Type-safe protocol references
- Protocol-specific validation

## Future Improvements

Potential future enhancements that the architecture supports:

1. **Feature Flags**: For optional functionality (e.g., protocol-specific features)
2. **Async Support**: For async schema generation (if needed)
3. **Components and $ref**: For reusable schema definitions
4. **CLI Tool**: Separate crate using this library for validation and generation
5. **IDE Support**: Language server or plugin for better editor integration
