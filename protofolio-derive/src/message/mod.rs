//! Main AsyncApiMessage derive macro implementation

mod attrs;
mod codegen;

use crate::message::{
    attrs::MessageAttrs,
    codegen::{
        generate_examples_code, generate_external_docs_code, generate_headers_code,
        generate_impl_block, generate_optional_field_code, generate_tags_code,
    },
};
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use syn::{DeriveInput, Error};

/// Derive `AsyncApiMessage` implementation
#[allow(clippy::too_many_lines)] // Macro code is inherently long
pub fn derive_asyncapi_message(input: DeriveInput) -> Result<TokenStream, Error> {
    let ident = &input.ident;

    // Parse attributes to extract all message attributes
    let mut channel = None;
    let mut summary = None;
    let mut description = None;
    let mut message_id = None;
    let mut name = None;
    let mut title = None;
    let mut content_type = None;
    let mut tags = None;
    let mut external_docs = None;
    let mut example = None;
    let mut examples = None;
    let mut headers = None;

    for attr in &input.attrs {
        if attr.path().is_ident("asyncapi") {
            // Parse the attribute tokens directly
            let tokens = match attr.meta.require_list() {
                Ok(meta) => meta.tokens.clone(),
                Err(e) => {
                    abort!(
                        attr,
                        "Failed to parse asyncapi attribute: {}\n\nHint: Ensure the attribute syntax is correct. Example: #[asyncapi(channel = \"events\", messageId = \"event-v1\")]",
                        e
                    );
                }
            };

            match syn::parse2::<MessageAttrs>(tokens) {
                Ok(attrs) => {
                    channel = attrs.channel;
                    summary = attrs.summary;
                    description = attrs.description;
                    message_id = attrs.message_id;
                    name = attrs.name;
                    title = attrs.title;
                    content_type = attrs.content_type;
                    tags = attrs.tags;
                    external_docs = attrs.external_docs;
                    example = attrs.example;
                    examples = attrs.examples;
                    headers = attrs.headers;
                }
                Err(e) => {
                    abort!(
                        attr,
                        "Failed to parse asyncapi attributes: {}\n\nHint: Check the attribute syntax. Example: #[asyncapi(channel = \"events\", messageId = \"event-v1\", tags = [\"tag1\", \"tag2\"])]",
                        e
                    );
                }
            }
        }
    }

    // abort! never returns, so let...else pattern doesn't apply
    #[allow(clippy::option_if_let_else)]
    let channel_lit = if let Some(ch) = channel {
        ch
    } else {
        abort!(
            ident,
            "AsyncApiMessage requires 'channel' attribute.\n\nExample: #[asyncapi(channel = \"events\", messageId = \"event-v1\")]\n\nHint: The channel attribute specifies which channel this message is published to."
        );
    };

    // Generate optional field code
    let summary_opt = generate_optional_field_code(&summary);
    let desc_opt = generate_optional_field_code(&description);
    let message_id_opt = generate_optional_field_code(&message_id);
    let name_opt = generate_optional_field_code(&name);
    let title_opt = generate_optional_field_code(&title);
    let content_type_opt = generate_optional_field_code(&content_type);
    let tags_opt = generate_tags_code(&tags);
    let external_docs_opt = generate_external_docs_code(&external_docs);
    let examples_opt = generate_examples_code(&example, &examples);
    let headers_opt = generate_headers_code(&headers);

    // Generate code that stores metadata
    Ok(generate_impl_block(
        ident,
        &channel_lit,
        summary_opt,
        desc_opt,
        message_id_opt,
        name_opt,
        title_opt,
        content_type_opt,
        tags_opt,
        external_docs_opt,
        examples_opt,
        headers_opt,
    ))
}
