//! Parser structures and implementations for AsyncApiMessage attributes

use crate::parse_utils::{parse_optional_comma, parse_tags_array};
use syn::{parse::Parse, Error, LitStr, Token};

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
        
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            
            if lookahead.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                let ident_str = ident.to_string();
                
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
                        "channel" => channel = Some(lit),
                        "summary" => summary = Some(lit),
                        "description" => description = Some(lit),
                        "messageId" | "message_id" => message_id = Some(lit),
                        "name" => name = Some(lit),
                        "title" => title = Some(lit),
                        "contentType" | "content_type" => content_type = Some(lit),
                        _ => {
                            return Err(Error::new(
                                span,
                                format!(
                                    "Unknown attribute '{ident_str}'. Expected one of: channel, summary, description, messageId, name, title, contentType, tags\n\nExample: #[asyncapi(channel = \"events\", messageId = \"event-v1\", name = \"Event\", summary = \"An event\", tags = [\"events\"])]"
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
        })
    }
}

