//! protofolio - Generate AsyncAPI 3.0 specifications from Rust code
//!
//! This crate provides the runtime library for generating AsyncAPI specifications.
//! Use the `protofolio-derive` crate for procedural macros.
//!
//! Lints are configured in the workspace Cargo.toml and inherited here.
#![deny(
    rustdoc::broken_intra_doc_links,
    unsafe_code
)]
#![warn(
    missing_docs,
    missing_debug_implementations
)]
#![allow(
    // Documentation - can be fixed incrementally
    clippy::missing_docs_in_private_items,
    rustdoc::missing_doc_code_examples,
    rustdoc::private_intra_doc_links, // Private items can link to each other
    // Proc-macro specific patterns
    clippy::result_large_err, // Result<TokenStream, Error> is fine for macros
    // Style preferences (optional improvements)
    clippy::must_use_candidate, // #[must_use] is nice but not required
    clippy::use_self, // Self is preferred but not required everywhere
    clippy::uninlined_format_args, // Old format! style is acceptable
    clippy::redundant_closure, // Closures are sometimes clearer
    clippy::wildcard_imports, // Wildcard imports are fine for re-exports
)]

//! # Quick Start
//!
//! ```rust,no_run
//! use protofolio::AsyncApi;
//! use protofolio_derive::{AsyncApi, AsyncApiMessage};
//! use serde::{Deserialize, Serialize};
//! use schemars::JsonSchema;
//!
//! #[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
//! #[asyncapi(channel = "events", messageId = "event-v1")]
//! pub struct Event {
//!     pub id: String,
//! }
//!
//! #[derive(AsyncApi)]
//! #[asyncapi(
//!     info(title = "My API", version = "1.0.0"),
//!     channels("events"),
//!     messages(Event)
//! )]
//! pub struct MyApi;
//!
//! let spec = MyApi::asyncapi();
//! ```
//!
//! # Features
//!
//! - **Code-first**: Generate specs from Rust types
//! - **Type-safe**: Documentation matches your code
//! - **Compile-time checks**: Validates channel and message references
//! - **JSON Schema**: Automatic schema generation from Rust types
//!
//! # Limitations
//!
//! - Full compile-time channel validation is limited by Rust's const evaluation
//! - Generic types require manual `JsonSchema` implementation
//! - Supports NATS, Kafka, and MQTT protocols
//!
//! See the [README](../README.md) for complete documentation and examples.

// Core modules
mod error;
mod types;
mod spec;
mod traits;
mod builder;
mod schema;
mod validation;
mod protocol;
mod internal;

// Public API - carefully curated exports
pub use error::{SchemaError, ValidationError};
pub use types::OperationAction;
pub use spec::*;
pub use traits::{AsyncApi, AsyncApiOperation};
pub use builder::AsyncApiBuilder;
pub use schema::{generate_schema, schema_for_type};
pub use validation::validate_spec;
// Protocol exports (conditional on features)
pub use protocol::Protocol;

#[cfg(feature = "nats")]
pub use protocol::{
    NatsProtocol, NATS_PROTOCOL, NATS_DEFAULT_PORT,
    NatsChannelBinding, NatsChannelConfig, NatsMessageBinding, NatsMessageConfig,
};

#[cfg(feature = "kafka")]
pub use protocol::{
    KafkaProtocol, KAFKA_PROTOCOL, KAFKA_DEFAULT_PORT,
    KafkaChannelBinding, KafkaChannelConfig, KafkaMessageBinding, KafkaMessageConfig,
};

#[cfg(feature = "mqtt")]
pub use protocol::{
    MqttProtocol, MQTT_PROTOCOL, MQTT_DEFAULT_PORT, MQTT_DEFAULT_SECURE_PORT,
    MqttQos,
    MqttChannelBinding, MqttChannelConfig, MqttMessageBinding, MqttMessageConfig,
};

/// Convert an AsyncAPI specification to YAML string
///
/// Helper function for converting an AsyncApiSpec to YAML format.
///
/// # Example
///
/// ```rust,no_run
/// use protofolio::to_yaml;
/// # use protofolio::AsyncApi;
/// # use protofolio_derive::AsyncApi;
/// #
/// # #[derive(AsyncApi)]
/// # #[asyncapi(info(title = "Test", version = "1.0.0"), channels("events"), messages())]
/// # struct MyApi;
///
/// let spec = MyApi::asyncapi();
/// let yaml = to_yaml(&spec)?;
/// println!("{}", yaml);
/// # Ok::<(), serde_yaml_ng::Error>(())
/// ```
pub fn to_yaml(spec: &AsyncApiSpec) -> Result<String, serde_yaml_ng::Error> {
    serde_yaml_ng::to_string(spec)
}

/// Convert an AsyncAPI specification to JSON string
///
/// Helper function for converting an AsyncApiSpec to JSON format.
///
/// # Example
///
/// ```rust,no_run
/// use protofolio::to_json;
/// # use protofolio::AsyncApi;
/// # use protofolio_derive::AsyncApi;
/// #
/// # #[derive(AsyncApi)]
/// # #[asyncapi(info(title = "Test", version = "1.0.0"), channels("events"), messages())]
/// # struct MyApi;
///
/// let spec = MyApi::asyncapi();
/// let json = to_json(&spec)?;
/// println!("{}", json);
/// # Ok::<(), serde_json::Error>(())
/// ```
pub fn to_json(spec: &AsyncApiSpec) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(spec)
}
