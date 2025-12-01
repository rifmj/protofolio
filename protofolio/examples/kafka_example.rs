//! Kafka-specific example
//!
//! This example demonstrates Kafka-specific features:
//! - Kafka server configuration
//! - Topic-based channel naming
//! - Kafka message patterns

use protofolio::AsyncApi;
use protofolio_derive::{AsyncApi, AsyncApiMessage};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Define message types following Kafka topic naming conventions
// Kafka uses topic-based routing, often with service prefixes

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "payment-service.payments.processed",
    messageId = "payment-processed-v1",
    name = "PaymentProcessed",
    title = "Payment Processed Event",
    summary = "Published when a payment is successfully processed",
    description = "This event is published to the payment-service.payments.processed topic",
    tags = ["payments", "events"]
)]
pub struct PaymentProcessed {
    pub payment_id: String,
    pub order_id: String,
    pub amount: f64,
    pub currency: String,
    pub processed_at: i64,
}

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "payment-service.payments.failed",
    messageId = "payment-failed-v1",
    name = "PaymentFailed",
    title = "Payment Failed Event",
    summary = "Published when a payment processing fails",
    tags = ["payments", "errors"]
)]
pub struct PaymentFailed {
    pub payment_id: String,
    pub order_id: String,
    pub error_code: String,
    pub error_message: String,
    pub failed_at: i64,
}

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "payment-service.refunds.processed",
    messageId = "refund-processed-v1",
    name = "RefundProcessed",
    title = "Refund Processed Event",
    summary = "Published when a refund is processed",
    tags = ["payments", "refunds"]
)]
pub struct RefundProcessed {
    pub refund_id: String,
    pub payment_id: String,
    pub amount: f64,
    pub reason: String,
    pub processed_at: i64,
}

// Define the Kafka AsyncAPI specification
#[derive(AsyncApi)]
#[asyncapi(
    info(
        title = "Kafka Payment Service API",
        version = "1.0.0",
        description = "Payment processing events over Kafka"
    ),
    servers(
        (name = "kafka", url = "kafka://localhost:9092", protocol = "kafka"),
        (name = "kafka-cluster", url = "kafka://kafka-cluster:9092", protocol = "kafka")
    ),
    channels(
        "payment-service.payments.processed",
        "payment-service.payments.failed",
        "payment-service.refunds.processed"
    ),
    messages(PaymentProcessed, PaymentFailed, RefundProcessed)
)]
pub struct KafkaPaymentApi;

fn main() {
    println!("=== Kafka Example ===\n");
    println!("This example demonstrates Kafka-specific configuration.\n");
    println!("Kafka uses topic-based routing with service prefixes:");
    println!("  - payment-service.payments.processed");
    println!("  - payment-service.payments.failed");
    println!("  - payment-service.refunds.processed\n");

    // Display JSON output
    let json = KafkaPaymentApi::asyncapi_json().expect("Failed to generate JSON");
    println!("Generated AsyncAPI specification (JSON):");
    println!("{}", json);

    // Display YAML output
    let yaml = KafkaPaymentApi::asyncapi_yaml().expect("Failed to generate YAML");
    println!("\nGenerated AsyncAPI specification (YAML):");
    println!("{}", yaml);

    println!("\n=== Kafka Best Practices ===");
    println!("✓ Use meaningful topic names: service.entity.action");
    println!("✓ Configure appropriate partition counts for parallelism");
    println!("✓ Use message keys for partitioning related messages");
    println!("✓ Set appropriate retention policies based on use case");
    println!("✓ Consider replication factor for fault tolerance");
}
