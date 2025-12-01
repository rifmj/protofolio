#[cfg(test)]
mod tests {
    use crate::builder::AsyncApiBuilder;
    use crate::spec::{Channel, Info, Parameter, Server, ServerVariable};
    use crate::types::ASYNCAPI_VERSION;
    use std::collections::HashMap;

    #[test]
    fn test_builder_new() {
        let builder = AsyncApiBuilder::new();
        let spec = builder.build();
        assert_eq!(spec.asyncapi, ASYNCAPI_VERSION);
        assert!(spec.info.title.is_empty());
        assert!(spec.info.version.is_empty());
    }

    #[test]
    fn test_builder_info() {
        let spec = AsyncApiBuilder::new()
            .info(Info {
                title: "Test API".to_string(),
                version: "1.0.0".to_string(),
                description: Some("Test description".to_string()),
                external_docs: None,
                contact: None,
                license: None,
                terms_of_service: None,
            })
            .build();

        assert_eq!(spec.info.title, "Test API");
        assert_eq!(spec.info.version, "1.0.0");
        assert_eq!(spec.info.description, Some("Test description".to_string()));
    }

    #[test]
    fn test_builder_server() {
        let spec = AsyncApiBuilder::new()
            .info(Info {
                title: "Test".to_string(),
                version: "1.0.0".to_string(),
                description: None,
                external_docs: None,
                contact: None,
                license: None,
                terms_of_service: None,
            })
            .server(
                "nats".to_string(),
                Server {
                    url: "nats://localhost:4222".to_string(),
                    protocol: "nats".to_string(),
                    description: None,
                    security: None,
                    variables: None,
                },
            )
            .build();

        assert!(spec.servers.is_some());
        let servers = spec.servers.unwrap();
        assert!(servers.contains_key("nats"));
        assert_eq!(servers["nats"].url, "nats://localhost:4222");
    }

    #[test]
    fn test_builder_server_with_variables() {
        let mut variables = HashMap::new();
        variables.insert(
            "host".to_string(),
            ServerVariable {
                default: Some("localhost".to_string()),
                description: Some("Server hostname".to_string()),
                enum_values: None,
                examples: None,
            },
        );
        variables.insert(
            "port".to_string(),
            ServerVariable {
                default: Some("4222".to_string()),
                description: Some("Server port".to_string()),
                enum_values: Some(vec![
                    "4222".to_string(),
                    "4223".to_string(),
                    "4224".to_string(),
                ]),
                examples: Some(vec!["4222".to_string()]),
            },
        );

        let spec = AsyncApiBuilder::new()
            .info(Info {
                title: "Test".to_string(),
                version: "1.0.0".to_string(),
                description: None,
                external_docs: None,
                contact: None,
                license: None,
                terms_of_service: None,
            })
            .server(
                "nats".to_string(),
                Server {
                    url: "nats://{host}:{port}".to_string(),
                    protocol: "nats".to_string(),
                    description: None,
                    security: None,
                    variables: Some(variables),
                },
            )
            .build();

        assert!(spec.servers.is_some());
        let servers = spec.servers.unwrap();
        assert!(servers.contains_key("nats"));
        let server = &servers["nats"];
        assert_eq!(server.url, "nats://{host}:{port}");
        assert!(server.variables.is_some());
        let vars = server.variables.as_ref().unwrap();
        assert!(vars.contains_key("host"));
        assert!(vars.contains_key("port"));
        assert_eq!(vars["host"].default, Some("localhost".to_string()));
        assert_eq!(vars["port"].default, Some("4222".to_string()));
        assert_eq!(
            vars["port"].enum_values,
            Some(vec![
                "4222".to_string(),
                "4223".to_string(),
                "4224".to_string()
            ])
        );
    }

    #[test]
    fn test_builder_channel() {
        let spec = AsyncApiBuilder::new()
            .info(Info {
                title: "Test".to_string(),
                version: "1.0.0".to_string(),
                description: None,
                external_docs: None,
                contact: None,
                license: None,
                terms_of_service: None,
            })
            .channel(
                "test.channel".to_string(),
                Channel {
                    address: "test.channel".to_string(),
                    description: Some("Test channel".to_string()),
                    messages: HashMap::new(),
                    servers: None,
                    parameters: None,
                    bindings: None,
                },
            )
            .build();

        assert!(spec.channels.contains_key("test.channel"));
        assert_eq!(
            spec.channels["test.channel"].description,
            Some("Test channel".to_string())
        );
    }

    #[test]
    fn test_builder_channel_with_params() {
        let mut params = HashMap::new();
        params.insert(
            "tripId".to_string(),
            Parameter {
                description: Some("Trip ID".to_string()),
                schema: Some(serde_json::json!({"type": "string"})),
                location: None,
            },
        );

        let spec = AsyncApiBuilder::new()
            .info(Info {
                title: "Test".to_string(),
                version: "1.0.0".to_string(),
                description: None,
                external_docs: None,
                contact: None,
                license: None,
                terms_of_service: None,
            })
            .channel_with_params(
                "trip.{tripId}".to_string(),
                Channel {
                    address: "trip.{tripId}".to_string(),
                    description: None,
                    messages: HashMap::new(),
                    servers: None,
                    parameters: None,
                    bindings: None,
                },
                params.clone(),
            )
            .build();

        assert!(spec.channels.contains_key("trip.{tripId}"));
        assert!(spec.channels["trip.{tripId}"].parameters.is_some());
        let channel_params = spec.channels["trip.{tripId}"].parameters.as_ref().unwrap();
        assert!(channel_params.contains_key("tripId"));
    }

    #[test]
    fn test_builder_channel_with_bindings() {
        let bindings = serde_json::json!({
            "nats": {
                "queue": "workers"
            }
        });

        let spec = AsyncApiBuilder::new()
            .info(Info {
                title: "Test".to_string(),
                version: "1.0.0".to_string(),
                description: None,
                external_docs: None,
                contact: None,
                license: None,
                terms_of_service: None,
            })
            .channel_with_bindings(
                "test.channel".to_string(),
                Channel {
                    address: "test.channel".to_string(),
                    description: None,
                    messages: HashMap::new(),
                    servers: None,
                    parameters: None,
                    bindings: None,
                },
                bindings.clone(),
            )
            .build();

        assert!(spec.channels.contains_key("test.channel"));
        assert!(spec.channels["test.channel"].bindings.is_some());
        let channel_bindings = spec.channels["test.channel"].bindings.as_ref().unwrap();
        match channel_bindings {
            crate::spec::ChannelBindingsOrRef::Bindings(b) => {
                assert_eq!(b["nats"]["queue"], "workers");
            }
            crate::spec::ChannelBindingsOrRef::Ref(_) => {
                panic!("Expected bindings, got reference");
            }
        }
    }
}
