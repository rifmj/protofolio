//! Procedural macros for protofolio
//!
//! This crate provides the derive macros for generating `AsyncAPI` specifications.
//! Lints are configured in the workspace Cargo.toml and inherited here.
#![deny(rustdoc::broken_intra_doc_links, unsafe_code)]
#![warn(
    missing_docs,
    missing_debug_implementations
)]
#![allow(
    // Documentation - can be fixed incrementally
    clippy::missing_docs_in_private_items,
    rustdoc::missing_doc_code_examples,
    rustdoc::private_intra_doc_links, // Private items can link to each other
    // Proc-macro specific patterns
    clippy::result_large_err, // Result<TokenStream, Error> is standard for proc macros
    clippy::too_many_lines, // Macro code is inherently long (also allowed per-function)
    // Style preferences (optional improvements)
    clippy::must_use_candidate, // #[must_use] is nice but not required
    clippy::use_self, // Self is preferred but not required everywhere
    clippy::uninlined_format_args, // Old format! style is acceptable
    clippy::redundant_closure, // Closures are sometimes clearer
    clippy::option_if_let_else, // if let/else is fine with abort! (never returns)
)]

//! # Macros
//!
//! - [`AsyncApi`] - Main derive macro for `AsyncAPI` specifications
//! - [`AsyncApiMessage`] - Derive macro for message types
//! - [`AsyncApiOperation`] - Derive macro for operation types
//!
//! # Macro Expansion
//!
//! The macros perform compile-time validation and generate code that builds
//! the `AsyncAPI` specification at runtime. See the README for detailed
//! documentation on the expansion process.
//!
//! # Error Handling
//!
//! All macros provide detailed error messages with suggestions when validation fails.
//! Compile-time errors occur when:
//! - Required attributes are missing
//! - `CHANNEL` consts don't exist (indicates missing derive macro)
//! - Invalid attribute values
//!
//! Runtime errors occur when:
//! - Channel references don't match declared channels
//! - Message references don't exist in channels
//! - Schema generation fails

