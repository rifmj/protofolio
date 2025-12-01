//! Code generation for message handling in AsyncApi derive macro

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// Generate code for message handling (panic version for asyncapi())
pub fn generate_messages_code(messages: &[syn::Path], ident: &Ident) -> Vec<TokenStream> {
    messages
        .iter()
        .map(|message_type| {
            let message_type_ident = message_type;
            quote! {
                {
                    // Compile-time validation: ensure message channel exists
                    const _: () = {
                        const _CHECK: &str = #message_type_ident::CHANNEL;
                    };

                    use schemars::JsonSchema;
                    let schema = protofolio::schema_for_type::<#message_type_ident>()
                        .unwrap_or_else(|e| {
                            panic!(
                                "Failed to generate schema for message type '{}': {}. Ensure the type implements JsonSchema trait (derive JsonSchema).",
                                stringify!(#message_type_ident),
                                e
                            );
                        });
                    let message_name_str = stringify!(#message_type_ident);
                    let channel_name = #message_type_ident::channel();

                    if !channels_map.contains_key(channel_name) {
                        let available: Vec<_> = channels_map.keys().collect();
                        let available_str = if available.is_empty() {
                            format!("No channels declared. Add channels(\"{}\", ...) to your #[asyncapi] attribute on {}", channel_name, stringify!(#ident))
                        } else {
                            format!("Available channels: {:?}. Add '{}' to channels(...) in your #[asyncapi] attribute on {}", available, channel_name, stringify!(#ident))
                        };
                        panic!(
                            "Message '{}' (type: {}) references channel '{}' which is not declared. {}\n\nHint: Update your #[derive(AsyncApi)] on {} to include: channels(\"{}\", ...)",
                            message_name_str,
                            stringify!(#message_type_ident),
                            channel_name,
                            available_str,
                            stringify!(#ident),
                            channel_name
                        );
                    }

                    let message = Message {
                        message_id: #message_type_ident::message_id().map(|s| s.to_string()),
                        name: #message_type_ident::name().map(|s| s.to_string()),
                        title: #message_type_ident::title().map(|s| s.to_string()),
                        summary: #message_type_ident::summary().map(|s| s.to_string()),
                        description: #message_type_ident::description().map(|s| s.to_string()),
                        content_type: #message_type_ident::content_type().map(|s| s.to_string()),
                        tags: #message_type_ident::tags(),
                        payload: MessagePayload {
                            schema: schema,
                        },
                        external_docs: #message_type_ident::external_docs(),
                        examples: #message_type_ident::examples(),
                        headers: #message_type_ident::headers(),
                        correlation_id: #message_type_ident::correlation_id(),
                    };

                    channels_map.get_mut(channel_name)
                        .expect(&format!("Channel '{}' should exist (validated at compile time)", channel_name))
                        .messages.insert(message_name_str.to_string(), protofolio::MessageOrRef::message(message));
                }
            }
        })
        .collect()
}

/// Generate code for message handling (error-returning version for try_asyncapi())
pub fn generate_messages_try_code(messages: &[syn::Path], ident: &Ident) -> Vec<TokenStream> {
    messages
        .iter()
        .map(|message_type| {
            let message_type_ident = message_type;
            quote! {
                {
                    const _: () = {
                        const _CHECK: &str = #message_type_ident::CHANNEL;
                    };

                    use schemars::JsonSchema;
                    let schema = match protofolio::schema_for_type::<#message_type_ident>() {
                        Ok(s) => s,
                        Err(e) => {
                            return Err(protofolio::ValidationError::SchemaGenerationFailed(
                                stringify!(#message_type_ident).to_string(),
                                format!("Ensure the type implements JsonSchema trait (derive JsonSchema): {}", e)
                            ));
                        }
                    };
                    let message_name_str = stringify!(#message_type_ident);
                    let channel_name = #message_type_ident::channel();

                    if !channels_map.contains_key(channel_name) {
                        let available: Vec<_> = channels_map.keys().collect();
                        let available_str = if available.is_empty() {
                            format!("No channels declared. Add channels(\"{}\", ...) to your #[asyncapi] attribute on {}", channel_name, stringify!(#ident))
                        } else {
                            format!("Available channels: {:?}. Add '{}' to channels(...) in your #[asyncapi] attribute on {}", available, channel_name, stringify!(#ident))
                        };
                        return Err(protofolio::ValidationError::InvalidChannelReference(
                            format!("Message '{}' (type: {}) references channel '{}' which is not declared. {}", message_name_str, stringify!(#message_type_ident), channel_name, available_str)
                        ));
                    }

                    let message = Message {
                        message_id: #message_type_ident::message_id().map(|s| s.to_string()),
                        name: #message_type_ident::name().map(|s| s.to_string()),
                        title: #message_type_ident::title().map(|s| s.to_string()),
                        summary: #message_type_ident::summary().map(|s| s.to_string()),
                        description: #message_type_ident::description().map(|s| s.to_string()),
                        content_type: #message_type_ident::content_type().map(|s| s.to_string()),
                        tags: #message_type_ident::tags(),
                        payload: MessagePayload {
                            schema: schema,
                        },
                        external_docs: #message_type_ident::external_docs(),
                        examples: #message_type_ident::examples(),
                        headers: #message_type_ident::headers(),
                        correlation_id: #message_type_ident::correlation_id(),
                    };

                    channels_map.get_mut(channel_name)
                        .ok_or_else(|| protofolio::ValidationError::InvalidChannelReference(
                            format!("Channel '{}' should exist (validated above)", channel_name)
                        ))?
                        .messages.insert(message_name_str.to_string(), protofolio::MessageOrRef::message(message));
                }
            }
        })
        .collect()
}
