use agent_client_protocol::schema::{v1 as acp_v1, v2 as acp_v2};
use anyhow::{Result, bail};

pub fn from_v1(notice: acp_v1::Notice) -> Result<acp_v2::Notice> {
    let severity = match notice.severity {
        acp_v1::NoticeSeverity::Info => acp_v2::NoticeSeverity::Info,
        acp_v1::NoticeSeverity::Warning => acp_v2::NoticeSeverity::Warning,
        acp_v1::NoticeSeverity::Error => acp_v2::NoticeSeverity::Error,
        acp_v1::NoticeSeverity::Other(severity) => acp_v2::NoticeSeverity::Other(severity),
        _ => bail!("Unsupported legacy notice severity"),
    };
    Ok(acp_v2::Notice::new(severity, notice.title)
        .description(notice.description)
        .meta(notice.meta))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn legacy_notices_preserve_fields_and_unknown_severities() -> Result<()> {
        for (legacy, expected) in [
            (
                acp_v1::Notice::new(acp_v1::NoticeSeverity::Info, "Using default configuration"),
                json!({"severity": "info", "title": "Using default configuration"}),
            ),
            (
                serde_json::from_value(json!({
                    "severity": "warning",
                    "title": "MCP server unavailable",
                    "description": null,
                    "_meta": null
                }))?,
                json!({"severity": "warning", "title": "MCP server unavailable"}),
            ),
            (
                acp_v1::Notice::new(acp_v1::NoticeSeverity::Error, "Optional integration failed")
                    .description("")
                    .meta(acp_v1::Meta::new()),
                json!({
                    "severity": "error",
                    "title": "Optional integration failed",
                    "description": "",
                    "_meta": {}
                }),
            ),
            (
                acp_v1::Notice::new(
                    acp_v1::NoticeSeverity::Other("critical".into()),
                    "Future severity",
                )
                .description("**Plain text**, not Markdown.\nWork continues.")
                .meta(acp_v1::Meta::from_iter([(
                    "extension".into(),
                    json!({"nested": [null, true, {"value": "retained"}]}),
                )])),
                json!({
                    "severity": "critical",
                    "title": "Future severity",
                    "description": "**Plain text**, not Markdown.\nWork continues.",
                    "_meta": {"extension": {"nested": [null, true, {"value": "retained"}]}}
                }),
            ),
            (
                acp_v1::Notice::new(
                    acp_v1::NoticeSeverity::Other("_custom/β".into()),
                    "Custom severity",
                ),
                json!({"severity": "_custom/β", "title": "Custom severity"}),
            ),
        ] {
            assert_eq!(serde_json::to_value(from_v1(legacy)?)?, expected);
        }
        Ok(())
    }
}
