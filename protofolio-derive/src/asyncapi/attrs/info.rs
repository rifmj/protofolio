//! Parser structures and implementations for info and external documentation attributes

use crate::parse_utils::parse_optional_comma;
use syn::{parse::Parse, Error, LitStr, Token};

/// Parser structure for external documentation attributes
pub struct ExternalDocsAttrs {
    pub url: LitStr,
    pub description: Option<LitStr>,
}

/// Parser structure for info attributes
pub struct InfoAttrs {
    pub title: Option<LitStr>,
    pub version: Option<LitStr>,
    pub description: Option<LitStr>,
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

impl Parse for InfoAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut title = None;
        let mut version = None;
        let mut description = None;
        let mut external_docs = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                let ident_str = ident.to_string();

                if ident_str == "external_docs" || ident_str == "externalDocs" {
                    let content;
                    syn::parenthesized!(content in input);
                    external_docs = Some(content.parse()?);
                } else {
                    input.parse::<Token![=]>()?;
                    let lit: LitStr = input.parse()?;

                    match ident_str.as_str() {
                        "title" => title = Some(lit),
                        "version" => version = Some(lit),
                        "description" => description = Some(lit),
                        _ => {
                            return Err(Error::new(
                                ident.span(),
                                format!(
                                    "Unknown info attribute '{ident}'. Expected one of: title, version, description, external_docs\n\nExample: #[asyncapi(info(title = \"My API\", version = \"1.0.0\", description = \"API description\", external_docs(url = \"https://example.com/docs\")))]"
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
            title,
            version,
            description,
            external_docs,
        })
    }
}
