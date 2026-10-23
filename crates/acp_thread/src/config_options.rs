use agent_client_protocol::schema::{v1 as acp_v1, v2 as acp_v2};
use anyhow::{Result, bail};

pub fn from_v1(
    options: Vec<acp_v1::SessionConfigOption>,
) -> Result<Vec<acp_v2::SessionConfigOption>> {
    options.into_iter().map(option_from_v1).collect()
}

fn option_from_v1(option: acp_v1::SessionConfigOption) -> Result<acp_v2::SessionConfigOption> {
    let kind = match option.kind {
        acp_v1::SessionConfigKind::Select(select) => {
            let options = match select.options {
                acp_v1::SessionConfigSelectOptions::Ungrouped(options) => {
                    acp_v2::SessionConfigSelectOptions::Ungrouped(
                        options.into_iter().map(select_option_from_v1).collect(),
                    )
                }
                acp_v1::SessionConfigSelectOptions::Grouped(groups) => {
                    acp_v2::SessionConfigSelectOptions::Grouped(
                        groups
                            .into_iter()
                            .map(|group| {
                                acp_v2::SessionConfigSelectGroup::new(
                                    acp_v2::SessionConfigGroupId::new(group.group.0),
                                    group.name,
                                    group
                                        .options
                                        .into_iter()
                                        .map(select_option_from_v1)
                                        .collect(),
                                )
                                .meta(group.meta)
                            })
                            .collect(),
                    )
                }
                _ => bail!("Unsupported legacy configuration choices"),
            };
            acp_v2::SessionConfigKind::Select(acp_v2::SessionConfigSelect::new(
                acp_v2::SessionConfigValueId::new(select.current_value.0),
                options,
            ))
        }
        acp_v1::SessionConfigKind::Boolean(boolean) => acp_v2::SessionConfigKind::Boolean(
            acp_v2::SessionConfigBoolean::new(boolean.current_value),
        ),
        _ => bail!("Unsupported legacy configuration option"),
    };
    let category = option
        .category
        .map(|category| {
            Ok(match category {
                acp_v1::SessionConfigOptionCategory::Mode => {
                    acp_v2::SessionConfigOptionCategory::Mode
                }
                acp_v1::SessionConfigOptionCategory::Model => {
                    acp_v2::SessionConfigOptionCategory::Model
                }
                acp_v1::SessionConfigOptionCategory::ModelConfig => {
                    acp_v2::SessionConfigOptionCategory::ModelConfig
                }
                acp_v1::SessionConfigOptionCategory::ThoughtLevel => {
                    acp_v2::SessionConfigOptionCategory::ThoughtLevel
                }
                acp_v1::SessionConfigOptionCategory::Other(category) => {
                    acp_v2::SessionConfigOptionCategory::Other(category)
                }
                _ => bail!("Unsupported legacy configuration category"),
            })
        })
        .transpose()?;
    Ok(acp_v2::SessionConfigOption::new(
        acp_v2::SessionConfigId::new(option.id.0),
        option.name,
        kind,
    )
    .description(option.description)
    .category(category)
    .meta(option.meta))
}

fn select_option_from_v1(
    option: acp_v1::SessionConfigSelectOption,
) -> acp_v2::SessionConfigSelectOption {
    acp_v2::SessionConfigSelectOption::new(
        acp_v2::SessionConfigValueId::new(option.value.0),
        option.name,
    )
    .description(option.description)
    .meta(option.meta)
}

pub fn value_to_v1(
    value: acp_v2::SessionConfigOptionValue,
) -> Result<acp_v1::SessionConfigOptionValue> {
    match value {
        acp_v2::SessionConfigOptionValue::Id { value } => Ok(
            acp_v1::SessionConfigOptionValue::value_id(acp_v1::SessionConfigValueId::new(value.0)),
        ),
        acp_v2::SessionConfigOptionValue::Boolean { value } => {
            Ok(acp_v1::SessionConfigOptionValue::boolean(value))
        }
        _ => bail!("This configuration value is not supported by ACP v1"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn legacy_options_preserve_fields_in_v2() -> Result<()> {
        let options = vec![
            acp_v1::SessionConfigOption::boolean("enabled", "Enabled", false)
                .meta(acp_v1::Meta::new()),
            acp_v1::SessionConfigOption::select(
                "mode",
                "Mode",
                "fast",
                vec![
                    acp_v1::SessionConfigSelectOption::new("fast", "Fast")
                        .description("")
                        .meta(acp_v1::Meta::new()),
                ],
            )
            .description("")
            .category(acp_v1::SessionConfigOptionCategory::Other("_custom".into()))
            .meta(acp_v1::Meta::from_iter([(
                "nested".into(),
                json!([1, true]),
            )])),
            acp_v1::SessionConfigOption::select(
                "model",
                "Model",
                "large",
                vec![
                    acp_v1::SessionConfigSelectGroup::new(
                        "provider",
                        "Provider",
                        vec![acp_v1::SessionConfigSelectOption::new("large", "Large")],
                    )
                    .meta(acp_v1::Meta::from_iter([(
                        "group".into(),
                        json!("metadata"),
                    )])),
                ],
            )
            .category(acp_v1::SessionConfigOptionCategory::Model),
        ];
        assert_eq!(
            serde_json::to_value(from_v1(options)?)?,
            json!([
                {
                    "configId": "enabled", "name": "Enabled",
                    "type": "boolean", "currentValue": false, "_meta": {}
                },
                {
                    "configId": "mode", "name": "Mode", "description": "",
                    "category": "_custom", "type": "select", "currentValue": "fast",
                    "options": [{"value": "fast", "name": "Fast", "description": "", "_meta": {}}],
                    "_meta": {"nested": [1, true]}
                },
                {
                    "configId": "model", "name": "Model", "category": "model",
                    "type": "select", "currentValue": "large",
                    "options": [{
                        "groupId": "provider", "name": "Provider",
                        "options": [{"value": "large", "name": "Large"}],
                        "_meta": {"group": "metadata"}
                    }]
                }
            ])
        );
        Ok(())
    }

    #[test]
    fn legacy_values_convert_known_shapes_and_reject_unknown_payloads() -> Result<()> {
        assert_eq!(
            value_to_v1(acp_v2::SessionConfigOptionValue::id("opaque-id"))?,
            acp_v1::SessionConfigOptionValue::value_id("opaque-id")
        );
        assert_eq!(
            value_to_v1(acp_v2::SessionConfigOptionValue::boolean(false))?,
            acp_v1::SessionConfigOptionValue::boolean(false)
        );
        let value =
            acp_v2::SessionConfigOptionValue::Other(acp_v2::OtherSessionConfigOptionValue::new(
                "_custom",
                json!("private-value"),
                Default::default(),
            ));
        let error = value_to_v1(value).expect_err("unknown value must not lose its type");
        assert!(!error.to_string().contains("private-value"));
        Ok(())
    }
}
