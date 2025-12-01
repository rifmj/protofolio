//! Validation utilities for AsyncAPI specifications
//!
//! This module provides validation functions to ensure generated AsyncAPI specs
//! conform to the AsyncAPI 3.0 specification.
//!
//! # Usage
//!
//! The main validation function is [`validate_spec`](crate::validate_spec), which
//! checks:
//!
//! - AsyncAPI version is 3.0.0
//! - Required fields (info.title, info.version) are present
//! - At least one channel is defined
//! - All channels have messages
//! - Server references are valid
//! - Message IDs are unique
//! - Protocol identifiers are supported
//! - Protocol-specific bindings are valid
//!
//! # Example
//!
//! ```rust,no_run
//! use protofolio::{AsyncApi, validate_spec};
//! # use protofolio_derive::AsyncApi;
//! #
//! # #[derive(AsyncApi)]
//! # #[asyncapi(info(title = "Test", version = "1.0.0"), channels("events"), messages())]
//! # struct MyApi;
//!
//! let spec = MyApi::asyncapi();
//! if let Err(e) = validate_spec(&spec) {
//!     eprintln!("Validation failed: {}", e);
//! }
//! ```
//!
//! Note: The `try_asyncapi()` method automatically validates the spec, so you
//! typically don't need to call `validate_spec` separately.

mod bindings;
mod validator;

pub use validator::*;

