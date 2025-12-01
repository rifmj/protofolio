//! Parser structures and implementations for info and external documentation attributes

use crate::parse_utils::parse_optional_comma;
use syn::{parse::Parse, Error, LitStr, Token};

/// Parser structure for external documentation attributes
pub struct ExternalDocsAttrs {
    pub url: LitStr,
    pub description: Option<LitStr>,
}

/// Parser structure for contact attributes
pub struct ContactAttrs {
    pub name: Option<LitStr>,
    pub url: Option<LitStr>,
    pub email: Option<LitStr>,
}

/// Parser structure for license attributes
pub struct LicenseAttrs {
    pub name: LitStr,
    pub url: Option<LitStr>,
}

/// Parser structure for info attributes
pub struct InfoAttrs {
    pub title: Option<LitStr>,
    pub version: Option<LitStr>,
    pub description: Option<LitStr>,
    pub external_docs: Option<ExternalDocsAttrs>,
    pub contact: Option<ContactAttrs>,
    pub license: Option<LicenseAttrs>,
    pub terms_of_service: Option<LitStr>,
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

impl Parse for ContactAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut url = None;
        let mut email = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                input.parse::<Token![=]>()?;
                let lit: LitStr = input.parse()?;

                match ident.to_string().as_str() {
                    "name" => name = Some(lit),
                    "url" => url = Some(lit),
                    "email" => email = Some(lit),
                    _ => {
                        return Err(Error::new(
                            ident.span(),
                            format!(
                                "Unknown contact attribute '{ident}'. Expected one of: name, url, email\n\nExample: #[asyncapi(info(contact(name = \"API Support\", email = \"support@example.com\", url = \"https://example.com/contact\")))]"
                            ),
                        ));
                    }
                }
            } else {
                return Err(lookahead.error());
            }

            parse_optional_comma(input)?;
        }

        Ok(Self { name, url, email })
    }
}

impl Parse for LicenseAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut url = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                input.parse::<Token![=]>()?;
                let lit: LitStr = input.parse()?;

                match ident.to_string().as_str() {
                    "name" => name = Some(lit),
                    "url" => url = Some(lit),
                    _ => {
                        return Err(Error::new(
                            ident.span(),
                            format!(
                                "Unknown license attribute '{ident}'. Expected one of: name, url\n\nExample: #[asyncapi(info(license(name = \"Apache 2.0\", url = \"https://www.apache.org/licenses/LICENSE-2.0\")))]"
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
            name: name.ok_or_else(|| input.error("license requires 'name'"))?,
            url,
        })
    }
}

impl Parse for InfoAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut title = None;
        let mut version = None;
        let mut description = None;
        let mut external_docs = None;
        let mut contact = None;
        let mut license = None;
        let mut terms_of_service = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                let ident_str = ident.to_string();

                if ident_str == "external_docs" || ident_str == "externalDocs" {
                    let content;
                    syn::parenthesized!(content in input);
                    external_docs = Some(content.parse()?);
                } else if ident_str == "contact" {
                    let content;
                    syn::parenthesized!(content in input);
                    contact = Some(content.parse()?);
                } else if ident_str == "license" {
                    let content;
                    syn::parenthesized!(content in input);
                    license = Some(content.parse()?);
                } else {
                    input.parse::<Token![=]>()?;
                    let lit: LitStr = input.parse()?;

                    match ident_str.as_str() {
                        "title" => title = Some(lit),
                        "version" => version = Some(lit),
                        "description" => description = Some(lit),
                        "terms_of_service" | "termsOfService" => terms_of_service = Some(lit),
                        _ => {
                            return Err(Error::new(
                                ident.span(),
                                format!(
                                    "Unknown info attribute '{ident}'. Expected one of: title, version, description, external_docs, contact, license, terms_of_service\n\nExample: #[asyncapi(info(title = \"My API\", version = \"1.0.0\", description = \"API description\", contact(name = \"Support\", email = \"support@example.com\"), license(name = \"Apache 2.0\"), terms_of_service = \"https://example.com/terms\"))]"
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
            contact,
            license,
            terms_of_service,
        })
    }
}
