//! AsyncAPI specification validator implementation

use crate::error::ValidationError;
use crate::protocol;
use crate::spec::*;
use crate::types::ASYNCAPI_VERSION;

use super::bindings::{get_channel_protocol, validate_channel_bindings};

/// Validate an AsyncAPI specification
///
/// Returns `Ok(())` if the specification is valid, or an error describing
/// what validation failed.
///
/// # Example
///
/// ```rust,no_run
/// use protofolio::{AsyncApi, validate_spec, ValidationError};
/// # use protofolio_derive::AsyncApi;
/// #
/// # #[derive(AsyncApi)]
/// # #[asyncapi(info(title = "Test", version = "1.0.0"), channels("events"), messages())]
/// # struct MyApi;
///
/// let spec = MyApi::asyncapi();
/// match validate_spec(&spec) {
///     Ok(()) => println!("Spec is valid!"),
///     Err(ValidationError::InvalidChannelReference(channel)) => {
///         eprintln!("Invalid channel reference: {}", channel);
///     }
///     Err(ValidationError::DuplicateMessageId(id)) => {
///         eprintln!("Duplicate message ID: {}", id);
///     }
///     Err(e) => eprintln!("Validation error: {}", e),
/// }
/// ```
pub fn validate_spec(spec: &AsyncApiSpec) -> Result<(), ValidationError> {
    // Validate AsyncAPI version
    if spec.asyncapi != ASYNCAPI_VERSION {
        return Err(ValidationError::InvalidAsyncApiVersion(
            spec.asyncapi.clone(),
        ));
    }

    // Validate info section
    if spec.info.title.is_empty() {
        return Err(ValidationError::MissingRequiredField(
            "info.title".to_string(),
        ));
    }
    if spec.info.version.is_empty() {
        return Err(ValidationError::MissingRequiredField(
            "info.version".to_string(),
        ));
    }

    // Validate channels
    if spec.channels.is_empty() {
        return Err(ValidationError::EmptyChannels);
    }

    // Collect server names if servers are defined
    let server_names: std::collections::HashSet<String> = spec
        .servers
        .as_ref()
        .map(|servers| servers.keys().cloned().collect())
        .unwrap_or_default();

    // Track message IDs for duplicate detection
    let mut message_ids = std::collections::HashSet::new();

    // Validate each channel
    for (channel_name, channel) in &spec.channels {
        // Check if channel has messages
        if channel.messages.is_empty() {
            return Err(ValidationError::ChannelWithoutMessages(
                channel_name.clone(),
            ));
        }

        // Validate server references in channel
        if let Some(ref channel_servers) = channel.servers {
            for server_name in channel_servers {
                if !server_names.contains(server_name) {
                    let available: Vec<_> = server_names.iter().collect();
                    let suggestion = if available.is_empty() {
                        format!("No servers defined. Add servers(...) to your #[asyncapi] attribute. Referenced server: '{}'", server_name)
                    } else {
                        format!("Server '{}' not found. Available servers: {:?}. Update your channel's server reference or add the server in servers(...)", server_name, available)
                    };
                    return Err(ValidationError::InvalidServerReference(format!(
                        "{}: {}",
                        server_name, suggestion
                    )));
                }
            }
        }

        // Validate messages in channel
        for (message_name, message_or_ref) in &channel.messages {
            match message_or_ref {
                crate::spec::MessageOrRef::Message(message) => {
                    // Basic message validation - ensure payload schema exists
                    if message.payload.schema.is_null() {
                        return Err(ValidationError::InvalidSchema(format!(
                            "Message '{}' in channel '{}' has null schema",
                            message_name, channel_name
                        )));
                    }

                    // Check for duplicate message IDs
                    if let Some(ref msg_id) = message.message_id {
                        if !message_ids.insert(msg_id.clone()) {
                            return Err(ValidationError::DuplicateMessageId(format!(
                                "Message ID '{}' is used by multiple messages. Each message must have a unique messageId. Found in channel '{}', message '{}'",
                                msg_id, channel_name, message_name
                            )));
                        }
                    }

                    // Also check message IDs in component messages if this message references one
                    // (This will be handled when we validate components)
                }
                crate::spec::MessageOrRef::Ref(msg_ref) => {
                    // Validate component reference exists
                    if msg_ref.ref_path.starts_with("#/components/messages/") {
                        let component_name = msg_ref
                            .ref_path
                            .strip_prefix("#/components/messages/")
                            .unwrap_or("");

                        // Check if component exists
                        if let Some(ref components) = spec.components {
                            if let Some(ref messages) = components.messages {
                                if !messages.contains_key(component_name) {
                                    return Err(ValidationError::InvalidSchema(format!(
                                        "Message '{}' in channel '{}' references component '{}' which does not exist in components.messages",
                                        message_name, channel_name, component_name
                                    )));
                                }
                            } else {
                                return Err(ValidationError::InvalidSchema(format!(
                                    "Message '{}' in channel '{}' references component '{}' but no components.messages are defined",
                                    message_name, channel_name, component_name
                                )));
                            }
                        } else {
                            return Err(ValidationError::InvalidSchema(format!(
                                "Message '{}' in channel '{}' references component '{}' but no components section is defined",
                                message_name, channel_name, component_name
                            )));
                        }
                    } else if msg_ref.ref_path.starts_with("#/channels/") {
                        // Channel message reference - validate it exists
                        // Extract channel and message name from path like "#/channels/{channel}/messages/{message}"
                        let path_parts: Vec<&str> = msg_ref
                            .ref_path
                            .strip_prefix("#/channels/")
                            .unwrap_or("")
                            .split("/messages/")
                            .collect();

                        if path_parts.len() == 2 {
                            let ref_channel = path_parts[0];
                            let ref_message = path_parts[1];

                            if let Some(ref_channel_obj) = spec.channels.get(ref_channel) {
                                if !ref_channel_obj.messages.contains_key(ref_message) {
                                    return Err(ValidationError::InvalidSchema(format!(
                                        "Message '{}' in channel '{}' references message '{}' in channel '{}' which does not exist",
                                        message_name, channel_name, ref_message, ref_channel
                                    )));
                                }
                            } else {
                                return Err(ValidationError::InvalidSchema(format!(
                                    "Message '{}' in channel '{}' references channel '{}' which does not exist",
                                    message_name, channel_name, ref_channel
                                )));
                            }
                        }
                    } else {
                        return Err(ValidationError::InvalidSchema(format!(
                            "Invalid message reference format in channel '{}', message '{}': {}. Expected '#/components/messages/...' or '#/channels/.../messages/...'",
                            channel_name, message_name, msg_ref.ref_path
                        )));
                    }
                }
            }
        }
    }

    // Validate operations if present
    if let Some(ref operations) = spec.operations {
        for (op_id, op) in operations {
            // Validate channel reference format
            if !op.channel.ref_path.starts_with("#/channels/") {
                return Err(ValidationError::InvalidChannelReference(
                    op.channel.ref_path.clone(),
                ));
            }

            // Validate message references
            for msg_ref in &op.messages {
                // Message references can point to:
                // - Channel messages: "#/channels/{channel}/messages/{message}"
                // - Component messages: "#/components/messages/{message}"
                if !msg_ref.ref_path.starts_with("#/channels/")
                    && !msg_ref.ref_path.starts_with("#/components/messages/")
                {
                    return Err(ValidationError::InvalidSchema(format!(
                        "Invalid message reference format in operation '{}': {}. Expected '#/channels/.../messages/...' or '#/components/messages/...'",
                        op_id, msg_ref.ref_path
                    )));
                }

                // If it's a component reference, validate it exists
                if msg_ref.ref_path.starts_with("#/components/messages/") {
                    let component_name = msg_ref
                        .ref_path
                        .strip_prefix("#/components/messages/")
                        .unwrap_or("");

                    if let Some(ref components) = spec.components {
                        if let Some(ref messages) = components.messages {
                            if !messages.contains_key(component_name) {
                                return Err(ValidationError::InvalidSchema(format!(
                                    "Operation '{}' references component message '{}' which does not exist in components.messages",
                                    op_id, component_name
                                )));
                            }
                        } else {
                            return Err(ValidationError::InvalidSchema(format!(
                                "Operation '{}' references component message '{}' but no components.messages are defined",
                                op_id, component_name
                            )));
                        }
                    } else {
                        return Err(ValidationError::InvalidSchema(format!(
                            "Operation '{}' references component message '{}' but no components section is defined",
                            op_id, component_name
                        )));
                    }
                }
            }
        }
    }

    // Validate protocol identifiers
    if let Some(ref servers) = spec.servers {
        for (server_name, server) in servers {
            protocol::validate_protocol(&server.protocol).map_err(|e| match e {
                ValidationError::UnsupportedProtocol {
                    protocol,
                    supported,
                } => ValidationError::InvalidProtocol(format!(
                    "Server '{}' uses unsupported protocol '{}'. Supported protocols: {:?}",
                    server_name, protocol, supported
                )),
                _ => e,
            })?;
        }
    }

    // Validate protocol-specific bindings
    for (channel_name, channel) in &spec.channels {
        if let Some(ref bindings) = channel.bindings {
            // Validate bindings structure matches protocol
            if let Some(protocol) = get_channel_protocol(channel, spec) {
                validate_channel_bindings(&protocol, bindings, channel_name)?;
            }
        }
    }

    // Validate component messages (check for duplicate message IDs across components)
    if let Some(ref components) = spec.components {
        if let Some(ref messages) = components.messages {
            for (component_name, message) in messages {
                // Validate component message has valid schema
                if message.payload.schema.is_null() {
                    return Err(ValidationError::InvalidSchema(format!(
                        "Component message '{}' has null schema",
                        component_name
                    )));
                }

                // Check for duplicate message IDs in components
                if let Some(ref msg_id) = message.message_id {
                    if !message_ids.insert(msg_id.clone()) {
                        return Err(ValidationError::DuplicateMessageId(format!(
                            "Message ID '{}' is used by multiple messages. Found in component message '{}'",
                            msg_id, component_name
                        )));
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::AsyncApiBuilder;
    use std::collections::HashMap;

    #[test]
    fn test_validate_valid_spec() {
        let spec = AsyncApiBuilder::new()
            .info(Info {
                title: "Test API".to_string(),
                version: "1.0.0".to_string(),
                description: None,
                external_docs: None,
            })
            .channel(
                "test.channel".to_string(),
                Channel {
                    address: "test.channel".to_string(),
                    description: None,
                    messages: {
                        use crate::spec::MessageOrRef;
                        let mut m = HashMap::new();
                        m.insert(
                            "TestMessage".to_string(),
                            MessageOrRef::Message(Message {
                                message_id: None,
                                name: None,
                                title: None,
                                summary: None,
                                description: None,
                                external_docs: None,
                                content_type: None,
                                tags: None,
                                payload: MessagePayload {
                                    schema: serde_json::json!({"type": "object"}),
                                },
                                examples: None,
                                headers: None,
                                correlation_id: None,
                            }),
                        );
                        m
                    },
                    servers: None,
                    parameters: None,
                    bindings: None,
                },
            )
            .build();

        assert!(validate_spec(&spec).is_ok());
    }

    #[test]
    fn test_validate_missing_title() {
        let spec = AsyncApiBuilder::new()
            .info(Info {
                title: String::new(),
                version: "1.0.0".to_string(),
                description: None,
                external_docs: None,
            })
            .build();

        assert!(matches!(
            validate_spec(&spec),
            Err(ValidationError::MissingRequiredField(_))
        ));
    }

    #[test]
    fn test_validate_empty_channels() {
        let spec = AsyncApiBuilder::new()
            .info(Info {
                title: "Test".to_string(),
                version: "1.0.0".to_string(),
                description: None,
                external_docs: None,
            })
            .build();

        assert!(matches!(
            validate_spec(&spec),
            Err(ValidationError::EmptyChannels)
        ));
    }

    #[test]
    fn test_validate_invalid_server_reference() {
        let spec = AsyncApiBuilder::new()
            .info(Info {
                title: "Test".to_string(),
                version: "1.0.0".to_string(),
                description: None,
                external_docs: None,
            })
            .channel(
                "test.channel".to_string(),
                Channel {
                    address: "test.channel".to_string(),
                    description: None,
                    messages: {
                        use crate::spec::MessageOrRef;
                        let mut m = HashMap::new();
                        m.insert(
                            "TestMessage".to_string(),
                            MessageOrRef::Message(Message {
                                message_id: None,
                                name: None,
                                title: None,
                                summary: None,
                                description: None,
                                external_docs: None,
                                content_type: None,
                                tags: None,
                                payload: MessagePayload {
                                    schema: serde_json::json!({"type": "object"}),
                                },
                                examples: None,
                                headers: None,
                                correlation_id: None,
                            }),
                        );
                        m
                    },
                    servers: Some(vec!["nonexistent".to_string()]),
                    parameters: None,
                    bindings: None,
                },
            )
            .build();

        assert!(matches!(
            validate_spec(&spec),
            Err(ValidationError::InvalidServerReference(_))
        ));
    }

    #[test]
    fn test_validate_channel_without_messages() {
        let spec = AsyncApiBuilder::new()
            .info(Info {
                title: "Test".to_string(),
                version: "1.0.0".to_string(),
                description: None,
                external_docs: None,
            })
            .channel(
                "test.channel".to_string(),
                Channel {
                    address: "empty.channel".to_string(),
                    description: None,
                    messages: HashMap::new(),
                    servers: None,
                    parameters: None,
                    bindings: None,
                },
            )
            .build();

        assert!(matches!(
            validate_spec(&spec),
            Err(ValidationError::ChannelWithoutMessages(_))
        ));
    }

    #[test]
    fn test_validate_duplicate_message_ids() {
        let spec = AsyncApiBuilder::new()
            .info(Info {
                title: "Test".to_string(),
                version: "1.0.0".to_string(),
                description: None,
                external_docs: None,
            })
            .channel(
                "test.channel".to_string(),
                Channel {
                    description: None,
                    messages: {
                        use crate::spec::MessageOrRef;
                        let mut m = HashMap::new();
                        m.insert(
                            "Message1".to_string(),
                            MessageOrRef::Message(Message {
                                message_id: Some("duplicate-id".to_string()),
                                name: None,
                                title: None,
                                summary: None,
                                description: None,
                                content_type: None,
                                tags: None,
                                external_docs: None,
                                payload: MessagePayload {
                                    schema: serde_json::json!({"type": "object"}),
                                },
                                examples: None,
                                headers: None,
                                correlation_id: None,
                            }),
                        );
                        m.insert(
                            "Message2".to_string(),
                            MessageOrRef::Message(Message {
                                message_id: Some("duplicate-id".to_string()),
                                name: None,
                                title: None,
                                summary: None,
                                description: None,
                                content_type: None,
                                tags: None,
                                external_docs: None,
                                payload: MessagePayload {
                                    schema: serde_json::json!({"type": "object"}),
                                },
                                examples: None,
                                headers: None,
                                correlation_id: None,
                            }),
                        );
                        m
                    },
                    servers: None,
                    parameters: None,
                    bindings: None,
                },
            )
            .build();

        assert!(matches!(
            validate_spec(&spec),
            Err(ValidationError::DuplicateMessageId(_))
        ));
    }

    #[test]
    fn test_validate_component_message_ref() {
        use crate::spec::{MessageOrRef, MessageReference};

        let spec = AsyncApiBuilder::new()
            .info(Info {
                title: "Test".to_string(),
                version: "1.0.0".to_string(),
                description: None,
                external_docs: None,
            })
            .component_message(
                "CommonMessage".to_string(),
                Message {
                    message_id: Some("common-v1".to_string()),
                    name: Some("CommonMessage".to_string()),
                    title: None,
                    summary: None,
                    description: None,
                    content_type: None,
                    tags: None,
                    external_docs: None,
                    payload: MessagePayload {
                        schema: serde_json::json!({"type": "object"}),
                    },
                    examples: None,
                    headers: None,
                },
            )
            .channel(
                "test.channel".to_string(),
                Channel {
                    address: "common.channel".to_string(),
                    description: None,
                    messages: {
                        let mut m = HashMap::new();
                        m.insert(
                            "CommonMessage".to_string(),
                            MessageOrRef::component_ref("CommonMessage"),
                        );
                        m
                    },
                    servers: None,
                    parameters: None,
                    bindings: None,
                },
            )
            .build();

        assert!(validate_spec(&spec).is_ok());
    }

    #[test]
    fn test_validate_invalid_component_ref() {
        use crate::spec::MessageOrRef;

        let spec = AsyncApiBuilder::new()
            .info(Info {
                title: "Test".to_string(),
                version: "1.0.0".to_string(),
                description: None,
                external_docs: None,
            })
            .channel(
                "test.channel".to_string(),
                Channel {
                    address: "ref.channel".to_string(),
                    description: None,
                    messages: {
                        let mut m = HashMap::new();
                        m.insert(
                            "NonExistent".to_string(),
                            MessageOrRef::component_ref("NonExistent"),
                        );
                        m
                    },
                    servers: None,
                    parameters: None,
                    bindings: None,
                },
            )
            .build();

        assert!(matches!(
            validate_spec(&spec),
            Err(ValidationError::InvalidSchema(_))
        ));
    }

    #[test]
    fn test_validate_operation_with_component_ref() {
        use crate::spec::{ChannelReference, MessageOrRef, MessageReference, Operation};
        use std::collections::HashMap;

        let mut spec = AsyncApiBuilder::new()
            .info(Info {
                title: "Test".to_string(),
                version: "1.0.0".to_string(),
                description: None,
                external_docs: None,
            })
            .component_message(
                "ComponentMsg".to_string(),
                Message {
                    message_id: Some("component-v1".to_string()),
                    name: Some("ComponentMsg".to_string()),
                    title: None,
                    summary: None,
                    description: None,
                    content_type: None,
                    tags: None,
                    external_docs: None,
                    payload: MessagePayload {
                        schema: serde_json::json!({"type": "object"}),
                    },
                    examples: None,
                    headers: None,
                    correlation_id: None,
                },
            )
            .channel(
                "test.channel".to_string(),
                Channel {
                    address: "test.channel".to_string(),
                    description: None,
                    messages: {
                        let mut m = HashMap::new();
                        m.insert(
                            "InlineMessage".to_string(),
                            MessageOrRef::Message(Message {
                                message_id: None,
                                name: None,
                                title: None,
                                summary: None,
                                description: None,
                                content_type: None,
                                tags: None,
                                external_docs: None,
                                payload: MessagePayload {
                                    schema: serde_json::json!({"type": "object"}),
                                },
                                examples: None,
                                headers: None,
                                correlation_id: None,
                            }),
                        );
                        m
                    },
                    servers: None,
                    parameters: None,
                    bindings: None,
                },
            )
            .build();

        // Add operation with component reference
        let mut operations = HashMap::new();
        operations.insert(
            "testOp".to_string(),
            Operation {
                operation_id: "test-operation".to_string(),
                action: "send".to_string(),
                channel: ChannelReference {
                    ref_path: "#/channels/test.channel".to_string(),
                },
                messages: vec![MessageReference {
                    ref_path: "#/components/messages/ComponentMsg".to_string(),
                }],
                summary: None,
                description: None,
                tags: None,
                external_docs: None,
            },
        );
        spec.operations = Some(operations);

        assert!(validate_spec(&spec).is_ok());
    }
}
