# Integration Examples

This page contains examples of integrating `protofolio` with web frameworks and other tools.

## Integration with Axum

Example of serving AsyncAPI specs via HTTP endpoints:

```rust
use axum::{response::{Json, Response}, routing::get, Router};
use protofolio::AsyncApi;

async fn asyncapi_json() -> Json<protofolio::AsyncApiSpec> {
    Json(ECommerceApi::asyncapi())
}

async fn asyncapi_yaml() -> Response<String> {
    let yaml = ECommerceApi::asyncapi_yaml().unwrap();
    Response::builder()
        .header("content-type", "application/yaml")
        .body(yaml)
        .unwrap()
}

let app = Router::new()
    .route("/asyncapi.json", get(asyncapi_json))
    .route("/asyncapi.yaml", get(asyncapi_yaml));
```

## Error Handling in Web Handlers

Example with proper error handling:

```rust
use axum::{response::Response, http::StatusCode};
use protofolio::{AsyncApi, ValidationError};

async fn asyncapi_json() -> Result<Json<protofolio::AsyncApiSpec>, StatusCode> {
    match ECommerceApi::try_asyncapi() {
        Ok(spec) => Ok(Json(spec)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn asyncapi_yaml() -> Result<Response<String>, StatusCode> {
    match ECommerceApi::try_asyncapi_yaml() {
        Ok(yaml) => {
            let response = Response::builder()
                .header("content-type", "application/yaml")
                .body(yaml)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(response)
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
```

## TypeScript Type Generation

Generate TypeScript types from your AsyncAPI specifications:

```bash
# Generate AsyncAPI spec
cargo run --example generate_spec --package protofolio > asyncapi.json

# Generate TypeScript types
./target/release/protofolio generate --spec asyncapi.json --output ./types
```

Then use the generated types in your TypeScript project:

```typescript
import { Event } from "./types";

const event: Event = {
  id: "123",
  data: "example",
};
```

See the [TypeScript Generation Guide](../guides/typescript-generation.md) for more details.

## See Also

- [Basic Examples](basic.md) - Basic usage examples
- [Advanced Examples](advanced.md) - Advanced patterns
- [TypeScript Generation Guide](../guides/typescript-generation.md) - Generate TypeScript types from AsyncAPI specs
