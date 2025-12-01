//! Integration tests for protofolio
//!
//! These tests verify the full macro-generated code paths and serialization.

use protofolio::{AsyncApi, AsyncApiOperation, Tag, validate_spec};
use protofolio_derive::{AsyncApi, AsyncApiMessage, AsyncApiOperation};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Test message types
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "test.channel",
    messageId = "test-message-v1",
    name = "TestMessage",
    title = "Test Message",
    summary = "A test message",
    description = "This is a test message for integration testing",
    contentType = "application/json",
    tags = ["test", "integration"]
)]
pub struct TestMessage {
    pub id: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "simple.channel",
    summary = "Simple message"
)]
pub struct SimpleMessage {
    pub text: String,
}

// Test AsyncAPI specification
#[derive(AsyncApi)]
#[asyncapi(
    info(
        title = "Test AsyncAPI",
        version = "1.0.0",
        description = "Test AsyncAPI specification for integration tests"
    ),
    servers(
        (name = "nats", url = "nats://localhost:4222", protocol = "nats")
    ),
    channels("test.channel", "simple.channel"),
    messages(TestMessage, SimpleMessage)
)]
pub struct TestAsyncApi;

#[test]
fn test_asyncapi_derive() {
    let spec = TestAsyncApi::asyncapi();
    assert_eq!(spec.asyncapi, "3.0.0");
    assert_eq!(spec.info.title, "Test AsyncAPI");
    assert_eq!(spec.info.version, "1.0.0");
}

#[test]
fn test_asyncapi_has_channels() {
    let spec = TestAsyncApi::asyncapi();
    assert!(spec.channels.contains_key("test.channel"));
    assert!(spec.channels.contains_key("simple.channel"));
}

#[test]
fn test_asyncapi_has_servers() {
    let spec = TestAsyncApi::asyncapi();
    assert!(spec.servers.is_some());
    let servers = spec.servers.as_ref().unwrap();
    assert!(servers.contains_key("nats"));
    assert_eq!(servers["nats"].url, "nats://localhost:4222");
}

#[test]
fn test_message_attributes() {
    let spec = TestAsyncApi::asyncapi();
    let channel = spec.channels.get("test.channel").unwrap();
    let message = channel.messages.get("TestMessage").unwrap();

    assert_eq!(message.message_id, Some("test-message-v1".to_string()));
    assert_eq!(message.name, Some("TestMessage".to_string()));
    assert_eq!(message.title, Some("Test Message".to_string()));
    assert_eq!(message.summary, Some("A test message".to_string()));
    assert_eq!(
        message.description,
        Some("This is a test message for integration testing".to_string())
    );
    assert_eq!(
        message.content_type,
        Some("application/json".to_string())
    );
    assert!(message.tags.is_some());
    let tags = message.tags.as_ref().unwrap();
    assert_eq!(tags.len(), 2);
    assert!(tags.contains(&Tag {
        name: "test".to_string(),
        description: None,
    }));
}

#[test]
fn test_message_schema_generation() {
    let spec = TestAsyncApi::asyncapi();
    let channel = spec.channels.get("test.channel").unwrap();
    let message = channel.messages.get("TestMessage").unwrap();

    // Verify schema was generated
    assert!(!message.payload.schema.is_null());
    let schema = &message.payload.schema;
    assert_eq!(schema["type"], "object");
    assert!(schema["properties"].is_object());
}

#[test]
fn test_asyncapi_json_serialization() {
    let json = TestAsyncApi::asyncapi_json().unwrap();
    assert!(!json.is_empty());
    assert!(json.contains("Test AsyncAPI"));
    assert!(json.contains("3.0.0"));
    
    // Verify it's valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["asyncapi"], "3.0.0");
}

#[test]
fn test_asyncapi_yaml_serialization() {
    let yaml = TestAsyncApi::asyncapi_yaml().unwrap();
    assert!(!yaml.is_empty());
    assert!(yaml.contains("Test AsyncAPI"));
    assert!(yaml.contains("3.0.0"));
    
    // Verify it's valid YAML
    let parsed: serde_yaml_ng::Value = serde_yaml_ng::from_str(&yaml).unwrap();
    assert_eq!(parsed["asyncapi"].as_str().unwrap(), "3.0.0");
}

#[test]
fn test_validate_spec() {
    let spec = TestAsyncApi::asyncapi();
    assert!(validate_spec(&spec).is_ok());
}

#[test]
fn test_simple_message() {
    let spec = TestAsyncApi::asyncapi();
    let channel = spec.channels.get("simple.channel").unwrap();
    let message = channel.messages.get("SimpleMessage").unwrap();

    assert_eq!(message.summary, Some("Simple message".to_string()));
    // Optional fields should be None
    assert!(message.message_id.is_none());
    assert!(message.name.is_none());
    assert!(message.title.is_none());
}

