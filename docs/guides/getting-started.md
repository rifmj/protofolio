# Getting Started 🚀

Welcome to `protofolio`! This guide will help you get started with generating AsyncAPI 3.0 specifications from your Rust code. Let's dive in! 💪

## Installation & Requirements 📦

### Requirements

- 🦀 **Rust**: 1.80 or later
- 📚 **Dependencies**: Your message types must implement:
  - `Serialize` and `Deserialize` (from `serde`)
  - `JsonSchema` (from `schemars`)

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
protofolio = "0.1.0"
protofolio-derive = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
schemars = { version = "1.0", features = ["derive"] }
```

## Quick Start ⚡

Here's a minimal example to get you started. It's super simple! 🎯

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
    info(
        title = "My API",
        version = "1.0.0",
        description = "My awesome API",
        contact(name = "Support", email = "support@example.com"),
        license(name = "MIT")
    ),
    channels("events"),
    messages(Event)
)]
pub struct MyApi;

// Generate the spec
let spec = MyApi::asyncapi();

// Or with error handling
let spec = MyApi::try_asyncapi()?;

// Generate JSON or YAML
let json = MyApi::asyncapi_json()?;
let yaml = MyApi::asyncapi_yaml()?;
```

## Generating TypeScript Types 📘

Want to use your AsyncAPI specs in TypeScript/JavaScript projects? Use the `protofolio-cli` tool to generate TypeScript type definitions:

```bash
# Build the CLI
cargo build --release --package protofolio-cli

# Generate TypeScript types
./target/release/protofolio generate --spec asyncapi.json --output ./types
```

See the [TypeScript Generation Guide](typescript-generation.md) for detailed instructions! 🚀

## What's Next? 🎯

Ready to learn more? Here's where to go next:

- 💬 Learn about [Messages](messages.md) - how to define and configure message types
- 🔄 Explore [Operations](operations.md) - how to define publish/subscribe operations
- 📘 Check out [TypeScript Generation](typescript-generation.md) - generate TypeScript types from your specs
- 💡 Check out [Examples](../examples/basic.md) - more detailed examples
- ⭐ Read [Best Practices](best-practices.md) - recommended patterns and conventions

Happy coding! 🎉
