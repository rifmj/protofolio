//! Parser structure and implementation for security scheme attributes

use crate::parse_utils::parse_optional_comma;
use syn::{parse::Parse, Error, LitStr, Token};

/// Parser structure for security scheme attributes
pub struct SecuritySchemeAttrs {
    pub name: LitStr,
    pub scheme_type: LitStr, // "userPassword", "apiKey", "http", "oauth2", etc.
    pub description: Option<LitStr>,
    // Additional fields for specific scheme types
    pub scheme: Option<LitStr>,        // For http: "basic", "bearer", etc.
    pub bearer_format: Option<LitStr>, // For http bearer
    pub in_: Option<LitStr>,           // For apiKey/httpApiKey: "header", "query", "cookie"
    pub name_param: Option<LitStr>,    // For httpApiKey: parameter name
    pub open_id_connect_url: Option<LitStr>, // For openIdConnect
}

impl Parse for SecuritySchemeAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut scheme_type = None;
        let mut description = None;
        let mut scheme = None;
        let mut bearer_format = None;
        let mut in_ = None;
        let mut name_param = None;
        let mut open_id_connect_url = None;

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let lit: LitStr = input.parse()?;

            match ident.to_string().as_str() {
                "name" => name = Some(lit),
                "type" => scheme_type = Some(lit),
                "description" => description = Some(lit),
                "scheme" => scheme = Some(lit),
                "bearer_format" | "bearerFormat" => bearer_format = Some(lit),
                "in" | "in_" => in_ = Some(lit),
                "name_param" | "nameParam" => name_param = Some(lit),
                "open_id_connect_url" | "openIdConnectUrl" => open_id_connect_url = Some(lit),
                _ => {
                    return Err(Error::new(
                        ident.span(),
                        format!(
                            "Unknown security scheme attribute '{ident}'. Expected one of: name, type, description, scheme, bearer_format, in, name_param, open_id_connect_url\n\nExample: #[asyncapi(security_schemes((name = \"userPassword\", type = \"userPassword\", description = \"User and password authentication\")))]"
                        ),
                    ));
                }
            }

            parse_optional_comma(input)?;
        }

        let name = name.ok_or_else(|| input.error("security scheme requires 'name'"))?;
        let scheme_type_val =
            scheme_type.ok_or_else(|| input.error("security scheme requires 'type'"))?;
        let scheme_type_str = scheme_type_val.value();

        // Validate required fields based on scheme type
        match scheme_type_str.as_str() {
            "http" => {
                if scheme.is_none() {
                    return Err(Error::new(
                        scheme_type_val.span(),
                        "http security scheme requires 'scheme' attribute (e.g., 'basic', 'bearer')"
                    ));
                }
            }
            "httpApiKey" => {
                if name_param.is_none() {
                    return Err(Error::new(
                        scheme_type_val.span(),
                        "httpApiKey security scheme requires 'name_param' attribute",
                    ));
                }
                if in_.is_none() {
                    return Err(Error::new(
                        scheme_type_val.span(),
                        "httpApiKey security scheme requires 'in' attribute (e.g., 'header', 'query', 'cookie')"
                    ));
                }
            }
            "openIdConnect" => {
                if open_id_connect_url.is_none() {
                    return Err(Error::new(
                        scheme_type_val.span(),
                        "openIdConnect security scheme requires 'open_id_connect_url' attribute",
                    ));
                }
            }
            _ => {}
        }

        Ok(Self {
            name,
            scheme_type: scheme_type_val,
            description,
            scheme,
            bearer_format,
            in_,
            name_param,
            open_id_connect_url,
        })
    }
}
