use lsp::{Uri, VersionedTextDocumentIdentifier};
use serde::{Deserialize, Serialize};

pub enum CheckStatus {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckStatusParams {
    pub local_checks_only: bool,
}

impl lsp::request::Request for CheckStatus {
    type Params = CheckStatusParams;
    type Result = SignInStatus;
    const METHOD: &'static str = "checkStatus";
}

pub enum SignIn {}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInParams {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptUserDeviceFlow {
    pub user_code: String,
    pub command: lsp::Command,
}

impl lsp::request::Request for SignIn {
    type Params = SignInParams;
    type Result = PromptUserDeviceFlow;
    const METHOD: &'static str = "signIn";
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum SignInStatus {
    #[serde(rename = "OK")]
    Ok {
        user: Option<String>,
    },
    MaybeOk {
        user: String,
    },
    AlreadySignedIn {
        user: String,
    },
    NotAuthorized {
        user: String,
    },
    NotSignedIn,
}

pub enum SignOut {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignOutParams {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignOutResult {}

impl lsp::request::Request for SignOut {
    type Params = SignOutParams;
    type Result = SignOutResult;
    const METHOD: &'static str = "signOut";
}

pub enum DidChangeStatus {}

#[derive(Debug, Serialize, Deserialize)]
pub struct DidChangeStatusParams {
    #[serde(default)]
    pub message: Option<String>,
    pub kind: StatusKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusKind {
    Normal,
    Error,
    Warning,
    Inactive,
}

impl lsp::notification::Notification for DidChangeStatus {
    type Params = DidChangeStatusParams;
    const METHOD: &'static str = "didChangeStatus";
}

pub enum SetEditorInfo {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetEditorInfoParams {
    pub editor_info: EditorInfo,
    pub editor_plugin_info: EditorPluginInfo,
}

impl lsp::request::Request for SetEditorInfo {
    type Params = SetEditorInfoParams;
    type Result = String;
    const METHOD: &'static str = "setEditorInfo";
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorPluginInfo {
    pub name: String,
    pub version: String,
}

pub enum NotifyAccepted {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyAcceptedParams {
    pub uuid: String,
}

impl lsp::request::Request for NotifyAccepted {
    type Params = NotifyAcceptedParams;
    type Result = String;
    const METHOD: &'static str = "notifyAccepted";
}

pub enum NotifyRejected {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyRejectedParams {
    pub uuids: Vec<String>,
}

impl lsp::request::Request for NotifyRejected {
    type Params = NotifyRejectedParams;
    type Result = String;
    const METHOD: &'static str = "notifyRejected";
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextEditSuggestions;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextEditSuggestionsParams {
    pub(crate) text_document: VersionedTextDocumentIdentifier,
    pub(crate) position: lsp::Position,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextEditSuggestion {
    pub text: String,
    pub text_document: VersionedTextDocumentIdentifier,
    pub range: lsp::Range,
    pub command: Option<lsp::Command>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextEditSuggestionsResult {
    pub edits: Vec<NextEditSuggestion>,
}

impl lsp::request::Request for NextEditSuggestions {
    type Params = NextEditSuggestionsParams;
    type Result = NextEditSuggestionsResult;

    const METHOD: &'static str = "textDocument/copilotInlineEdit";
}

pub(crate) struct DidFocus;

#[derive(Serialize, Deserialize)]
pub(crate) struct DidFocusParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) uri: Option<Uri>,
}

impl lsp::notification::Notification for DidFocus {
    type Params = DidFocusParams;
    const METHOD: &'static str = "textDocument/didFocus";
}

pub(crate) struct DidShowInlineEdit;

#[derive(Serialize, Deserialize)]
pub(crate) struct DidShowInlineEditParams {
    pub(crate) item: serde_json::Value,
}

impl lsp::notification::Notification for DidShowInlineEdit {
    type Params = DidShowInlineEditParams;
    const METHOD: &'static str = "textDocument/didShowInlineEdit";
}

// Inline Completions (non-NES) - textDocument/inlineCompletion

pub enum InlineCompletions {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineCompletionsParams {
    pub text_document: VersionedTextDocumentIdentifier,
    pub position: lsp::Position,
    pub context: InlineCompletionContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatting_options: Option<FormattingOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineCompletionContext {
    pub trigger_kind: InlineCompletionTriggerKind,
}

#[derive(Debug, Clone, Copy)]
pub enum InlineCompletionTriggerKind {
    Invoked = 1,
    Automatic = 2,
}

impl Serialize for InlineCompletionTriggerKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for InlineCompletionTriggerKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        match value {
            1 => Ok(InlineCompletionTriggerKind::Invoked),
            2 => Ok(InlineCompletionTriggerKind::Automatic),
            _ => Err(serde::de::Error::custom("invalid trigger kind")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormattingOptions {
    pub tab_size: u32,
    pub insert_spaces: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineCompletionsResult {
    pub items: Vec<InlineCompletionItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineCompletionItem {
    pub insert_text: String,
    pub range: lsp::Range,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<lsp::Command>,
}

impl lsp::request::Request for InlineCompletions {
    type Params = InlineCompletionsParams;
    type Result = InlineCompletionsResult;

    const METHOD: &'static str = "textDocument/inlineCompletion";
}

// Telemetry notifications for inline completions

pub(crate) struct DidShowCompletion;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DidShowCompletionParams {
    pub(crate) item: InlineCompletionItem,
}

impl lsp::notification::Notification for DidShowCompletion {
    type Params = DidShowCompletionParams;
    const METHOD: &'static str = "textDocument/didShowCompletion";
}
