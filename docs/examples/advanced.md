# Advanced Examples

This page contains advanced examples and edge cases.

## Operations with Multiple Messages

An operation can reference multiple messages:

```rust
#[derive(AsyncApiOperation)]
#[asyncapi(
    id = "handle-product-events",
    action = "receive",
    channel = "product.events",
    messages(ProductCreated, ProductUpdated),
    summary = "Handle all product events"
)]
pub struct HandleProductEvents;
```

## Optional Message Fields

Messages can have optional fields, which will be reflected in the JSON Schema:

```rust
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(channel = "user.notifications")]
pub struct UserNotification {
    pub user_id: String,
    pub title: String,
    pub message: String,
    pub priority: Option<String>,  // Optional field
    pub metadata: Option<serde_json::Value>,  // Optional nested object
}
```

## Nested Types

Complex nested types are automatically converted to JSON Schema:

```rust
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub location: Location,
}

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(channel = "warehouse.location")]
pub struct WarehouseLocationUpdate {
    pub warehouse_id: String,
    pub address: Address,  // Nested type
    pub timestamp: i64,
}
```

## Enums in Messages

Enum types are properly converted to JSON Schema:

```rust
#[derive(Serialize, Deserialize, JsonSchema)]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(channel = "order.status")]
pub struct OrderStatusUpdate {
    pub order_id: String,
    pub status: OrderStatus,  // Enum type
}
```

## Channel Parameters

Channels can have parameters for dynamic routing:

```rust
use protofolio::{AsyncApiBuilder, Channel, Parameter};
use std::collections::HashMap;

let mut params = HashMap::new();
params.insert(
    "userId".to_string(),
    Parameter {
        description: Some("User ID".to_string()),
        schema: Some(serde_json::json!({"type": "string"})),
        location: None,
    },
);

let spec = AsyncApiBuilder::new()
    .info(Info {
        title: "User Events".to_string(),
        version: "1.0.0".to_string(),
        description: None,
    })
    .channel_with_params(
        "user.{userId}.events".to_string(),
        Channel {
            description: None,
            messages: HashMap::new(),
            servers: None,
            parameters: None,
            bindings: None,
        },
        params,
    )
    .build();
```

## Security Schemes

Advanced security configuration with multiple scheme types:

```rust
#[derive(AsyncApi)]
#[asyncapi(
    info(title = "Secure Multi-Protocol API", version = "1.0.0"),
    servers(
        (name = "nats", url = "nats://localhost:4222", protocol = "nats", security = ["bearerAuth"]),
        (name = "kafka", url = "kafka://localhost:9092", protocol = "kafka", security = ["apiKey"])
    ),
    security_schemes(
        // HTTP Bearer token authentication
        (name = "bearerAuth", type = "http", scheme = "bearer", bearer_format = "JWT", description = "JWT Bearer token"),
        // API Key in header
        (name = "apiKey", type = "apiKey", in = "header", description = "API key authentication"),
        // HTTP Basic authentication
        (name = "basicAuth", type = "http", scheme = "basic", description = "Basic HTTP authentication"),
        // HTTP API Key (alternative to apiKey)
        (name = "headerApiKey", type = "httpApiKey", name_param = "X-API-Key", in = "header", description = "API key in X-API-Key header"),
        // User/Password authentication
        (name = "userPassword", type = "userPassword", description = "User and password authentication")
    ),
    channels("events"),
    messages(Event)
)]
pub struct SecureMultiProtocolApi;
```

## Error Handling

Example with comprehensive error handling:

```rust
use protofolio::{AsyncApi, ValidationError};

match MyApi::try_asyncapi() {
    Ok(spec) => {
        // Validate the spec
        match protofolio::validate_spec(&spec) {
            Ok(()) => {
                // Generate JSON
                match protofolio::to_json(&spec) {
                    Ok(json) => println!("{}", json),
                    Err(e) => eprintln!("Failed to serialize: {}", e),
                }
            }
            Err(e) => eprintln!("Validation error: {}", e),
        }
    }
    Err(e) => match e {
        ValidationError::SchemaGenerationFailed(msg, details) => {
            eprintln!("Schema generation failed for {}: {}", msg, details);
        }
        ValidationError::InvalidChannelReference(msg) => {
            eprintln!("Invalid channel reference: {}", msg);
        }
        ValidationError::MessageNotFound { channel, message } => {
            eprintln!("Message '{}' not found in channel '{}'", message, channel);
        }
        _ => eprintln!("Validation error: {}", e),
    },
}
```

## See Also

- [Basic Examples](basic.md) - Simpler examples
- [Integration Examples](integration.md) - Framework integration

