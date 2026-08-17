//! Agent-owned terminal output reported for display by clients.

use std::sync::Arc;

use derive_more::{Display, From};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::{DefaultOnError, serde_as, skip_serializing_none};

use super::{AbsolutePath, Meta};
use crate::{IntoMaybeUndefined, IntoOption, MaybeUndefined};

/// Unique identifier for an agent-owned terminal within a session.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Display, From)]
#[serde(transparent)]
#[from(forward)]
#[non_exhaustive]
pub struct TerminalId(pub Arc<str>);

impl TerminalId {
    /// Wraps a protocol string as a typed [`TerminalId`].
    #[must_use]
    pub fn new(id: impl Into<Self>) -> Self {
        id.into()
    }
}

impl IntoOption<TerminalId> for &str {
    fn into_option(self) -> Option<TerminalId> {
        Some(TerminalId::new(self))
    }
}

/// A display-only reference to an agent-owned terminal.
///
/// Terminal state and output are delivered separately through
/// [`TerminalUpdate`] and [`TerminalOutputChunk`].
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Terminal {
    /// The ID of the terminal to display.
    pub terminal_id: TerminalId,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys. This metadata is scoped to the content reference. Omitted
    /// and `null` are equivalent and mean no item metadata was provided.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl Terminal {
    /// Builds a terminal reference for the given terminal ID.
    #[must_use]
    pub fn new(terminal_id: impl Into<TerminalId>) -> Self {
        Self {
            terminal_id: terminal_id.into(),
            meta: None,
        }
    }

    /// Sets or clears metadata scoped to this content reference.
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// An authoritative replacement snapshot of terminal output bytes.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TerminalOutput {
    /// Base64-encoded replacement terminal output bytes.
    #[schemars(extend("contentEncoding" = "base64"))]
    pub data: String,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys. This metadata is scoped to the replacement snapshot. Omitted
    /// and `null` are equivalent and mean no snapshot metadata was provided.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl TerminalOutput {
    /// Builds an authoritative terminal output replacement.
    #[must_use]
    pub fn new(data: impl Into<String>) -> Self {
        Self {
            data: data.into(),
            meta: None,
        }
    }

    /// Sets or clears metadata scoped to this replacement snapshot.
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// Exit information for an agent-owned terminal.
///
/// The presence of this object marks the terminal as exited, even when neither
/// an exit code nor a signal is known.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TerminalExitStatus {
    /// Process exit code, when known. Omitted and `null` are equivalent.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub exit_code: Option<u32>,
    /// Signal that terminated the process, when known.
    ///
    /// Agents should use the conventional platform signal name. POSIX examples
    /// include `SIGTERM`, `SIGKILL`, and `SIGINT`. Other platforms may use a
    /// platform-specific name. Omitted and `null` are equivalent.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub signal: Option<String>,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys. This metadata is scoped to the exit information. Omitted
    /// and `null` are equivalent and mean no exit metadata was provided.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl TerminalExitStatus {
    /// Builds terminal exit information with no known exit code or signal.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets or clears the optional exit code.
    #[must_use]
    pub fn exit_code(mut self, exit_code: impl IntoOption<u32>) -> Self {
        self.exit_code = exit_code.into_option();
        self
    }

    /// Sets or clears the optional terminating signal.
    #[must_use]
    pub fn signal(mut self, signal: impl IntoOption<String>) -> Self {
        self.signal = signal.into_option();
        self
    }

    /// Sets or clears metadata scoped to this exit information.
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// An upsert for the stored state of an agent-owned terminal.
///
/// Only [`TerminalUpdate::terminal_id`] is required. Other fields have patch
/// semantics: omitted fields leave the stored value unchanged, `null` clears
/// it, and concrete values replace it. When the terminal ID is new, omitted
/// fields start unknown.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TerminalUpdate {
    /// Unique identifier for this terminal within the session.
    pub terminal_id: TerminalId,
    /// The command being run.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default, skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub command: MaybeUndefined<String>,
    /// The absolute working directory of the command.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default, skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub cwd: MaybeUndefined<AbsolutePath>,
    /// An authoritative replacement snapshot of terminal output bytes.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default, skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub output: MaybeUndefined<TerminalOutput>,
    /// Exit information. A concrete object marks the terminal as exited.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default, skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub exit_status: MaybeUndefined<TerminalExitStatus>,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Omitted means no metadata update; `null` is an
    /// explicit clear signal. Implementations MUST NOT make assumptions about values at these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError<MaybeUndefined<_>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "MaybeUndefined::is_undefined"
    )]
    pub meta: MaybeUndefined<Meta>,
}

