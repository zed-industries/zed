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

pub enum StatusNotification {}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusNotificationParams {
    pub message: String,
    pub status: String, // One of Normal/InProgress
}

impl lsp::notification::Notification for StatusNotification {
    type Params = StatusNotificationParams;
    const METHOD: &'static str = "statusNotification";
}

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
