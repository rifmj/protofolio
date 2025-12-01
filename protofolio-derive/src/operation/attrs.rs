//! Parser structures and implementations for AsyncApiOperation attributes

use crate::parse_utils::{parse_optional_comma, parse_tags_array};
use syn::{parse::Parse, Error, LitStr, Token};

/// Parser structure for operation attributes
pub struct OperationAttrs {
    pub operation_id: Option<LitStr>,
    pub action: Option<LitStr>,
    pub channel: Option<LitStr>,
    pub messages: Vec<syn::Path>,
    pub summary: Option<LitStr>,
    pub description: Option<LitStr>,
    pub tags: Option<Vec<LitStr>>,
}

impl Parse for OperationAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut operation_id = None;
        let mut action = None;
        let mut channel = None;
        let mut messages = Vec::new();
        let mut summary = None;
        let mut description = None;
        let mut tags = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();

            if lookahead.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                let ident_str = ident.to_string();

                if ident == "messages" {
                    // messages can be either messages = (...) or messages(...)
                    let lookahead2 = input.lookahead1();
                    if lookahead2.peek(Token![=]) {
                        input.parse::<Token![=]>()?;
                    }
                    let content;
                    syn::parenthesized!(content in input);
                    while !content.is_empty() {
                        let message_path: syn::Path = content.parse()?;
                        messages.push(message_path);
                        if content.peek(Token![,]) {
                            content.parse::<Token![,]>()?;
                        }
                    }
                } else {
                    // Check if this is a tags array
                    if ident == "tags" {
                        input.parse::<Token![=]>()?;
                        tags = Some(parse_tags_array(input)?);
                    } else {
                        // Parse the = and value
                        input.parse::<Token![=]>()?;
                        let lit: LitStr = input.parse()?;
                        let span = ident.span();

                        match ident_str.as_str() {
                            "id" | "operationId" | "operation_id" => operation_id = Some(lit),
                            "action" => action = Some(lit),
                            "channel" => channel = Some(lit),
                            "summary" => summary = Some(lit),
                            "description" => description = Some(lit),
                            _ => {
                                return Err(Error::new(
                                    span,
                                    format!(
                                        "Unknown attribute '{}'. Expected one of: id, action, channel, messages, summary, description, tags\n\nExample: #[asyncapi(id = \"op-1\", action = \"send\", channel = \"events\", messages(MyMessage), summary = \"Operation summary\", tags = [\"tag1\"])]",
                                        ident_str
                                    ),
                                ));
                            }
                        }
                    }
                }
            } else {
                return Err(lookahead.error());
            }

            parse_optional_comma(input)?;
        }

        Ok(Self {
            operation_id,
            action,
            channel,
            messages,
            summary,
            description,
            tags,
        })
    }
}