mod asyncapi;
mod message;
mod operation;
mod parse_utils;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for generating `AsyncAPI` specifications
///
/// Use this on a struct that represents your complete `AsyncAPI` specification.
///
/// # Requirements
///
/// - `info(title = "...", version = "...")` - Required API information
/// - `channels(...)` - At least one channel must be declared
/// - `messages(...)` - Message types must be listed (they must have `#[derive(AsyncApiMessage)]`)
///
/// # Compile-Time Validation
///
/// The macro validates that:
/// - All message types have `CHANNEL` consts (ensures they have `#[derive(AsyncApiMessage)]`)
/// - All operation types have `CHANNEL` consts (ensures they have `#[derive(AsyncApiOperation)]`)
/// - Required attributes are present
///
/// # Runtime Validation
///
/// At runtime, the generated code validates:
/// - Message channels exist in the declared channels list
/// - Operation channels exist in the declared channels list
/// - Operation messages exist in their channels
///
/// # Example
///
/// ```rust,ignore
/// use protofolio_derive::{AsyncApi, AsyncApiMessage};
/// use serde::{Deserialize, Serialize};
/// use schemars::JsonSchema;
///
/// #[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
/// #[asyncapi(channel = "events", messageId = "event-v1")]
/// pub struct MyMessage {
///     pub id: String,
/// }
///
/// #[derive(AsyncApi)]
/// #[asyncapi(
///     info(title = "My API", version = "1.0.0"),
///     servers((name = "nats", url = "nats://localhost:4222", protocol = "nats")),
///     channels("events"),
///     messages(MyMessage)
/// )]
/// pub struct MyAsyncApi;
/// ```
///
/// # Error Messages
///
/// If validation fails, you'll get detailed error messages with:
/// - Available channels/messages
/// - Hints on how to fix the issue
/// - Exact location of the problem
#[proc_macro_derive(AsyncApi, attributes(asyncapi))]
#[proc_macro_error]
pub fn derive_asyncapi(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    asyncapi::derive_asyncapi(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// Derive macro for `AsyncAPI` message types
///
/// Use this on structs that represent message payloads. The struct must also
/// implement `Serialize`, `Deserialize`, and `JsonSchema`.
///
/// # Requirements
///
/// - `channel = "..."` - Required channel name
/// - The type must implement `JsonSchema` (usually via `#[derive(JsonSchema)]`)
///
/// # Generated Code
///
/// The macro generates:
/// - `CHANNEL` const for compile-time validation
/// - Static methods: `channel()`, `message_id()`, `name()`, `title()`, etc.
///
/// # Example
///
/// ```rust,ignore
/// use protofolio_derive::AsyncApiMessage;
/// use serde::{Deserialize, Serialize};
/// use schemars::JsonSchema;
///
/// #[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
/// #[asyncapi(
///     channel = "trip.status.changed",
///     messageId = "trip-status-changed-v1",
///     name = "TripStatusChanged",
///     summary = "Trip Status Changed Event",
///     description = "Published when trip status changes",
///     contentType = "application/json",
///     tags = ["trip", "status"]
/// )]
/// pub struct TripStatusChanged {
///     pub trip_id: String,
///     pub user_id: String,
///     pub new_status: String,
/// }
/// ```
///
/// # Attributes
///
/// - `channel` (required) - Channel name
/// - `messageId` - Unique message identifier
/// - `name` - Message name
/// - `title` - Message title
/// - `summary` - Brief summary
/// - `description` - Detailed description
/// - `contentType` - Content type (default: "application/json")
/// - `tags` - Array of tag names: `tags = ["tag1", "tag2"]`
#[proc_macro_derive(AsyncApiMessage, attributes(asyncapi))]
#[proc_macro_error]
pub fn derive_asyncapi_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    message::derive_asyncapi_message(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// Derive macro for `AsyncAPI` operation types
///
/// Use this on structs that represent operations (publish/subscribe).
/// Operations define how messages are sent or received on channels.
///
/// # Requirements
///
/// - `id = "..."` - Required unique operation identifier
/// - `action = "send" | "receive"` - Required action type
/// - `channel = "..."` - Required channel name
/// - `messages(...)` - Required list of message types (at least one)
///
/// # Generated Code
///
/// The macro generates:
/// - `CHANNEL` const for compile-time validation
/// - `MESSAGE_TYPES` const array
/// - `AsyncApiOperation` trait implementation
///
/// # Example
///
/// ```rust,ignore
/// use protofolio_derive::AsyncApiOperation;
///
/// #[derive(AsyncApiOperation)]
/// #[asyncapi(
///     id = "publish-trip-created",
///     action = "send",
///     channel = "trip.created",
///     messages(TripCreated),
///     summary = "Publish trip created event",
///     description = "Publishes a trip created event to the channel",
///     tags = ["trips", "events"]
/// )]
/// pub struct PublishTripCreated;
///
/// #[derive(AsyncApiOperation)]
/// #[asyncapi(
///     id = "subscribe-trip-status",
///     action = "receive",
///     channel = "trip.status.changed",
///     messages(TripStatusChanged),
///     summary = "Subscribe to trip status changes"
/// )]
/// pub struct SubscribeTripStatus;
/// ```
///
/// # Attributes
///
/// - `id` (required) - Unique operation identifier
/// - `action` (required) - Either "send" or "receive"
/// - `channel` (required) - Channel name
/// - `messages(...)` (required) - List of message types: `messages(Message1, Message2)`
/// - `summary` - Brief summary
/// - `description` - Detailed description
/// - `tags` - Array of tag names: `tags = ["tag1", "tag2"]`
///
/// # Validation
///
/// The macro validates:
/// - `action` is either "send" or "receive"
/// - At least one message is specified
/// - All attributes are correctly formatted
#[proc_macro_derive(AsyncApiOperation, attributes(asyncapi))]
#[proc_macro_error]
pub fn derive_asyncapi_operation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    operation::derive_asyncapi_operation(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
