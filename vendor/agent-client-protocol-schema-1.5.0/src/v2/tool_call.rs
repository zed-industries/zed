//! Tool calls represent actions that language models request agents to perform.
//!
//! When an LLM determines it needs to interact with external systems—like reading files,
//! running code, or fetching data—it generates tool calls that the agent executes on its behalf.
//!
/// See protocol docs: [Tool Calls](https://agentclientprotocol.com/protocol/tool-calls)
use std::{collections::BTreeMap, sync::Arc};

use derive_more::{Display, From};
use schemars::{JsonSchema, Schema};
use serde::{Deserialize, Serialize};
use serde_with::{DefaultOnError, VecSkipError, serde_as, skip_serializing_none};

use super::{AbsolutePath, ContentBlock, MediaType, Meta, Terminal};
use crate::{IntoMaybeUndefined, IntoOption, MaybeUndefined, SkipListener};

/// Represents an upsert for a tool call that the language model has requested.
///
/// Tool calls are actions that the agent executes on behalf of the language model,
/// such as reading files, executing code, or fetching data from external sources.
///
/// Only [`ToolCallUpdate::tool_call_id`] is required. Other fields have patch semantics:
/// omitted fields leave the existing tool call value unchanged, `null` clears or
/// unsets the value, and concrete values replace the previous value. For
/// collection fields, concrete arrays replace the previous collection, and both
/// `null` and `[]` clear the collection. When a client receives a tool call ID it
/// has not seen before, omitted fields use client defaults.
///
/// See protocol docs: [Tool Calls](https://agentclientprotocol.com/protocol/tool-calls)
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ToolCallUpdate {
    /// Unique identifier for this tool call within the session.
    pub tool_call_id: ToolCallId,
    /// Human-readable title describing what the tool is doing.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default, skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub title: MaybeUndefined<String>,
    /// The category of tool being invoked.
    /// Helps clients choose appropriate icons and UI treatment.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default, skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub kind: MaybeUndefined<ToolKind>,
    /// Current execution status of the tool call.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default, skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub status: MaybeUndefined<ToolCallStatus>,
    /// Content produced by the tool call.
    #[serde_as(deserialize_as = "DefaultOnError<MaybeUndefined<VecSkipError<_, SkipListener>>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub content: MaybeUndefined<Vec<ToolCallContent>>,
    /// File locations affected by this tool call.
    /// Enables "follow-along" features in clients.
    #[serde_as(deserialize_as = "DefaultOnError<MaybeUndefined<VecSkipError<_, SkipListener>>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default, skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub locations: MaybeUndefined<Vec<ToolCallLocation>>,
    /// Raw input parameters sent to the tool.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default, skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub raw_input: MaybeUndefined<serde_json::Value>,
    /// Raw output returned by the tool.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default, skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub raw_output: MaybeUndefined<serde_json::Value>,
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

impl ToolCallUpdate {
    /// Builds [`ToolCallUpdate`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(tool_call_id: impl Into<ToolCallId>) -> Self {
        Self {
            tool_call_id: tool_call_id.into(),
            title: MaybeUndefined::Undefined,
            kind: MaybeUndefined::Undefined,
            status: MaybeUndefined::Undefined,
            content: MaybeUndefined::Undefined,
            locations: MaybeUndefined::Undefined,
            raw_input: MaybeUndefined::Undefined,
            raw_output: MaybeUndefined::Undefined,
            meta: MaybeUndefined::Undefined,
        }
    }

    /// Human-readable title describing what the tool is doing.
    #[must_use]
    pub fn title(mut self, title: impl IntoMaybeUndefined<String>) -> Self {
        self.title = title.into_maybe_undefined();
        self
    }

    /// The category of tool being invoked.
    /// Helps clients choose appropriate icons and UI treatment.
    #[must_use]
    pub fn kind(mut self, kind: impl IntoMaybeUndefined<ToolKind>) -> Self {
        self.kind = kind.into_maybe_undefined();
        self
    }

    /// Current execution status of the tool call.
    #[must_use]
    pub fn status(mut self, status: impl IntoMaybeUndefined<ToolCallStatus>) -> Self {
        self.status = status.into_maybe_undefined();
        self
    }

    /// Content produced by the tool call.
    #[must_use]
    pub fn content(mut self, content: impl IntoMaybeUndefined<Vec<ToolCallContent>>) -> Self {
        self.content = content.into_maybe_undefined();
        self
    }

    /// File locations affected by this tool call.
    /// Enables "follow-along" features in clients.
    #[must_use]
    pub fn locations(mut self, locations: impl IntoMaybeUndefined<Vec<ToolCallLocation>>) -> Self {
        self.locations = locations.into_maybe_undefined();
        self
    }

    /// Raw input parameters sent to the tool.
    #[must_use]
    pub fn raw_input(mut self, raw_input: impl IntoMaybeUndefined<serde_json::Value>) -> Self {
        self.raw_input = raw_input.into_maybe_undefined();
        self
    }

    /// Raw output returned by the tool.
    #[must_use]
    pub fn raw_output(mut self, raw_output: impl IntoMaybeUndefined<serde_json::Value>) -> Self {
        self.raw_output = raw_output.into_maybe_undefined();
        self
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Omitted means no metadata update; `null` is an
    /// explicit clear signal. Implementations MUST NOT make assumptions about values at these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoMaybeUndefined<Meta>) -> Self {
        self.meta = meta.into_maybe_undefined();
        self
    }

    /// Applies a later tool-call patch to this stored tool-call state.
    ///
    /// Fields set to `null` are preserved as `null` so callers can decide how to
    /// render an explicitly cleared value.
    pub fn apply_update(&mut self, update: ToolCallUpdate) {
        debug_assert_eq!(self.tool_call_id, update.tool_call_id);
        if !update.title.is_undefined() {
            self.title = update.title;
        }
        if !update.kind.is_undefined() {
            self.kind = update.kind;
        }
        if !update.status.is_undefined() {
            self.status = update.status;
        }
        if !update.content.is_undefined() {
            self.content = update.content;
        }
        if !update.locations.is_undefined() {
            self.locations = update.locations;
        }
        if !update.raw_input.is_undefined() {
            self.raw_input = update.raw_input;
        }
        if !update.raw_output.is_undefined() {
            self.raw_output = update.raw_output;
        }
        if !update.meta.is_undefined() {
            self.meta = update.meta;
        }
    }
}

