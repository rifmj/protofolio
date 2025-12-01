//! Parser structure and implementation for tag attributes

use crate::parse_utils::parse_optional_comma;
use syn::{parse::Parse, Error, LitStr, Token};

/// Parser structure for tag attributes
pub struct TagAttrs {
    pub name: LitStr,
    pub description: Option<LitStr>,
}

impl Parse for TagAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut description = None;

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let lit: LitStr = input.parse()?;

            match ident.to_string().as_str() {
                "name" => name = Some(lit),
                "description" => description = Some(lit),
                _ => {
                    return Err(Error::new(
                        ident.span(),
                        format!(
                            "Unknown tag attribute '{ident}'. Expected one of: name, description\n\nExample: #[asyncapi(tags((name = \"orders\", description = \"Order-related operations\")))]"
                        ),
                    ));
                }
            }

            parse_optional_comma(input)?;
        }

        Ok(Self {
            name: name.ok_or_else(|| input.error("tag requires 'name'"))?,
            description,
        })
    }
}
