//! Parser structures and implementations for AsyncApiOperation attributes

use crate::parse_utils::{parse_optional_comma, parse_tags_array};
use syn::{parse::Parse, Error, LitStr, Token};

/// Parser structure for external documentation attributes
pub struct ExternalDocsAttrs {
    pub url: LitStr,
    pub description: Option<LitStr>,
}

/// Parser structure for operation attributes
pub struct OperationAttrs {
    pub operation_id: Option<LitStr>,
    pub action: Option<LitStr>,
    pub channel: Option<LitStr>,
    pub messages: Vec<syn::Path>,
    pub summary: Option<LitStr>,
    pub description: Option<LitStr>,
    pub tags: Option<Vec<LitStr>>,
    pub external_docs: Option<ExternalDocsAttrs>,
}

impl Parse for ExternalDocsAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut url = None;
        let mut description = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                input.parse::<Token![=]>()?;
                let lit: LitStr = input.parse()?;

                match ident.to_string().as_str() {
                    "url" => url = Some(lit),
                    "description" => description = Some(lit),
                    _ => {
                        return Err(Error::new(
                            ident.span(),
                            format!(
                                "Unknown external_docs attribute '{ident}'. Expected one of: url, description\n\nExample: #[asyncapi(external_docs(url = \"https://example.com/docs\", description = \"External documentation\"))]"
                            ),
                        ));
                    }
                }
            } else {
                return Err(lookahead.error());
            }

            parse_optional_comma(input)?;
        }

        Ok(Self {
            url: url.ok_or_else(|| input.error("external_docs requires 'url'"))?,
            description,
        })
    }
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
        let mut external_docs = None;

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
                } else if ident_str == "external_docs" || ident_str == "externalDocs" {
                    let content;
                    syn::parenthesized!(content in input);
                    external_docs = Some(content.parse()?);
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
                                        "Unknown attribute '{}'. Expected one of: id, action, channel, messages, summary, description, tags, external_docs\n\nExample: #[asyncapi(id = \"op-1\", action = \"send\", channel = \"events\", messages(MyMessage), summary = \"Operation summary\", tags = [\"tag1\"], external_docs(url = \"https://example.com/docs\"))]",
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
            external_docs,
        })
    }
}
