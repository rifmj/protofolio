//! Code generation for AsyncApiMessage derive macro

use crate::message::attrs::{CorrelationIdAttrs, ExternalDocsAttrs};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitStr, Path};

/// Generate optional field code
pub fn generate_optional_field_code(option: &Option<LitStr>) -> TokenStream {
    option
        .as_ref()
        .map_or_else(|| quote! { None }, |s| quote! { Some(#s) })
}

/// Generate tags code
pub fn generate_tags_code(tags: &Option<Vec<LitStr>>) -> TokenStream {
    tags.as_ref().map_or_else(
        || quote! { None },
        |tag_list| {
            let tag_exprs: Vec<_> = tag_list
                .iter()
                .map(|tag| {
                    quote! {
                        protofolio::Tag {
                            name: #tag.to_string(),
                            description: None,
                        }
                    }
                })
                .collect();
            quote! { Some(vec![#(#tag_exprs),*]) }
        },
    )
}

/// Generate external documentation code
pub fn generate_external_docs_code(external_docs: &Option<ExternalDocsAttrs>) -> TokenStream {
    external_docs.as_ref().map_or_else(
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
    )
}

/// Generate examples code
/// Supports both single example and multiple examples
pub fn generate_examples_code(
    example: &Option<LitStr>,
    examples: &Option<Vec<LitStr>>,
) -> TokenStream {
    // If both are provided, examples takes precedence
    if let Some(examples_list) = examples {
        let example_exprs: Vec<_> = examples_list
            .iter()
            .map(|ex| {
                quote! {
                    serde_json::from_str(#ex).unwrap_or_else(|e| {
                        panic!("Failed to parse example JSON '{}': {}", #ex, e)
                    })
                }
            })
            .collect();
        quote! { Some(vec![#(#example_exprs),*]) }
    } else if let Some(ex) = example {
        quote! {
            Some(vec![
                serde_json::from_str(#ex).unwrap_or_else(|e| {
                    panic!("Failed to parse example JSON '{}': {}", #ex, e)
                })
            ])
        }
    } else {
        quote! { None }
    }
}

/// Generate headers code
/// Headers is a type path that should implement JsonSchema
pub fn generate_headers_code(headers: &Option<Path>) -> TokenStream {
    headers.as_ref().map_or_else(
        || quote! { None },
        |headers_type| {
            quote! {
                {
                    use schemars::JsonSchema;
                    match protofolio::schema_for_type::<#headers_type>() {
                        Ok(schema) => Some(protofolio::MessagePayload { schema }),
                        Err(e) => {
                            panic!(
                                "Failed to generate schema for headers type '{}': {}. Ensure the type implements JsonSchema trait (derive JsonSchema).",
                                stringify!(#headers_type),
                                e
                            )
                        }
                    }
                }
            }
        },
    )
}

/// Generate correlation ID code
pub fn generate_correlation_id_code(correlation_id: &Option<CorrelationIdAttrs>) -> TokenStream {
    correlation_id.as_ref().map_or_else(
        || quote! { None },
        |corr_id| {
            let location_lit = &corr_id.location;
            let desc_expr = corr_id.description.as_ref().map_or_else(
                || quote! { None },
                |desc| {
                    let desc_str = desc.value();
                    quote! { Some(#desc_str.to_string()) }
                },
            );
            quote! {
                Some(protofolio::CorrelationId {
                    location: #location_lit.to_string(),
                    description: #desc_expr,
                })
            }
        },
    )
}

/// Generate the complete impl block for AsyncApiMessage
pub fn generate_impl_block(
    ident: &Ident,
    channel_lit: &LitStr,
    summary_opt: TokenStream,
    desc_opt: TokenStream,
    message_id_opt: TokenStream,
    name_opt: TokenStream,
    title_opt: TokenStream,
    content_type_opt: TokenStream,
    tags_opt: TokenStream,
    external_docs_opt: TokenStream,
    examples_opt: TokenStream,
    headers_opt: TokenStream,
    correlation_id_opt: TokenStream,
) -> TokenStream {
    quote! {
        impl #ident {
            /// Get the channel name for this message
            pub fn channel() -> &'static str {
                #channel_lit
            }

            /// Channel name constant for compile-time validation
            pub const CHANNEL: &'static str = #channel_lit;

            /// Get the summary for this message
            pub fn summary() -> Option<&'static str> {
                #summary_opt
            }

            /// Get the description for this message
            pub fn description() -> Option<&'static str> {
                #desc_opt
            }

            /// Get the message ID for this message
            pub fn message_id() -> Option<&'static str> {
                #message_id_opt
            }

            /// Get the name for this message
            pub fn name() -> Option<&'static str> {
                #name_opt
            }

            /// Get the title for this message
            pub fn title() -> Option<&'static str> {
                #title_opt
            }

            /// Get the content type for this message
            pub fn content_type() -> Option<&'static str> {
                #content_type_opt
            }

            /// Get the tags for this message
            pub fn tags() -> Option<Vec<protofolio::Tag>> {
                #tags_opt
            }

            /// Get the external documentation for this message
            pub fn external_docs() -> Option<protofolio::ExternalDocumentation> {
                #external_docs_opt
            }

            /// Get the examples for this message
            pub fn examples() -> Option<Vec<serde_json::Value>> {
                #examples_opt
            }

            /// Get the headers schema for this message
            pub fn headers() -> Option<protofolio::MessagePayload> {
                #headers_opt
            }

            /// Get the correlation ID for this message
            pub fn correlation_id() -> Option<protofolio::CorrelationId> {
                #correlation_id_opt
            }
        }
    }
}
