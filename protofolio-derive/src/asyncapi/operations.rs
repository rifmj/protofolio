//! Code generation for operation handling in AsyncApi derive macro

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// Generate code for operation handling (panic version for asyncapi())
pub fn generate_operations_code(
    operations: &[syn::Path],
    ident: &Ident,
) -> Vec<TokenStream> {
    operations
        .iter()
        .map(|operation_type| {
            let operation_type_ident = operation_type;
            quote! {
                {
                    const _: () = {
                        const _CHECK: &str = #operation_type_ident::CHANNEL;
                    };
                    
                    use protofolio::AsyncApiOperation;
                    let operation = #operation_type_ident::to_operation();
                    let operation_id = #operation_type_ident::operation_id();
                    
                    let channel_name = #operation_type_ident::channel();
                    if !channels_map.contains_key(channel_name) {
                        let available: Vec<_> = channels_map.keys().collect();
                        let available_str = if available.is_empty() {
                            format!("No channels declared. Add channels(\"{}\", ...) to your #[asyncapi] attribute on {}", channel_name, stringify!(#ident))
                        } else {
                            format!("Available channels: {:?}. Add '{}' to channels(...) in your #[asyncapi] attribute on {}", available, channel_name, stringify!(#ident))
                        };
                        panic!(
                            "Operation '{}' (type: {}) references channel '{}' which is not declared. {}\n\nHint: Update your #[derive(AsyncApi)] on {} to include: channels(\"{}\", ...)",
                            operation_id,
                            stringify!(#operation_type_ident),
                            channel_name,
                            available_str,
                            stringify!(#ident),
                            channel_name
                        );
                    }
                    
                    let channel = channels_map.get(channel_name)
                        .expect(&format!("Channel '{}' should exist (validated above)", channel_name));
                    let message_names = #operation_type_ident::message_names();
                    for msg_name in &message_names {
                        if !channel.messages.contains_key(msg_name) {
                            let available: Vec<_> = channel.messages.keys().collect();
                            let available_str = if available.is_empty() {
                                format!("No messages in channel '{}'. Add messages({}, ...) to your #[asyncapi] attribute on {}", channel_name, msg_name, stringify!(#ident))
                            } else {
                                format!("Available messages in channel '{}': {:?}. Make sure '{}' is registered in messages(...) in your #[asyncapi] attribute on {}", channel_name, available, msg_name, stringify!(#ident))
                            };
                            panic!(
                                "Operation '{}' (type: {}) references message '{}' in channel '{}' which does not exist. {}\n\nHint: Update your #[derive(AsyncApi)] on {} to include: messages({}, ...)",
                                operation_id,
                                stringify!(#operation_type_ident),
                                msg_name,
                                channel_name,
                                available_str,
                                stringify!(#ident),
                                msg_name
                            );
                        }
                    }
                    
                    operations_map.insert(operation_id.to_string(), operation);
                }
            }
        })
        .collect()
}

/// Generate code for operation handling (error-returning version for try_asyncapi())
pub fn generate_operations_try_code(
    operations: &[syn::Path],
    ident: &Ident,
) -> Vec<TokenStream> {
    operations
        .iter()
        .map(|operation_type| {
            let operation_type_ident = operation_type;
            quote! {
                {
                    const _: () = {
                        const _CHECK: &str = #operation_type_ident::CHANNEL;
                    };
                    
                    use protofolio::AsyncApiOperation;
                    let operation = #operation_type_ident::to_operation();
                    let operation_id = #operation_type_ident::operation_id();
                    
                    let channel_name = #operation_type_ident::channel();
                    if !channels_map.contains_key(channel_name) {
                        let available: Vec<_> = channels_map.keys().collect();
                        let available_str = if available.is_empty() {
                            format!("No channels declared. Add channels(\"{}\", ...) to your #[asyncapi] attribute on {}", channel_name, stringify!(#ident))
                        } else {
                            format!("Available channels: {:?}. Add '{}' to channels(...) in your #[asyncapi] attribute on {}", available, channel_name, stringify!(#ident))
                        };
                        return Err(protofolio::ValidationError::InvalidChannelReference(
                            format!("Operation '{}' (type: {}) references channel '{}' which is not declared. {}", operation_id, stringify!(#operation_type_ident), channel_name, available_str)
                        ));
                    }
                    
                    let channel = channels_map.get(channel_name)
                        .ok_or_else(|| protofolio::ValidationError::InvalidChannelReference(
                            format!("Channel '{}' should exist (validated above)", channel_name)
                        ))?;
                    let message_names = #operation_type_ident::message_names();
                    for msg_name in &message_names {
                        if !channel.messages.contains_key(msg_name) {
                            let available: Vec<_> = channel.messages.keys().collect();
                            let available_str = if available.is_empty() {
                                format!("No messages in channel '{}'. Add messages({}, ...) to your #[asyncapi] attribute on {}", channel_name, msg_name, stringify!(#ident))
                            } else {
                                format!("Available messages in channel '{}': {:?}. Make sure '{}' is registered in messages(...) in your #[asyncapi] attribute on {}", channel_name, available, msg_name, stringify!(#ident))
                            };
                            return Err(protofolio::ValidationError::MessageNotFound {
                                channel: channel_name.to_string(),
                                message: msg_name.clone(),
                            });
                        }
                    }
                    
                    operations_map.insert(operation_id.to_string(), operation);
                }
            }
        })
        .collect()
}

