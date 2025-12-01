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
//!         address: "events".to_string(),
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
//!                 external_docs: None,
//!                 examples: None,
//!                 headers: None,
//!                 correlation_id: None,
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
use crate::spec::{Tag, *};
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
                    contact: None,
                    license: None,
                    terms_of_service: None,
                },
                servers: None,
                channels: Default::default(),
                operations: None,
                components: None,
                tags: None,
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

    /// Add a message to a channel (inline message)
    #[must_use]
    pub fn channel_message(
        mut self,
        channel_name: String,
        message_name: String,
        message: Message,
    ) -> Self {
        use crate::spec::MessageOrRef;

        if let Some(channel) = self.spec.channels.get_mut(&channel_name) {
            channel
                .messages
                .insert(message_name, MessageOrRef::message(message));
        }
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
        use crate::spec::ChannelBindingsOrRef;
        let mut ch = channel;
        ch.bindings = Some(ChannelBindingsOrRef::bindings(bindings));
        self.spec.channels.insert(name, ch);
        self
    }

    /// Add a channel with bindings reference
    #[must_use]
    pub fn channel_with_bindings_ref(
        mut self,
        name: String,
        channel: Channel,
        component_name: String,
    ) -> Self {
        use crate::spec::ChannelBindingsOrRef;
        let mut ch = channel;
        ch.bindings = Some(ChannelBindingsOrRef::component_ref(&component_name));
        self.spec.channels.insert(name, ch);
        self
    }

    /// Add a component message
    #[must_use]
    pub fn component_message(mut self, name: String, message: Message) -> Self {
        if self.spec.components.is_none() {
            self.spec.components = Some(Components::default());
        }
        if let Some(ref mut components) = self.spec.components {
            if components.messages.is_none() {
                components.messages = Some(Default::default());
            }
            if let Some(ref mut messages) = components.messages {
                messages.insert(name, message);
            }
        }
        self
    }

    /// Add a component schema
    #[must_use]
    pub fn component_schema(mut self, name: String, schema: serde_json::Value) -> Self {
        if self.spec.components.is_none() {
            self.spec.components = Some(Components::default());
        }
        if let Some(ref mut components) = self.spec.components {
            if components.schemas.is_none() {
                components.schemas = Some(Default::default());
            }
            if let Some(ref mut schemas) = components.schemas {
                schemas.insert(name, schema);
            }
        }
        self
    }

    /// Set root-level tags
    #[must_use]
    pub fn tags(mut self, tags: Vec<Tag>) -> Self {
        self.spec.tags = Some(tags);
        self
    }

    /// Add a message reference to a channel (references a component message)
    #[must_use]
    pub fn channel_message_ref(
        mut self,
        channel_name: String,
        message_name: String,
        component_name: String,
    ) -> Self {
        use crate::spec::{MessageOrRef, MessageReference};

        if let Some(channel) = self.spec.channels.get_mut(&channel_name) {
            let ref_path = format!("#/components/messages/{}", component_name);
            channel.messages.insert(
                message_name,
                MessageOrRef::Ref(MessageReference { ref_path }),
            );
        }
        self
    }

    /// Add a component parameter
    #[must_use]
    pub fn component_parameter(mut self, name: String, parameter: crate::spec::Parameter) -> Self {
        if self.spec.components.is_none() {
            self.spec.components = Some(Components::default());
        }
        if let Some(ref mut components) = self.spec.components {
            if components.parameters.is_none() {
                components.parameters = Some(Default::default());
            }
            if let Some(ref mut parameters) = components.parameters {
                parameters.insert(name, parameter);
            }
        }
        self
    }

    /// Add a component channel bindings
    #[must_use]
    pub fn component_channel_bindings(mut self, name: String, bindings: serde_json::Value) -> Self {
        if self.spec.components.is_none() {
            self.spec.components = Some(Components::default());
        }
        if let Some(ref mut components) = self.spec.components {
            if components.channel_bindings.is_none() {
                components.channel_bindings = Some(Default::default());
            }
            if let Some(ref mut channel_bindings) = components.channel_bindings {
                channel_bindings.insert(name, bindings);
            }
        }
        self
    }

    /// Add a component message bindings
    #[must_use]
    pub fn component_message_bindings(mut self, name: String, bindings: serde_json::Value) -> Self {
        if self.spec.components.is_none() {
            self.spec.components = Some(Components::default());
        }
        if let Some(ref mut components) = self.spec.components {
            if components.message_bindings.is_none() {
                components.message_bindings = Some(Default::default());
            }
            if let Some(ref mut message_bindings) = components.message_bindings {
                message_bindings.insert(name, bindings);
            }
        }
        self
    }

    /// Add a component server bindings
    #[must_use]
    pub fn component_server_bindings(mut self, name: String, bindings: serde_json::Value) -> Self {
        if self.spec.components.is_none() {
            self.spec.components = Some(Components::default());
        }
        if let Some(ref mut components) = self.spec.components {
            if components.server_bindings.is_none() {
                components.server_bindings = Some(Default::default());
            }
            if let Some(ref mut server_bindings) = components.server_bindings {
                server_bindings.insert(name, bindings);
            }
        }
        self
    }

    /// Add a component operation trait
    #[must_use]
    pub fn component_operation_trait(
        mut self,
        name: String,
        trait_: crate::spec::OperationTrait,
    ) -> Self {
        if self.spec.components.is_none() {
            self.spec.components = Some(Components::default());
        }
        if let Some(ref mut components) = self.spec.components {
            if components.operation_traits.is_none() {
                components.operation_traits = Some(Default::default());
            }
            if let Some(ref mut operation_traits) = components.operation_traits {
                operation_traits.insert(name, trait_);
            }
        }
        self
    }

    /// Add a component message trait
    #[must_use]
    pub fn component_message_trait(
        mut self,
        name: String,
        trait_: crate::spec::MessageTrait,
    ) -> Self {
        if self.spec.components.is_none() {
            self.spec.components = Some(Components::default());
        }
        if let Some(ref mut components) = self.spec.components {
            if components.message_traits.is_none() {
                components.message_traits = Some(Default::default());
            }
            if let Some(ref mut message_traits) = components.message_traits {
                message_traits.insert(name, trait_);
            }
        }
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