/// A streamed item of tool-call content.
///
/// Tool-call content chunks append one [`ToolCallContent`] item to the current
/// content for the matching [`ToolCallId`]. Agents can use
/// [`ToolCallUpdate::content`] when they need to replace the whole content
/// collection instead.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ToolCallContentChunk {
    /// The ID of the tool call this content belongs to.
    pub tool_call_id: ToolCallId,
    /// A single item of content produced by the tool call.
    pub content: ToolCallContent,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys. This field is chunk-scoped.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl ToolCallContentChunk {
    /// Builds [`ToolCallContentChunk`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(tool_call_id: impl Into<ToolCallId>, content: impl Into<ToolCallContent>) -> Self {
        Self {
            tool_call_id: tool_call_id.into(),
            content: content.into(),
            meta: None,
        }
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys. This field is chunk-scoped.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// Unique identifier for a tool call within a session.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Display, From)]
#[serde(transparent)]
#[from(forward)]
#[non_exhaustive]
pub struct ToolCallId(pub Arc<str>);

impl ToolCallId {
    /// Wraps a protocol string as a typed [`ToolCallId`].
    #[must_use]
    pub fn new(id: impl Into<Self>) -> Self {
        id.into()
    }
}

impl IntoOption<ToolCallId> for &str {
    fn into_option(self) -> Option<ToolCallId> {
        Some(ToolCallId::new(self))
    }
}

/// Categories of tools that can be invoked.
///
/// Tool kinds help clients choose appropriate icons and optimize how they
/// display tool execution progress.
///
/// See protocol docs: [Creating](https://agentclientprotocol.com/protocol/tool-calls#creating)
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ToolKind {
    /// Reading files or data.
    Read,
    /// Modifying files or content.
    Edit,
    /// Removing files or data.
    Delete,
    /// Moving or renaming files.
    Move,
    /// Searching for information.
    Search,
    /// Running commands or code.
    Execute,
    /// Internal reasoning or planning.
    Think,
    /// Retrieving external data.
    Fetch,
    /// Switching the current session mode.
    SwitchMode,
    /// Other tool types (default).
    #[default]
    Other,
    /// Custom or future tool kind.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(untagged)]
    Unknown(String),
}

/// Execution status of a tool call.
///
/// Tool calls progress through different statuses during their lifecycle.
///
/// See protocol docs: [Status](https://agentclientprotocol.com/protocol/tool-calls#status)
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ToolCallStatus {
    /// The tool call hasn't started running yet because the input is either
    /// streaming or we're awaiting approval.
    #[default]
    Pending,
    /// The tool call is currently running.
    InProgress,
    /// The tool call completed successfully.
    Completed,
    /// The tool call failed with an error.
    Failed,
    /// Custom or future tool call status.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(untagged)]
    Other(String),
}

/// Content produced by a tool call.
///
/// Tool calls can produce different types of content including standard
/// content blocks (text, images), file diffs, or display-only terminals.
///
/// See protocol docs: [Content](https://agentclientprotocol.com/protocol/tool-calls#content)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ToolCallContent {
    /// Standard content block (text, images, resources).
    Content(Box<Content>),
    /// File modification shown as a diff.
    Diff(Diff),
    /// A display-only reference to an agent-owned terminal.
    Terminal(Terminal),
    /// Custom or future tool call content.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    ///
    /// Receivers that do not understand this content type should preserve the
    /// raw payload when storing, replaying, proxying, or forwarding tool call
    /// output, and otherwise ignore it or display it generically.
    #[serde(untagged)]
    Other(OtherToolCallContent),
}

