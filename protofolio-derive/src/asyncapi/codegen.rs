//! Code generation for servers, channels, and impl block in AsyncApi derive macro

use crate::asyncapi::attrs::{SecuritySchemeAttrs, ServerAttrs};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// Generate code for server initialization
pub fn generate_servers_code(servers: &[ServerAttrs]) -> Vec<TokenStream> {
    servers
        .iter()
        .map(|server| {
            let name_lit = &server.name;
            let url_lit = &server.url;
            let protocol_lit = &server.protocol;
            
            // Generate security requirements if present
            let security_expr = if server.security.is_empty() {
                quote! { None }
            } else {
                let security_reqs: Vec<TokenStream> = server.security
                    .iter()
                    .map(|req_list| {
                        let scheme_names: Vec<TokenStream> = req_list
                            .iter()
                            .map(|scheme_name| {
                                let name_str = scheme_name.value();
                                quote! {
                                    (#name_str.to_string(), vec![])
                                }
                            })
                            .collect();
                        quote! {
                            {
                                let mut req = std::collections::HashMap::new();
                                #(
                                    req.insert(#scheme_names);
                                )*
                                req
                            }
                        }
                    })
                    .collect();
                quote! {
                    Some(vec![
                        #(#security_reqs),*
                    ])
                }
            };
            
            quote! {
                builder = builder.server(
                    #name_lit.to_string(),
                    Server {
                        url: #url_lit.to_string(),
                        protocol: #protocol_lit.to_string(),
                        description: None,
                        security: #security_expr,
                    }
                );
            }
        })
        .collect()
}

/// Generate code for security scheme initialization
pub fn generate_security_schemes_code(schemes: &[SecuritySchemeAttrs]) -> TokenStream {
    if schemes.is_empty() {
        quote! {
            let security_schemes_map: Option<std::collections::HashMap<String, protofolio::SecurityScheme>> = None;
        }
    } else {
        let scheme_code: Vec<TokenStream> = schemes
            .iter()
            .map(|scheme| {
                let name_lit = &scheme.name;
                let scheme_type = scheme.scheme_type.value();
                let desc_expr = scheme.description.as_ref().map_or_else(
                    || quote! { None },
                    |desc| {
                        let desc_str = desc.value();
                        quote! { Some(#desc_str.to_string()) }
                    },
                );
                
                let scheme_expr = match scheme_type.as_str() {
                    "userPassword" => {
                        quote! {
                            protofolio::SecurityScheme::UserPassword {
                                description: #desc_expr,
                            }
                        }
                    }
                    "apiKey" => {
                        let in_expr = scheme.in_.as_ref().map_or_else(
                            || quote! { None },
                            |in_val| {
                                let in_str = in_val.value();
                                quote! { Some(#in_str.to_string()) }
                            },
                        );
                        quote! {
                            protofolio::SecurityScheme::ApiKey {
                                in_: #in_expr,
                                description: #desc_expr,
                            }
                        }
                    }
                    "http" => {
                        let scheme_val = scheme.scheme.as_ref().unwrap_or_else(|| {
                            panic!("http security scheme requires 'scheme' attribute (e.g., 'basic', 'bearer')");
                        });
                        let scheme_str = scheme_val.value();
                        let bearer_format_expr = scheme.bearer_format.as_ref().map_or_else(
                            || quote! { None },
                            |bf| {
                                let bf_str = bf.value();
                                quote! { Some(#bf_str.to_string()) }
                            },
                        );
                        quote! {
                            protofolio::SecurityScheme::Http {
                                scheme: #scheme_str.to_string(),
                                bearer_format: #bearer_format_expr,
                                description: #desc_expr,
                            }
                        }
                    }
                    "httpApiKey" => {
                        let name_param = scheme.name_param.as_ref().unwrap_or_else(|| {
                            panic!("httpApiKey security scheme requires 'name_param' attribute");
                        });
                        let name_param_str = name_param.value();
                        let in_val = scheme.in_.as_ref().unwrap_or_else(|| {
                            panic!("httpApiKey security scheme requires 'in' attribute (e.g., 'header', 'query', 'cookie')");
                        });
                        let in_str = in_val.value();
                        quote! {
                            protofolio::SecurityScheme::HttpApiKey {
                                name: #name_param_str.to_string(),
                                in_: #in_str.to_string(),
                                description: #desc_expr,
                            }
                        }
                    }
                    "openIdConnect" => {
                        let oidc_url = scheme.open_id_connect_url.as_ref().unwrap_or_else(|| {
                            panic!("openIdConnect security scheme requires 'open_id_connect_url' attribute");
                        });
                        let oidc_url_str = oidc_url.value();
                        quote! {
                            protofolio::SecurityScheme::OpenIdConnect {
                                open_id_connect_url: #oidc_url_str.to_string(),
                                description: #desc_expr,
                            }
                        }
                    }
                    "X509" => {
                        quote! {
                            protofolio::SecurityScheme::X509 {
                                description: #desc_expr,
                            }
                        }
                    }
                    "symmetricEncryption" => {
                        quote! {
                            protofolio::SecurityScheme::SymmetricEncryption {
                                description: #desc_expr,
                            }
                        }
                    }
                    "asymmetricEncryption" => {
                        quote! {
                            protofolio::SecurityScheme::AsymmetricEncryption {
                                description: #desc_expr,
                            }
                        }
                    }
                    "mutualTLS" => {
                        quote! {
                            protofolio::SecurityScheme::MutualTls {
                                description: #desc_expr,
                            }
                        }
                    }
                    "oauth2" => {
                        // OAuth2 is complex, for now we'll create a minimal structure
                        // Full OAuth2 flow configuration would require more attributes
                        quote! {
                            protofolio::SecurityScheme::OAuth2 {
                                flows: protofolio::OAuth2Flows {
                                    authorization_code: None,
                                    client_credentials: None,
                                    implicit: None,
                                    password: None,
                                },
                                description: #desc_expr,
                            }
                        }
                    }
                    _ => {
                        panic!("Unknown security scheme type: {}. Supported types: userPassword, apiKey, http, httpApiKey, oauth2, openIdConnect, X509, symmetricEncryption, asymmetricEncryption, mutualTLS", scheme_type);
                    }
                };
                
                quote! {
                    security_schemes_map.insert(
                        #name_lit.value().to_string(),
                        #scheme_expr
                    );
                }
            })
            .collect();
        
        quote! {
            let mut security_schemes_map: std::collections::HashMap<String, protofolio::SecurityScheme> = std::collections::HashMap::new();
            #(#scheme_code)*
            let security_schemes_map: Option<std::collections::HashMap<String, protofolio::SecurityScheme>> = Some(security_schemes_map);
        }
    }
}

/// Generate code for channel initialization
pub fn generate_channels_code(channels: &[syn::LitStr]) -> Vec<TokenStream> {
    channels
        .iter()
        .map(|channel| {
            let channel_name_lit = channel;
            quote! {
                channels_map.insert(
                    #channel_name_lit.to_string(),
                    Channel {
                        description: None,
                        messages: HashMap::new(),
                        servers: None,
                        parameters: None,
                        bindings: None,
                    }
                );
            }
        })
        .collect()
}

/// Generate operations map initialization code (panic version)
pub fn generate_operations_map_code(operations: &[TokenStream]) -> TokenStream {
    if operations.is_empty() {
        quote! {
            let operations_map: Option<HashMap<String, Operation>> = None;
        }
    } else {
        quote! {
            let mut operations_map: HashMap<String, Operation> = HashMap::new();
            #(#operations)*
            let operations_map: Option<HashMap<String, Operation>> = Some(operations_map);
        }
    }
}

/// Generate operations map initialization code (error-returning version)
pub fn generate_operations_map_try_code(operations: &[TokenStream]) -> TokenStream {
    if operations.is_empty() {
        quote! {
            let operations_map: Option<HashMap<String, Operation>> = None;
        }
    } else {
        quote! {
            let mut operations_map: HashMap<String, Operation> = HashMap::new();
            #(#operations)*
            let operations_map: Option<HashMap<String, Operation>> = Some(operations_map);
        }
    }
}

/// Generate the complete impl block for AsyncApi trait
pub fn generate_impl_block(
    ident: &Ident,
    info_title: &str,
    info_version: &str,
    info_desc_expr: TokenStream,
    servers: &[TokenStream],
    security_schemes_code: TokenStream,
    channels: &[TokenStream],
    messages: &[TokenStream],
    messages_try: &[TokenStream],
    operations_code: TokenStream,
    operations_code_try: TokenStream,
) -> TokenStream {
    quote! {
        impl protofolio::AsyncApi for #ident {
            fn asyncapi() -> protofolio::AsyncApiSpec {
                use protofolio::{AsyncApiBuilder, Info, Server, Channel, Message, MessagePayload, Operation, schema_for_type};
                use std::collections::HashMap;
                use serde_json::json;
                use schemars::JsonSchema;
                
                let mut builder = AsyncApiBuilder::new()
                    .info(Info {
                        title: #info_title.to_string(),
                        version: #info_version.to_string(),
                        description: #info_desc_expr,
                    });
                
                // Add servers
                #(#servers)*
                
                // Build channels with messages
                let mut channels_map: HashMap<String, Channel> = HashMap::new();
                
                // Initialize channels
                #(#channels)*
                
                // Add messages to channels
                #(#messages)*
                
                // Build operations
                #operations_code
                
                // Add channels to builder
                for (name, channel) in channels_map {
                    builder = builder.channel(name, channel);
                }
                
                let mut spec = builder.build();
                
                // Add operations to spec if any
                spec.operations = operations_map;
                
                // Add security schemes to components if any
                if let Some(ref schemes) = security_schemes_map {
                    if spec.components.is_none() {
                        spec.components = Some(protofolio::Components::default());
                    }
                    if let Some(ref mut components) = spec.components {
                        components.security_schemes = Some(schemes.clone());
                    }
                }
                
                spec
            }
            
            fn try_asyncapi() -> Result<protofolio::AsyncApiSpec, protofolio::ValidationError> {
                use protofolio::{AsyncApiBuilder, Info, Server, Channel, Message, MessagePayload, Operation, schema_for_type, ValidationError};
                use std::collections::HashMap;
                use serde_json::json;
                use schemars::JsonSchema;
                
                let mut builder = AsyncApiBuilder::new()
                    .info(Info {
                        title: #info_title.to_string(),
                        version: #info_version.to_string(),
                        description: #info_desc_expr,
                    });
                
                // Add servers
                #(#servers)*
                
                // Generate security schemes
                #security_schemes_code
                
                // Build channels with messages
                let mut channels_map: HashMap<String, Channel> = HashMap::new();
                
                // Initialize channels
                #(#channels)*
                
                // Add messages to channels (with error handling)
                #(#messages_try)*
                
                // Build operations (with error handling)
                #operations_code_try
                
                // Add channels to builder
                for (name, channel) in channels_map {
                    builder = builder.channel(name, channel);
                }
                
                let mut spec = builder.build();
                
                // Add operations to spec if any
                spec.operations = operations_map;
                
                // Add security schemes to components if any
                if let Some(ref schemes) = security_schemes_map {
                    if spec.components.is_none() {
                        spec.components = Some(protofolio::Components::default());
                    }
                    if let Some(ref mut components) = spec.components {
                        components.security_schemes = Some(schemes.clone());
                    }
                }
                
                // Validate the spec
                protofolio::validate_spec(&spec)?;
                
                Ok(spec)
            }
        }
    }
}

