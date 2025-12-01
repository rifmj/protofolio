//! Main `AsyncApi` derive macro implementation

mod attrs;
mod codegen;
mod messages;
mod operations;

use crate::asyncapi::{
    attrs::AsyncApiAttrs,
    codegen::{
        generate_channels_code, generate_impl_block, generate_operations_map_code,
        generate_operations_map_try_code, generate_security_schemes_code, generate_servers_code,
        generate_tags_code,
    },
    messages::{generate_messages_code, generate_messages_try_code},
    operations::{generate_operations_code, generate_operations_try_code},
};
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::{DeriveInput, Error};

/// Derive `AsyncApi` implementation
#[allow(clippy::too_many_lines)] // Macro code is inherently long
pub fn derive_asyncapi(input: DeriveInput) -> Result<TokenStream, Error> {
    let ident = &input.ident;

    // Parse attributes to extract spec information
    let mut info_title = None;
    let mut info_version = None;
    let mut info_description = None;
    let mut info_external_docs = None;
    let mut info_contact = None;
    let mut info_license = None;
    let mut info_terms_of_service = None;
    let mut servers = Vec::new();
    let mut security_schemes = Vec::new();
    let mut channels = Vec::new();
    let mut messages = Vec::new();
    let mut operations = Vec::new();
    let mut tags = Vec::new();

    for attr in &input.attrs {
        if attr.path().is_ident("asyncapi") {
            // Parse the attribute tokens directly
            let tokens = match attr.meta.require_list() {
                Ok(meta) => meta.tokens.clone(),
                Err(e) => {
                    abort!(
                        attr,
                        "Failed to parse asyncapi attribute: {}\n\nHint: Ensure the attribute syntax is correct. Example: #[asyncapi(info(title = \"...\", version = \"...\"))]",
                        e
                    );
                }
            };

            // Parse the tokens as AsyncApiAttrs
            let parser = syn::parse2::<AsyncApiAttrs>(tokens)?;

            // Process info
            if let Some(info) = parser.info {
                if let Some(title) = info.title {
                    info_title = Some(title.value());
                }
                if let Some(version) = info.version {
                    info_version = Some(version.value());
                }
                if let Some(description) = info.description {
                    info_description = Some(description.value());
                }
                info_external_docs = info.external_docs;
                info_contact = info.contact;
                info_license = info.license;
                if let Some(terms) = info.terms_of_service {
                    info_terms_of_service = Some(terms.value());
                }
            }

            // Process servers
            servers.extend(parser.servers);

            // Process security schemes
            security_schemes.extend(parser.security_schemes);

            // Process channels
            channels.extend(parser.channels);

            // Process messages
            messages.extend(parser.messages);

            // Process operations
            operations.extend(parser.operations);

            // Process tags
            tags.extend(parser.tags);
        }
    }

    // These use abort! which never returns, so let...else pattern doesn't apply
    #[allow(clippy::option_if_let_else)]
    let info_title = if let Some(title) = info_title {
        title
    } else {
        abort!(
            ident,
            "AsyncApi requires 'info(title = ...)' attribute.\n\nExample: #[asyncapi(info(title = \"My API\", version = \"1.0.0\"))]"
        );
    };

    #[allow(clippy::option_if_let_else)]
    let info_version = if let Some(version) = info_version {
        version
    } else {
        abort!(
            ident,
            "AsyncApi requires 'info(version = ...)' attribute.\n\nExample: #[asyncapi(info(title = \"My API\", version = \"1.0.0\"))]"
        );
    };

    let info_desc_expr = info_description.as_ref().map_or_else(
        || quote! { None },
        |desc| {
            let desc_str = desc.as_str();
            quote! { Some(#desc_str.to_string()) }
        },
    );

    let info_external_docs_expr = info_external_docs.as_ref().map_or_else(
        || quote! { None },
        |ext_docs| {
            let url_lit = &ext_docs.url;
            let desc_expr = ext_docs.description.as_ref().map_or_else(
                || quote! { None },
                |desc| {
                    let desc_str = desc.value();
                    quote! { Some(#desc_str.to_string()) }
                },
            );
            quote! {
                Some(protofolio::ExternalDocumentation {
                    url: #url_lit.to_string(),
                    description: #desc_expr,
                })
            }
        },
    );

    let info_contact_expr = info_contact.as_ref().map_or_else(
        || quote! { None },
        |contact| {
            let name_expr = contact.name.as_ref().map_or_else(
                || quote! { None },
                |name| {
                    let name_str = name.value();
                    quote! { Some(#name_str.to_string()) }
                },
            );
            let url_expr = contact.url.as_ref().map_or_else(
                || quote! { None },
                |url| {
                    let url_str = url.value();
                    quote! { Some(#url_str.to_string()) }
                },
            );
            let email_expr = contact.email.as_ref().map_or_else(
                || quote! { None },
                |email| {
                    let email_str = email.value();
                    quote! { Some(#email_str.to_string()) }
                },
            );
            quote! {
                Some(protofolio::Contact {
                    name: #name_expr,
                    url: #url_expr,
                    email: #email_expr,
                })
            }
        },
    );

    let info_license_expr = info_license.as_ref().map_or_else(
        || quote! { None },
        |license| {
            let name_lit = &license.name;
            let url_expr = license.url.as_ref().map_or_else(
                || quote! { None },
                |url| {
                    let url_str = url.value();
                    quote! { Some(#url_str.to_string()) }
                },
            );
            quote! {
                Some(protofolio::License {
                    name: #name_lit.to_string(),
                    url: #url_expr,
                })
            }
        },
    );

    let info_terms_of_service_expr = info_terms_of_service.as_ref().map_or_else(
        || quote! { None },
        |terms| {
            let terms_str = terms.as_str();
            quote! { Some(#terms_str.to_string()) }
        },
    );

    // Generate code for servers
    let servers_code = generate_servers_code(&servers);

    // Generate code for security schemes
    let security_schemes_code = generate_security_schemes_code(&security_schemes);

    // Generate code for channels
    let channels_code = generate_channels_code(&channels);

    // Generate code for messages (both panic and try versions)
    let messages_code = generate_messages_code(&messages, ident);
    let messages_try_code = generate_messages_try_code(&messages, ident);

    // Generate code for operations (both panic and try versions)
    let operations_code_vec = generate_operations_code(&operations, ident);
    let operations_try_code_vec = generate_operations_try_code(&operations, ident);

    // Generate operations map code
    let operations_code = generate_operations_map_code(&operations_code_vec);
    let operations_code_try = generate_operations_map_try_code(&operations_try_code_vec);

    // Generate code for tags
    let tags_code = generate_tags_code(&tags);

    // Generate the impl block
    Ok(generate_impl_block(
        ident,
        &info_title,
        &info_version,
        info_desc_expr,
        info_external_docs_expr,
        info_contact_expr,
        info_license_expr,
        info_terms_of_service_expr,
        &servers_code,
        security_schemes_code,
        &channels_code,
        &messages_code,
        &messages_try_code,
        operations_code,
        operations_code_try,
        tags_code,
    ))
}
