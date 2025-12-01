//! JSON Schema generation utilities
//!
//! This module provides helpers for generating JSON Schema from Rust types that
//! implement the `JsonSchema` trait (from the `schemars` crate).
//!
//! # Usage
//!
//! Typically, you don't need to call these functions directly - the `AsyncApi`
//! derive macro automatically generates schemas for your message types. However,
//! you can use these functions if you need to generate schemas programmatically:
//!
//! ```rust,no_run
//! use protofolio::generate_schema;
//! use schemars::JsonSchema;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize, JsonSchema)]
//! struct MyType {
//!     field: String,
//! }
//!
//! let schema = generate_schema::<MyType>()?;
//! # Ok::<(), protofolio::SchemaError>(())
//! ```
//!
//! # Performance
//!
//! Schemas are automatically cached by type ID, so repeated calls for the same
//! type are fast. The cache uses `Arc` internally to avoid cloning on cache hits.

mod generator;

pub use generator::*;
