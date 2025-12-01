//! NATS-specific example
//!
//! This example demonstrates NATS-specific features:
//! - NATS server configuration
//! - Subject-based channel naming
//! - NATS-specific message patterns

use protofolio::AsyncApi;
use protofolio_derive::{AsyncApi, AsyncApiMessage};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Define message types following NATS subject naming conventions
// NATS uses dot-separated hierarchical subjects

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "orders.created",
    messageId = "order-created-v1",
    name = "OrderCreated",
    title = "Order Created Event",
    summary = "Published when a new order is created",
    tags = ["orders", "events"]
)]
pub struct OrderCreated {
    pub order_id: String,
    pub user_id: String,
    pub items: Vec<OrderItem>,
    pub total: f64,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct OrderItem {
    pub product_id: String,
    pub quantity: u32,
    pub price: f64,
}

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "orders.status.updated",
    messageId = "order-status-updated-v1",
    name = "OrderStatusUpdated",
    title = "Order Status Updated",
    summary = "Published when order status changes",
    tags = ["orders", "status"]
)]
pub struct OrderStatusUpdated {
    pub order_id: String,
    pub old_status: String,
    pub new_status: String,
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "orders.cancelled",
    messageId = "order-cancelled-v1",
    name = "OrderCancelled",
    title = "Order Cancelled",
    summary = "Published when an order is cancelled",
    tags = ["orders", "events"]
)]
pub struct OrderCancelled {
    pub order_id: String,
    pub reason: String,
    pub cancelled_at: i64,
}

// Define the NATS AsyncAPI specification
#[derive(AsyncApi)]
#[asyncapi(
    info(
        title = "NATS Order Service API",
        version = "1.0.0",
        description = "Order management events over NATS messaging"
    ),
    servers(
        (name = "nats", url = "nats://localhost:4222", protocol = "nats"),
        (name = "nats-cluster", url = "nats://nats-cluster:4222", protocol = "nats")
    ),
    channels(
        "orders.created",
        "orders.status.updated",
        "orders.cancelled"
    ),
    messages(OrderCreated, OrderStatusUpdated, OrderCancelled)
)]
pub struct NatsOrderApi;

fn main() {
    println!("=== NATS Example ===\n");
    println!("This example demonstrates NATS-specific configuration.\n");
    println!("NATS uses subject-based routing with dot-separated hierarchies:");
    println!("  - orders.created");
    println!("  - orders.status.updated");
    println!("  - orders.cancelled\n");

    // Display JSON output
    let json = NatsOrderApi::asyncapi_json().expect("Failed to generate JSON");
    println!("Generated AsyncAPI specification (JSON):");
    println!("{}", json);

    // Display YAML output
    let yaml = NatsOrderApi::asyncapi_yaml().expect("Failed to generate YAML");
    println!("\nGenerated AsyncAPI specification (YAML):");
    println!("{}", yaml);

    println!("\n=== NATS Best Practices ===");
    println!("✓ Use dot-separated hierarchical subject names");
    println!("✓ Follow naming convention: service.entity.action");
    println!("✓ Configure queue groups for load balancing");
    println!("✓ Use wildcards (*, >) for subject patterns when appropriate");
}
