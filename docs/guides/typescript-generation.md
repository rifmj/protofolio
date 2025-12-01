# TypeScript Generation Guide 📘

Generate TypeScript type definitions from your AsyncAPI specifications! This guide shows you how to use the `protofolio-cli` tool to create TypeScript types that match your Rust message types. 🚀

## Overview 📋

The `protofolio-cli` tool generates TypeScript type definitions from AsyncAPI 3.0 specifications using [Modelina](https://modelina.org/). This enables:

- ✅ **Type safety** in TypeScript/JavaScript projects
- ✅ **Automatic synchronization** with your Rust types
- ✅ **Single source of truth** - types generated from the same code
- ✅ **Developer experience** - autocomplete and type checking in your frontend

## Installation 📦

### Prerequisites

- **Node.js** - Required for running the TypeScript generation script
- **Rust** - For building the CLI tool

### Building the CLI

Build the CLI from source:

```bash
cargo build --release --package protofolio-cli
```

The binary will be available at `target/release/protofolio`.

### Installing Dependencies

Install the required Node.js dependencies:

```bash
npm install
```

This installs `@asyncapi/modelina`, which is used to generate TypeScript types.

## Usage 🎯

### Step 1: Generate AsyncAPI Specification

First, generate your AsyncAPI specification from your Rust code:

```rust
use protofolio::AsyncApi;
use protofolio_derive::AsyncApi;

#[derive(AsyncApi)]
#[asyncapi(
    info(title = "My API", version = "1.0.0"),
    channels("events"),
    messages(Event)
)]
pub struct MyApi;

fn main() {
    // Generate and save the spec
    let json = MyApi::asyncapi_json().unwrap();
    std::fs::write("asyncapi.json", json).unwrap();
}
```

Or use the helper example:

```bash
cargo run --example generate_spec --package protofolio > asyncapi.json
```

### Step 2: Generate TypeScript Types

Use the CLI to generate TypeScript types:

```bash
protofolio generate --spec asyncapi.json --output ./types
```

### Command Options

- `--spec` / `-s`: Path to the AsyncAPI specification file (JSON or YAML)
- `--output` / `-o`: Output directory for generated TypeScript types (default: `./types`)
- `--format` / `-f`: Format of the input spec file (`json` or `yaml`). Auto-detected from file extension if not specified.

### Example Workflow

```bash
# 1. Generate AsyncAPI spec from Rust
cargo run --example generate_spec --package protofolio > asyncapi.json

# 2. Generate TypeScript types
./target/release/protofolio generate --spec asyncapi.json --output ./types

# 3. Use the types in your TypeScript project
```

## Generated Output 📁

The CLI generates:

- **Individual TypeScript files** - One file per message type (e.g., `Event.ts`)
- **Index file** - `index.ts` that exports all types for easy importing

### Example Generated Types

```typescript
// types/Event.ts
export interface Event {
  id: string;
  data: string;
}

// types/index.ts
export * from './Event';
```

### Using Generated Types

Import and use the generated types in your TypeScript/JavaScript project:

```typescript
import { Event } from './types';

const event: Event = {
  id: "123",
  data: "example data"
};

// TypeScript will provide autocomplete and type checking
console.log(event.id); // ✅ Type-safe access
```

## Integration with Build Systems 🔧

### Cargo Build Script

You can integrate TypeScript generation into your build process:

```rust
// build.rs
use std::process::Command;

fn main() {
    // Generate AsyncAPI spec
    let output = Command::new("cargo")
        .args(&["run", "--example", "generate_spec", "--package", "protofolio"])
        .output()
        .expect("Failed to generate spec");
    
    std::fs::write("asyncapi.json", output.stdout).unwrap();
    
    // Generate TypeScript types
    Command::new("./target/release/protofolio")
        .args(&["generate", "--spec", "asyncapi.json", "--output", "./types"])
        .status()
        .expect("Failed to generate TypeScript types");
}
```

### npm Scripts

Add to your `package.json`:

```json
{
  "scripts": {
    "generate-types": "protofolio generate --spec asyncapi.json --output ./types",
    "build": "npm run generate-types && tsc"
  }
}
```

### CI/CD Integration

Example GitHub Actions workflow:

```yaml
name: Generate Types

on:
  push:
    branches: [main]

jobs:
  generate-types:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
      - run: cargo build --release --package protofolio-cli
      - run: npm install
      - run: cargo run --example generate_spec --package protofolio > asyncapi.json
      - run: ./target/release/protofolio generate --spec asyncapi.json --output ./types
      - uses: actions/upload-artifact@v3
        with:
          name: typescript-types
          path: types/
```

## Troubleshooting 🔍

### Script Not Found Error

If you see "Could not find generate-types.js script", ensure:

1. The script exists at `scripts/generate-types.js`
2. You're running the CLI from the workspace root
3. The script path is correct relative to your current directory

### Node.js Module Errors

If you encounter ES module errors, ensure:

1. `package.json` has `"type": "module"` (which is correct for the script)
2. Node.js version is 18 or later
3. Dependencies are installed with `npm install`

### No Types Generated

If no TypeScript types are generated:

1. Verify your AsyncAPI spec is valid JSON/YAML
2. Check that your spec contains message definitions
3. Review the CLI output for error messages

## See Also 📚

- [Getting Started](getting-started.md) - Basic setup and usage
- [Messages Guide](messages.md) - How to define message types
- [Integration Examples](../examples/integration.md) - More integration patterns
- [Best Practices](best-practices.md) - Recommended patterns

