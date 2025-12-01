//! Code generation for AsyncApiOperation derive macro

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

/// Generate the complete impl block for AsyncApiOperation
pub fn generate_impl_block(
    ident: &Ident,
    channel_lit: &LitStr,
    operation_id_lit: &LitStr,
    action_lit: &LitStr,
    messages: &[syn::Path],
    summary_opt: TokenStream,
    desc_opt: TokenStream,
    tags_opt: TokenStream,
) -> TokenStream {
    quote! {
        impl #ident {
            /// Channel name constant for compile-time validation
            pub const CHANNEL: &'static str = #channel_lit;

            /// Message type names constant for compile-time validation
            pub const MESSAGE_TYPES: &'static [&'static str] = &[#(stringify!(#messages)),*];
        }

        impl protofolio::AsyncApiOperation for #ident {
            fn operation_id() -> &'static str {
                #operation_id_lit
            }

            fn action() -> &'static str {
                #action_lit
            }

            fn channel() -> &'static str {
                #channel_lit
            }

            fn message_types() -> &'static [&'static str] {
                &[#(stringify!(#messages)),*]
            }

            fn message_names() -> Vec<String> {
                vec![#(stringify!(#messages).to_string()),*]
            }

            fn summary() -> Option<&'static str> {
                #summary_opt
            }

            fn description() -> Option<&'static str> {
                #desc_opt
            }

            fn tags() -> Option<Vec<protofolio::Tag>> {
                #tags_opt
            }
        }
    }
}

