use acp_thread::{AcpThread, DisplayTerminalOutput, DisplayTerminalPatch};
use agent_client_protocol::schema::{MaybeUndefined, v2 as acp_v2};
use agent_client_protocol::{JsonRpcMessage, UntypedMessage};
use anyhow::{Context as _, Result};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use gpui::Context;

#[derive(Debug)]
pub struct DecodedTerminalNotification {
    pub session_id: acp_v2::SessionId,
    pub meta: Option<acp_v2::Meta>,
    pub update: DecodedTerminalUpdate,
}

#[derive(Debug)]
pub enum DecodedTerminalUpdate {
    Patch {
        terminal_id: acp_v2::TerminalId,
        patch: DisplayTerminalPatch,
    },
    Output {
        terminal_id: acp_v2::TerminalId,
        data: Vec<u8>,
        meta: Option<acp_v2::Meta>,
    },
}

impl DecodedTerminalNotification {
    /// Applies a fully decoded update to its session. Envelope and chunk metadata
    /// remain event-scoped and are not promoted to terminal or snapshot metadata.
    pub fn apply(self, thread: &mut AcpThread, cx: &mut Context<AcpThread>) -> Result<()> {
        anyhow::ensure!(
            self.session_id.0.as_ref() == thread.session_id().0.as_ref(),
            "Terminal notification belongs to a different ACP session"
        );
        match self.update {
            DecodedTerminalUpdate::Patch { terminal_id, patch } => {
                thread.upsert_display_terminal(terminal_id, patch, cx)
            }
            DecodedTerminalUpdate::Output {
                terminal_id, data, ..
            } => thread.append_display_terminal_output(terminal_id, &data, cx),
        }
    }
}

pub(super) fn parse_terminal_notification(
    message: &UntypedMessage,
) -> Result<acp_v2::UpdateSessionNotification, agent_client_protocol::Error> {
    // SDK deserialization errors include the complete params, which its warning logger would print.
    let notification =
        acp_v2::UpdateSessionNotification::parse_message(message.method(), message.params())
            .map_err(|_| {
                agent_client_protocol::Error::invalid_params()
                    .data("Malformed ACP v2 terminal notification")
            })?;
    // The SDK defaults malformed output to Undefined; don't apply sibling fields in that case.
    if let acp_v2::SessionUpdate::TerminalUpdate(update) = &notification.update
        && message
            .params()
            .get("update")
            .and_then(|update| update.get("output"))
            .is_some_and(|output| !output.is_null())
        && update.output.is_undefined()
    {
        return Err(agent_client_protocol::Error::invalid_params()
            .data("Malformed ACP v2 terminal output snapshot"));
    }
    Ok(notification)
}

