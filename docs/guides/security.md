# Security Schemes Guide

This guide covers how to define and use security schemes in your AsyncAPI specifications.

## Overview

Security schemes define authentication and authorization mechanisms for your API. `protofolio` supports all AsyncAPI 3.0 security scheme types, allowing you to secure your servers and operations.

## Supported Security Scheme Types

- **userPassword** - User and password authentication
- **apiKey** - API key authentication
- **http** - HTTP authentication (basic, bearer, digest)
- **httpApiKey** - HTTP API key authentication
- **oauth2** - OAuth2 authentication
- **openIdConnect** - OpenID Connect authentication
- **X509** - X.509 certificate authentication
- **symmetricEncryption** - Symmetric encryption
- **asymmetricEncryption** - Asymmetric encryption
- **mutualTLS** - Mutual TLS authentication

## Basic Usage

### Defining Security Schemes

Define security schemes in the `security_schemes` attribute of your `AsyncApi` struct:

```rust
use protofolio::AsyncApi;
use protofolio_derive::AsyncApi;

#[derive(AsyncApi)]
#[asyncapi(
    info(title = "My API", version = "1.0.0"),
    security_schemes(
        (name = "userPassword", type = "userPassword", description = "User and password authentication")
    ),
    channels("events"),
    messages(Event)
)]
pub struct MyApi;
```

### Applying Security to Servers

Apply security schemes to servers using the `security` attribute:

```rust
#[derive(AsyncApi)]
#[asyncapi(
    info(title = "My API", version = "1.0.0"),
    servers(
        (name = "nats", url = "nats://localhost:4222", protocol = "nats", security = ["userPassword"])
    ),
    security_schemes(
        (name = "userPassword", type = "userPassword", description = "User and password authentication")
    ),
    channels("events"),
    messages(Event)
)]
pub struct MyApi;
```

## Security Scheme Types

### User Password

Simple username and password authentication:

```rust
security_schemes(
    (name = "userPassword", type = "userPassword", description = "User and password authentication")
)
```

### API Key

API key authentication with configurable location:

```rust
security_schemes(
    // API key in header
    (name = "apiKeyHeader", type = "apiKey", in = "header", description = "API key in header"),
    // API key in query parameter
    (name = "apiKeyQuery", type = "apiKey", in = "query", description = "API key in query"),
    // API key in cookie
    (name = "apiKeyCookie", type = "apiKey", in = "cookie", description = "API key in cookie")
)
```

**Note**: The `in` attribute is optional but recommended. Valid values: `"header"`, `"query"`, `"cookie"`, or `"user"`.

### HTTP Authentication

HTTP authentication schemes (basic, bearer, digest):

```rust
security_schemes(
    // Basic authentication
    (name = "basicAuth", type = "http", scheme = "basic", description = "Basic HTTP authentication"),
    
    // Bearer token authentication
    (name = "bearerAuth", type = "http", scheme = "bearer", bearer_format = "JWT", description = "JWT Bearer token"),
    
    // Digest authentication
    (name = "digestAuth", type = "http", scheme = "digest", description = "Digest authentication")
)
```

**Required attributes**:
- `scheme` - The HTTP authentication scheme (`"basic"`, `"bearer"`, or `"digest"`)

**Optional attributes**:
- `bearer_format` - Format of the bearer token (e.g., `"JWT"` for JWT tokens)

### HTTP API Key

HTTP API key authentication with named parameter:

```rust
security_schemes(
    (name = "headerApiKey", type = "httpApiKey", name_param = "X-API-Key", in = "header", description = "API key in X-API-Key header"),
    (name = "queryApiKey", type = "httpApiKey", name_param = "api_key", in = "query", description = "API key in query parameter")
)
```

**Required attributes**:
- `name_param` - Name of the header, query, or cookie parameter
- `in` - Location of the API key (`"header"`, `"query"`, or `"cookie"`)

### OpenID Connect

OpenID Connect authentication:

```rust
security_schemes(
    (name = "openIdConnect", type = "openIdConnect", open_id_connect_url = "https://example.com/.well-known/openid-configuration", description = "OpenID Connect authentication")
)
```

**Required attributes**:
- `open_id_connect_url` - URL to the OpenID Connect configuration

### OAuth2

OAuth2 authentication (basic structure):

