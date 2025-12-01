//! Parser structures and implementations for `AsyncApiMessage` attributes

use crate::parse_utils::{parse_examples_array, parse_optional_comma, parse_tags_array};
use syn::{parse::Parse, Error, LitStr, Path, Token};

/// Parser structure for external documentation attributes
pub struct ExternalDocsAttrs {
    pub url: LitStr,
    pub description: Option<LitStr>,
}

/// Parser structure for correlation ID attributes
pub struct CorrelationIdAttrs {
    pub location: LitStr,
    pub description: Option<LitStr>,
}

/// Parser structure for message attributes
pub struct MessageAttrs {
    pub channel: Option<LitStr>,
    pub summary: Option<LitStr>,
    pub description: Option<LitStr>,
    pub message_id: Option<LitStr>,
    pub name: Option<LitStr>,
    pub title: Option<LitStr>,
    pub content_type: Option<LitStr>,
    pub tags: Option<Vec<LitStr>>,
    pub external_docs: Option<ExternalDocsAttrs>,
    pub example: Option<LitStr>,
    pub examples: Option<Vec<LitStr>>,
    pub headers: Option<Path>,
    pub correlation_id: Option<CorrelationIdAttrs>,
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

impl Parse for CorrelationIdAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut location = None;
        let mut description = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                input.parse::<Token![=]>()?;
                let lit: LitStr = input.parse()?;

                match ident.to_string().as_str() {
                    "location" => location = Some(lit),
                    "description" => description = Some(lit),
                    _ => {
                        return Err(Error::new(
                            ident.span(),
                            format!(
                                "Unknown correlation_id attribute '{ident}'. Expected one of: location, description\n\nExample: #[asyncapi(correlation_id(location = \"$message.header#/correlationId\", description = \"Correlation ID\"))]"
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
            location: location.ok_or_else(|| input.error("correlation_id requires 'location'"))?,
            description,
        })
    }
}

impl Parse for MessageAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut channel = None;
        let mut summary = None;
        let mut description = None;
        let mut message_id = None;
        let mut name = None;
        let mut title = None;
        let mut content_type = None;
        let mut tags = None;
        let mut external_docs = None;
        let mut example = None;
        let mut examples = None;
        let mut headers = None;
        let mut correlation_id = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();

            if lookahead.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                let ident_str = ident.to_string();

                // Check if this is a tags array
                if ident == "tags" {
                    input.parse::<Token![=]>()?;
                    tags = Some(parse_tags_array(input)?);
                } else if ident == "examples" {
                    input.parse::<Token![=]>()?;
                    examples = Some(parse_examples_array(input)?);
                } else if ident_str == "external_docs" || ident_str == "externalDocs" {
                    let content;
                    syn::parenthesized!(content in input);
                    external_docs = Some(content.parse()?);
                } else if ident_str == "correlation_id" || ident_str == "correlationId" {
                    let content;
                    syn::parenthesized!(content in input);
                    correlation_id = Some(content.parse()?);
                } else if ident == "headers" {
                    input.parse::<Token![=]>()?;
                    headers = Some(input.parse::<Path>()?);
                } else {
                    // Parse the = and value
                    input.parse::<Token![=]>()?;
                    let span = ident.span();

                    match ident_str.as_str() {
                        "channel" => {
                            let lit: LitStr = input.parse()?;
                            channel = Some(lit);
                        }
                        "summary" => {
                            let lit: LitStr = input.parse()?;
                            summary = Some(lit);
                        }
                        "description" => {
                            let lit: LitStr = input.parse()?;
                            description = Some(lit);
                        }
                        "messageId" | "message_id" => {
                            let lit: LitStr = input.parse()?;
                            message_id = Some(lit);
                        }
                        "name" => {
                            let lit: LitStr = input.parse()?;
                            name = Some(lit);
                        }
                        "title" => {
                            let lit: LitStr = input.parse()?;
                            title = Some(lit);
                        }
                        "contentType" | "content_type" => {
                            let lit: LitStr = input.parse()?;
                            content_type = Some(lit);
                        }
                        "example" => {
                            let lit: LitStr = input.parse()?;
                            example = Some(lit);
                        }
                        _ => {
                            return Err(Error::new(
                                span,
                                format!(
                                    "Unknown attribute '{ident_str}'. Expected one of: channel, summary, description, messageId, name, title, contentType, tags, example, examples, headers, external_docs, correlation_id\n\nExample: #[asyncapi(channel = \"events\", messageId = \"event-v1\", name = \"Event\", summary = \"An event\", tags = [\"events\"], example = \"{{\\\"id\\\": \\\"123\\\"}}\", headers = MyHeaders, external_docs(url = \"https://example.com/docs\"), correlation_id(location = \"$message.header#/correlationId\"))]"
                                ),
                            ));
                        }
                    }
                }
            } else {
                return Err(lookahead.error());
            }

            parse_optional_comma(input)?;
        }

        Ok(Self {
            channel,
            summary,
            description,
            message_id,
            name,
            title,
            content_type,
            tags,
            external_docs,
            example,
            examples,
            headers,
            correlation_id,
        })
    }
}
