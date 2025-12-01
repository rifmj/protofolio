# protofolio 🚀

[![CI](https://github.com/rifmj/protofolio/actions/workflows/ci.yml/badge.svg)](https://github.com/rifmj/protofolio/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/rifmj/protofolio/branch/main/graph/badge.svg)](https://codecov.io/gh/rifmj/protofolio)
[![MSRV](https://img.shields.io/badge/MSRV-1.80-orange)](https://www.rust-lang.org)

> **Generate AsyncAPI 3.0 specs from your Rust code** ✨  
> Just like `utoipa` does for OpenAPI, but for async messaging! 🎯

## Overview

`protofolio` provides procedural macros and runtime support to generate AsyncAPI 3.0 specifications directly from your Rust code. This ensures your documentation stays in sync with your code automatically. **No more manual spec maintenance!** 🎉

## Quick Start ⚡

Get up and running in minutes! Here's a taste:

```rust
use protofolio::AsyncApi;
use protofolio_derive::{AsyncApi, AsyncApiMessage};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// Define your message type
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(channel = "events", messageId = "event-v1")]
pub struct Event {
    pub id: String,
    pub data: String,
}

// Define your AsyncAPI specification
#[derive(AsyncApi)]
#[asyncapi(
    info(title = "My API", version = "1.0.0"),
    channels("events"),
    messages(Event)
)]
pub struct MyApi;

// Generate the spec 🎨
let spec = MyApi::asyncapi();
let json = MyApi::asyncapi_json()?;
```

See the [Getting Started Guide](docs/guides/getting-started.md) for installation and detailed instructions. 📚

## Features ✨

- 🎯 **Code-first approach**: Generate AsyncAPI specs from Rust types
- 🔍 **Compile-time validation**: Channel and message references validated at compile time
- 🛡️ **Type safety**: Documentation always matches your code
- 🌐 **Multi-protocol support**: Built-in support for NATS, Kafka, and MQTT
- 📄 **YAML and JSON output**: Generate specs in both YAML and JSON formats
- 📘 **TypeScript generation**: CLI tool to generate TypeScript types from AsyncAPI specs
- 🏷️ **Enhanced attributes**: Support for messageId, name, title, contentType, tags, and more
- 🎨 **Root-level tags**: Reusable tag definitions at the specification level for better organization
- 🔗 **Components and `$ref` references**: Reusable messages, schemas, parameters, bindings, and traits with component references
- ✅ **Validation**: Built-in validation for generated specifications
- 🎭 **Error handling**: Both panic-on-error (`asyncapi()`) and Result-based (`try_asyncapi()`) APIs
- ⚡ **Schema caching**: Automatic caching of generated JSON schemas for performance
- 🔧 **Extensible**: Structured for easy expansion to other protocols

## Documentation 📚

### Guides 📖

- 🚀 **[Getting Started](docs/guides/getting-started.md)** - Installation, requirements, and quick start
- 💬 **[Messages](docs/guides/messages.md)** - How to define and configure message types
- 🔄 **[Operations](docs/guides/operations.md)** - How to define publish/subscribe operations
- 🔐 **[Security](docs/guides/security.md)** - How to define and use security schemes
- ✅ **[Validation](docs/guides/validation.md)** - Understanding validation and error handling
- 📘 **[TypeScript Generation](docs/guides/typescript-generation.md)** - Generate TypeScript types from AsyncAPI specs
- ⭐ **[Best Practices](docs/guides/best-practices.md)** - Recommended patterns and conventions

### Examples 💡

- 🎯 **[Basic Examples](docs/examples/basic.md)** - Simple usage examples
- 🚀 **[Advanced Examples](docs/examples/advanced.md)** - Complex patterns and edge cases
- 🔌 **[Integration Examples](docs/examples/integration.md)** - Framework integration (Axum, etc.)

### Reference 🔍

- 🛠️ **[Troubleshooting](docs/reference/troubleshooting.md)** - Common scenarios and solutions
- 💭 **[Considerations](docs/reference/limitations.md)** - Design decisions and recommended approaches
- 🔄 **[Migration Guide](docs/reference/migration.md)** - Migrating from other approaches
- ⚙️ **[Macro Expansion](docs/reference/macro-expansion.md)** - How macros work internally

### Other Documentation

- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Architecture and design decisions
- **[PROTOCOLS.md](PROTOCOLS.md)** - Protocol-specific documentation and examples
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Guidelines for contributors
- **[API Documentation](https://docs.rs/protofolio)** - Full API reference (when published)

## Installation 📦

Add to your `Cargo.toml`:

```toml
[dependencies]
protofolio = "0.1.0"
protofolio-derive = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
schemars = { version = "1.0", features = ["derive"] }
```

**Requirements**: Rust 1.80 or later. Your message types must implement `Serialize`, `Deserialize`, and `JsonSchema`.

See the [Getting Started Guide](docs/guides/getting-started.md) for detailed installation instructions. 🎓

## Project Structure

This is a Rust workspace containing:

- `protofolio/` - Main runtime library with AsyncAPI data structures
- `protofolio-derive/` - Procedural macros for code generation
- `protofolio-cli/` - CLI tool for generating TypeScript types from AsyncAPI specs

## Status 🎯

This crate is in active development. Here's what's currently available:

- ✅ Basic AsyncAPI 3.0 spec generation
- ✅ Multi-protocol support (NATS, Kafka, MQTT)
- ✅ Message and channel mapping
- ✅ JSON Schema generation from Rust types with automatic caching
- ✅ JSON and YAML output formats
- ✅ Enhanced message attributes (messageId, name, title, contentType, tags)
- ✅ Channel parameters and bindings support
- ✅ Specification validation
- ✅ Comprehensive error handling (`try_asyncapi()` with Result types)
- ✅ Operations support (publish/subscribe with full attribute support)
- ✅ Protocol-specific bindings for NATS, Kafka, and MQTT
- ✅ Security schemes (userPassword, apiKey, http, oauth2, etc.)
- ✅ External documentation support (Info, Message, Operation)
- ✅ Info fields (contact, license, termsOfService)
- ✅ Server variables (templated URLs with variable definitions)
- ✅ Message examples (single or multiple example payloads)
- ✅ Message headers (schema definition for message headers)
- ✅ Components and `$ref` references (reusable messages, schemas, parameters, bindings, and traits)
- ✅ Correlation IDs (message correlation tracking with location expressions)
- ✅ Channel address field (required AsyncAPI 3.0 field)
- ✅ Operation ID field (unique operation identifiers)
- ✅ Root-level tags (reusable tag definitions at specification level)
- ✅ TypeScript type generation (CLI tool for generating TypeScript types from AsyncAPI specs)

🚧 **Coming soon**:

- Full AsyncAPI 3.0 feature set

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
