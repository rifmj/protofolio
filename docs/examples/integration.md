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

## See Also

- [Basic Examples](basic.md) - Basic usage examples
- [Advanced Examples](advanced.md) - Advanced patterns

