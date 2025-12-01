//! Protocol support for AsyncAPI
//!
//! This module provides protocol-specific implementations for messaging protocols
//! supported by AsyncAPI 3.0. Currently supports:
//!
//! - **NATS**: Lightweight, high-performance messaging system
//! - **Kafka**: Distributed event streaming platform
//! - **MQTT**: IoT messaging protocol
//!
//! # Usage
//!
//! Typically, you don't need to interact with this module directly - protocols
//! are specified in the `#[asyncapi]` attribute:
//!
//! ```rust,no_run
//! #[derive(AsyncApi)]
//! #[asyncapi(
//!     info(title = "My API", version = "1.0.0"),
//!     servers((name = "nats", url = "nats://localhost:4222", protocol = "nats")),
//!     // ...
//! )]
//! struct MyApi;
//! ```
//!
//! For programmatic access, use the protocol constants and types exported from
//! this module. See individual protocol modules for protocol-specific bindings
//! and configuration options.

mod traits;
mod bindings;

#[cfg(feature = "nats")]
pub mod nats;
#[cfg(feature = "kafka")]
pub mod kafka;
#[cfg(feature = "mqtt")]
pub mod mqtt;

pub use traits::*;
pub use bindings::*;

// Re-exports for convenience (conditional on features)
#[cfg(feature = "nats")]
pub use nats::{NatsProtocol, PROTOCOL as NATS_PROTOCOL, DEFAULT_PORT as NATS_DEFAULT_PORT};
#[cfg(feature = "kafka")]
pub use kafka::{KafkaProtocol, PROTOCOL as KAFKA_PROTOCOL, DEFAULT_PORT as KAFKA_DEFAULT_PORT};
#[cfg(feature = "mqtt")]
pub use mqtt::{
    MqttProtocol, 
    PROTOCOL as MQTT_PROTOCOL, 
    DEFAULT_PORT as MQTT_DEFAULT_PORT,
    DEFAULT_SECURE_PORT as MQTT_DEFAULT_SECURE_PORT,
    MqttQos,
};

/// Validate protocol identifier
pub fn validate_protocol(protocol: &str) -> Result<(), crate::error::ValidationError> {
    let mut supported = Vec::new();
    
    #[cfg(feature = "nats")]
    {
        if protocol == NATS_PROTOCOL {
            return Ok(());
        }
        supported.push(NATS_PROTOCOL.to_string());
    }
    
    #[cfg(feature = "kafka")]
    {
        if protocol == KAFKA_PROTOCOL {
            return Ok(());
        }
        supported.push(KAFKA_PROTOCOL.to_string());
    }
    
    #[cfg(feature = "mqtt")]
    {
        if protocol == MQTT_PROTOCOL {
            return Ok(());
        }
        supported.push(MQTT_PROTOCOL.to_string());
    }
    
    Err(crate::error::ValidationError::UnsupportedProtocol {
        protocol: protocol.to_string(),
        supported,
    })
}