#[test]
fn test_message_channel_methods() {
    assert_eq!(TestMessage::channel(), "test.channel");
    assert_eq!(TestMessage::summary(), Some("A test message"));
    assert_eq!(TestMessage::message_id(), Some("test-message-v1"));
    assert_eq!(TestMessage::name(), Some("TestMessage"));
    assert_eq!(TestMessage::title(), Some("Test Message"));
    assert_eq!(TestMessage::content_type(), Some("application/json"));
    
    let tags = TestMessage::tags().unwrap();
    assert_eq!(tags.len(), 2);
}

#[test]
fn test_helper_functions() {
    let spec = TestAsyncApi::asyncapi();
    
    // Test to_json helper
    let json = protofolio::to_json(&spec).unwrap();
    assert!(!json.is_empty());
    
    // Test to_yaml helper
    let yaml = protofolio::to_yaml(&spec).unwrap();
    assert!(!yaml.is_empty());
}

// Test operation types
#[derive(AsyncApiOperation)]
#[asyncapi(
    id = "publish-test-message",
    action = "send",
    channel = "test.channel",
    messages(TestMessage),
    summary = "Publish test message",
    description = "Publishes a test message to the test channel",
    tags = ["test", "publish"]
)]
pub struct PublishTestMessage;

#[derive(AsyncApiOperation)]
#[asyncapi(
    id = "subscribe-simple-message",
    action = "receive",
    channel = "simple.channel",
    messages(SimpleMessage),
    summary = "Subscribe to simple messages"
)]
pub struct SubscribeSimpleMessage;

// Test AsyncAPI specification with operations
#[derive(AsyncApi)]
#[asyncapi(
    info(
        title = "Test AsyncAPI with Operations",
        version = "1.0.0",
        description = "Test AsyncAPI specification with operations"
    ),
    servers(
        (name = "nats", url = "nats://localhost:4222", protocol = "nats")
    ),
    channels("test.channel", "simple.channel"),
    messages(TestMessage, SimpleMessage),
    operations(PublishTestMessage, SubscribeSimpleMessage)
)]
pub struct TestAsyncApiWithOperations;

#[test]
fn test_operation_derive() {
    assert_eq!(PublishTestMessage::operation_id(), "publish-test-message");
    assert_eq!(PublishTestMessage::action(), "send");
    assert_eq!(PublishTestMessage::channel(), "test.channel");
    assert_eq!(PublishTestMessage::summary(), Some("Publish test message"));
    assert_eq!(
        PublishTestMessage::description(),
        Some("Publishes a test message to the test channel")
    );
    
    let tags = PublishTestMessage::tags().unwrap();
    assert_eq!(tags.len(), 2);
    assert!(tags.contains(&Tag {
        name: "test".to_string(),
        description: None,
    }));
}

#[test]
fn test_operation_message_types() {
    let message_types = PublishTestMessage::message_types();
    assert_eq!(message_types.len(), 1);
    assert_eq!(message_types[0], "TestMessage");
    
    let message_names = PublishTestMessage::message_names();
    assert_eq!(message_names.len(), 1);
    assert_eq!(message_names[0], "TestMessage");
}

#[test]
fn test_operation_to_operation() {
    let operation = PublishTestMessage::to_operation();
    assert_eq!(operation.action, "send");
    assert_eq!(operation.channel.ref_path, "#/channels/test.channel");
    assert_eq!(operation.messages.len(), 1);
    assert_eq!(operation.messages[0].ref_path, "#/channels/test.channel/messages/TestMessage");
    assert_eq!(operation.summary, Some("Publish test message".to_string()));
    assert!(operation.tags.is_some());
}

#[test]
fn test_asyncapi_with_operations() {
    let spec = TestAsyncApiWithOperations::asyncapi();
    assert!(spec.operations.is_some());
    let operations = spec.operations.as_ref().unwrap();
    
    assert!(operations.contains_key("publish-test-message"));
    assert!(operations.contains_key("subscribe-simple-message"));
    
    let publish_op = operations.get("publish-test-message").unwrap();
    assert_eq!(publish_op.action, "send");
    assert_eq!(publish_op.channel.ref_path, "#/channels/test.channel");
    
    let subscribe_op = operations.get("subscribe-simple-message").unwrap();
    assert_eq!(subscribe_op.action, "receive");
    assert_eq!(subscribe_op.channel.ref_path, "#/channels/simple.channel");
}

#[test]
fn test_operations_serialization() {
    let spec = TestAsyncApiWithOperations::asyncapi();
    let json = protofolio::to_json(&spec).unwrap();
    
    // Verify operations are in JSON
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(parsed["operations"].is_object());
    let ops = parsed["operations"].as_object().unwrap();
    assert!(ops.contains_key("publish-test-message"));
    assert!(ops.contains_key("subscribe-simple-message"));
}

