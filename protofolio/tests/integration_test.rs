//! Integration tests for protofolio
//!
//! These tests verify the full macro-generated code paths and serialization.

use protofolio::{validate_spec, AsyncApi, AsyncApiOperation, Tag};
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
#[asyncapi(channel = "simple.channel", summary = "Simple message")]
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

// Test server with variables
#[derive(AsyncApi)]
#[asyncapi(
    info(
        title = "Test AsyncAPI with Variables",
        version = "1.0.0"
    ),
    servers(
        (
            name = "nats",
            url = "nats://{host}:{port}",
            protocol = "nats",
            variables = [
                (name = "host", default = "localhost", description = "Server hostname"),
                (name = "port", default = "4222", enum_values = ["4222", "4223", "4224"], description = "Server port")
            ]
        )
    ),
    channels("test.channel"),
    messages(TestMessage)
)]
pub struct TestAsyncApiWithVariables;

#[test]
fn test_asyncapi_server_variables() {
    let spec = TestAsyncApiWithVariables::asyncapi();
    assert!(spec.servers.is_some());
    let servers = spec.servers.as_ref().unwrap();
    assert!(servers.contains_key("nats"));
    let server = &servers["nats"];
    assert_eq!(server.url, "nats://{host}:{port}");
    assert!(server.variables.is_some());
    let vars = server.variables.as_ref().unwrap();
    assert!(vars.contains_key("host"));
    assert!(vars.contains_key("port"));
    assert_eq!(vars["host"].default, Some("localhost".to_string()));
    assert_eq!(
        vars["host"].description,
        Some("Server hostname".to_string())
    );
    assert_eq!(vars["port"].default, Some("4222".to_string()));
    assert_eq!(vars["port"].description, Some("Server port".to_string()));
    assert_eq!(
        vars["port"].enum_values,
        Some(vec![
            "4222".to_string(),
            "4223".to_string(),
            "4224".to_string()
        ])
    );
}

#[test]
fn test_message_attributes() {
    let spec = TestAsyncApi::asyncapi();
    let channel = spec.channels.get("test.channel").unwrap();
    let message_or_ref = channel.messages.get("TestMessage").unwrap();
    let message = match message_or_ref {
        protofolio::MessageOrRef::Message(msg) => msg,
        protofolio::MessageOrRef::Ref(_) => panic!("Expected inline message, got reference"),
    };

    assert_eq!(message.message_id, Some("test-message-v1".to_string()));
    assert_eq!(message.name, Some("TestMessage".to_string()));
    assert_eq!(message.title, Some("Test Message".to_string()));
    assert_eq!(message.summary, Some("A test message".to_string()));
    assert_eq!(
        message.description,
        Some("This is a test message for integration testing".to_string())
    );
    assert_eq!(message.content_type, Some("application/json".to_string()));
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
    let message_or_ref = channel.messages.get("TestMessage").unwrap();
    let message = match message_or_ref {
        protofolio::MessageOrRef::Message(msg) => msg,
        protofolio::MessageOrRef::Ref(_) => panic!("Expected inline message, got reference"),
    };

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
    let message_or_ref = channel.messages.get("SimpleMessage").unwrap();
    let message = match message_or_ref {
        protofolio::MessageOrRef::Message(msg) => msg,
        protofolio::MessageOrRef::Ref(_) => panic!("Expected inline message, got reference"),
    };

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
    assert_eq!(
        operation.messages[0].ref_path,
        "#/channels/test.channel/messages/TestMessage"
    );
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
fn test_message_examples() {
    // Test message with single example
    #[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
    #[asyncapi(
        channel = "examples.channel",
        messageId = "example-message-v1",
        example = r#"{"id": "123", "value": "test"}"#
    )]
    pub struct ExampleMessage {
        pub id: String,
        pub value: String,
    }

    // Test message with multiple examples
    #[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
    #[asyncapi(
        channel = "examples.channel",
        messageId = "multi-example-message-v1",
        examples = [r#"{"id": "1", "value": "first"}"#, r#"{"id": "2", "value": "second"}"#]
    )]
    pub struct MultiExampleMessage {
        pub id: String,
        pub value: String,
    }

    // Verify single example
    let examples = ExampleMessage::examples();
    assert!(examples.is_some());
    let examples_vec = examples.unwrap();
    assert_eq!(examples_vec.len(), 1);
    assert_eq!(examples_vec[0]["id"], "123");
    assert_eq!(examples_vec[0]["value"], "test");

    // Verify multiple examples
    let examples = MultiExampleMessage::examples();
    assert!(examples.is_some());
    let examples_vec = examples.unwrap();
    assert_eq!(examples_vec.len(), 2);
    assert_eq!(examples_vec[0]["id"], "1");
    assert_eq!(examples_vec[0]["value"], "first");
    assert_eq!(examples_vec[1]["id"], "2");
    assert_eq!(examples_vec[1]["value"], "second");
}