/// Custom or future tool call content payload.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
#[schemars(inline)]
#[schemars(transform = other_tool_call_content_schema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OtherToolCallContent {
    /// Custom or future tool call content type.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(rename = "type")]
    pub type_: String,
    /// Additional fields from the unknown tool call content payload.
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl OtherToolCallContent {
    /// Builds [`OtherToolCallContent`] from an unknown discriminator and preserves the remaining extension fields.
    #[must_use]
    pub fn new(type_: impl Into<String>, mut fields: BTreeMap<String, serde_json::Value>) -> Self {
        fields.remove("type");
        Self {
            type_: type_.into(),
            fields,
        }
    }
}

impl<'de> Deserialize<'de> for OtherToolCallContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut fields = BTreeMap::<String, serde_json::Value>::deserialize(deserializer)?;
        let type_ = fields
            .remove("type")
            .ok_or_else(|| serde::de::Error::missing_field("type"))?;
        let serde_json::Value::String(type_) = type_ else {
            return Err(serde::de::Error::custom("`type` must be a string"));
        };

        if is_known_tool_call_content_type(&type_) {
            return Err(serde::de::Error::custom(format!(
                "known tool call content `{type_}` did not match its schema"
            )));
        }

        Ok(Self { type_, fields })
    }
}

fn is_known_tool_call_content_type(type_: &str) -> bool {
    matches!(type_, "content" | "diff" | "terminal")
}

fn other_tool_call_content_schema(schema: &mut Schema) {
    super::schema_util::reject_known_string_discriminators(
        schema,
        "type",
        &["content", "diff", "terminal"],
    );
}

impl<T: Into<ContentBlock>> From<T> for ToolCallContent {
    fn from(content: T) -> Self {
        ToolCallContent::Content(Box::new(Content::new(content)))
    }
}

