//! Shared parsing utilities for protofolio-derive
//!
//! This module provides common parsing functions to reduce code duplication
//! across message, operation, and asyncapi derive macros.

use syn::{parse::ParseStream, LitStr, Token};

/// Parse a tags array from bracketed content
///
/// Expects format: `["tag1", "tag2", ...]`
pub fn parse_tags_array(input: ParseStream) -> syn::Result<Vec<LitStr>> {
    let content;
    syn::bracketed!(content in input);
    let mut tag_list = Vec::new();
    while !content.is_empty() {
        let lit: LitStr = content.parse()?;
        tag_list.push(lit);
        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
        }
    }
    Ok(tag_list)
}

/// Parse optional comma separator
pub fn parse_optional_comma(input: ParseStream) -> syn::Result<()> {
    if !input.is_empty() {
        input.parse::<Token![,]>()?;
    }
    Ok(())
}

/// Parse an examples array from bracketed content
///
/// Expects format: `["example1", "example2", ...]`
pub fn parse_examples_array(input: ParseStream) -> syn::Result<Vec<LitStr>> {
    let content;
    syn::bracketed!(content in input);
    let mut example_list = Vec::new();
    while !content.is_empty() {
        let lit: LitStr = content.parse()?;
        example_list.push(lit);
        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
        }
    }
    Ok(example_list)
}
