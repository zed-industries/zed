use agent_client_protocol::schema::{v1 as acp_v1, v2 as acp_v2};
use anyhow::{Result, bail};

pub fn from_v1(methods: Vec<acp_v1::AuthMethod>) -> Result<Vec<acp_v2::AuthMethod>> {
    methods
        .into_iter()
        .map(|method| {
            Ok(match method {
                acp_v1::AuthMethod::Agent(method) => acp_v2::AuthMethod::Agent(
                    acp_v2::AuthMethodAgent::new(method.id.0, method.name)
                        .description(method.description)
                        .meta(method.meta),
                ),
                acp_v1::AuthMethod::Terminal(method) => {
                    let mut environment: Vec<_> = method
                        .env
                        .into_iter()
                        .map(|(name, value)| acp_v2::EnvVariable::new(name, value))
                        .collect();
                    environment.sort_by(|left, right| left.name.cmp(&right.name));
                    acp_v2::AuthMethod::Terminal(
                        acp_v2::AuthMethodTerminal::new(method.id.0, method.name)
                            .description(method.description)
                            .args(method.args)
                            .env(environment)
                            .meta(method.meta),
                    )
                }
                _ => bail!("Unsupported legacy authentication method"),
            })
        })
        .collect()
}

pub fn is_supported(method: &acp_v2::AuthMethod) -> bool {
    matches!(
        method,
        acp_v2::AuthMethod::Agent(_) | acp_v2::AuthMethod::Terminal(_)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn legacy_definitions_preserve_authentication_data() -> Result<()> {
        let methods = from_v1(vec![
            serde_json::from_value(json!({"id": "/opaque.login", "name": "Default login"}))?,
            acp_v1::AuthMethod::Agent(
                acp_v1::AuthMethodAgent::new("empty", "")
                    .description("")
                    .meta(acp_v1::Meta::new()),
            ),
            acp_v1::AuthMethod::Agent(
                acp_v1::AuthMethodAgent::new("legacy", "Legacy login")
                    .description("Interactive login")
                    .meta(acp_v1::Meta::from_iter([
                        (
                            "terminal-auth".into(),
                            json!({
                                "label": "Legacy login",
                                "command": "agent",
                                "args": ["auth", "--interactive"],
                                "env": {"AUTH_MODE": "interactive"}
                            }),
                        ),
                        ("extension".into(), json!({"nested": [null, true]})),
                    ])),
            ),
            acp_v1::AuthMethod::Terminal(acp_v1::AuthMethodTerminal::new("terminal", "Login")),
            acp_v1::AuthMethod::Terminal(
                acp_v1::AuthMethodTerminal::new("terminal/opaque", "Terminal login")
                    .description("Sign in")
                    .args(vec!["auth".into(), "--interactive".into(), "".into()])
                    .env(std::collections::HashMap::from_iter([
                        ("SHARED".into(), "override".into()),
                        ("EMPTY".into(), "".into()),
                    ]))
                    .meta(acp_v1::Meta::from_iter([(
                        "extension".into(),
                        json!({"nested": [1, "retained"]}),
                    )])),
            ),
        ])?;
        assert!(methods.iter().all(is_supported));
        assert_eq!(
            serde_json::to_value(methods)?,
            json!([
                {"type": "agent", "methodId": "/opaque.login", "name": "Default login"},
                {
                    "type": "agent", "methodId": "empty", "name": "",
                    "description": "", "_meta": {}
                },
                {
                    "type": "agent", "methodId": "legacy", "name": "Legacy login",
                    "description": "Interactive login",
                    "_meta": {
                        "terminal-auth": {
                            "label": "Legacy login", "command": "agent",
                            "args": ["auth", "--interactive"],
                            "env": {"AUTH_MODE": "interactive"}
                        },
                        "extension": {"nested": [null, true]}
                    }
                },
                {"type": "terminal", "methodId": "terminal", "name": "Login"},
                {
                    "type": "terminal", "methodId": "terminal/opaque", "name": "Terminal login",
                    "description": "Sign in", "args": ["auth", "--interactive", ""],
                    "env": [
                        {"name": "EMPTY", "value": ""},
                        {"name": "SHARED", "value": "override"}
                    ],
                    "_meta": {"extension": {"nested": [1, "retained"]}}
                }
            ])
        );
        Ok(())
    }
}
