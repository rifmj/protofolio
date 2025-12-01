//! Parser structures and implementations for server attributes

use crate::parse_utils::parse_optional_comma;
use syn::{parse::Parse, Error, LitStr, Token};

/// Parser structure for server variable attributes
pub struct ServerVariableAttrs {
    pub name: LitStr,
    pub default: Option<LitStr>,
    pub description: Option<LitStr>,
    pub enum_values: Option<Vec<LitStr>>,
    pub examples: Option<Vec<LitStr>>,
}

/// Parser structure for server attributes
pub struct ServerAttrs {
    pub name: LitStr,
    pub url: LitStr,
    pub protocol: LitStr,
    pub security: Vec<Vec<LitStr>>, // List of security requirement lists
    pub variables: Vec<ServerVariableAttrs>,
}

impl Parse for ServerVariableAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut default = None;
        let mut description = None;
        let mut enum_values = None;
        let mut examples = None;

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            let ident_str = ident.to_string();

            if ident_str == "enum" || ident_str == "enum_values" {
                input.parse::<Token![=]>()?;
                let content;
                syn::bracketed!(content in input);
                let mut enum_list = Vec::new();
                while !content.is_empty() {
                    let enum_val: LitStr = content.parse()?;
                    enum_list.push(enum_val);
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }
                }
                enum_values = Some(enum_list);
            } else if ident_str == "examples" {
                input.parse::<Token![=]>()?;
                let content;
                syn::bracketed!(content in input);
                let mut examples_list = Vec::new();
                while !content.is_empty() {
                    let example_val: LitStr = content.parse()?;
                    examples_list.push(example_val);
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }
                }
                examples = Some(examples_list);
            } else {
                input.parse::<Token![=]>()?;
                let lit: LitStr = input.parse()?;

                match ident_str.as_str() {
                    "name" => name = Some(lit),
                    "default" => default = Some(lit),
                    "description" => description = Some(lit),
                    _ => {
                        return Err(Error::new(
                            ident.span(),
                            format!(
                                "Unknown server variable attribute '{ident}'. Expected one of: name, default, description, enum, examples\n\nExample: #[asyncapi(servers((name = \"nats\", url = \"nats://{{host}}:{{port}}\", protocol = \"nats\", variables = [(name = \"host\", default = \"localhost\"), (name = \"port\", default = \"4222\", enum = [\"4222\", \"4223\"])])))]"
                            ),
                        ));
                    }
                }
            }

            parse_optional_comma(input)?;
        }

        Ok(Self {
            name: name.ok_or_else(|| input.error("server variable requires 'name'"))?,
            default,
            description,
            enum_values,
            examples,
        })
    }
}

impl Parse for ServerAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut url = None;
        let mut protocol = None;
        let mut security = Vec::new();
        let mut variables = Vec::new();

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
            } else if ident_str == "variables" {
                input.parse::<Token![=]>()?;
                let content;
                syn::bracketed!(content in input);
                while !content.is_empty() {
                    let var_content;
                    syn::parenthesized!(var_content in content);
                    variables.push(var_content.parse()?);
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }
                }
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
                                "Unknown server attribute '{ident}'. Expected one of: name, url, protocol, security, variables\n\nExample: #[asyncapi(servers((name = \"nats\", url = \"nats://localhost:4222\", protocol = \"nats\", security = [\"userPassword\"], variables = [(name = \"host\", default = \"localhost\")])))]"
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
            variables,
        })
    }
}