impl TerminalUpdate {
    /// Builds a terminal upsert with only its required ID set.
    #[must_use]
    pub fn new(terminal_id: impl Into<TerminalId>) -> Self {
        Self {
            terminal_id: terminal_id.into(),
            command: MaybeUndefined::Undefined,
            cwd: MaybeUndefined::Undefined,
            output: MaybeUndefined::Undefined,
            exit_status: MaybeUndefined::Undefined,
            meta: MaybeUndefined::Undefined,
        }
    }

    /// Sets, clears, or leaves unchanged the command being run.
    #[must_use]
    pub fn command(mut self, command: impl IntoMaybeUndefined<String>) -> Self {
        self.command = command.into_maybe_undefined();
        self
    }

    /// Sets, clears, or leaves unchanged the absolute working directory.
    #[must_use]
    pub fn cwd(mut self, cwd: impl IntoMaybeUndefined<AbsolutePath>) -> Self {
        self.cwd = cwd.into_maybe_undefined();
        self
    }

    /// Sets, clears, or leaves unchanged the authoritative output snapshot.
    #[must_use]
    pub fn output(mut self, output: impl IntoMaybeUndefined<TerminalOutput>) -> Self {
        self.output = output.into_maybe_undefined();
        self
    }

    /// Sets, clears, or leaves unchanged the terminal exit information.
    #[must_use]
    pub fn exit_status(mut self, exit_status: impl IntoMaybeUndefined<TerminalExitStatus>) -> Self {
        self.exit_status = exit_status.into_maybe_undefined();
        self
    }

    /// Sets, clears, or leaves unchanged terminal metadata.
    #[must_use]
    pub fn meta(mut self, meta: impl IntoMaybeUndefined<Meta>) -> Self {
        self.meta = meta.into_maybe_undefined();
        self
    }

    /// Applies a later terminal patch to this stored terminal state.
    ///
    /// Fields set to `null` remain `null` so callers can distinguish explicit
    /// clearing from an update that did not mention the field.
    pub fn apply_update(&mut self, update: TerminalUpdate) {
        debug_assert_eq!(self.terminal_id, update.terminal_id);
        if !update.command.is_undefined() {
            self.command = update.command;
        }
        if !update.cwd.is_undefined() {
            self.cwd = update.cwd;
        }
        if !update.output.is_undefined() {
            self.output = update.output;
        }
        if !update.exit_status.is_undefined() {
            self.exit_status = update.exit_status;
        }
        if !update.meta.is_undefined() {
            self.meta = update.meta;
        }
    }
}