#[test]
fn test_operation_with_tags() {
    let operation = PublishTestMessage::to_operation();
    assert!(operation.tags.is_some());
    let tags = operation.tags.as_ref().unwrap();
    assert_eq!(tags.len(), 2);
    assert!(tags.iter().any(|t| t.name == "test"));
    assert!(tags.iter().any(|t| t.name == "publish"));
}

#[test]
fn test_operation_without_optional_fields() {
    let operation = SubscribeSimpleMessage::to_operation();
    assert_eq!(operation.action, "receive");
    assert_eq!(operation.summary, Some("Subscribe to simple messages".to_string()));
    assert!(operation.description.is_none());
    assert!(operation.tags.is_none());
}

// Tests for try_asyncapi() method

#[test]
fn test_try_asyncapi_success() {
    let result = TestAsyncApi::try_asyncapi();
    assert!(result.is_ok());
    
    let spec = result.unwrap();
    assert_eq!(spec.asyncapi, "3.0.0");
    assert_eq!(spec.info.title, "Test AsyncAPI");
    assert_eq!(spec.info.version, "1.0.0");
}

#[test]
fn test_try_asyncapi_matches_asyncapi() {
    // Verify try_asyncapi() produces same result as asyncapi() for valid specs
    let spec1 = TestAsyncApi::asyncapi();
    let spec2 = TestAsyncApi::try_asyncapi().unwrap();
    
    // Compare key fields
    assert_eq!(spec1.asyncapi, spec2.asyncapi);
    assert_eq!(spec1.info.title, spec2.info.title);
    assert_eq!(spec1.info.version, spec2.info.version);
    assert_eq!(spec1.channels.len(), spec2.channels.len());
    // Compare channel keys (order may differ in HashMap)
    let mut keys1: Vec<_> = spec1.channels.keys().collect();
    let mut keys2: Vec<_> = spec2.channels.keys().collect();
    keys1.sort();
    keys2.sort();
    assert_eq!(keys1, keys2);
}

#[test]
fn test_try_asyncapi_with_operations() {
    let result = TestAsyncApiWithOperations::try_asyncapi();
    assert!(result.is_ok());
    
    let spec = result.unwrap();
    assert!(spec.operations.is_some());
    let operations = spec.operations.as_ref().unwrap();
    assert!(operations.contains_key("publish-test-message"));
    assert!(operations.contains_key("subscribe-simple-message"));
}

#[test]
fn test_try_asyncapi_serialization_consistency() {
    // Test that try_asyncapi() results can be serialized the same way
    let spec1 = TestAsyncApi::asyncapi();
    let spec2 = TestAsyncApi::try_asyncapi().unwrap();
    
    let json1 = protofolio::to_json(&spec1).unwrap();
    let json2 = protofolio::to_json(&spec2).unwrap();
    
    // Parse and compare
    let parsed1: serde_json::Value = serde_json::from_str(&json1).unwrap();
    let parsed2: serde_json::Value = serde_json::from_str(&json2).unwrap();
    
    assert_eq!(parsed1["asyncapi"], parsed2["asyncapi"]);
    assert_eq!(parsed1["info"]["title"], parsed2["info"]["title"]);
    assert_eq!(parsed1["info"]["version"], parsed2["info"]["version"]);
}

#[test]
fn test_try_asyncapi_yaml_consistency() {
    // Test that try_asyncapi() works with YAML serialization
    let spec = TestAsyncApi::try_asyncapi().unwrap();
    let yaml = protofolio::to_yaml(&spec).unwrap();
    
    assert!(!yaml.is_empty());
    assert!(yaml.contains("Test AsyncAPI"));
    assert!(yaml.contains("3.0.0"));
    
    // Verify it's valid YAML
    let parsed: serde_yaml_ng::Value = serde_yaml_ng::from_str(&yaml).unwrap();
    assert_eq!(parsed["asyncapi"].as_str().unwrap(), "3.0.0");
}

#[test]
fn test_try_asyncapi_error_recovery() {
    // Test that we can handle errors gracefully
    let result = TestAsyncApi::try_asyncapi();
    
    match result {
        Ok(spec) => {
            // Success case - verify spec is valid
            assert_eq!(spec.info.title, "Test AsyncAPI");
            let validation_result = protofolio::validate_spec(&spec);
            assert!(validation_result.is_ok(), "Generated spec should be valid");
        }
        Err(e) => {
            // Error case - verify error is informative
            let error_msg = format!("{}", e);
            assert!(!error_msg.is_empty());
            panic!("try_asyncapi() should succeed for valid TestAsyncApi, got error: {}", error_msg);
        }
    }
}

