# Getting Started

This guide will help you get started with `protofolio` - a Rust crate for generating AsyncAPI 3.0 specifications from your code.

## Installation & Requirements

### Requirements

- **Rust**: 1.80 or later
- **Dependencies**: Your message types must implement:
  - `Serialize` and `Deserialize` (from `serde`)
  - `JsonSchema` (from `schemars`)

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
protofolio = "0.1.0"
protofolio-derive = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
schemars = { version = "0.8", features = ["derive"] }
```

## Quick Start

Here's a minimal example to get you started:

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

// Generate the spec
let spec = MyApi::asyncapi();

// Or with error handling
let spec = MyApi::try_asyncapi()?;

// Generate JSON or YAML
let json = MyApi::asyncapi_json()?;
let yaml = MyApi::asyncapi_yaml()?;
```

## What's Next?

- Learn about [Messages](messages.md) - how to define and configure message types
- Explore [Operations](operations.md) - how to define publish/subscribe operations
- Check out [Examples](../examples/basic.md) - more detailed examples
- Read [Best Practices](best-practices.md) - recommended patterns and conventions
