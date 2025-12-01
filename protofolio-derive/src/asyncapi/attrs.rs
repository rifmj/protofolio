//! Parser structures and implementations for AsyncApi attributes

use crate::parse_utils::parse_optional_comma;
use syn::{parse::Parse, Error, LitStr, Token};

/// Parser structure for asyncapi attributes
pub struct AsyncApiAttrs {
    pub info: Option<InfoAttrs>,
    pub servers: Vec<ServerAttrs>,
    pub security_schemes: Vec<SecuritySchemeAttrs>,
    pub channels: Vec<LitStr>,
    pub messages: Vec<syn::Path>,
    pub operations: Vec<syn::Path>,
}

/// Parser structure for info attributes
pub struct InfoAttrs {
    pub title: Option<LitStr>,
    pub version: Option<LitStr>,
    pub description: Option<LitStr>,
}

/// Parser structure for server attributes
pub struct ServerAttrs {
    pub name: LitStr,
    pub url: LitStr,
    pub protocol: LitStr,
    pub security: Vec<Vec<LitStr>>, // List of security requirement lists
}

/// Parser structure for security scheme attributes
pub struct SecuritySchemeAttrs {
    pub name: LitStr,
    pub scheme_type: LitStr, // "userPassword", "apiKey", "http", "oauth2", etc.
    pub description: Option<LitStr>,
    // Additional fields for specific scheme types
    pub scheme: Option<LitStr>, // For http: "basic", "bearer", etc.
    pub bearer_format: Option<LitStr>, // For http bearer
    pub in_: Option<LitStr>, // For apiKey/httpApiKey: "header", "query", "cookie"
    pub name_param: Option<LitStr>, // For httpApiKey: parameter name
    pub open_id_connect_url: Option<LitStr>, // For openIdConnect
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

impl Parse for InfoAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut title = None;
        let mut version = None;
        let mut description = None;
        
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                input.parse::<Token![=]>()?;
                let lit: LitStr = input.parse()?;
                
                match ident.to_string().as_str() {
                    "title" => title = Some(lit),
                    "version" => version = Some(lit),
                    "description" => description = Some(lit),
                    _ => {
                        return Err(Error::new(
                            ident.span(),
                            format!(
                                "Unknown info attribute '{ident}'. Expected one of: title, version, description\n\nExample: #[asyncapi(info(title = \"My API\", version = \"1.0.0\", description = \"API description\"))]"
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
            title,
            version,
            description,
        })
    }
}

impl Parse for ServerAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut url = None;
        let mut protocol = None;
        let mut security = Vec::new();
        
        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            let ident_str = ident.to_string();
            
            if ident_str == "security" {
                input.parse::<Token![=]>()?;
                let content;
                syn::bracketed!(content in input);
                let mut req_list = Vec::new();
                while !content.is_empty() {
                    let scheme_name: LitStr = content.parse()?;
                    req_list.push(scheme_name);
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }
                }
                security.push(req_list);
            } else {
                input.parse::<Token![=]>()?;
                let lit: LitStr = input.parse()?;
                
                match ident_str.as_str() {
                    "name" => name = Some(lit),
                    "url" => url = Some(lit),
                    "protocol" => protocol = Some(lit),
                    _ => {
                        return Err(Error::new(
                            ident.span(),
                            format!(
                                "Unknown server attribute '{ident}'. Expected one of: name, url, protocol, security\n\nExample: #[asyncapi(servers((name = \"nats\", url = \"nats://localhost:4222\", protocol = \"nats\", security = [\"userPassword\"])))]"
                            ),
                        ));
                    }
                }
            }
            
            parse_optional_comma(input)?;
        }
        
        Ok(Self {
            name: name.ok_or_else(|| input.error("server requires 'name'"))?,
            url: url.ok_or_else(|| input.error("server requires 'url'"))?,
            protocol: protocol.ok_or_else(|| input.error("server requires 'protocol'"))?,
            security,
        })
    }
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
        
        Ok(Self {
            name: name.ok_or_else(|| input.error("security scheme requires 'name'"))?,
            scheme_type: scheme_type.ok_or_else(|| input.error("security scheme requires 'type'"))?,
            description,
            scheme,
            bearer_format,
            in_,
            name_param,
            open_id_connect_url,
        })
    }
}

