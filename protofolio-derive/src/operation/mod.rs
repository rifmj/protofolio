//! Main AsyncApiOperation derive macro implementation

mod attrs;
mod codegen;

use crate::operation::{
    attrs::OperationAttrs,
    codegen::{
        generate_external_docs_code, generate_impl_block, generate_optional_field_code,
        generate_tags_code,
    },
};
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use syn::{DeriveInput, Error};

/// Derive `AsyncApiOperation` implementation
#[allow(clippy::too_many_lines)] // Macro code is inherently long
pub fn derive_asyncapi_operation(input: DeriveInput) -> Result<TokenStream, Error> {
    let ident = &input.ident;

    // Parse attributes to extract all operation attributes
    let mut operation_id = None;
    let mut action = None;
    let mut channel = None;
    let mut messages = Vec::new();
    let mut summary = None;
    let mut description = None;
    let mut tags = None;
    let mut external_docs = None;

    for attr in &input.attrs {
        if attr.path().is_ident("asyncapi") {
            // Parse the attribute tokens directly
            let tokens = match attr.meta.require_list() {
                Ok(meta) => meta.tokens.clone(),
                Err(e) => {
                    abort!(
                        attr,
                        "Failed to parse asyncapi attribute: {}\n\nHint: Ensure the attribute syntax is correct. Example: #[asyncapi(id = \"op-1\", action = \"send\", channel = \"events\", messages(MyMessage))]",
                        e
                    );
                }
            };

            match syn::parse2::<OperationAttrs>(tokens) {
                Ok(attrs) => {
                    operation_id = attrs.operation_id;
                    action = attrs.action;
                    channel = attrs.channel;
                    messages = attrs.messages;
                    summary = attrs.summary;
                    description = attrs.description;
                    tags = attrs.tags;
                    external_docs = attrs.external_docs;
                }
                Err(e) => {
                    abort!(
                        attr,
                        "Failed to parse asyncapi attributes: {}\n\nHint: Check the attribute syntax. Example: #[asyncapi(id = \"op-1\", action = \"send\", channel = \"events\", messages(MyMessage), tags = [\"tag1\"])]",
                        e
                    );
                }
            }
        }
    }

    // abort! never returns, so let...else pattern doesn't apply
    #[allow(clippy::option_if_let_else)]
    let operation_id_lit = if let Some(id) = operation_id {
        id
    } else {
        abort!(
            ident,
            "AsyncApiOperation requires 'id' attribute.\n\nExample: #[asyncapi(id = \"publish-event\", action = \"send\", channel = \"events\", messages(MyMessage))]\n\nHint: The id attribute provides a unique identifier for this operation."
        );
    };

    #[allow(clippy::option_if_let_else)]
    let action_lit = if let Some(act) = action {
        act
    } else {
        abort!(
            ident,
            "AsyncApiOperation requires 'action' attribute.\n\nExample: #[asyncapi(id = \"publish-event\", action = \"send\", channel = \"events\", messages(MyMessage))]\n\nHint: The action must be either 'send' (publish) or 'receive' (subscribe)."
        );
    };

    // Validate action is "send" or "receive"
    let action_value = action_lit.value();
    if action_value != "send" && action_value != "receive" {
        abort!(
            action_lit,
            "Invalid action value '{}'. Expected 'send' or 'receive'.\n\nHint: Use 'send' for publishing messages and 'receive' for subscribing to messages.\nExample: #[asyncapi(id = \"op-1\", action = \"send\", channel = \"events\", messages(MyMessage))]",
            action_value
        );
    }

    #[allow(clippy::option_if_let_else)]
    let channel_lit = if let Some(ch) = channel {
        ch
    } else {
        abort!(
            ident,
            "AsyncApiOperation requires 'channel' attribute.\n\nExample: #[asyncapi(id = \"publish-event\", action = \"send\", channel = \"events\", messages(MyMessage))]\n\nHint: The channel attribute specifies which channel this operation uses."
        );
    };

    if messages.is_empty() {
        abort!(
            ident,
            "AsyncApiOperation requires at least one message in 'messages(...)' attribute.\n\nExample: #[asyncapi(id = \"op-1\", action = \"send\", channel = \"events\", messages(MyMessage))]\n\nHint: List the message types this operation handles, e.g., messages(Message1, Message2)."
        );
    }

    // Generate optional field code
    let summary_opt = generate_optional_field_code(&summary);
    let desc_opt = generate_optional_field_code(&description);
    let tags_opt = generate_tags_code(&tags);
    let external_docs_opt = generate_external_docs_code(&external_docs);

    // Generate code that stores metadata
    Ok(generate_impl_block(
        ident,
        &channel_lit,
        &operation_id_lit,
        &action_lit,
        &messages,
        summary_opt,
        desc_opt,
        tags_opt,
        external_docs_opt,
    ))
}