impl From<Diff> for ToolCallContent {
    fn from(diff: Diff) -> Self {
        ToolCallContent::Diff(diff)
    }
}

impl From<Terminal> for ToolCallContent {
    fn from(terminal: Terminal) -> Self {
        ToolCallContent::Terminal(terminal)
    }
}

/// Standard content block (text, images, resources).
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Content {
    /// The actual content block.
    pub content: ContentBlock,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl Content {
    /// Builds [`Content`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(content: impl Into<ContentBlock>) -> Self {
        Self {
            content: content.into(),
            meta: None,
        }
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// File changes produced by a tool call.
///
/// `changes` is authoritative for affected absolute paths and operations.
/// `patch` optionally carries renderable text for some or all of those changes
/// and MUST be consistent with `changes`. Agents SHOULD provide `patch` whenever
/// feasible. Clients MUST handle diffs where `patch` is omitted or `null`.
///
/// See protocol docs: [Content](https://agentclientprotocol.com/protocol/tool-calls#content)
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Diff {
    /// Structured file changes described by this diff.
    ///
    /// Clients can use this field without parsing patch text to determine affected paths.
    #[serde_as(deserialize_as = "VecSkipError<_, SkipListener>")]
    #[schemars(extend("x-deserialize-skip-invalid-items" = true))]
    pub changes: Vec<DiffChange>,
    /// Renderable patch text for some or all of the structured changes.
    ///
    /// Agents SHOULD provide patch text whenever feasible. Omitted or `null`
    /// means no renderable patch text was provided.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub patch: Option<DiffPatch>,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl Diff {
    /// Builds [`Diff`] with structured file changes.
    #[must_use]
    pub fn new(changes: Vec<DiffChange>) -> Self {
        Self {
            changes,
            patch: None,
            meta: None,
        }
    }

    /// Builds [`Diff`] with Git `--patch` text and structured file changes.
    #[must_use]
    pub fn patch(text: impl Into<String>, changes: Vec<DiffChange>) -> Self {
        Self::new(changes).with_patch(DiffPatch::new(text))
    }

    /// Sets renderable patch text.
    #[must_use]
    pub fn with_patch(mut self, patch: impl IntoOption<DiffPatch>) -> Self {
        self.patch = patch.into_option();
        self
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// Renderable patch text and its format.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DiffPatch {
    /// Patch format. The only ACP-defined value is `git_patch`.
    pub format: DiffPatchFormat,
    /// Patch text in the format named by `format`.
    pub text: String,
}

impl DiffPatch {
    /// Builds [`DiffPatch`] with Git `--patch` text.
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            format: DiffPatchFormat::GitPatch,
            text: text.into(),
        }
    }
}

/// Text patch format used by [`DiffPatch`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DiffPatchFormat {
    /// One or more `diff --git` sections in Git's `--patch` (`-p`) text format.
    ///
    /// Paths MUST be absolute. Surrounding commit metadata and email envelopes
    /// MUST NOT be included.
    GitPatch,
    /// Custom or future patch format.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(untagged)]
    Other(String),
}

/// Kind of file content represented by a diff change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DiffFileType {
    /// Text content.
    Text,
    /// Binary or otherwise non-text content.
    Binary,
    /// Directory entry.
    Directory,
    /// Symbolic link.
    Symlink,
    /// Custom or future file type.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(untagged)]
    Other(String),
}