pub fn decode_terminal_notification(
    notification: acp_v2::UpdateSessionNotification,
) -> Result<Option<DecodedTerminalNotification>> {
    let update = match notification.update {
        acp_v2::SessionUpdate::TerminalUpdate(update) => {
            let output = match update.output {
                MaybeUndefined::Undefined => MaybeUndefined::Undefined,
                MaybeUndefined::Null => MaybeUndefined::Null,
                MaybeUndefined::Value(output) => MaybeUndefined::Value(DisplayTerminalOutput {
                    data: STANDARD
                        .decode(output.data)
                        .context("invalid base64 in ACP v2 terminal output snapshot")?,
                    meta: output.meta,
                }),
            };
            DecodedTerminalUpdate::Patch {
                terminal_id: update.terminal_id,
                patch: DisplayTerminalPatch {
                    command: update.command,
                    cwd: update.cwd,
                    output,
                    exit_status: update.exit_status,
                    meta: update.meta,
                },
            }
        }
        acp_v2::SessionUpdate::TerminalOutputChunk(chunk) => DecodedTerminalUpdate::Output {
            terminal_id: chunk.terminal_id,
            data: STANDARD
                .decode(chunk.data)
                .context("invalid base64 in ACP v2 terminal output chunk")?,
            meta: chunk.meta,
        },
        _ => return Ok(None),
    };

    Ok(Some(DecodedTerminalNotification {
        session_id: notification.session_id,
        meta: notification.meta,
        update,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::bail;
    use serde_json::json;

    fn decode_fixture(fixture: &str) -> Result<DecodedTerminalNotification> {
        decode_terminal_notification(serde_json::from_str(fixture)?)?
            .context("expected a terminal notification")
    }

    fn scoped_meta(scope: &str) -> acp_v2::Meta {
        acp_v2::Meta::from_iter([("scope".to_owned(), json!(scope))])
    }

    #[test]
    fn sdk_parse_errors_do_not_echo_private_notification_params() -> Result<()> {
        let message = UntypedMessage::new(
            "session/update",
            json!({
                "sessionId": "private-session-marker",
                "_meta": {"private": "private-envelope-marker"},
                "update": {
                    "sessionUpdate": "terminal_output_chunk",
                    "terminalId": "private-terminal-marker",
                    "data": 7,
                    "_meta": {"private": "private-chunk-marker"}
                }
            }),
        )?;
        let error =
            parse_terminal_notification(&message).expect_err("malformed chunk must be rejected");
        assert_eq!(error.code, agent_client_protocol::ErrorCode::InvalidParams);
        let serialized = serde_json::to_string(&error)?;
        assert!(!serialized.contains("private-"));
        assert!(serialized.len() < 200);
        Ok(())
    }

    #[test]
    fn patch_preserves_omission_and_null() -> Result<()> {
        let omitted = decode_fixture(
            r#"{
                "sessionId": "session-1",
                "update": {"sessionUpdate": "terminal_update", "terminalId": "terminal-1"}
            }"#,
        )?;
        assert_eq!(omitted.session_id, acp_v2::SessionId::new("session-1"));
        assert_eq!(omitted.meta, None);
        let DecodedTerminalUpdate::Patch { terminal_id, patch } = omitted.update else {
            bail!("expected a terminal patch");
        };
        assert_eq!(terminal_id, acp_v2::TerminalId::new("terminal-1"));
        assert_eq!(patch.command, MaybeUndefined::Undefined);
        assert_eq!(patch.cwd, MaybeUndefined::Undefined);
        assert!(matches!(patch.output, MaybeUndefined::Undefined));
        assert_eq!(patch.exit_status, MaybeUndefined::Undefined);
        assert_eq!(patch.meta, MaybeUndefined::Undefined);

        let cleared = decode_fixture(
            r#"{
                "sessionId": "session-1",
                "_meta": null,
                "update": {
                    "sessionUpdate": "terminal_update",
                    "terminalId": "terminal-1",
                    "command": null,
                    "cwd": null,
                    "output": null,
                    "exitStatus": null,
                    "_meta": null
                }
            }"#,
        )?;
        assert_eq!(cleared.meta, None);
        let DecodedTerminalUpdate::Patch { patch, .. } = cleared.update else {
            bail!("expected a terminal patch");
        };
        assert_eq!(patch.command, MaybeUndefined::Null);
        assert_eq!(patch.cwd, MaybeUndefined::Null);
        assert!(matches!(patch.output, MaybeUndefined::Null));
        assert_eq!(patch.exit_status, MaybeUndefined::Null);
        assert_eq!(patch.meta, MaybeUndefined::Null);
        Ok(())
    }

    #[test]
    fn patch_preserves_values_and_separate_metadata_scopes() -> Result<()> {
        let decoded = decode_fixture(
            r#"{
                "sessionId": "session-1",
                "_meta": {"scope": "envelope"},
                "update": {
                    "sessionUpdate": "terminal_update",
                    "terminalId": "terminal-1",
                    "command": "printf '\\377\\000'",
                    "cwd": "/tmp",
                    "output": {"data": "/wCA", "_meta": {"scope": "snapshot"}},
                    "exitStatus": {
                        "exitCode": 7,
                        "signal": "SIGTERM",
                        "_meta": {"scope": "exit"}
                    },
                    "_meta": {"scope": "terminal"}
                }
            }"#,
        )?;
        assert_eq!(decoded.meta, Some(scoped_meta("envelope")));
        let DecodedTerminalUpdate::Patch { patch, .. } = decoded.update else {
            bail!("expected a terminal patch");
        };
        assert_eq!(
            patch.command,
            MaybeUndefined::Value("printf '\\377\\000'".to_owned())
        );
        assert_eq!(
            patch.cwd,
            MaybeUndefined::Value(acp_v2::AbsolutePath::new("/tmp"))
        );
        assert_eq!(patch.meta, MaybeUndefined::Value(scoped_meta("terminal")));
        assert_eq!(
            patch.exit_status,
            MaybeUndefined::Value(
                acp_v2::TerminalExitStatus::new()
                    .exit_code(7u32)
                    .signal("SIGTERM")
                    .meta(scoped_meta("exit"))
            )
        );
        let MaybeUndefined::Value(output) = patch.output else {
            bail!("expected an output snapshot");
        };
        assert_eq!(output.data, [0xff, 0x00, 0x80]);
        assert_eq!(output.meta, Some(scoped_meta("snapshot")));
        Ok(())
    }

    #[test]
    fn empty_snapshot_preserves_empty_metadata_distinct_from_absence() -> Result<()> {
        let decoded = decode_fixture(
            r#"{
                "sessionId": "session-1",
                "_meta": {},
                "update": {
                    "sessionUpdate": "terminal_update",
                    "terminalId": "terminal-1",
                    "output": {"data": "", "_meta": {}},
                    "exitStatus": {"_meta": {}},
                    "_meta": {}
                }
            }"#,
        )?;
        assert_eq!(decoded.meta, Some(acp_v2::Meta::new()));
        let DecodedTerminalUpdate::Patch { patch, .. } = decoded.update else {
            bail!("expected a terminal patch");
        };
        assert_eq!(patch.meta, MaybeUndefined::Value(acp_v2::Meta::new()));
        assert_eq!(
            patch.exit_status,
            MaybeUndefined::Value(acp_v2::TerminalExitStatus::new().meta(acp_v2::Meta::new()))
        );
        let MaybeUndefined::Value(output) = patch.output else {
            bail!("expected an output snapshot");
        };
        assert!(output.data.is_empty());
        assert_eq!(output.meta, Some(acp_v2::Meta::new()));

        let decoded = decode_fixture(
            r#"{
                "sessionId": "session-1",
                "update": {
                    "sessionUpdate": "terminal_update",
                    "terminalId": "terminal-1",
                    "output": {"data": ""}
                }
            }"#,
        )?;
        assert_eq!(decoded.meta, None);
        let DecodedTerminalUpdate::Patch { patch, .. } = decoded.update else {
            bail!("expected a terminal patch");
        };
        assert_eq!(patch.meta, MaybeUndefined::Undefined);
        let MaybeUndefined::Value(output) = patch.output else {
            bail!("expected an output snapshot");
        };
        assert!(output.data.is_empty());
        assert_eq!(output.meta, None);
        Ok(())
    }

    #[test]
    fn chunks_decode_independently_across_ansi_and_utf8_boundaries() -> Result<()> {
        let first = decode_fixture(
            r#"{
                "sessionId": "session-1",
                "_meta": {"scope": "envelope"},
                "update": {
                    "sessionUpdate": "terminal_output_chunk",
                    "terminalId": "terminal-1",
                    "data": "G1s=",
                    "_meta": {"scope": "chunk"}
                }
            }"#,
        )?;
        assert_eq!(first.session_id, acp_v2::SessionId::new("session-1"));
        assert_eq!(first.meta, Some(scoped_meta("envelope")));
        let DecodedTerminalUpdate::Output {
            terminal_id,
            data,
            meta,
        } = first.update
        else {
            bail!("expected an output chunk");
        };
        assert_eq!(terminal_id, acp_v2::TerminalId::new("terminal-1"));
        assert_eq!(data, b"\x1b[");
        assert_eq!(meta, Some(scoped_meta("chunk")));

        let second = decode_fixture(
            r#"{
                "sessionId": "session-1",
                "update": {
                    "sessionUpdate": "terminal_output_chunk",
                    "terminalId": "terminal-1",
                    "data": "MzFtwg=="
                }
            }"#,
        )?;
        assert_eq!(second.meta, None);
        let DecodedTerminalUpdate::Output { data, meta, .. } = second.update else {
            bail!("expected an output chunk");
        };
        assert_eq!(data, b"31m\xc2");
        assert_eq!(meta, None);

        let third = decode_fixture(
            r#"{
                "sessionId": "session-1",
                "update": {
                    "sessionUpdate": "terminal_output_chunk",
                    "terminalId": "terminal-1",
                    "data": "og==",
                    "_meta": null
                }
            }"#,
        )?;
        let DecodedTerminalUpdate::Output { data, meta, .. } = third.update else {
            bail!("expected an output chunk");
        };
        assert_eq!(data, [0xa2]);
        assert_eq!(meta, None);
        Ok(())
    }

    #[test]
    fn empty_chunk_preserves_empty_metadata() -> Result<()> {
        let decoded = decode_fixture(
            r#"{
                "sessionId": "session-1",
                "_meta": {},
                "update": {
                    "sessionUpdate": "terminal_output_chunk",
                    "terminalId": "terminal-1",
                    "data": "",
                    "_meta": {}
                }
            }"#,
        )?;
        assert_eq!(decoded.meta, Some(acp_v2::Meta::new()));
        let DecodedTerminalUpdate::Output { data, meta, .. } = decoded.update else {
            bail!("expected an output chunk");
        };
        assert!(data.is_empty());
        assert_eq!(meta, Some(acp_v2::Meta::new()));
        Ok(())
    }

    #[test]
    fn invalid_snapshot_and_chunk_are_rejected_without_echoing_payloads() -> Result<()> {
        for (fixture, context) in [
            (
                r#"{
                    "sessionId": "session-1",
                    "update": {
                        "sessionUpdate": "terminal_update",
                        "terminalId": "terminal-1",
                        "command": "echo ready",
                        "cwd": "/tmp",
                        "output": {"data": "private-output!"}
                    }
                }"#,
                "terminal output snapshot",
            ),
            (
                r#"{
                    "sessionId": "session-1",
                    "update": {
                        "sessionUpdate": "terminal_output_chunk",
                        "terminalId": "terminal-1",
                        "data": "private-output!"
                    }
                }"#,
                "terminal output chunk",
            ),
        ] {
            let error = match decode_terminal_notification(serde_json::from_str(fixture)?) {
                Err(error) => error,
                Ok(_) => bail!("expected invalid base64 to reject the entire notification"),
            };
            let message = format!("{error:#}");
            assert!(message.contains(context));
            assert!(!message.contains("private-output!"));
        }
        Ok(())
    }

    #[test]
    fn unrelated_update_is_unclaimed() -> Result<()> {
        let notification = serde_json::from_str(
            r#"{
                "sessionId": "session-1",
                "update": {
                    "sessionUpdate": "agent_message_chunk",
                    "messageId": "message-1",
                    "content": {"type": "text", "text": "hello"}
                }
            }"#,
        )?;
        assert!(decode_terminal_notification(notification)?.is_none());
        Ok(())
    }
}
