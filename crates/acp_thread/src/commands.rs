use agent_client_protocol::schema::{v1 as acp_v1, v2 as acp_v2};
use anyhow::{Result, bail};

pub fn from_v1(commands: Vec<acp_v1::AvailableCommand>) -> Result<Vec<acp_v2::AvailableCommand>> {
    commands
        .into_iter()
        .map(|command| {
            let input = command
                .input
                .map(|input| {
                    Ok(match input {
                        acp_v1::AvailableCommandInput::Unstructured(input) => {
                            acp_v2::AvailableCommandInput::Text(
                                acp_v2::TextCommandInput::new(input.hint).meta(input.meta),
                            )
                        }
                        _ => bail!("Unsupported legacy command input"),
                    })
                })
                .transpose()?;
            Ok(
                acp_v2::AvailableCommand::new(command.name, command.description)
                    .input(input)
                    .meta(command.meta),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn legacy_definitions_preserve_input_and_metadata() -> Result<()> {
        let commands = vec![
            acp_v1::AvailableCommand::new("/opaque.name", ""),
            acp_v1::AvailableCommand::new("empty", "").meta(acp_v1::Meta::new()),
            acp_v1::AvailableCommand::new("deploy", "Deploy code")
                .input(acp_v1::AvailableCommandInput::Unstructured(
                    acp_v1::UnstructuredCommandInput::new("").meta(acp_v1::Meta::from_iter([(
                        "input".into(),
                        json!([null, true]),
                    )])),
                ))
                .meta(acp_v1::Meta::from_iter([
                    ("command_category".into(), json!("mcp")),
                    ("command".into(), json!({"nested": [1, "value"]})),
                ])),
        ];
        assert_eq!(
            serde_json::to_value(from_v1(commands)?)?,
            json!([
                {"name": "/opaque.name", "description": ""},
                {"name": "empty", "description": "", "_meta": {}},
                {
                    "name": "deploy", "description": "Deploy code",
                    "input": {"type": "text", "hint": "", "_meta": {"input": [null, true]}},
                    "_meta": {"command_category": "mcp", "command": {"nested": [1, "value"]}}
                }
            ])
        );
        Ok(())
    }
}
