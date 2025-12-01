//! Tests for error handling in try_asyncapi()

use protofolio::{AsyncApi, AsyncApiBuilder, Info, Channel, Message, MessagePayload, ValidationError};
use protofolio_derive::{AsyncApi, AsyncApiMessage};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "test.channel",
    messageId = "test-message-v1",
    name = "TestMessage"
)]
pub struct TestMessage {
    pub id: String,
    pub value: String,
}

#[derive(AsyncApi)]
#[asyncapi(
    info(title = "Test API", version = "1.0.0"),
    channels("test.channel"),
    messages(TestMessage)
)]
pub struct TestAsyncApi;

#[test]
fn test_try_asyncapi_success() {
    // Test that try_asyncapi() works for valid specs
    let result = TestAsyncApi::try_asyncapi();
    assert!(result.is_ok());
    
    let spec = result.unwrap();
    assert_eq!(spec.info.title, "Test API");
    assert_eq!(spec.info.version, "1.0.0");
    assert!(spec.channels.contains_key("test.channel"));
}

#[test]
fn test_try_asyncapi_matches_asyncapi() {
    // Test that try_asyncapi() produces the same result as asyncapi() for valid specs
    let spec1 = TestAsyncApi::asyncapi();
    let spec2 = TestAsyncApi::try_asyncapi().unwrap();
    
    // Compare key fields
    assert_eq!(spec1.info.title, spec2.info.title);
    assert_eq!(spec1.info.version, spec2.info.version);
    assert_eq!(spec1.asyncapi, spec2.asyncapi);
    assert_eq!(spec1.channels.len(), spec2.channels.len());
}

#[test]
fn test_validation_error_on_invalid_spec() {
    // Create an invalid spec manually (empty channels)
    let invalid_spec = AsyncApiBuilder::new()
        .info(Info {
            title: "Test".to_string(),
            version: "1.0.0".to_string(),
            description: None,
        })
        .build();
    
    // Validate should fail
    let result = protofolio::validate_spec(&invalid_spec);
    assert!(result.is_err());
    
    if let Err(ValidationError::EmptyChannels) = result {
        // Expected error
    } else {
        panic!("Expected EmptyChannels error, got: {:?}", result);
    }
}

#[test]
fn test_validation_error_channel_without_messages() {
    // Create a spec with a channel that has no messages
    let invalid_spec = AsyncApiBuilder::new()
        .info(Info {
            title: "Test".to_string(),
            version: "1.0.0".to_string(),
            description: None,
        })
        .channel(
            "empty.channel".to_string(),
            Channel {
                description: None,
                messages: HashMap::new(),
                servers: None,
                parameters: None,
                bindings: None,
            },
        )
        .build();
    
    let result = protofolio::validate_spec(&invalid_spec);
    assert!(result.is_err());
    
    if let Err(ValidationError::ChannelWithoutMessages(_)) = result {
        // Expected error
    } else {
        panic!("Expected ChannelWithoutMessages error, got: {:?}", result);
    }
}

#[test]
fn test_validation_error_duplicate_message_id() {
    // Create a spec with duplicate message IDs
    let mut messages1 = HashMap::new();
    messages1.insert(
        "Message1".to_string(),
        Message {
            message_id: Some("duplicate-id".to_string()),
            name: None,
            title: None,
            summary: None,
            description: None,
            content_type: None,
            tags: None,
            payload: MessagePayload {
                schema: serde_json::json!({"type": "object"}),
            },
        },
    );
    
    let mut messages2 = HashMap::new();
    messages2.insert(
        "Message2".to_string(),
        Message {
            message_id: Some("duplicate-id".to_string()),
            name: None,
            title: None,
            summary: None,
            description: None,
            content_type: None,
            tags: None,
            payload: MessagePayload {
                schema: serde_json::json!({"type": "object"}),
            },
        },
    );
    
    let invalid_spec = AsyncApiBuilder::new()
        .info(Info {
            title: "Test".to_string(),
            version: "1.0.0".to_string(),
            description: None,
        })
        .channel(
            "channel1".to_string(),
            Channel {
                description: None,
                messages: messages1,
                servers: None,
                parameters: None,
                bindings: None,
            },
        )
        .channel(
            "channel2".to_string(),
            Channel {
                description: None,
                messages: messages2,
                servers: None,
                parameters: None,
                bindings: None,
            },
        )
        .build();
    
    let result = protofolio::validate_spec(&invalid_spec);
    assert!(result.is_err());
    
    if let Err(ValidationError::DuplicateMessageId(_)) = result {
        // Expected error
    } else {
        panic!("Expected DuplicateMessageId error, got: {:?}", result);
    }
}

#[test]
fn test_error_message_quality() {
    // Test that error messages are informative
    let invalid_spec = AsyncApiBuilder::new()
        .info(Info {
            title: "Test".to_string(),
            version: "1.0.0".to_string(),
            description: None,
        })
        .build();
    
    let result = protofolio::validate_spec(&invalid_spec);
    assert!(result.is_err());
    
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("Empty channels"));
    assert!(error_msg.contains("specification must have at least one channel"));
}

