//! Full-featured example demonstrating all major features of protofolio
//!
//! This example showcases:
//! - Multiple message types with enhanced attributes
//! - Operations (publish/subscribe)
//! - Multiple channels
//! - Multiple servers
//! - Error handling with try_asyncapi()
//! - Validation

use protofolio::{AsyncApi, validate_spec};
use protofolio_derive::{AsyncApi, AsyncApiMessage, AsyncApiOperation};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Define multiple message types with enhanced attributes

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "trip.created",
    messageId = "trip-created-v1",
    name = "TripCreated",
    title = "Trip Created Event",
    summary = "Published when a new trip is created",
    description = "This event is published whenever a user creates a new trip in the system",
    contentType = "application/json",
    tags = ["trip", "events"]
)]
pub struct TripCreated {
    pub trip_id: String,
    pub user_id: String,
    pub destination: String,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "trip.status.changed",
    messageId = "trip-status-changed-v1",
    name = "TripStatusChanged",
    title = "Trip Status Changed Event",
    summary = "Published when trip status changes",
    description = "This event is published whenever the status of a trip changes",
    contentType = "application/json",
    tags = ["trip", "status"]
)]
pub struct TripStatusChanged {
    pub trip_id: String,
    pub old_status: String,
    pub new_status: String,
    pub changed_at: i64,
}

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "user.notifications",
    messageId = "user-notification-v1",
    name = "UserNotification",
    title = "User Notification",
    summary = "Notification sent to a user",
    description = "Various types of notifications sent to users",
    contentType = "application/json",
    tags = ["user", "notifications"]
)]
pub struct UserNotification {
    pub user_id: String,
    pub title: String,
    pub message: String,
    pub priority: Option<String>,
}

// Define operations

#[derive(AsyncApiOperation)]
#[asyncapi(
    id = "publish-trip-created",
    action = "send",
    channel = "trip.created",
    messages(TripCreated),
    summary = "Publish trip created event",
    description = "Published when a new trip is created",
    tags = ["trips", "events"]
)]
pub struct PublishTripCreated;

#[derive(AsyncApiOperation)]
#[asyncapi(
    id = "subscribe-trip-status",
    action = "receive",
    channel = "trip.status.changed",
    messages(TripStatusChanged),
    summary = "Subscribe to trip status changes",
    description = "Subscribe to receive notifications when trip status changes"
)]
pub struct SubscribeTripStatus;

#[derive(AsyncApiOperation)]
#[asyncapi(
    id = "send-user-notification",
    action = "send",
    channel = "user.notifications",
    messages(UserNotification),
    summary = "Send user notification",
    tags = ["user", "notifications"]
)]
pub struct SendUserNotification;

// Define the full-featured AsyncAPI specification
#[derive(AsyncApi)]
#[asyncapi(
    info(
        title = "Full-Featured Example API",
        version = "1.0.0",
        description = "A comprehensive example demonstrating all features of protofolio"
    ),
    servers(
        (name = "production", url = "nats://prod:4222", protocol = "nats"),
        (name = "staging", url = "nats://staging:4222", protocol = "nats")
    ),
    channels("trip.created", "trip.status.changed", "user.notifications"),
    messages(TripCreated, TripStatusChanged, UserNotification),
    operations(PublishTripCreated, SubscribeTripStatus, SendUserNotification)
)]
pub struct FullFeaturedApi;

fn main() {
    println!("=== Full-Featured AsyncAPI Example ===\n");

    // Use try_asyncapi() for proper error handling
    match FullFeaturedApi::try_asyncapi() {
        Ok(spec) => {
            println!("✓ Specification generated successfully\n");

            // Validate the specification
            match validate_spec(&spec) {
                Ok(()) => {
                    println!("✓ Specification validation passed\n");
                }
                Err(e) => {
                    eprintln!("✗ Validation error: {}", e);
                    return;
                }
            }

            // Generate and display JSON
            match FullFeaturedApi::asyncapi_json() {
                Ok(json) => {
                    println!("Generated AsyncAPI specification (JSON):");
                    println!("{}", json);
                }
                Err(e) => {
                    eprintln!("✗ Failed to generate JSON: {}", e);
                }
            }

            // Generate and display YAML
            match FullFeaturedApi::asyncapi_yaml() {
                Ok(yaml) => {
                    println!("\nGenerated AsyncAPI specification (YAML):");
                    println!("{}", yaml);
                }
                Err(e) => {
                    eprintln!("✗ Failed to generate YAML: {}", e);
                }
            }

            // Display some statistics
            println!("\n=== Specification Statistics ===");
            println!("Channels: {}", spec.channels.len());
            println!("Operations: {}", spec.operations.as_ref().map(|ops| ops.len()).unwrap_or(0));
            println!("Servers: {}", spec.servers.as_ref().map(|srv| srv.len()).unwrap_or(0));
        }
        Err(e) => {
            eprintln!("✗ Failed to generate specification: {}", e);
        }
    }
}