/// One file-level change described by a [`Diff`].
///
/// Structured change metadata lets clients identify affected files and
/// operations without parsing the text patch.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DiffChange {
    /// File content kind.
    ///
    /// Omitted or `null` means the content kind is unknown.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub file_type: Option<DiffFileType>,
    /// MIME type of the file contents.
    ///
    /// Omitted or `null` means the MIME type is unknown.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub mime_type: Option<MediaType>,
    /// File operation-specific fields.
    #[serde(flatten)]
    pub operation: DiffChangeOperation,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl DiffChange {
    /// Builds [`DiffChange`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(operation: DiffChangeOperation) -> Self {
        Self {
            file_type: None,
            mime_type: None,
            operation,
            meta: None,
        }
    }

    /// Builds a file add change.
    #[must_use]
    pub fn add(path: impl Into<AbsolutePath>) -> Self {
        Self::new(DiffChangeOperation::Add(DiffPathChange::new(path)))
    }

    /// Builds a file delete change.
    #[must_use]
    pub fn delete(path: impl Into<AbsolutePath>) -> Self {
        Self::new(DiffChangeOperation::Delete(DiffPathChange::new(path)))
    }

    /// Builds a file modify change.
    #[must_use]
    pub fn modify(path: impl Into<AbsolutePath>) -> Self {
        Self::new(DiffChangeOperation::Modify(DiffPathChange::new(path)))
    }

    /// Builds a file move or rename change.
    #[must_use]
    pub fn move_file(old_path: impl Into<AbsolutePath>, path: impl Into<AbsolutePath>) -> Self {
        Self::new(DiffChangeOperation::Move(DiffPathPairChange::new(
            old_path, path,
        )))
    }

    /// Builds a file copy change.
    #[must_use]
    pub fn copy(old_path: impl Into<AbsolutePath>, path: impl Into<AbsolutePath>) -> Self {
        Self::new(DiffChangeOperation::Copy(DiffPathPairChange::new(
            old_path, path,
        )))
    }

    /// File content kind.
    ///
    /// Omitted or `null` means the content kind is unknown.
    #[must_use]
    pub fn file_type(mut self, file_type: impl IntoOption<DiffFileType>) -> Self {
        self.file_type = file_type.into_option();
        self
    }

    /// MIME type of the file contents.
    ///
    /// Omitted or `null` means the MIME type is unknown.
    #[must_use]
    pub fn mime_type(mut self, mime_type: impl IntoOption<MediaType>) -> Self {
        self.mime_type = mime_type.into_option();
        self
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// File operation for a [`DiffChange`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "operation", rename_all = "snake_case")]
#[non_exhaustive]
pub enum DiffChangeOperation {
    /// A file was added.
    Add(DiffPathChange),
    /// A file was deleted.
    Delete(DiffPathChange),
    /// A file was modified in place.
    Modify(DiffPathChange),
    /// A file was moved or renamed.
    Move(DiffPathPairChange),
    /// A file was copied.
    Copy(DiffPathPairChange),
    /// Custom or future file operation.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(untagged)]
    Other(OtherDiffChange),
}

/// Operation metadata for add, delete, and modify changes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DiffPathChange {
    /// Absolute path for the operation.
    pub path: AbsolutePath,
}

impl DiffPathChange {
    /// Builds [`DiffPathChange`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(path: impl Into<AbsolutePath>) -> Self {
        Self { path: path.into() }
    }
}

/// Operation metadata for move and copy changes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DiffPathPairChange {
    /// Absolute path before the operation.
    pub old_path: AbsolutePath,
    /// Absolute path after the operation.
    pub path: AbsolutePath,
}

impl DiffPathPairChange {
    /// Builds [`DiffPathPairChange`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(old_path: impl Into<AbsolutePath>, path: impl Into<AbsolutePath>) -> Self {
        Self {
            old_path: old_path.into(),
            path: path.into(),
        }
    }
}

/// Custom or future file operation payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
#[schemars(inline)]
#[schemars(transform = other_diff_change_schema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OtherDiffChange {
    /// Custom or future file operation.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    pub operation: String,
    /// Additional fields from the unknown file operation payload.
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl OtherDiffChange {
    /// Builds [`OtherDiffChange`] from an unknown discriminator and preserves the remaining extension fields.
    #[must_use]
    pub fn new(
        operation: impl Into<String>,
        mut fields: BTreeMap<String, serde_json::Value>,
    ) -> Self {
        fields.remove("operation");
        fields.remove("fileType");
        fields.remove("mimeType");
        fields.remove("_meta");
        Self {
            operation: operation.into(),
            fields,
        }
    }
}

