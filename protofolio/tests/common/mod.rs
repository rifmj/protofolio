//! Common test utilities and helpers

use protofolio::{AsyncApiBuilder, Channel, Info, Message, MessagePayload, Server};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Create a basic valid AsyncAPI spec for testing
pub fn create_test_spec() -> protofolio::AsyncApiSpec {
    AsyncApiBuilder::new()
        .info(Info {
            title: "Test API".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test API description".to_string()),
            external_docs: None,
            contact: None,
            license: None,
            terms_of_service: None,
        })
        .server(
            "nats".to_string(),
            Server {
                url: "nats://localhost:4222".to_string(),
                protocol: "nats".to_string(),
                description: None,
            },
        )
        .channel(
            "test.channel".to_string(),
            Channel {
                description: Some("Test channel".to_string()),
                messages: {
                    let mut m = HashMap::new();
                    m.insert(
                        "TestMessage".to_string(),
                        Message {
                            message_id: None,
                            name: Some("TestMessage".to_string()),
                            title: None,
                            summary: Some("Test message".to_string()),
                            description: None,
                            content_type: Some("application/json".to_string()),
                            tags: None,
                            payload: MessagePayload {
                                schema: serde_json::json!({
                                    "type": "object",
                                    "properties": {
                                        "id": {"type": "string"},
                                        "value": {"type": "string"}
                                    }
                                }),
                            },
                            external_docs: None,
                            examples: None,
                            headers: None,
                            correlation_id: None,
                            traits: None,
                            bindings: None,
                        },
                    );
                    m
                },
                servers: Some(vec!["nats".to_string()]),
                parameters: None,
                bindings: None,
            },
        )
        .build()
}

/// Test message type
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct TestMessage {
    pub id: String,
    pub value: String,
}

