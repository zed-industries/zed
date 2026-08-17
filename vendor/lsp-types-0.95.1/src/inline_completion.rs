use crate::{
    Command, InsertTextFormat, Range, StaticRegistrationOptions, TextDocumentPositionParams,
    TextDocumentRegistrationOptions, WorkDoneProgressOptions, WorkDoneProgressParams,
};
use serde::{Deserialize, Serialize};

/// Client capabilities specific to inline completions.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineCompletionClientCapabilities {
    /// Whether implementation supports dynamic registration for inline completion providers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,
}

/// Inline completion options used during static registration.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
pub struct InlineCompletionOptions {
    #[serde(flatten)]
    pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Inline completion options used during static or dynamic registration.
///
// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
pub struct InlineCompletionRegistrationOptions {
    #[serde(flatten)]
    pub inline_completion_options: InlineCompletionOptions,

    #[serde(flatten)]
    pub text_document_registration_options: TextDocumentRegistrationOptions,

    #[serde(flatten)]
    pub static_registration_options: StaticRegistrationOptions,
}

/// A parameter literal used in inline completion requests.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineCompletionParams {
    #[serde(flatten)]
    pub work_done_progress_params: WorkDoneProgressParams,

    #[serde(flatten)]
    pub text_document_position: TextDocumentPositionParams,

    /// Additional information about the context in which inline completions were requested.
    pub context: InlineCompletionContext,
}

/// Describes how an [`InlineCompletionItemProvider`] was triggered.
///
/// @since 3.18.0
#[derive(Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub struct InlineCompletionTriggerKind(i32);
lsp_enum! {
impl InlineCompletionTriggerKind {
    /// Completion was triggered explicitly by a user gesture.
    /// Return multiple completion items to enable cycling through them.
    pub const Invoked: InlineCompletionTriggerKind = InlineCompletionTriggerKind(1);

    /// Completion was triggered automatically while editing.
    /// It is sufficient to return a single completion item in this case.
    pub const Automatic: InlineCompletionTriggerKind = InlineCompletionTriggerKind(2);
}
}

/// Describes the currently selected completion item.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct SelectedCompletionInfo {
    /// The range that will be replaced if this completion item is accepted.
    pub range: Range,
    /// The text the range will be replaced with if this completion is
    /// accepted.
    pub text: String,
}

/// Provides information about the context in which an inline completion was
/// requested.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineCompletionContext {
    /// Describes how the inline completion was triggered.
    pub trigger_kind: InlineCompletionTriggerKind,
    /// Provides information about the currently selected item in the
    /// autocomplete widget if it is visible.
    ///
    /// If set, provided inline completions must extend the text of the
    /// selected item and use the same range, otherwise they are not shown as
    /// preview.
    /// As an example, if the document text is `console.` and the selected item
    /// is `.log` replacing the `.` in the document, the inline completion must
    /// also replace `.` and start with `.log`, for example `.log()`.
    ///
    /// Inline completion providers are requested again whenever the selected
    /// item changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_completion_info: Option<SelectedCompletionInfo>,
}

/// InlineCompletion response can be multiple completion items, or a list of completion items
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlineCompletionResponse {
    Array(Vec<InlineCompletionItem>),
    List(InlineCompletionList),
}

/// Represents a collection of [`InlineCompletionItem`] to be presented in the editor.
///
/// @since 3.18.0
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct InlineCompletionList {
    /// The inline completion items
    pub items: Vec<InlineCompletionItem>,
}

/// An inline completion item represents a text snippet that is proposed inline
/// to complete text that is being typed.
///
/// @since 3.18.0
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineCompletionItem {
    /// The text to replace the range with. Must be set.
    /// Is used both for the preview and the accept operation.
    pub insert_text: String,
    /// A text that is used to decide if this inline completion should be
    /// shown. When `falsy` the [`InlineCompletionItem::insertText`] is
    /// used.
    ///
    /// An inline completion is shown if the text to replace is a prefix of the
    /// filter text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_text: Option<String>,
    /// The range to replace.
    /// Must begin and end on the same line.
    ///
    /// Prefer replacements over insertions to provide a better experience when
    /// the user deletes typed text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
    /// An optional command that is executed *after* inserting this
    /// completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Command>,
    /// The format of the insert text. The format applies to the `insertText`.
    /// If omitted defaults to `InsertTextFormat.PlainText`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text_format: Option<InsertTextFormat>,
}
