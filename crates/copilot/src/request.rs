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

pub enum SignInInitiate {}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInInitiateParams {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum SignInInitiateResult {
    AlreadySignedIn { user: String },
    PromptUserDeviceFlow(PromptUserDeviceFlow),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptUserDeviceFlow {
    pub user_code: String,
    pub verification_uri: String,
}

impl lsp::request::Request for SignInInitiate {
    type Params = SignInInitiateParams;
    type Result = SignInInitiateResult;
    const METHOD: &'static str = "signInInitiate";
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
        user: String,
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