#[test]
fn test_message_headers() {
    // Define a header type
    #[derive(Serialize, Deserialize, JsonSchema)]
    pub struct MessageHeaders {
        pub correlation_id: String,
        pub user_id: Option<String>,
    }

    // Test message with headers
    #[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
    #[asyncapi(
        channel = "headers.channel",
        messageId = "header-message-v1",
        headers = MessageHeaders
    )]
    pub struct HeaderMessage {
        pub id: String,
        pub data: String,
    }

    // Verify headers schema is generated
    let headers = HeaderMessage::headers();
    assert!(headers.is_some());
    let headers_payload = headers.unwrap();
    // Verify it's a valid JSON schema
    assert!(headers_payload.schema.is_object());
    // Check that the schema contains the expected fields
    if let Some(properties) = headers_payload.schema.get("properties") {
        assert!(properties.get("correlation_id").is_some());
        assert!(properties.get("user_id").is_some());
    }
}

#[test]
fn test_message_with_examples_and_headers_in_spec() {
    // Define a header type
    #[derive(Serialize, Deserialize, JsonSchema)]
    pub struct TestHeaders {
        pub correlation_id: String,
    }

    // Test message with both examples and headers
    #[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
    #[asyncapi(
        channel = "full.channel",
        messageId = "full-message-v1",
        example = r#"{"id": "test", "data": "example"}"#,
        headers = TestHeaders
    )]
    pub struct FullMessage {
        pub id: String,
        pub data: String,
    }

    #[derive(AsyncApi)]
    #[asyncapi(
        info(title = "Full Test API", version = "1.0.0"),
        channels("full.channel"),
        messages(FullMessage)
    )]
    pub struct FullTestApi;

    let spec = FullTestApi::asyncapi();
    let channel = spec.channels.get("full.channel").unwrap();
    let message_or_ref = channel.messages.get("FullMessage").unwrap();
    let message = match message_or_ref {
        protofolio::MessageOrRef::Message(msg) => msg,
        protofolio::MessageOrRef::Ref(_) => panic!("Expected inline message, got reference"),
    };

    // Verify examples are included
    assert!(message.examples.is_some());
    let examples = message.examples.as_ref().unwrap();
    assert_eq!(examples.len(), 1);
    assert_eq!(examples[0]["id"], "test");

    // Verify headers are included
    assert!(message.headers.is_some());
    let headers = message.headers.as_ref().unwrap();
    assert!(headers.schema.is_object());
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
    assert_eq!(
        operation.summary,
        Some("Subscribe to simple messages".to_string())
    );
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
            panic!(
                "try_asyncapi() should succeed for valid TestAsyncApi, got error: {}",
                error_msg
            );
        }
    }
}

