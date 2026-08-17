//! Next Edit Suggestions (NES) types and constants.
//!
//! NES allows agents to provide predictive code edits via capability negotiation,
//! document events, and a suggestion request/response flow. NES sessions are
//! independent of chat sessions and have their own lifecycle.

use std::{collections::BTreeMap, sync::Arc};

use derive_more::{Display, From};
use schemars::{JsonSchema, Schema};
use serde::{Deserialize, Serialize};
use serde_with::{DefaultOnError, VecSkipError, serde_as, skip_serializing_none};

use super::{Meta, SessionId};
use crate::{IntoOption, SkipListener};

// Method name constants

/// Method name for starting an NES session.
pub(crate) const NES_START_METHOD_NAME: &str = "nes/start";
/// Method name for requesting a suggestion.
pub(crate) const NES_SUGGEST_METHOD_NAME: &str = "nes/suggest";
/// Method name for accepting a suggestion.
pub(crate) const NES_ACCEPT_METHOD_NAME: &str = "nes/accept";
/// Method name for rejecting a suggestion.
pub(crate) const NES_REJECT_METHOD_NAME: &str = "nes/reject";

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Unique identifier for an NES suggestion.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Display, From)]
#[serde(transparent)]
#[from(forward)]
#[non_exhaustive]
pub struct NesSuggestionId(pub Arc<str>);

impl NesSuggestionId {
    /// Wraps a protocol string as a typed [`NesSuggestionId`].
    #[must_use]
    pub fn new(id: impl Into<Self>) -> Self {
        id.into()
    }
}
/// Method name for closing an NES session.
pub(crate) const NES_CLOSE_METHOD_NAME: &str = "nes/close";
/// Notification name for document open events.
pub(crate) const DOCUMENT_DID_OPEN_METHOD_NAME: &str = "document/didOpen";
/// Notification name for document change events.
pub(crate) const DOCUMENT_DID_CHANGE_METHOD_NAME: &str = "document/didChange";
/// Notification name for document close events.
pub(crate) const DOCUMENT_DID_CLOSE_METHOD_NAME: &str = "document/didClose";
/// Notification name for document save events.
pub(crate) const DOCUMENT_DID_SAVE_METHOD_NAME: &str = "document/didSave";
/// Notification name for document focus events.
pub(crate) const DOCUMENT_DID_FOCUS_METHOD_NAME: &str = "document/didFocus";

// Position primitives

/// The encoding used for character offsets in positions.
///
/// Follows the same conventions as LSP 3.17. The default is UTF-16.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub enum PositionEncodingKind {
    /// Character offsets count UTF-16 code units. This is the default.
    #[serde(rename = "utf-16")]
    Utf16,
    /// Character offsets count Unicode code points.
    #[serde(rename = "utf-32")]
    Utf32,
    /// Character offsets count UTF-8 code units (bytes).
    #[serde(rename = "utf-8")]
    Utf8,
}

/// A zero-based position in a text document.
///
/// The meaning of `character` depends on the negotiated position encoding.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Position {
    /// Zero-based line number.
    pub line: u32,
    /// Zero-based character offset (encoding-dependent).
    pub character: u32,
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