/// A chunk of bytes appended to an agent-owned terminal's output.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TerminalOutputChunk {
    /// The terminal receiving these bytes.
    pub terminal_id: TerminalId,
    /// Independently base64-encoded terminal output bytes.
    #[schemars(extend("contentEncoding" = "base64"))]
    pub data: String,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys. This field is chunk-scoped. Omitted and `null` are
    /// equivalent and mean no chunk metadata was provided.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl TerminalOutputChunk {
    /// Builds a terminal output chunk with the required fields set.
    #[must_use]
    pub fn new(terminal_id: impl Into<TerminalId>, data: impl Into<String>) -> Self {
        Self {
            terminal_id: terminal_id.into(),
            data: data.into(),
            meta: None,
        }
    }

    /// Sets or clears chunk-scoped metadata.
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_reference_meta_is_optional_and_content_scoped() {
        let omitted = Terminal::new("term_1");
        assert_eq!(
            serde_json::to_value(&omitted).unwrap(),
            serde_json::json!({
                "terminalId": "term_1"
            })
        );

        let null: Terminal = serde_json::from_value(serde_json::json!({
            "terminalId": "term_1",
            "_meta": null
        }))
        .unwrap();
        assert_eq!(null.meta, None);

        let mut meta = Meta::new();
        meta.insert("view".to_string(), serde_json::json!("expanded"));
        assert_eq!(
            serde_json::to_value(Terminal::new("term_1").meta(meta)).unwrap(),
            serde_json::json!({
                "terminalId": "term_1",
                "_meta": {
                    "view": "expanded"
                }
            })
        );
    }

    #[test]
    fn terminal_update_distinguishes_omitted_null_and_value() {
        let update = TerminalUpdate::new("term_1")
            .command("cargo test")
            .cwd("/workspace/project")
            .output(None::<TerminalOutput>)
            .exit_status(TerminalExitStatus::new().exit_code(0));

        assert_eq!(
            serde_json::to_value(update).unwrap(),
            serde_json::json!({
                "terminalId": "term_1",
                "command": "cargo test",
                "cwd": "/workspace/project",
                "output": null,
                "exitStatus": {
                    "exitCode": 0
                }
            })
        );

        let parsed: TerminalUpdate = serde_json::from_value(serde_json::json!({
            "terminalId": "term_1",
            "command": null,
            "cwd": "/workspace/project"
        }))
        .unwrap();
        assert_eq!(parsed.command, MaybeUndefined::Null);
        assert_eq!(
            parsed.cwd,
            MaybeUndefined::Value(AbsolutePath::new("/workspace/project"))
        );
        assert_eq!(parsed.output, MaybeUndefined::Undefined);
        assert_eq!(parsed.exit_status, MaybeUndefined::Undefined);
        assert_eq!(parsed.meta, MaybeUndefined::Undefined);
    }

    #[test]
    fn terminal_update_applies_patch_fields() {
        let mut stored = TerminalUpdate::new("term_1")
            .command("cargo test")
            .cwd("/workspace/project")
            .output(TerminalOutput::new("b2xk"));

        stored.apply_update(
            TerminalUpdate::new("term_1")
                .command(None::<String>)
                .output(TerminalOutput::new("bmV3"))
                .exit_status(TerminalExitStatus::new().signal("SIGTERM")),
        );

        assert_eq!(stored.command, MaybeUndefined::Null);
        assert_eq!(
            stored.cwd,
            MaybeUndefined::Value(AbsolutePath::new("/workspace/project"))
        );
        assert_eq!(
            stored.output,
            MaybeUndefined::Value(TerminalOutput::new("bmV3"))
        );
        assert_eq!(
            stored.exit_status,
            MaybeUndefined::Value(TerminalExitStatus::new().signal("SIGTERM"))
        );
    }

    #[test]
    fn terminal_output_chunk_serializes_base64_data_and_meta() {
        let mut meta = Meta::new();
        meta.insert("source".to_string(), serde_json::json!("pty"));

        assert_eq!(
            serde_json::to_value(TerminalOutputChunk::new("term_1", "8J+agA==").meta(meta))
                .unwrap(),
            serde_json::json!({
                "terminalId": "term_1",
                "data": "8J+agA==",
                "_meta": {
                    "source": "pty"
                }
            })
        );
    }

    #[test]
    fn terminal_output_meta_is_optional_and_snapshot_scoped() {
        let omitted = TerminalOutput::new("b2s=");
        assert_eq!(
            serde_json::to_value(&omitted).unwrap(),
            serde_json::json!({ "data": "b2s=" })
        );

        let null: TerminalOutput = serde_json::from_value(serde_json::json!({
            "data": "b2s=",
            "_meta": null
        }))
        .unwrap();
        assert_eq!(null, omitted);

        let mut meta = Meta::new();
        meta.insert("source".to_string(), serde_json::json!("replay"));
        assert_eq!(
            serde_json::to_value(TerminalOutput::new("b2s=").meta(meta)).unwrap(),
            serde_json::json!({
                "data": "b2s=",
                "_meta": {
                    "source": "replay"
                }
            })
        );
    }

    #[test]
    fn terminal_output_fields_use_base64_content_encoding() {
        let output = serde_json::to_value(schemars::schema_for!(TerminalOutput)).unwrap();
        assert_eq!(output["properties"]["data"]["contentEncoding"], "base64");
        assert!(output["properties"]["data"].get("format").is_none());
        assert_eq!(output["required"], serde_json::json!(["data"]));
        assert!(output["properties"].get("_meta").is_some());
        assert!(output["properties"].get("truncated").is_none());

        let chunk = serde_json::to_value(schemars::schema_for!(TerminalOutputChunk)).unwrap();
        assert_eq!(chunk["properties"]["data"]["contentEncoding"], "base64");
        assert!(chunk["properties"]["data"].get("format").is_none());
    }

    #[test]
    fn terminal_exit_status_treats_omitted_and_null_as_unknown() {
        let omitted: TerminalExitStatus = serde_json::from_value(serde_json::json!({})).unwrap();
        let null: TerminalExitStatus = serde_json::from_value(serde_json::json!({
            "exitCode": null,
            "signal": null,
            "_meta": null
        }))
        .unwrap();

        assert_eq!(omitted, TerminalExitStatus::new());
        assert_eq!(null, TerminalExitStatus::new());
        assert_eq!(serde_json::to_value(null).unwrap(), serde_json::json!({}));
    }

    #[test]
    fn terminal_exit_status_meta_is_exit_scoped() {
        let mut meta = Meta::new();
        meta.insert("reason".to_string(), serde_json::json!("timeout"));

        assert_eq!(
            serde_json::to_value(TerminalExitStatus::new().signal("SIGTERM").meta(meta)).unwrap(),
            serde_json::json!({
                "signal": "SIGTERM",
                "_meta": {
                    "reason": "timeout"
                }
            })
        );
    }
}
