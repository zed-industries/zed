use serde::{Deserialize, Serialize};

pub enum GetCompletions {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCompletionsParams {
    pub doc: GetCompletionsDocument,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCompletionsDocument {
    pub tab_size: u32,
    pub indent_size: u32,
    pub insert_spaces: bool,
    pub uri: lsp::Url,
    pub relative_path: String,
    pub position: lsp::Position,
    pub version: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCompletionsResult {
    pub completions: Vec<Completion>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Completion {
    pub text: String,
    pub position: lsp::Position,
    pub uuid: String,
    pub range: lsp::Range,
    pub display_text: String,
}

impl lsp::request::Request for GetCompletions {
    type Params = GetCompletionsParams;
    type Result = GetCompletionsResult;
    const METHOD: &'static str = "getCompletions";
}

pub enum GetCompletionsCycling {}

impl lsp::request::Request for GetCompletionsCycling {
    type Params = GetCompletionsParams;
    type Result = GetCompletionsResult;
    const METHOD: &'static str = "getCompletionsCycling";
}

pub enum LogMessage {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogMessageParams {
    pub level: u8,
    pub message: String,
    pub metadata_str: String,
    pub extra: Vec<String>,
}

impl lsp::notification::Notification for LogMessage {
    type Params = LogMessageParams;
    const METHOD: &'static str = "LogMessage";
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

// LSP 3.18 and custom Copilot requests/notifications

// Authentication

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum SignInResult {
    AlreadySignedIn { user: String },
    PromptUserDeviceFlow(PromptUserDeviceFlow),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptUserDeviceFlow {
    pub user_code: String,
    pub verification_uri: String,
}

impl lsp::request::Request for SignIn {
    type Params = SignInParams;
    type Result = SignInResult;
    const METHOD: &'static str = "signIn";
}

pub enum SignInConfirm {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInConfirmParams {
    pub user_code: String,
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

impl lsp::request::Request for SignInConfirm {
    type Params = SignInConfirmParams;
    type Result = SignInStatus;
    const METHOD: &'static str = "signInConfirm";
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

// Initialization

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializationOptions {
    pub editor_info: EditorInfo,
    pub editor_plugin_info: EditorPluginInfo,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorPluginInfo {
    pub name: String,
    pub version: String,
}

// Status Notification
pub enum DidChangeStatus {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeStatusParams {
    pub message: String,
    pub kind: String, // 'Normal', 'Error', 'Warning', 'Inactive'
}

impl lsp::notification::Notification for DidChangeStatus {
    type Params = DidChangeStatusParams;
    const METHOD: &'static str = "didChangeStatus";
}

// Standard LSP window/showMessageRequest request
pub enum WindowShowMessageRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowMessageParams {
    /// The message type. See {@link MessageType}
    #[serde(rename = "type")]
    pub type_: MessageType,
    /// The actual message
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[repr(i32)]
pub enum MessageType {
    Error = 1,
    Warning = 2,
    Info = 3,
    Log = 4,
    Debug = 5,
}

impl MessageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MessageType::Error => "Error",
            MessageType::Warning => "Warning",
            MessageType::Info => "Info",
            MessageType::Log => "Log",
            MessageType::Debug => "Debug",
        }
    }
}

impl lsp::notification::Notification for WindowShowMessageRequest {
    type Params = ShowMessageParams;
    const METHOD: &'static str = "window/showMessageRequest";
}

// Inline Completions
pub enum TextDocumentInlineCompletion {}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextDocumentInlineCompletionParams {
    pub text_document: TextDocumentIdentifier,
    pub position: lsp::Position,
    pub context: InlineCompletionContext,
    pub formatting_options: FormattingOptions,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentIdentifier {
    pub uri: lsp::Url,
    pub version: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineCompletionContext {
    pub trigger_kind: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormattingOptions {
    pub tab_size: u32,
    pub insert_spaces: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextDocumentInlineCompletionResult {
    pub items: Vec<InlineCompletionItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineCompletionItem {
    pub insert_text: String,
    pub range: lsp::Range,
    pub command: lsp::Command,
}

impl lsp::request::Request for TextDocumentInlineCompletion {
    type Params = TextDocumentInlineCompletionParams;
    type Result = TextDocumentInlineCompletionResult;
    const METHOD: &'static str = "textDocument/inlineCompletion";
}

// Show Completion Notification
pub enum TextDocumentDidShowCompletion {}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextDocumentDidShowCompletionParams {
    pub item: InlineCompletionItem,
}

impl lsp::notification::Notification for TextDocumentDidShowCompletion {
    type Params = TextDocumentDidShowCompletionParams;
    const METHOD: &'static str = "textDocument/didShowCompletion";
}

// Partially Accept Completion Notification
pub enum TextDocumentDidPartiallyAcceptCompletion {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentDidPartiallyAcceptCompletionParams {
    pub item: InlineCompletionItem,
    pub accepted_length: u32,
}

impl lsp::notification::Notification for TextDocumentDidPartiallyAcceptCompletion {
    type Params = TextDocumentDidPartiallyAcceptCompletionParams;
    const METHOD: &'static str = "textDocument/didPartiallyAcceptCompletion";
}

// Text Document Focus Notification
pub enum TextDocumentDidFocus {}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextDocumentDidFocusParams {
    pub text_document: Option<TextDocumentIdentifier>,
}

impl lsp::notification::Notification for TextDocumentDidFocus {
    type Params = TextDocumentDidFocusParams;
    const METHOD: &'static str = "textDocument/didFocus";
}

// Panel Completions
pub enum TextDocumentCopilotPanelCompletion {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentCopilotPanelCompletionParams {
    pub text_document: TextDocumentIdentifier,
    pub position: lsp::Position,
    pub partial_result_token: Option<String>,
}

impl lsp::request::Request for TextDocumentCopilotPanelCompletion {
    type Params = TextDocumentCopilotPanelCompletionParams;
    type Result = TextDocumentInlineCompletionResult;
    const METHOD: &'static str = "textDocument/copilotPanelCompletion";
}