#[test]
fn test_correlation_id_in_message() {
    #[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
    #[asyncapi(
        channel = "correlation.channel",
        messageId = "correlation-message-v1",
        correlation_id(
            location = "$message.header#/correlationId",
            description = "Correlation ID for tracking"
        )
    )]
    pub struct CorrelationMessage {
        pub id: String,
        pub data: String,
    }

    #[derive(AsyncApi)]
    #[asyncapi(
        info(title = "Correlation Test API", version = "1.0.0"),
        channels("correlation.channel"),
        messages(CorrelationMessage)
    )]
    pub struct CorrelationTestApi;

    let spec = CorrelationTestApi::asyncapi();
    let channel = spec.channels.get("correlation.channel").unwrap();
    let message_or_ref = channel.messages.get("CorrelationMessage").unwrap();
    let message = match message_or_ref {
        protofolio::MessageOrRef::Message(msg) => msg,
        protofolio::MessageOrRef::Ref(_) => panic!("Expected inline message, got reference"),
    };

    // Verify correlation ID is present
    assert!(message.correlation_id.is_some());
    let corr_id = message.correlation_id.as_ref().unwrap();
    assert_eq!(corr_id.location, "$message.header#/correlationId");
    assert_eq!(
        corr_id.description,
        Some("Correlation ID for tracking".to_string())
    );
}

#[test]
fn test_channel_address_field() {
    let spec = TestAsyncApi::asyncapi();
    let channel = spec.channels.get("test.channel").unwrap();

    // Verify address field is present and equals channel name (default behavior)
    assert_eq!(channel.address, "test.channel");
}

#[test]
fn test_info_contact_license_terms() {
    #[derive(AsyncApi)]
    #[asyncapi(
        info(
            title = "Info Fields Test API",
            version = "1.0.0",
            description = "Test API with contact, license, and terms of service",
            contact(
                name = "API Support",
                email = "support@example.com",
                url = "https://example.com/contact"
            ),
            license(
                name = "Apache 2.0",
                url = "https://www.apache.org/licenses/LICENSE-2.0"
            ),
            terms_of_service = "https://example.com/terms"
        ),
        channels("test.channel"),
        messages(TestMessage)
    )]
    pub struct InfoFieldsTestApi;

    let spec = InfoFieldsTestApi::asyncapi();

    // Verify contact
    assert!(spec.info.contact.is_some());
    let contact = spec.info.contact.as_ref().unwrap();
    assert_eq!(contact.name, Some("API Support".to_string()));
    assert_eq!(contact.email, Some("support@example.com".to_string()));
    assert_eq!(contact.url, Some("https://example.com/contact".to_string()));

    // Verify license
    assert!(spec.info.license.is_some());
    let license = spec.info.license.as_ref().unwrap();
    assert_eq!(license.name, "Apache 2.0");
    assert_eq!(
        license.url,
        Some("https://www.apache.org/licenses/LICENSE-2.0".to_string())
    );

    // Verify terms of service
    assert_eq!(
        spec.info.terms_of_service,
        Some("https://example.com/terms".to_string())
    );
}

#[test]
fn test_info_optional_fields() {
    #[derive(AsyncApi)]
    #[asyncapi(
        info(
            title = "Minimal Info API",
            version = "1.0.0",
            contact(name = "Support")
        ),
        channels("test.channel"),
        messages(TestMessage)
    )]
    pub struct MinimalInfoApi;

    let spec = MinimalInfoApi::asyncapi();

    // Verify contact with only name
    assert!(spec.info.contact.is_some());
    let contact = spec.info.contact.as_ref().unwrap();
    assert_eq!(contact.name, Some("Support".to_string()));
    assert!(contact.email.is_none());
    assert!(contact.url.is_none());

    // Verify license and terms are None
    assert!(spec.info.license.is_none());
    assert!(spec.info.terms_of_service.is_none());
}

#[test]
fn test_operation_id_field() {
    let spec = TestAsyncApiWithOperations::asyncapi();
    let operations = spec.operations.as_ref().unwrap();

    // Verify operation ID is present in the operation struct
    let publish_op = operations.get("publish-test-message").unwrap();
    assert_eq!(publish_op.operation_id, "publish-test-message");

    let subscribe_op = operations.get("subscribe-simple-message").unwrap();
    assert_eq!(subscribe_op.operation_id, "subscribe-simple-message");
}

