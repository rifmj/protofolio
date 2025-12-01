//! AsyncAPI specification builder implementation
//!
//! The builder pattern allows programmatic construction of AsyncAPI specifications
//! without using the derive macros. This is useful when:
//! - Building specs dynamically at runtime
//! - Integrating with other code generation tools
//! - Testing and prototyping
//!
//! # Example
//!
//! ```rust,no_run
//! use protofolio::{AsyncApiBuilder, Info, Server, Channel, Message, MessagePayload};
//! use std::collections::HashMap;
//!
//! let spec = AsyncApiBuilder::new()
//!     .info(Info {
//!         title: "My API".to_string(),
//!         version: "1.0.0".to_string(),
//!         description: Some("API description".to_string()),
//!         external_docs: None,
//!     })
//!     .server("nats".to_string(), Server {
//!         url: "nats://localhost:4222".to_string(),
//!         protocol: "nats".to_string(),
//!         description: None,
//!         security: None,
//!     })
//!     .channel("events".to_string(), Channel {
//!         description: None,
//!         messages: {
//!             let mut m = HashMap::new();
//!             m.insert("Event".to_string(), Message {
//!                 message_id: None,
//!                 name: None,
//!                 title: None,
//!                 summary: None,
//!                 description: None,
//!                 content_type: None,
//!                 tags: None,
//!                 payload: MessagePayload {
//!                     schema: serde_json::json!({"type": "object"}),
//!                 },
//!             });
//!             m
//!         },
//!         servers: None,
//!         parameters: None,
//!         bindings: None,
//!     })
//!     .build();
//! ```

use crate::error::ValidationError;
use crate::spec::*;
use crate::types::ASYNCAPI_VERSION;
use crate::validation;

/// Builder for AsyncAPI specifications
///
/// Use this when you need to build specs programmatically instead of using
/// the derive macros. See the module-level documentation above for examples.
#[derive(Debug, Clone)]
pub struct AsyncApiBuilder {
    spec: AsyncApiSpec,
}

impl AsyncApiBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            spec: AsyncApiSpec {
                asyncapi: ASYNCAPI_VERSION.to_string(),
                info: Info {
                    title: String::new(),
                    version: String::new(),
                    description: None,
                    external_docs: None,
                },
                servers: None,
                channels: Default::default(),
                operations: None,
                components: None,
            },
        }
    }

    /// Set the info section
    #[must_use]
    pub fn info(mut self, info: Info) -> Self {
        self.spec.info = info;
        self
    }

    /// Add a server
    #[must_use]
    pub fn server(mut self, name: String, server: Server) -> Self {
        if self.spec.servers.is_none() {
            self.spec.servers = Some(Default::default());
        }
        if let Some(ref mut servers) = self.spec.servers {
            servers.insert(name, server);
        }
        self
    }

    /// Add a channel
    #[must_use]
    pub fn channel(mut self, name: String, channel: Channel) -> Self {
        self.spec.channels.insert(name, channel);
        self
    }

    /// Add a channel with parameters
    #[must_use]
    pub fn channel_with_params(
        mut self,
        name: String,
        channel: Channel,
        parameters: std::collections::HashMap<String, Parameter>,
    ) -> Self {
        let mut ch = channel;
        ch.parameters = Some(parameters);
        self.spec.channels.insert(name, ch);
        self
    }

    /// Add a channel with bindings
    #[must_use]
    pub fn channel_with_bindings(
        mut self,
        name: String,
        channel: Channel,
        bindings: serde_json::Value,
    ) -> Self {
        let mut ch = channel;
        ch.bindings = Some(bindings);
        self.spec.channels.insert(name, ch);
        self
    }

    /// Build the final specification
    #[must_use]
    pub fn build(self) -> AsyncApiSpec {
        self.spec
    }

    /// Build and validate the final specification
    ///
    /// Returns the specification if valid, or a validation error.
    pub fn build_and_validate(self) -> Result<AsyncApiSpec, ValidationError> {
        let spec = self.build();
        validation::validate_spec(&spec)?;
        Ok(spec)
    }

    /// Add a Kafka channel with bindings
    #[must_use]
    pub fn kafka_channel(
        self,
        name: String,
        channel: Channel,
        topic: Option<String>,
        partitions: Option<u32>,
        replicas: Option<u32>,
    ) -> Self {
        use crate::protocol::KafkaProtocol;
        let bindings = KafkaProtocol::channel_binding(topic, partitions, replicas);
        self.channel_with_bindings(name, channel, bindings)
    }

    /// Add an MQTT channel with bindings
    #[must_use]
    pub fn mqtt_channel(
        self,
        name: String,
        channel: Channel,
        topic: Option<String>,
        qos: Option<crate::protocol::MqttQos>,
        retain: Option<bool>,
    ) -> Self {
        use crate::protocol::MqttProtocol;
        let bindings = MqttProtocol::channel_binding(topic, qos, retain);
        self.channel_with_bindings(name, channel, bindings)
    }
}

impl Default for AsyncApiBuilder {
    fn default() -> Self {
        Self::new()
    }
}