```rust
security_schemes(
    (name = "oauth2", type = "oauth2", description = "OAuth2 authentication")
)
```

**Note**: Full OAuth2 flow configuration (authorization URLs, token URLs, scopes) is planned for future releases. Currently, a basic OAuth2 structure is created.

### Certificate-Based Authentication

X.509 certificate authentication:

```rust
security_schemes(
    (name = "x509", type = "X509", description = "X.509 certificate authentication")
)
```

### Encryption Schemes

Symmetric and asymmetric encryption:

```rust
security_schemes(
    (name = "symmetric", type = "symmetricEncryption", description = "Symmetric encryption"),
    (name = "asymmetric", type = "asymmetricEncryption", description = "Asymmetric encryption")
)
```

### Mutual TLS

Mutual TLS authentication:

```rust
security_schemes(
    (name = "mutualTLS", type = "mutualTLS", description = "Mutual TLS authentication")
)
```

## Multiple Security Schemes

You can define multiple security schemes and apply different ones to different servers:

```rust
#[derive(AsyncApi)]
#[asyncapi(
    info(title = "Multi-Security API", version = "1.0.0"),
    servers(
        (name = "public", url = "nats://public:4222", protocol = "nats", security = ["apiKey"]),
        (name = "private", url = "nats://private:4222", protocol = "nats", security = ["bearerAuth"])
    ),
    security_schemes(
        (name = "apiKey", type = "apiKey", in = "header", description = "API key for public endpoints"),
        (name = "bearerAuth", type = "http", scheme = "bearer", bearer_format = "JWT", description = "JWT token for private endpoints")
    ),
    channels("events"),
    messages(Event)
)]
pub struct MultiSecurityApi;
```

## Security Requirements

The `security` attribute on servers accepts a list of security scheme names. Each name must match a security scheme defined in `security_schemes`:

```rust
servers(
    // Single security scheme
    (name = "server1", url = "nats://localhost:4222", protocol = "nats", security = ["userPassword"]),
    
    // Multiple security schemes (OR relationship - any one is sufficient)
    (name = "server2", url = "nats://localhost:4223", protocol = "nats", security = ["apiKey", "bearerAuth"])
)
```

## Complete Example

```rust
use protofolio::AsyncApi;
use protofolio_derive::{AsyncApi, AsyncApiMessage};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(channel = "events", messageId = "event-v1")]
pub struct Event {
    pub id: String,
    pub data: String,
}

#[derive(AsyncApi)]
#[asyncapi(
    info(
        title = "Secure Events API",
        version = "1.0.0",
        description = "A secure event streaming API"
    ),
    servers(
        (name = "production", url = "nats://prod:4222", protocol = "nats", security = ["bearerAuth"]),
        (name = "staging", url = "nats://staging:4222", protocol = "nats", security = ["apiKey"])
    ),
    security_schemes(
        (name = "bearerAuth", type = "http", scheme = "bearer", bearer_format = "JWT", description = "JWT Bearer token authentication"),
        (name = "apiKey", type = "apiKey", in = "header", description = "API key authentication")
    ),
    channels("events"),
    messages(Event)
)]
pub struct SecureEventsApi;
```

## Validation

Security scheme validation happens at compile time:

- Required attributes are validated based on scheme type
- Security scheme names referenced in server `security` attributes must be defined in `security_schemes`
- Invalid scheme types produce compile-time errors

## Best Practices

1. **Use descriptive names**: Choose clear, descriptive names for your security schemes (e.g., `bearerAuth` instead of `auth1`)

2. **Provide descriptions**: Always include descriptions to help API consumers understand the authentication mechanism

3. **Match protocol requirements**: Choose security schemes appropriate for your messaging protocol (e.g., HTTP-based schemes for HTTP protocols)

4. **Document scopes**: For OAuth2, document required scopes in the description until full OAuth2 flow support is available

5. **Test security**: Verify that your security schemes work correctly with your messaging infrastructure

## See Also

- [Basic Examples](../examples/basic.md) - Simple security scheme examples
- [Advanced Examples](../examples/advanced.md) - Complex security configurations
- [AsyncAPI Security Specification](https://www.asyncapi.com/docs/specifications/v3.0.0#securitySchemeObject) - Official AsyncAPI security scheme documentation