impl<'de> Deserialize<'de> for OtherDiffChange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut fields = BTreeMap::<String, serde_json::Value>::deserialize(deserializer)?;
        let operation = fields
            .remove("operation")
            .ok_or_else(|| serde::de::Error::missing_field("operation"))?;
        let serde_json::Value::String(operation) = operation else {
            return Err(serde::de::Error::custom("`operation` must be a string"));
        };

        if is_known_diff_change_operation(&operation) {
            return Err(serde::de::Error::custom(format!(
                "known diff change operation `{operation}` did not match its schema"
            )));
        }
        fields.remove("fileType");
        fields.remove("mimeType");
        fields.remove("_meta");

        Ok(Self { operation, fields })
    }
}

fn is_known_diff_change_operation(operation: &str) -> bool {
    matches!(operation, "add" | "delete" | "modify" | "move" | "copy")
}

fn other_diff_change_schema(schema: &mut Schema) {
    super::schema_util::reject_known_string_discriminators(
        schema,
        "operation",
        &["add", "delete", "modify", "move", "copy"],
    );
}

/// A file location being accessed or modified by a tool.
///
/// Enables clients to implement "follow-along" features that track
/// which files the agent is working with in real-time.
///
/// See protocol docs: [Following the Agent](https://agentclientprotocol.com/protocol/tool-calls#following-the-agent)
#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ToolCallLocation {
    /// The absolute file path being accessed or modified.
    pub path: AbsolutePath,
    /// Optional line number within the file.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub line: Option<u32>,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl ToolCallLocation {
    /// Builds [`ToolCallLocation`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(path: impl Into<AbsolutePath>) -> Self {
        Self {
            path: path.into(),
            line: None,
            meta: None,
        }
    }

    /// Optional line number within the file.
    #[must_use]
    pub fn line(mut self, line: impl IntoOption<u32>) -> Self {
        self.line = line.into_option();
        self
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MaybeUndefined;

    #[test]
    fn tool_call_serializes_as_upsert() {
        let tool_call = ToolCallUpdate::new("tc_1")
            .title("Reading configuration")
            .status(ToolCallStatus::InProgress)
            .raw_input(serde_json::json!({"path": "settings.json"}));

        assert_eq!(
            serde_json::to_value(tool_call).unwrap(),
            serde_json::json!({
                "toolCallId": "tc_1",
                "title": "Reading configuration",
                "status": "in_progress",
                "rawInput": {
                    "path": "settings.json"
                }
            })
        );
    }

    #[test]
    fn tool_call_update_distinguishes_omitted_null_and_value() {
        let tool_call = ToolCallUpdate::new("tc_1")
            .status(ToolCallStatus::Completed)
            .content(None::<Vec<ToolCallContent>>);

        assert_eq!(
            serde_json::to_value(tool_call).unwrap(),
            serde_json::json!({
                "toolCallId": "tc_1",
                "status": "completed",
                "content": null
            })
        );

        let deserialized: ToolCallUpdate = serde_json::from_value(serde_json::json!({
            "toolCallId": "tc_1",
            "status": null,
            "locations": []
        }))
        .unwrap();
        assert_eq!(deserialized.title, MaybeUndefined::Undefined);
        assert_eq!(deserialized.status, MaybeUndefined::Null);
        assert_eq!(deserialized.locations, MaybeUndefined::Value(Vec::new()));
    }

    #[test]
    fn tool_call_update_distinguishes_meta_omitted_null_and_value() {
        let mut meta = Meta::new();
        meta.insert("source".to_string(), serde_json::json!("tool-call"));

        assert_eq!(
            serde_json::to_value(ToolCallUpdate::new("tc_1").meta(meta.clone())).unwrap(),
            serde_json::json!({
                "toolCallId": "tc_1",
                "_meta": {
                    "source": "tool-call"
                }
            })
        );

        assert_eq!(
            serde_json::to_value(ToolCallUpdate::new("tc_1").meta(None::<Meta>)).unwrap(),
            serde_json::json!({
                "toolCallId": "tc_1",
                "_meta": null
            })
        );

        let deserialized: ToolCallUpdate = serde_json::from_value(serde_json::json!({
            "toolCallId": "tc_1",
            "_meta": null
        }))
        .unwrap();
        assert_eq!(deserialized.meta, MaybeUndefined::Null);

        let patch = ToolCallUpdate::new("tc_1");
        assert_eq!(patch.meta, MaybeUndefined::Undefined);

        let mut stored = ToolCallUpdate::new("tc_1").meta(meta);
        stored.apply_update(ToolCallUpdate::new("tc_1").meta(None::<Meta>));
        assert_eq!(stored.meta, MaybeUndefined::Null);
    }

    #[test]
    fn tool_call_update_skips_malformed_list_items() {
        let deserialized: ToolCallUpdate = serde_json::from_value(serde_json::json!({
            "toolCallId": "tc_1",
            "content": [
                {
                    "type": "content",
                    "content": {
                        "type": "text",
                        "text": "ok"
                    }
                },
                {
                    "type": "diff",
                    "path": "/bad"
                }
            ],
            "locations": [
                {
                    "path": "/ok",
                    "line": 3
                },
                {
                    "line": 4
                }
            ]
        }))
        .unwrap();

        let MaybeUndefined::Value(content) = deserialized.content else {
            panic!("content should deserialize to a value");
        };
        assert_eq!(content.len(), 1);

        let MaybeUndefined::Value(locations) = deserialized.locations else {
            panic!("locations should deserialize to a value");
        };
        assert_eq!(locations.len(), 1);
    }

    #[test]
    fn tool_call_content_chunk_serializes_single_content_item() {
        let chunk = ToolCallContentChunk::new(
            "tc_1",
            ContentBlock::Text(crate::v2::TextContent::new("partial output")),
        );

        assert_eq!(
            serde_json::to_value(chunk).unwrap(),
            serde_json::json!({
                "toolCallId": "tc_1",
                "content": {
                    "type": "content",
                    "content": {
                        "type": "text",
                        "text": "partial output"
                    }
                }
            })
        );
    }

    #[test]
    fn terminal_content_serializes_as_display_reference() {
        let terminal = ToolCallContent::from(Terminal::new("term_1"));

        assert_eq!(
            serde_json::to_value(terminal).unwrap(),
            serde_json::json!({
                "type": "terminal",
                "terminalId": "term_1"
            })
        );
    }

    #[test]
    fn diff_patch_serializes_git_patch_with_structured_changes() {
        let patch_text = "diff --git /repo/config.json /repo/config.json\n--- /repo/config.json\n+++ /repo/config.json\n@@ -1 +1 @@\n-old\n+new\n";
        let diff = ToolCallContent::Diff(Diff::patch(
            patch_text,
            vec![
                DiffChange::modify("/repo/config.json")
                    .file_type(DiffFileType::Text)
                    .mime_type("application/json"),
            ],
        ));

        assert_eq!(
            serde_json::to_value(diff).unwrap(),
            serde_json::json!({
                "type": "diff",
                "changes": [
                    {
                        "operation": "modify",
                        "path": "/repo/config.json",
                        "fileType": "text",
                        "mimeType": "application/json"
                    }
                ],
                "patch": {
                    "format": "git_patch",
                    "text": patch_text
                }
            })
        );
    }

    #[test]
    fn diff_patch_requires_text() {
        let result = serde_json::from_value::<DiffPatch>(serde_json::json!({
            "format": "git_patch",
            "diff": "diff --git /repo/config.json /repo/config.json\n"
        }));

        assert!(result.is_err());
    }

    #[test]
    fn diff_serializes_binary_modify_without_patch_text() {
        let diff = ToolCallContent::Diff(Diff::new(vec![
            DiffChange::modify("/repo/assets/logo.png")
                .file_type(DiffFileType::Binary)
                .mime_type("image/png"),
        ]));

        assert_eq!(
            serde_json::to_value(diff).unwrap(),
            serde_json::json!({
                "type": "diff",
                "changes": [
                    {
                        "operation": "modify",
                        "path": "/repo/assets/logo.png",
                        "fileType": "binary",
                        "mimeType": "image/png"
                    }
                ]
            })
        );
    }

    #[test]
    fn diff_move_serializes_shared_fields_with_operation_payload() {
        let diff = ToolCallContent::Diff(Diff::new(vec![
            DiffChange::move_file("/repo/src/old.rs", "/repo/src/new.rs")
                .file_type(DiffFileType::Text)
                .mime_type("text/rust"),
        ]));

        assert_eq!(
            serde_json::to_value(diff).unwrap(),
            serde_json::json!({
                "type": "diff",
                "changes": [
                    {
                        "operation": "move",
                        "oldPath": "/repo/src/old.rs",
                        "path": "/repo/src/new.rs",
                        "fileType": "text",
                        "mimeType": "text/rust"
                    }
                ]
            })
        );
    }

    #[test]
    fn diff_changes_skip_malformed_list_items() {
        let patch_text = "diff --git /ok /ok\ndeleted file mode 100644\n--- /ok\n+++ /dev/null\n@@ -1 +0,0 @@\n-old\n";
        let content: ToolCallContent = serde_json::from_value(serde_json::json!({
            "type": "diff",
            "changes": [
                {
                    "operation": "modify"
                },
                {
                    "operation": "delete",
                    "path": "/ok"
                }
            ],
            "patch": {
                "format": "git_patch",
                "text": patch_text
            }
        }))
        .unwrap();

        let ToolCallContent::Diff(diff) = content else {
            panic!("expected diff content");
        };
        assert_eq!(diff.changes, vec![DiffChange::delete("/ok")]);
        assert_eq!(diff.patch, Some(DiffPatch::new(patch_text)));
    }

    #[test]
    fn tool_kind_preserves_unknown_variant() {
        let kind: ToolKind = serde_json::from_str("\"review\"").unwrap();
        assert_eq!(kind, ToolKind::Unknown("review".to_string()));
        assert_eq!(serde_json::to_value(&kind).unwrap(), "review");
    }

    #[test]
    fn tool_call_status_preserves_unknown_variant() {
        let status: ToolCallStatus = serde_json::from_str("\"deferred\"").unwrap();
        assert_eq!(status, ToolCallStatus::Other("deferred".to_string()));
        assert_eq!(serde_json::to_value(&status).unwrap(), "deferred");
    }

    #[test]
    fn tool_call_content_preserves_unknown_variant() {
        let content: ToolCallContent = serde_json::from_value(serde_json::json!({
            "type": "_chart",
            "title": "Tests",
            "data": [1, 2, 3]
        }))
        .unwrap();

        let ToolCallContent::Other(unknown) = content else {
            panic!("expected unknown tool call content");
        };

        assert_eq!(unknown.type_, "_chart");
        assert_eq!(
            unknown.fields.get("title"),
            Some(&serde_json::json!("Tests"))
        );
        assert_eq!(
            serde_json::to_value(ToolCallContent::Other(unknown)).unwrap(),
            serde_json::json!({
                "type": "_chart",
                "title": "Tests",
                "data": [1, 2, 3]
            })
        );
    }

    #[test]
    fn tool_call_content_does_not_hide_malformed_known_variant() {
        assert!(
            serde_json::from_value::<ToolCallContent>(serde_json::json!({
                "type": "diff"
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<ToolCallContent>(serde_json::json!({
                "type": "terminal"
            }))
            .is_err()
        );
    }
}
