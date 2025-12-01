//! Code generation for AsyncApiMessage derive macro

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitStr};

/// Generate optional field code
pub fn generate_optional_field_code(option: &Option<LitStr>) -> TokenStream {
    option.as_ref().map_or_else(
        || quote! { None },
        |s| quote! { Some(#s) },
    )
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
        }
    }
}

