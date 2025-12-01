//! Parser structure and implementation for `AsyncApi` attributes

use crate::parse_utils::parse_optional_comma;
use syn::{parse::Parse, Error, LitStr, Token};

use super::{info::InfoAttrs, security::SecuritySchemeAttrs, server::ServerAttrs};

/// Parser structure for asyncapi attributes
pub struct AsyncApiAttrs {
    pub info: Option<InfoAttrs>,
    pub servers: Vec<ServerAttrs>,
    pub security_schemes: Vec<SecuritySchemeAttrs>,
    pub channels: Vec<LitStr>,
    pub messages: Vec<syn::Path>,
    pub operations: Vec<syn::Path>,
}

impl Parse for AsyncApiAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut info = None;
        let mut servers = Vec::new();
        let mut security_schemes = Vec::new();
        let mut channels = Vec::new();
        let mut messages = Vec::new();
        let mut operations = Vec::new();

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            let ident_str = ident.to_string();

            if ident_str == "info" {
                let content;
                syn::parenthesized!(content in input);
                info = Some(content.parse()?);
            } else if ident_str == "servers" {
                let content;
                syn::parenthesized!(content in input);
                while !content.is_empty() {
                    let content2;
                    syn::parenthesized!(content2 in content);
                    servers.push(content2.parse()?);
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }
                }
            } else if ident_str == "security_schemes" || ident_str == "securitySchemes" {
                let content;
                syn::parenthesized!(content in input);
                while !content.is_empty() {
                    let content2;
                    syn::parenthesized!(content2 in content);
                    security_schemes.push(content2.parse()?);
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }
                }
            } else if ident_str == "channels" {
                let content;
                syn::parenthesized!(content in input);
                while !content.is_empty() {
                    channels.push(content.parse()?);
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }
                }
            } else if ident_str == "messages" {
                let content;
                syn::parenthesized!(content in input);
                while !content.is_empty() {
                    messages.push(content.parse()?);
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }
                }
            } else if ident_str == "operations" {
                let content;
                syn::parenthesized!(content in input);
                while !content.is_empty() {
                    operations.push(content.parse()?);
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }
                }
            } else {
                return Err(Error::new_spanned(
                        &ident,
                        format!(
                            "Unexpected identifier '{ident_str}'. Expected one of: info, servers, security_schemes, channels, messages, operations\n\nExample: #[asyncapi(info(title = \"...\", version = \"...\"), channels(\"channel1\"), messages(Message1))]"
                        ),
                    ));
            }

            parse_optional_comma(input)?;
        }

        Ok(Self {
            info,
            servers,
            security_schemes,
            channels,
            messages,
            operations,
        })
    }
}