impl Position {
    /// Builds a [`Position`] from protocol coordinate values.
    #[must_use]
    pub fn new(line: u32, character: u32) -> Self {
        Self {
            line,
            character,
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

/// A range in a text document, expressed as start and end positions.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Range {
    /// The start position (inclusive).
    pub start: Position,
    /// The end position (exclusive).
    pub end: Position,
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

impl Range {
    /// Builds a [`Range`] from protocol coordinate values.
    #[must_use]
    pub fn new(start: Position, end: Position) -> Self {
        Self {
            start,
            end,
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

// Agent NES capabilities

/// NES capabilities advertised by the agent during initialization.
///
/// Supplying `{}` means the agent supports the NES method surface. Omitted or
/// `null` both mean the agent does not advertise support for `nes/*` methods.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesCapabilities {
    /// Events the agent wants to receive.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub events: Option<NesEventCapabilities>,
    /// Context the agent wants attached to each suggestion request.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub context: Option<NesContextCapabilities>,
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

impl NesCapabilities {
    /// Builds an empty [`NesCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets or clears the optional `events` field.
    #[must_use]
    pub fn events(mut self, events: impl IntoOption<NesEventCapabilities>) -> Self {
        self.events = events.into_option();
        self
    }

    /// Sets or clears the optional `context` field.
    #[must_use]
    pub fn context(mut self, context: impl IntoOption<NesContextCapabilities>) -> Self {
        self.context = context.into_option();
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

/// Event capabilities the agent can consume.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesEventCapabilities {
    /// Document event capabilities.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub document: Option<NesDocumentEventCapabilities>,
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

impl NesEventCapabilities {
    /// Builds an empty [`NesEventCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets or clears the optional `document` field.
    #[must_use]
    pub fn document(mut self, document: impl IntoOption<NesDocumentEventCapabilities>) -> Self {
        self.document = document.into_option();
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

/// Document event capabilities the agent wants to receive.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesDocumentEventCapabilities {
    /// Whether the agent wants `document/didOpen` events.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub did_open: Option<NesDocumentDidOpenCapabilities>,
    /// Whether the agent wants `document/didChange` events, and the sync kind.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub did_change: Option<NesDocumentDidChangeCapabilities>,
    /// Whether the agent wants `document/didClose` events.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub did_close: Option<NesDocumentDidCloseCapabilities>,
    /// Whether the agent wants `document/didSave` events.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub did_save: Option<NesDocumentDidSaveCapabilities>,
    /// Whether the agent wants `document/didFocus` events.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub did_focus: Option<NesDocumentDidFocusCapabilities>,
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

impl NesDocumentEventCapabilities {
    /// Builds an empty [`NesDocumentEventCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets or clears the optional `didOpen` field.
    #[must_use]
    pub fn did_open(mut self, did_open: impl IntoOption<NesDocumentDidOpenCapabilities>) -> Self {
        self.did_open = did_open.into_option();
        self
    }

    /// Sets or clears the optional `didChange` field.
    #[must_use]
    pub fn did_change(
        mut self,
        did_change: impl IntoOption<NesDocumentDidChangeCapabilities>,
    ) -> Self {
        self.did_change = did_change.into_option();
        self
    }

    /// Sets or clears the optional `didClose` field.
    #[must_use]
    pub fn did_close(
        mut self,
        did_close: impl IntoOption<NesDocumentDidCloseCapabilities>,
    ) -> Self {
        self.did_close = did_close.into_option();
        self
    }

    /// Sets or clears the optional `didSave` field.
    #[must_use]
    pub fn did_save(mut self, did_save: impl IntoOption<NesDocumentDidSaveCapabilities>) -> Self {
        self.did_save = did_save.into_option();
        self
    }

    /// Sets or clears the optional `didFocus` field.
    #[must_use]
    pub fn did_focus(
        mut self,
        did_focus: impl IntoOption<NesDocumentDidFocusCapabilities>,
    ) -> Self {
        self.did_focus = did_focus.into_option();
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

/// Marker for `document/didOpen` capability support.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesDocumentDidOpenCapabilities {
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

impl NesDocumentDidOpenCapabilities {
    /// Builds an empty [`NesDocumentDidOpenCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Capabilities for `document/didChange` events.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesDocumentDidChangeCapabilities {
    /// The sync kind the agent wants: `"full"` or `"incremental"`.
    pub sync_kind: TextDocumentSyncKind,
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

impl NesDocumentDidChangeCapabilities {
    /// Builds an empty [`NesDocumentDidChangeCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new(sync_kind: TextDocumentSyncKind) -> Self {
        Self {
            sync_kind,
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

/// How the agent wants document changes delivered.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub enum TextDocumentSyncKind {
    /// Client sends the entire file content on each change.
    #[serde(rename = "full")]
    Full,
    /// Client sends only the changed ranges.
    #[serde(rename = "incremental")]
    Incremental,
}

/// Marker for `document/didClose` capability support.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesDocumentDidCloseCapabilities {
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

impl NesDocumentDidCloseCapabilities {
    /// Builds an empty [`NesDocumentDidCloseCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Marker for `document/didSave` capability support.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesDocumentDidSaveCapabilities {
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

impl NesDocumentDidSaveCapabilities {
    /// Builds an empty [`NesDocumentDidSaveCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Marker for `document/didFocus` capability support.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesDocumentDidFocusCapabilities {
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

impl NesDocumentDidFocusCapabilities {
    /// Builds an empty [`NesDocumentDidFocusCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Context capabilities the agent wants attached to each suggestion request.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesContextCapabilities {
    /// Whether the agent wants recent files context.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub recent_files: Option<NesRecentFilesCapabilities>,
    /// Whether the agent wants related snippets context.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub related_snippets: Option<NesRelatedSnippetsCapabilities>,
    /// Whether the agent wants edit history context.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub edit_history: Option<NesEditHistoryCapabilities>,
    /// Whether the agent wants user actions context.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub user_actions: Option<NesUserActionsCapabilities>,
    /// Whether the agent wants open files context.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub open_files: Option<NesOpenFilesCapabilities>,
    /// Whether the agent wants diagnostics context.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub diagnostics: Option<NesDiagnosticsCapabilities>,
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

impl NesContextCapabilities {
    /// Builds an empty [`NesContextCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets or clears the optional `recentFiles` field.
    #[must_use]
    pub fn recent_files(
        mut self,
        recent_files: impl IntoOption<NesRecentFilesCapabilities>,
    ) -> Self {
        self.recent_files = recent_files.into_option();
        self
    }

    /// Sets or clears the optional `relatedSnippets` field.
    #[must_use]
    pub fn related_snippets(
        mut self,
        related_snippets: impl IntoOption<NesRelatedSnippetsCapabilities>,
    ) -> Self {
        self.related_snippets = related_snippets.into_option();
        self
    }

    /// Sets or clears the optional `editHistory` field.
    #[must_use]
    pub fn edit_history(
        mut self,
        edit_history: impl IntoOption<NesEditHistoryCapabilities>,
    ) -> Self {
        self.edit_history = edit_history.into_option();
        self
    }

    /// Sets or clears the optional `userActions` field.
    #[must_use]
    pub fn user_actions(
        mut self,
        user_actions: impl IntoOption<NesUserActionsCapabilities>,
    ) -> Self {
        self.user_actions = user_actions.into_option();
        self
    }

    /// Sets or clears the optional `openFiles` field.
    #[must_use]
    pub fn open_files(mut self, open_files: impl IntoOption<NesOpenFilesCapabilities>) -> Self {
        self.open_files = open_files.into_option();
        self
    }

    /// Sets or clears the optional `diagnostics` field.
    #[must_use]
    pub fn diagnostics(mut self, diagnostics: impl IntoOption<NesDiagnosticsCapabilities>) -> Self {
        self.diagnostics = diagnostics.into_option();
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

/// Capabilities for recent files context.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesRecentFilesCapabilities {
    /// Maximum number of recent files the agent can use.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub max_count: Option<u32>,
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

impl NesRecentFilesCapabilities {
    /// Builds an empty [`NesRecentFilesCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Capabilities for related snippets context.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesRelatedSnippetsCapabilities {
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

impl NesRelatedSnippetsCapabilities {
    /// Builds an empty [`NesRelatedSnippetsCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Capabilities for edit history context.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesEditHistoryCapabilities {
    /// Maximum number of edit history entries the agent can use.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub max_count: Option<u32>,
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

impl NesEditHistoryCapabilities {
    /// Builds an empty [`NesEditHistoryCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Capabilities for user actions context.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesUserActionsCapabilities {
    /// Maximum number of user actions the agent can use.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub max_count: Option<u32>,
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

impl NesUserActionsCapabilities {
    /// Builds an empty [`NesUserActionsCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Capabilities for open files context.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesOpenFilesCapabilities {
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

impl NesOpenFilesCapabilities {
    /// Builds an empty [`NesOpenFilesCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Capabilities for diagnostics context.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesDiagnosticsCapabilities {
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

impl NesDiagnosticsCapabilities {
    /// Builds an empty [`NesDiagnosticsCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

// Client NES capabilities

/// NES capabilities advertised by the client during initialization.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ClientNesCapabilities {
    /// Whether the client supports the `jump` suggestion kind.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub jump: Option<NesJumpCapabilities>,
    /// Whether the client supports the `rename` suggestion kind.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub rename: Option<NesRenameCapabilities>,
    /// Whether the client supports the `searchAndReplace` suggestion kind.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub search_and_replace: Option<NesSearchAndReplaceCapabilities>,
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

impl ClientNesCapabilities {
    /// Builds an empty [`ClientNesCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets or clears the optional `jump` field.
    #[must_use]
    pub fn jump(mut self, jump: impl IntoOption<NesJumpCapabilities>) -> Self {
        self.jump = jump.into_option();
        self
    }

    /// Sets or clears the optional `rename` field.
    #[must_use]
    pub fn rename(mut self, rename: impl IntoOption<NesRenameCapabilities>) -> Self {
        self.rename = rename.into_option();
        self
    }

    /// Sets or clears the optional `searchAndReplace` field.
    #[must_use]
    pub fn search_and_replace(
        mut self,
        search_and_replace: impl IntoOption<NesSearchAndReplaceCapabilities>,
    ) -> Self {
        self.search_and_replace = search_and_replace.into_option();
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

/// Marker for jump suggestion support.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesJumpCapabilities {
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

impl NesJumpCapabilities {
    /// Builds an empty [`NesJumpCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Marker for rename suggestion support.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesRenameCapabilities {
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

impl NesRenameCapabilities {
    /// Builds an empty [`NesRenameCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Marker for search and replace suggestion support.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesSearchAndReplaceCapabilities {
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

impl NesSearchAndReplaceCapabilities {
    /// Builds an empty [`NesSearchAndReplaceCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

// Document event notifications (client -> agent)

/// Notification sent when a file is opened in the editor.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = DOCUMENT_DID_OPEN_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DidOpenDocumentNotification {
    /// The session ID for this notification.
    pub session_id: SessionId,
    /// The URI of the opened document.
    #[schemars(url)]
    pub uri: String,
    /// The language identifier of the document (e.g., "rust", "python").
    pub language_id: String,
    /// The version number of the document.
    pub version: i64,
    /// The full text content of the document.
    pub text: String,
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

impl DidOpenDocumentNotification {
    /// Builds [`DidOpenDocumentNotification`] with the required notification fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        session_id: impl Into<SessionId>,
        uri: impl Into<String>,
        language_id: impl Into<String>,
        version: i64,
        text: impl Into<String>,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            uri: uri.into(),
            language_id: language_id.into(),
            version,
            text: text.into(),
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

/// Notification sent when a file is edited.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = DOCUMENT_DID_CHANGE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DidChangeDocumentNotification {
    /// The session ID for this notification.
    pub session_id: SessionId,
    /// The URI of the changed document.
    #[schemars(url)]
    pub uri: String,
    /// The new version number of the document.
    pub version: i64,
    /// The content changes.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    pub content_changes: Vec<TextDocumentContentChangeEvent>,
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

impl DidChangeDocumentNotification {
    /// Builds [`DidChangeDocumentNotification`] with the required notification fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        session_id: impl Into<SessionId>,
        uri: impl Into<String>,
        version: i64,
        content_changes: Vec<TextDocumentContentChangeEvent>,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            uri: uri.into(),
            version,
            content_changes,
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

/// A content change event for a document.
///
/// When `range` is `None`, `text` is the full content of the document.
/// When `range` is `Some`, `text` replaces the given range.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TextDocumentContentChangeEvent {
    /// The range of the document that changed. If `None`, the entire content is replaced.
    #[serde(default)]
    pub range: Option<Range>,
    /// The new text for the range, or the full document content if `range` is `None`.
    pub text: String,
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

impl TextDocumentContentChangeEvent {
    /// Builds a full-document change event that replaces the entire document text.
    #[must_use]
    pub fn full(text: impl Into<String>) -> Self {
        Self {
            range: None,
            text: text.into(),
            meta: None,
        }
    }

    /// Builds an incremental document change event for a specific text range.
    #[must_use]
    pub fn incremental(range: Range, text: impl Into<String>) -> Self {
        Self {
            range: Some(range),
            text: text.into(),
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

/// Notification sent when a file is closed.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = DOCUMENT_DID_CLOSE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DidCloseDocumentNotification {
    /// The session ID for this notification.
    pub session_id: SessionId,
    /// The URI of the closed document.
    #[schemars(url)]
    pub uri: String,
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

impl DidCloseDocumentNotification {
    /// Builds [`DidCloseDocumentNotification`] with the required notification fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>, uri: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            uri: uri.into(),
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

/// Notification sent when a file is saved.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = DOCUMENT_DID_SAVE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DidSaveDocumentNotification {
    /// The session ID for this notification.
    pub session_id: SessionId,
    /// The URI of the saved document.
    #[schemars(url)]
    pub uri: String,
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

impl DidSaveDocumentNotification {
    /// Builds [`DidSaveDocumentNotification`] with the required notification fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>, uri: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            uri: uri.into(),
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

/// Notification sent when a file becomes the active editor tab.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = DOCUMENT_DID_FOCUS_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DidFocusDocumentNotification {
    /// The session ID for this notification.
    pub session_id: SessionId,
    /// The URI of the focused document.
    #[schemars(url)]
    pub uri: String,
    /// The version number of the document.
    pub version: i64,
    /// The current cursor position.
    pub position: Position,
    /// The portion of the file currently visible in the editor viewport.
    pub visible_range: Range,
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

impl DidFocusDocumentNotification {
    /// Builds [`DidFocusDocumentNotification`] with the required notification fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        session_id: impl Into<SessionId>,
        uri: impl Into<String>,
        version: i64,
        position: Position,
        visible_range: Range,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            uri: uri.into(),
            version,
            position,
            visible_range,
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

// NES session start

/// Request to start an NES session.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = NES_START_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StartNesRequest {
    /// The root URI of the workspace.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[schemars(url)]
    #[serde(default)]
    pub workspace_uri: Option<String>,
    /// The workspace folders.
    #[serde(default)]
    pub workspace_folders: Option<Vec<WorkspaceFolder>>,
    /// Repository metadata, if the workspace is a git repository.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub repository: Option<NesRepository>,
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

impl StartNesRequest {
    /// Builds [`StartNesRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new() -> Self {
        Self {
            workspace_uri: None,
            workspace_folders: None,
            repository: None,
            meta: None,
        }
    }

    /// Sets or clears the optional `workspaceUri` field.
    #[must_use]
    pub fn workspace_uri(mut self, workspace_uri: impl IntoOption<String>) -> Self {
        self.workspace_uri = workspace_uri.into_option();
        self
    }

    /// Sets or clears the optional `workspaceFolders` field.
    #[must_use]
    pub fn workspace_folders(
        mut self,
        workspace_folders: impl IntoOption<Vec<WorkspaceFolder>>,
    ) -> Self {
        self.workspace_folders = workspace_folders.into_option();
        self
    }

    /// Sets or clears the optional `repository` field.
    #[must_use]
    pub fn repository(mut self, repository: impl IntoOption<NesRepository>) -> Self {
        self.repository = repository.into_option();
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

impl Default for StartNesRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// A workspace folder.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct WorkspaceFolder {
    /// The URI of the folder.
    #[schemars(url)]
    pub uri: String,
    /// The display name of the folder.
    pub name: String,
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

impl WorkspaceFolder {
    /// Builds [`WorkspaceFolder`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(uri: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            name: name.into(),
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

/// Repository metadata for an NES session.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesRepository {
    /// The repository name.
    pub name: String,
    /// The repository owner.
    pub owner: String,
    /// The remote URL of the repository.
    pub remote_url: String,
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

impl NesRepository {
    /// Builds [`NesRepository`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        owner: impl Into<String>,
        remote_url: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            owner: owner.into(),
            remote_url: remote_url.into(),
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

/// Response to `nes/start`.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = NES_START_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StartNesResponse {
    /// The session ID for the newly started NES session.
    pub session_id: SessionId,
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

impl StartNesResponse {
    /// Builds [`StartNesResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>) -> Self {
        Self {
            session_id: session_id.into(),
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

// NES session close

/// Request to close an NES session.
///
/// The agent **must** cancel any ongoing work related to the NES session
/// and then free up any resources associated with the session.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = NES_CLOSE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CloseNesRequest {
    /// The ID of the NES session to close.
    pub session_id: SessionId,
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

impl CloseNesRequest {
    /// Builds [`CloseNesRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>) -> Self {
        Self {
            session_id: session_id.into(),
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

/// Response from closing an NES session.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = NES_CLOSE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CloseNesResponse {
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

impl CloseNesResponse {
    /// Builds [`CloseNesResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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

// NES suggest request

/// What triggered the suggestion request.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub enum NesTriggerKind {
    /// Triggered by user typing or cursor movement.
    #[serde(rename = "automatic")]
    Automatic,
    /// Triggered by a diagnostic appearing at or near the cursor.
    #[serde(rename = "diagnostic")]
    Diagnostic,
    /// Triggered by an explicit user action (keyboard shortcut).
    #[serde(rename = "manual")]
    Manual,
    /// Custom or future suggestion trigger kind.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(untagged)]
    Other(String),
}

/// Request for a code suggestion.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = NES_SUGGEST_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SuggestNesRequest {
    /// The session ID for this request.
    pub session_id: SessionId,
    /// The URI of the document to suggest for.
    #[schemars(url)]
    pub uri: String,
    /// The version number of the document.
    pub version: i64,
    /// The current cursor position.
    pub position: Position,
    /// The current text selection range, if any.
    #[serde(default)]
    pub selection: Option<Range>,
    /// What triggered this suggestion request.
    pub trigger_kind: NesTriggerKind,
    /// Context for the suggestion, included based on agent capabilities.
    #[serde(default)]
    pub context: Option<NesSuggestContext>,
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

impl SuggestNesRequest {
    /// Builds [`SuggestNesRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        session_id: impl Into<SessionId>,
        uri: impl Into<String>,
        version: i64,
        position: Position,
        trigger_kind: NesTriggerKind,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            uri: uri.into(),
            version,
            position,
            selection: None,
            trigger_kind,
            context: None,
            meta: None,
        }
    }

    /// Sets or clears the optional `selection` field.
    #[must_use]
    pub fn selection(mut self, selection: impl IntoOption<Range>) -> Self {
        self.selection = selection.into_option();
        self
    }

    /// Sets or clears the optional `context` field.
    #[must_use]
    pub fn context(mut self, context: impl IntoOption<NesSuggestContext>) -> Self {
        self.context = context.into_option();
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

/// Context attached to a suggestion request.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesSuggestContext {
    /// Recently accessed files.
    #[serde(default)]
    pub recent_files: Option<Vec<NesRecentFile>>,
    /// Related code snippets.
    #[serde(default)]
    pub related_snippets: Option<Vec<NesRelatedSnippet>>,
    /// Recent edit history.
    #[serde(default)]
    pub edit_history: Option<Vec<NesEditHistoryEntry>>,
    /// Recent user actions (typing, navigation, etc.).
    #[serde(default)]
    pub user_actions: Option<Vec<NesUserAction>>,
    /// Currently open files in the editor.
    #[serde(default)]
    pub open_files: Option<Vec<NesOpenFile>>,
    /// Current diagnostics (errors, warnings).
    #[serde(default)]
    pub diagnostics: Option<Vec<NesDiagnostic>>,
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

impl NesSuggestContext {
    /// Builds [`NesSuggestContext`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets or clears the optional `recentFiles` field.
    #[must_use]
    pub fn recent_files(mut self, recent_files: impl IntoOption<Vec<NesRecentFile>>) -> Self {
        self.recent_files = recent_files.into_option();
        self
    }

    /// Sets or clears the optional `relatedSnippets` field.
    #[must_use]
    pub fn related_snippets(
        mut self,
        related_snippets: impl IntoOption<Vec<NesRelatedSnippet>>,
    ) -> Self {
        self.related_snippets = related_snippets.into_option();
        self
    }

    /// Sets or clears the optional `editHistory` field.
    #[must_use]
    pub fn edit_history(mut self, edit_history: impl IntoOption<Vec<NesEditHistoryEntry>>) -> Self {
        self.edit_history = edit_history.into_option();
        self
    }

    /// Sets or clears the optional `userActions` field.
    #[must_use]
    pub fn user_actions(mut self, user_actions: impl IntoOption<Vec<NesUserAction>>) -> Self {
        self.user_actions = user_actions.into_option();
        self
    }

    /// Sets or clears the optional `openFiles` field.
    #[must_use]
    pub fn open_files(mut self, open_files: impl IntoOption<Vec<NesOpenFile>>) -> Self {
        self.open_files = open_files.into_option();
        self
    }

    /// Sets or clears the optional `diagnostics` field.
    #[must_use]
    pub fn diagnostics(mut self, diagnostics: impl IntoOption<Vec<NesDiagnostic>>) -> Self {
        self.diagnostics = diagnostics.into_option();
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

/// A recently accessed file.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesRecentFile {
    /// The URI of the file.
    #[schemars(url)]
    pub uri: String,
    /// The language identifier.
    pub language_id: String,
    /// The full text content of the file.
    pub text: String,
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

impl NesRecentFile {
    /// Builds [`NesRecentFile`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        uri: impl Into<String>,
        language_id: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            uri: uri.into(),
            language_id: language_id.into(),
            text: text.into(),
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

/// A related code snippet from a file.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesRelatedSnippet {
    /// The URI of the file containing the snippets.
    #[schemars(url)]
    pub uri: String,
    /// The code excerpts.
    pub excerpts: Vec<NesExcerpt>,
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

impl NesRelatedSnippet {
    /// Builds [`NesRelatedSnippet`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(uri: impl Into<String>, excerpts: Vec<NesExcerpt>) -> Self {
        Self {
            uri: uri.into(),
            excerpts,
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

/// A code excerpt from a file.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesExcerpt {
    /// The start line of the excerpt (zero-based).
    pub start_line: u32,
    /// The end line of the excerpt (zero-based).
    pub end_line: u32,
    /// The text content of the excerpt.
    pub text: String,
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

impl NesExcerpt {
    /// Builds [`NesExcerpt`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(start_line: u32, end_line: u32, text: impl Into<String>) -> Self {
        Self {
            start_line,
            end_line,
            text: text.into(),
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

/// An entry in the edit history.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesEditHistoryEntry {
    /// The URI of the edited file.
    #[schemars(url)]
    pub uri: String,
    /// A diff representing the edit.
    pub diff: String,
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

impl NesEditHistoryEntry {
    /// Builds [`NesEditHistoryEntry`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(uri: impl Into<String>, diff: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            diff: diff.into(),
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

/// A user action (typing, cursor movement, etc.).
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesUserAction {
    /// The kind of action (e.g., "insertChar", "cursorMovement").
    pub action: String,
    /// The URI of the file where the action occurred.
    #[schemars(url)]
    pub uri: String,
    /// The position where the action occurred.
    pub position: Position,
    /// Timestamp in milliseconds since epoch.
    pub timestamp_ms: u64,
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

impl NesUserAction {
    /// Builds [`NesUserAction`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        action: impl Into<String>,
        uri: impl Into<String>,
        position: Position,
        timestamp_ms: u64,
    ) -> Self {
        Self {
            action: action.into(),
            uri: uri.into(),
            position,
            timestamp_ms,
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

/// An open file in the editor.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesOpenFile {
    /// The URI of the file.
    #[schemars(url)]
    pub uri: String,
    /// The language identifier.
    pub language_id: String,
    /// The visible range in the editor, if any.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub visible_range: Option<Range>,
    /// Timestamp in milliseconds since epoch of when the file was last focused.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub last_focused_ms: Option<u64>,
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

impl NesOpenFile {
    /// Builds [`NesOpenFile`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(uri: impl Into<String>, language_id: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            language_id: language_id.into(),
            visible_range: None,
            last_focused_ms: None,
            meta: None,
        }
    }

    /// Sets or clears the optional `visibleRange` field.
    #[must_use]
    pub fn visible_range(mut self, visible_range: impl IntoOption<Range>) -> Self {
        self.visible_range = visible_range.into_option();
        self
    }

    /// Sets or clears the optional `lastFocusedMs` field.
    #[must_use]
    pub fn last_focused_ms(mut self, last_focused_ms: impl IntoOption<u64>) -> Self {
        self.last_focused_ms = last_focused_ms.into_option();
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

/// A diagnostic (error, warning, etc.).
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesDiagnostic {
    /// The URI of the file containing the diagnostic.
    #[schemars(url)]
    pub uri: String,
    /// The range of the diagnostic.
    pub range: Range,
    /// The severity of the diagnostic.
    pub severity: NesDiagnosticSeverity,
    /// The diagnostic message.
    pub message: String,
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

impl NesDiagnostic {
    /// Builds [`NesDiagnostic`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        uri: impl Into<String>,
        range: Range,
        severity: NesDiagnosticSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            uri: uri.into(),
            range,
            severity,
            message: message.into(),
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

/// Severity of a diagnostic.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub enum NesDiagnosticSeverity {
    /// An error.
    #[serde(rename = "error")]
    Error,
    /// A warning.
    #[serde(rename = "warning")]
    Warning,
    /// An informational message.
    #[serde(rename = "information")]
    Information,
    /// A hint.
    #[serde(rename = "hint")]
    Hint,
    /// Custom or future diagnostic severity.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(untagged)]
    Other(String),
}

// NES suggest response

/// Response to `nes/suggest`.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = NES_SUGGEST_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SuggestNesResponse {
    /// The list of suggestions.
    pub suggestions: Vec<NesSuggestion>,
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

impl SuggestNesResponse {
    /// Builds [`SuggestNesResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(suggestions: Vec<NesSuggestion>) -> Self {
        Self {
            suggestions,
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

/// A suggestion returned by the agent.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "camelCase")]
#[non_exhaustive]
pub enum NesSuggestion {
    /// A text edit suggestion.
    Edit(NesEditSuggestion),
    /// A jump-to-location suggestion.
    Jump(NesJumpSuggestion),
    /// A rename symbol suggestion.
    Rename(NesRenameSuggestion),
    /// A search-and-replace suggestion.
    SearchAndReplace(NesSearchAndReplaceSuggestion),
    /// Custom or future NES suggestion.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    ///
    /// Receivers that do not understand this suggestion kind should preserve
    /// the raw payload when storing, replaying, proxying, or forwarding
    /// suggestions, and otherwise ignore it or display it generically.
    #[serde(untagged)]
    Other(OtherNesSuggestion),
}

/// Custom or future NES suggestion payload.
#[derive(Debug, Clone, Serialize, JsonSchema, PartialEq, Eq)]
#[schemars(inline)]
#[schemars(transform = other_nes_suggestion_schema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OtherNesSuggestion {
    /// Custom or future NES suggestion kind.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    pub kind: String,
    /// Unique identifier for accept/reject tracking.
    pub suggestion_id: NesSuggestionId,
    /// Additional fields from the unknown NES suggestion payload.
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl OtherNesSuggestion {
    /// Builds [`OtherNesSuggestion`] from an unknown discriminator and preserves the remaining extension fields.
    #[must_use]
    pub fn new(
        kind: impl Into<String>,
        suggestion_id: impl Into<NesSuggestionId>,
        mut fields: BTreeMap<String, serde_json::Value>,
    ) -> Self {
        fields.remove("kind");
        fields.remove("suggestionId");
        Self {
            kind: kind.into(),
            suggestion_id: suggestion_id.into(),
            fields,
        }
    }
}

impl<'de> Deserialize<'de> for OtherNesSuggestion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut fields = BTreeMap::<String, serde_json::Value>::deserialize(deserializer)?;
        let kind = fields
            .remove("kind")
            .ok_or_else(|| serde::de::Error::missing_field("kind"))?;
        let serde_json::Value::String(kind) = kind else {
            return Err(serde::de::Error::custom("`kind` must be a string"));
        };
        let suggestion_id = fields
            .remove("suggestionId")
            .ok_or_else(|| serde::de::Error::missing_field("suggestionId"))?;
        let serde_json::Value::String(suggestion_id) = suggestion_id else {
            return Err(serde::de::Error::custom("`suggestionId` must be a string"));
        };

        if is_known_nes_suggestion_kind(&kind) {
            return Err(serde::de::Error::custom(format!(
                "known NES suggestion `{kind}` did not match its schema"
            )));
        }

        Ok(Self {
            kind,
            suggestion_id: NesSuggestionId::new(suggestion_id),
            fields,
        })
    }
}

fn is_known_nes_suggestion_kind(kind: &str) -> bool {
    matches!(kind, "edit" | "jump" | "rename" | "searchAndReplace")
}

fn other_nes_suggestion_schema(schema: &mut Schema) {
    super::schema_util::reject_known_string_discriminators(
        schema,
        "kind",
        &["edit", "jump", "rename", "searchAndReplace"],
    );
}

/// A text edit suggestion.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesEditSuggestion {
    /// Unique identifier for accept/reject tracking.
    pub suggestion_id: NesSuggestionId,
    /// The URI of the file to edit.
    #[schemars(url)]
    pub uri: String,
    /// The text edits to apply. Must contain at least one edit.
    #[schemars(length(min = 1))]
    pub edits: Vec<NesTextEdit>,
    /// Optional suggested cursor position after applying edits.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub cursor_position: Option<Position>,
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

impl NesEditSuggestion {
    /// Builds [`NesEditSuggestion`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        suggestion_id: impl Into<NesSuggestionId>,
        uri: impl Into<String>,
        edits: Vec<NesTextEdit>,
    ) -> Self {
        Self {
            suggestion_id: suggestion_id.into(),
            uri: uri.into(),
            edits,
            cursor_position: None,
            meta: None,
        }
    }

    /// Sets or clears the optional `cursorPosition` field.
    #[must_use]
    pub fn cursor_position(mut self, cursor_position: impl IntoOption<Position>) -> Self {
        self.cursor_position = cursor_position.into_option();
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

/// A text edit within a suggestion.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesTextEdit {
    /// The range to replace.
    pub range: Range,
    /// The replacement text.
    pub new_text: String,
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

impl NesTextEdit {
    /// Builds [`NesTextEdit`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(range: Range, new_text: impl Into<String>) -> Self {
        Self {
            range,
            new_text: new_text.into(),
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

/// A jump-to-location suggestion.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesJumpSuggestion {
    /// Unique identifier for accept/reject tracking.
    pub suggestion_id: NesSuggestionId,
    /// The file to navigate to.
    #[schemars(url)]
    pub uri: String,
    /// The target position within the file.
    pub position: Position,
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

impl NesJumpSuggestion {
    /// Builds [`NesJumpSuggestion`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        suggestion_id: impl Into<NesSuggestionId>,
        uri: impl Into<String>,
        position: Position,
    ) -> Self {
        Self {
            suggestion_id: suggestion_id.into(),
            uri: uri.into(),
            position,
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

/// A rename symbol suggestion.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesRenameSuggestion {
    /// Unique identifier for accept/reject tracking.
    pub suggestion_id: NesSuggestionId,
    /// The file URI containing the symbol.
    #[schemars(url)]
    pub uri: String,
    /// The position of the symbol to rename.
    pub position: Position,
    /// The new name for the symbol.
    pub new_name: String,
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

impl NesRenameSuggestion {
    /// Builds [`NesRenameSuggestion`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        suggestion_id: impl Into<NesSuggestionId>,
        uri: impl Into<String>,
        position: Position,
        new_name: impl Into<String>,
    ) -> Self {
        Self {
            suggestion_id: suggestion_id.into(),
            uri: uri.into(),
            position,
            new_name: new_name.into(),
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

/// A search-and-replace suggestion.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NesSearchAndReplaceSuggestion {
    /// Unique identifier for accept/reject tracking.
    pub suggestion_id: NesSuggestionId,
    /// The file URI to search within.
    #[schemars(url)]
    pub uri: String,
    /// The text or pattern to find.
    pub search: String,
    /// The replacement text.
    pub replace: String,
    /// Whether `search` is a regular expression. Defaults to `false`.
    #[serde(default)]
    pub is_regex: Option<bool>,
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

impl NesSearchAndReplaceSuggestion {
    /// Builds [`NesSearchAndReplaceSuggestion`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        suggestion_id: impl Into<NesSuggestionId>,
        uri: impl Into<String>,
        search: impl Into<String>,
        replace: impl Into<String>,
    ) -> Self {
        Self {
            suggestion_id: suggestion_id.into(),
            uri: uri.into(),
            search: search.into(),
            replace: replace.into(),
            is_regex: None,
            meta: None,
        }
    }

    /// Sets or clears the optional `isRegex` field.
    #[must_use]
    pub fn is_regex(mut self, is_regex: impl IntoOption<bool>) -> Self {
        self.is_regex = is_regex.into_option();
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

// NES accept/reject notifications

/// Notification sent when a suggestion is accepted.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = NES_ACCEPT_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AcceptNesNotification {
    /// The session ID for this notification.
    pub session_id: SessionId,
    /// The ID of the accepted suggestion.
    pub suggestion_id: NesSuggestionId,
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

impl AcceptNesNotification {
    /// Builds [`AcceptNesNotification`] with the required notification fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        session_id: impl Into<SessionId>,
        suggestion_id: impl Into<NesSuggestionId>,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            suggestion_id: suggestion_id.into(),
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

/// Notification sent when a suggestion is rejected.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "agent", "x-method" = NES_REJECT_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RejectNesNotification {
    /// The session ID for this notification.
    pub session_id: SessionId,
    /// The ID of the rejected suggestion.
    pub suggestion_id: NesSuggestionId,
    /// The reason for rejection.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub reason: Option<NesRejectReason>,
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

impl RejectNesNotification {
    /// Builds [`RejectNesNotification`] with the required notification fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        session_id: impl Into<SessionId>,
        suggestion_id: impl Into<NesSuggestionId>,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            suggestion_id: suggestion_id.into(),
            reason: None,
            meta: None,
        }
    }

    /// Sets or clears the optional `reason` field.
    #[must_use]
    pub fn reason(mut self, reason: impl IntoOption<NesRejectReason>) -> Self {
        self.reason = reason.into_option();
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

/// The reason a suggestion was rejected.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[non_exhaustive]
pub enum NesRejectReason {
    /// The user explicitly dismissed the suggestion.
    #[serde(rename = "rejected")]
    Rejected,
    /// The suggestion was shown but the user continued editing without interacting.
    #[serde(rename = "ignored")]
    Ignored,
    /// The suggestion was superseded by a newer suggestion.
    #[serde(rename = "replaced")]
    Replaced,
    /// The request was cancelled before the agent returned a response.
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Custom or future rejection reason.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(untagged)]
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_position_encoding_kind_serialization() {
        assert_eq!(
            serde_json::to_value(&PositionEncodingKind::Utf16).unwrap(),
            json!("utf-16")
        );
        assert_eq!(
            serde_json::to_value(&PositionEncodingKind::Utf32).unwrap(),
            json!("utf-32")
        );
        assert_eq!(
            serde_json::to_value(&PositionEncodingKind::Utf8).unwrap(),
            json!("utf-8")
        );

        assert_eq!(
            serde_json::from_value::<PositionEncodingKind>(json!("utf-16")).unwrap(),
            PositionEncodingKind::Utf16
        );
        assert_eq!(
            serde_json::from_value::<PositionEncodingKind>(json!("utf-32")).unwrap(),
            PositionEncodingKind::Utf32
        );
        assert_eq!(
            serde_json::from_value::<PositionEncodingKind>(json!("utf-8")).unwrap(),
            PositionEncodingKind::Utf8
        );
        assert!(serde_json::from_value::<PositionEncodingKind>(json!("_future")).is_err());
    }

    #[test]
    fn test_client_capabilities_skip_unknown_position_encodings() {
        let caps: crate::v2::ClientCapabilities = serde_json::from_value(json!({
            "positionEncodings": ["_future", "utf-8", "utf-16"]
        }))
        .unwrap();

        assert_eq!(
            caps.position_encodings,
            vec![PositionEncodingKind::Utf8, PositionEncodingKind::Utf16]
        );
    }

    #[test]
    fn test_agent_nes_capabilities_serialization() {
        let caps = NesCapabilities::new()
            .events(
                NesEventCapabilities::new().document(
                    NesDocumentEventCapabilities::new()
                        .did_open(NesDocumentDidOpenCapabilities::default())
                        .did_change(NesDocumentDidChangeCapabilities::new(
                            TextDocumentSyncKind::Incremental,
                        ))
                        .did_close(NesDocumentDidCloseCapabilities::default())
                        .did_save(NesDocumentDidSaveCapabilities::default())
                        .did_focus(NesDocumentDidFocusCapabilities::default()),
                ),
            )
            .context(
                NesContextCapabilities::new()
                    .recent_files(NesRecentFilesCapabilities {
                        max_count: Some(10),
                        meta: None,
                    })
                    .related_snippets(NesRelatedSnippetsCapabilities::default())
                    .edit_history(NesEditHistoryCapabilities {
                        max_count: Some(6),
                        meta: None,
                    })
                    .user_actions(NesUserActionsCapabilities {
                        max_count: Some(16),
                        meta: None,
                    })
                    .open_files(NesOpenFilesCapabilities::default())
                    .diagnostics(NesDiagnosticsCapabilities::default()),
            );

        let json = serde_json::to_value(&caps).unwrap();
        assert_eq!(
            json,
            json!({
                "events": {
                    "document": {
                        "didOpen": {},
                        "didChange": {
                            "syncKind": "incremental"
                        },
                        "didClose": {},
                        "didSave": {},
                        "didFocus": {}
                    }
                },
                "context": {
                    "recentFiles": {
                        "maxCount": 10
                    },
                    "relatedSnippets": {},
                    "editHistory": {
                        "maxCount": 6
                    },
                    "userActions": {
                        "maxCount": 16
                    },
                    "openFiles": {},
                    "diagnostics": {}
                }
            })
        );

        // Round-trip
        let deserialized: NesCapabilities = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, caps);
    }

    #[test]
    fn test_client_nes_capabilities_serialization() {
        let caps = ClientNesCapabilities::new()
            .jump(NesJumpCapabilities::default())
            .rename(NesRenameCapabilities::default())
            .search_and_replace(NesSearchAndReplaceCapabilities::default());

        let json = serde_json::to_value(&caps).unwrap();
        assert_eq!(
            json,
            json!({
                "jump": {},
                "rename": {},
                "searchAndReplace": {}
            })
        );

        let deserialized: ClientNesCapabilities = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, caps);
    }

    #[test]
    fn test_document_did_open_serialization() {
        let notification = DidOpenDocumentNotification::new(
            "session_123",
            "file:///path/to/file.rs",
            "rust",
            1,
            "fn main() {\n    println!(\"hello\");\n}\n",
        );

        let json = serde_json::to_value(&notification).unwrap();
        assert_eq!(
            json,
            json!({
                "sessionId": "session_123",
                "uri": "file:///path/to/file.rs",
                "languageId": "rust",
                "version": 1,
                "text": "fn main() {\n    println!(\"hello\");\n}\n"
            })
        );

        let deserialized: DidOpenDocumentNotification = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, notification);
    }

    #[test]
    fn test_document_did_change_incremental_serialization() {
        let notification = DidChangeDocumentNotification::new(
            "session_123",
            "file:///path/to/file.rs",
            2,
            vec![TextDocumentContentChangeEvent::incremental(
                Range::new(Position::new(1, 4), Position::new(1, 4)),
                "let x = 42;\n    ",
            )],
        );

        let json = serde_json::to_value(&notification).unwrap();
        assert_eq!(
            json,
            json!({
                "sessionId": "session_123",
                "uri": "file:///path/to/file.rs",
                "version": 2,
                "contentChanges": [
                    {
                        "range": {
                            "start": { "line": 1, "character": 4 },
                            "end": { "line": 1, "character": 4 }
                        },
                        "text": "let x = 42;\n    "
                    }
                ]
            })
        );
    }

    #[test]
    fn test_document_did_change_full_serialization() {
        let notification = DidChangeDocumentNotification::new(
            "session_123",
            "file:///path/to/file.rs",
            2,
            vec![TextDocumentContentChangeEvent::full(
                "fn main() {\n    let x = 42;\n    println!(\"hello\");\n}\n",
            )],
        );

        let json = serde_json::to_value(&notification).unwrap();
        assert_eq!(
            json,
            json!({
                "sessionId": "session_123",
                "uri": "file:///path/to/file.rs",
                "version": 2,
                "contentChanges": [
                    {
                        "text": "fn main() {\n    let x = 42;\n    println!(\"hello\");\n}\n"
                    }
                ]
            })
        );
    }

    #[test]
    fn test_document_did_close_serialization() {
        let notification =
            DidCloseDocumentNotification::new("session_123", "file:///path/to/file.rs");
        let json = serde_json::to_value(&notification).unwrap();
        assert_eq!(
            json,
            json!({ "sessionId": "session_123", "uri": "file:///path/to/file.rs" })
        );
    }

    #[test]
    fn test_document_did_save_serialization() {
        let notification =
            DidSaveDocumentNotification::new("session_123", "file:///path/to/file.rs");
        let json = serde_json::to_value(&notification).unwrap();
        assert_eq!(
            json,
            json!({ "sessionId": "session_123", "uri": "file:///path/to/file.rs" })
        );
    }

    #[test]
    fn test_document_did_focus_serialization() {
        let notification = DidFocusDocumentNotification::new(
            "session_123",
            "file:///path/to/file.rs",
            2,
            Position::new(5, 12),
            Range::new(Position::new(0, 0), Position::new(45, 0)),
        );

        let json = serde_json::to_value(&notification).unwrap();
        assert_eq!(
            json,
            json!({
                "sessionId": "session_123",
                "uri": "file:///path/to/file.rs",
                "version": 2,
                "position": { "line": 5, "character": 12 },
                "visibleRange": {
                    "start": { "line": 0, "character": 0 },
                    "end": { "line": 45, "character": 0 }
                }
            })
        );
    }

    #[test]
    fn test_nes_suggestion_edit_serialization() {
        let suggestion = NesSuggestion::Edit(
            NesEditSuggestion::new(
                "sugg_001",
                "file:///path/to/other_file.rs",
                vec![NesTextEdit::new(
                    Range::new(Position::new(5, 0), Position::new(5, 10)),
                    "let result = helper();",
                )],
            )
            .cursor_position(Position::new(5, 22)),
        );

        let json = serde_json::to_value(&suggestion).unwrap();
        assert_eq!(
            json,
            json!({
                "kind": "edit",
                "suggestionId": "sugg_001",
                "uri": "file:///path/to/other_file.rs",
                "edits": [
                    {
                        "range": {
                            "start": { "line": 5, "character": 0 },
                            "end": { "line": 5, "character": 10 }
                        },
                        "newText": "let result = helper();"
                    }
                ],
                "cursorPosition": { "line": 5, "character": 22 }
            })
        );

        let deserialized: NesSuggestion = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, suggestion);
    }

    #[test]
    fn test_nes_suggestion_unknown_variant() {
        let suggestion: NesSuggestion = serde_json::from_value(json!({
            "kind": "_preview",
            "suggestionId": "sugg_001",
            "label": "Preview generated file"
        }))
        .unwrap();

        let NesSuggestion::Other(unknown) = suggestion else {
            panic!("expected unknown NES suggestion");
        };

        assert_eq!(unknown.kind, "_preview");
        assert_eq!(unknown.suggestion_id.to_string(), "sugg_001");
        assert!(!unknown.fields.contains_key("suggestionId"));
        assert_eq!(
            serde_json::to_value(NesSuggestion::Other(unknown)).unwrap(),
            json!({
                "kind": "_preview",
                "suggestionId": "sugg_001",
                "label": "Preview generated file"
            })
        );
    }

    #[test]
    fn test_nes_suggestion_unknown_does_not_hide_malformed_known_variant() {
        assert!(
            serde_json::from_value::<NesSuggestion>(json!({
                "kind": "edit"
            }))
            .is_err()
        );
    }

    #[test]
    fn test_nes_suggestion_jump_serialization() {
        let suggestion = NesSuggestion::Jump(NesJumpSuggestion::new(
            "sugg_002",
            "file:///path/to/other_file.rs",
            Position::new(15, 4),
        ));

        let json = serde_json::to_value(&suggestion).unwrap();
        assert_eq!(
            json,
            json!({
                "kind": "jump",
                "suggestionId": "sugg_002",
                "uri": "file:///path/to/other_file.rs",
                "position": { "line": 15, "character": 4 }
            })
        );

        let deserialized: NesSuggestion = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, suggestion);
    }

    #[test]
    fn test_nes_suggestion_rename_serialization() {
        let suggestion = NesSuggestion::Rename(NesRenameSuggestion::new(
            "sugg_003",
            "file:///path/to/file.rs",
            Position::new(5, 10),
            "calculateTotal",
        ));

        let json = serde_json::to_value(&suggestion).unwrap();
        assert_eq!(
            json,
            json!({
                "kind": "rename",
                "suggestionId": "sugg_003",
                "uri": "file:///path/to/file.rs",
                "position": { "line": 5, "character": 10 },
                "newName": "calculateTotal"
            })
        );

        let deserialized: NesSuggestion = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, suggestion);
    }

    #[test]
    fn test_nes_suggestion_search_and_replace_serialization() {
        let suggestion = NesSuggestion::SearchAndReplace(
            NesSearchAndReplaceSuggestion::new(
                "sugg_004",
                "file:///path/to/file.rs",
                "oldFunction",
                "newFunction",
            )
            .is_regex(false),
        );

        let json = serde_json::to_value(&suggestion).unwrap();
        assert_eq!(
            json,
            json!({
                "kind": "searchAndReplace",
                "suggestionId": "sugg_004",
                "uri": "file:///path/to/file.rs",
                "search": "oldFunction",
                "replace": "newFunction",
                "isRegex": false
            })
        );

        let deserialized: NesSuggestion = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, suggestion);
    }

    #[test]
    fn test_nes_start_request_serialization() {
        let request = StartNesRequest::new()
            .workspace_uri("file:///Users/alice/projects/my-app")
            .workspace_folders(vec![WorkspaceFolder::new(
                "file:///Users/alice/projects/my-app",
                "my-app",
            )])
            .repository(NesRepository::new(
                "my-app",
                "alice",
                "https://github.com/alice/my-app.git",
            ));

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(
            json,
            json!({
                "workspaceUri": "file:///Users/alice/projects/my-app",
                "workspaceFolders": [
                    {
                        "uri": "file:///Users/alice/projects/my-app",
                        "name": "my-app"
                    }
                ],
                "repository": {
                    "name": "my-app",
                    "owner": "alice",
                    "remoteUrl": "https://github.com/alice/my-app.git"
                }
            })
        );
    }

    #[test]
    fn test_nes_start_response_serialization() {
        let response = StartNesResponse::new("session_abc123");
        let json = serde_json::to_value(&response).unwrap();
        assert_eq!(json, json!({ "sessionId": "session_abc123" }));
    }

    #[test]
    fn test_nes_trigger_kind_serialization() {
        assert_eq!(
            serde_json::to_value(&NesTriggerKind::Automatic).unwrap(),
            json!("automatic")
        );
        assert_eq!(
            serde_json::to_value(&NesTriggerKind::Diagnostic).unwrap(),
            json!("diagnostic")
        );
        assert_eq!(
            serde_json::to_value(&NesTriggerKind::Manual).unwrap(),
            json!("manual")
        );
    }

    #[test]
    fn test_nes_reject_reason_serialization() {
        assert_eq!(
            serde_json::to_value(&NesRejectReason::Rejected).unwrap(),
            json!("rejected")
        );
        assert_eq!(
            serde_json::to_value(&NesRejectReason::Ignored).unwrap(),
            json!("ignored")
        );
        assert_eq!(
            serde_json::to_value(&NesRejectReason::Replaced).unwrap(),
            json!("replaced")
        );
        assert_eq!(
            serde_json::to_value(&NesRejectReason::Cancelled).unwrap(),
            json!("cancelled")
        );
    }

    #[test]
    fn test_nes_accept_notification_serialization() {
        let notification = AcceptNesNotification::new("session_123", "sugg_001");
        let json = serde_json::to_value(&notification).unwrap();
        assert_eq!(
            json,
            json!({ "sessionId": "session_123", "suggestionId": "sugg_001" })
        );
    }

    #[test]
    fn test_nes_reject_notification_serialization() {
        let notification =
            RejectNesNotification::new("session_123", "sugg_001").reason(NesRejectReason::Rejected);
        let json = serde_json::to_value(&notification).unwrap();
        assert_eq!(
            json,
            json!({ "sessionId": "session_123", "suggestionId": "sugg_001", "reason": "rejected" })
        );
    }

    #[test]
    fn test_nes_suggest_request_with_context_serialization() {
        let request = SuggestNesRequest::new(
            "session_123",
            "file:///path/to/file.rs",
            2,
            Position::new(5, 12),
            NesTriggerKind::Automatic,
        )
        .selection(Range::new(Position::new(5, 4), Position::new(5, 12)))
        .context(
            NesSuggestContext::new()
                .recent_files(vec![NesRecentFile::new(
                    "file:///path/to/utils.rs",
                    "rust",
                    "pub fn helper() -> i32 { 42 }\n",
                )])
                .diagnostics(vec![NesDiagnostic::new(
                    "file:///path/to/file.rs",
                    Range::new(Position::new(5, 0), Position::new(5, 10)),
                    NesDiagnosticSeverity::Error,
                    "cannot find value `foo` in this scope",
                )]),
        );

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["sessionId"], "session_123");
        assert_eq!(json["uri"], "file:///path/to/file.rs");
        assert_eq!(json["version"], 2);
        assert_eq!(json["triggerKind"], "automatic");
        assert_eq!(
            json["context"]["recentFiles"][0]["uri"],
            "file:///path/to/utils.rs"
        );
        assert_eq!(json["context"]["diagnostics"][0]["severity"], "error");
    }

    #[test]
    fn test_text_document_sync_kind_serialization() {
        assert_eq!(
            serde_json::to_value(&TextDocumentSyncKind::Full).unwrap(),
            json!("full")
        );
        assert_eq!(
            serde_json::to_value(&TextDocumentSyncKind::Incremental).unwrap(),
            json!("incremental")
        );
        assert!(serde_json::from_value::<TextDocumentSyncKind>(json!("_future")).is_err());
    }

    #[test]
    fn test_document_event_capabilities_drop_unknown_did_change_sync_kind() {
        let caps: NesDocumentEventCapabilities = serde_json::from_value(json!({
            "didChange": {
                "syncKind": "_future"
            }
        }))
        .unwrap();

        assert_eq!(caps.did_change, None);
    }

    #[test]
    fn test_document_did_change_capabilities_requires_sync_kind() {
        assert!(serde_json::from_value::<NesDocumentDidChangeCapabilities>(json!({})).is_err());
    }

    #[test]
    fn test_nes_suggest_response_serialization() {
        let response = SuggestNesResponse::new(vec![
            NesSuggestion::Edit(NesEditSuggestion::new(
                "sugg_001",
                "file:///path/to/file.rs",
                vec![NesTextEdit::new(
                    Range::new(Position::new(5, 0), Position::new(5, 10)),
                    "let result = helper();",
                )],
            )),
            NesSuggestion::Jump(NesJumpSuggestion::new(
                "sugg_002",
                "file:///path/to/other.rs",
                Position::new(10, 0),
            )),
        ]);

        let json = serde_json::to_value(&response).unwrap();
        assert_eq!(json["suggestions"].as_array().unwrap().len(), 2);
        assert_eq!(json["suggestions"][0]["kind"], "edit");
        assert_eq!(json["suggestions"][1]["kind"], "jump");
    }
}