#[test]
fn test_correlation_id_serialization() {
    #[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
    #[asyncapi(
        channel = "serialization.channel",
        messageId = "serialization-message-v1",
        correlation_id(location = "$message.header#/correlationId")
    )]
    pub struct SerializationMessage {
        pub id: String,
    }

    #[derive(AsyncApi)]
    #[asyncapi(
        info(title = "Serialization Test API", version = "1.0.0"),
        channels("serialization.channel"),
        messages(SerializationMessage)
    )]
    pub struct SerializationTestApi;

    let spec = SerializationTestApi::asyncapi();
    let json = protofolio::to_json(&spec).unwrap();

    // Verify correlation ID is in JSON output
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    let channel = &parsed["channels"]["serialization.channel"];
    let message = &channel["messages"]["SerializationMessage"];
    assert!(message["correlationId"].is_object());
    assert_eq!(
        message["correlationId"]["location"],
        "$message.header#/correlationId"
    );
}

#[test]
fn test_channel_address_in_json() {
    let spec = TestAsyncApi::asyncapi();
    let json = protofolio::to_json(&spec).unwrap();

    // Verify address field is in JSON output
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    let channel = &parsed["channels"]["test.channel"];
    assert_eq!(channel["address"], "test.channel");
}

#[test]
fn test_operation_id_in_json() {
    let spec = TestAsyncApiWithOperations::asyncapi();
    let json = protofolio::to_json(&spec).unwrap();

    // Verify operation ID is in JSON output
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    let operation = &parsed["operations"]["publish-test-message"];
    assert_eq!(operation["operationId"], "publish-test-message");
}

// Test root-level tags
#[derive(AsyncApi)]
#[asyncapi(
    info(title = "Test API with Tags", version = "1.0.0"),
    tags(
        (name = "orders", description = "Order-related operations"),
        (name = "events", description = "Event notifications"),
        (name = "users")
    ),
    channels("test.channel"),
    messages(TestMessage)
)]
pub struct TestAsyncApiWithTags;

#[test]
fn test_root_level_tags() {
    let spec = TestAsyncApiWithTags::asyncapi();

    // Verify tags are present
    assert!(spec.tags.is_some());
    let tags = spec.tags.as_ref().unwrap();
    assert_eq!(tags.len(), 3);

    // Verify tag names
    let tag_names: Vec<&str> = tags.iter().map(|t| t.name.as_str()).collect();
    assert!(tag_names.contains(&"orders"));
    assert!(tag_names.contains(&"events"));
    assert!(tag_names.contains(&"users"));

    // Verify tag descriptions
    let orders_tag = tags.iter().find(|t| t.name == "orders").unwrap();
    assert_eq!(
        orders_tag.description,
        Some("Order-related operations".to_string())
    );

    let events_tag = tags.iter().find(|t| t.name == "events").unwrap();
    assert_eq!(
        events_tag.description,
        Some("Event notifications".to_string())
    );

    let users_tag = tags.iter().find(|t| t.name == "users").unwrap();
    assert_eq!(users_tag.description, None);
}

#[test]
fn test_root_level_tags_serialization() {
    let spec = TestAsyncApiWithTags::asyncapi();
    let json = protofolio::to_json(&spec).unwrap();

    // Parse JSON and verify tags are present
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(parsed["tags"].is_array());
    let tags = parsed["tags"].as_array().unwrap();
    assert_eq!(tags.len(), 3);

    // Verify first tag structure
    let orders_tag = tags.iter().find(|t| t["name"] == "orders").unwrap();
    assert_eq!(orders_tag["name"], "orders");
    assert_eq!(orders_tag["description"], "Order-related operations");

    // Verify tag without description
    let users_tag = tags.iter().find(|t| t["name"] == "users").unwrap();
    assert_eq!(users_tag["name"], "users");
    assert!(!users_tag.as_object().unwrap().contains_key("description"));
}

#[test]
fn test_root_level_tags_with_try_asyncapi() {
    let result = TestAsyncApiWithTags::try_asyncapi();
    assert!(result.is_ok());

    let spec = result.unwrap();
    assert!(spec.tags.is_some());
    assert_eq!(spec.tags.as_ref().unwrap().len(), 3);
}
