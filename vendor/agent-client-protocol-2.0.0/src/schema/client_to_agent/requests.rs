use crate::schema::v1::{
    AuthenticateRequest, AuthenticateResponse, CloseSessionRequest, CloseSessionResponse,
    DeleteSessionRequest, DeleteSessionResponse, InitializeRequest, InitializeResponse,
    ListSessionsRequest, ListSessionsResponse, LoadSessionRequest, LoadSessionResponse,
    LogoutRequest, LogoutResponse, NewSessionRequest, NewSessionResponse, PromptRequest,
    PromptResponse, ResumeSessionRequest, ResumeSessionResponse, SetSessionConfigOptionRequest,
    SetSessionConfigOptionResponse, SetSessionModeRequest, SetSessionModeResponse,
};
#[cfg(feature = "unstable_session_fork")]
use crate::schema::v1::{ForkSessionRequest, ForkSessionResponse};

impl_jsonrpc_request!(InitializeRequest, InitializeResponse, "initialize");
impl_jsonrpc_request!(AuthenticateRequest, AuthenticateResponse, "authenticate");
impl_jsonrpc_request!(LogoutRequest, LogoutResponse, "logout");
impl_jsonrpc_request!(LoadSessionRequest, LoadSessionResponse, "session/load");
impl_jsonrpc_request!(ListSessionsRequest, ListSessionsResponse, "session/list");
impl_jsonrpc_request!(
    DeleteSessionRequest,
    DeleteSessionResponse,
    "session/delete"
);
impl_jsonrpc_request!(NewSessionRequest, NewSessionResponse, "session/new");
impl_jsonrpc_request!(PromptRequest, PromptResponse, "session/prompt");
impl_jsonrpc_request!(
    SetSessionModeRequest,
    SetSessionModeResponse,
    "session/set_mode"
);
impl_jsonrpc_request!(
    SetSessionConfigOptionRequest,
    SetSessionConfigOptionResponse,
    "session/set_config_option"
);

#[cfg(feature = "unstable_session_fork")]
impl_jsonrpc_request!(ForkSessionRequest, ForkSessionResponse, "session/fork");
impl_jsonrpc_request!(
    ResumeSessionRequest,
    ResumeSessionResponse,
    "session/resume"
);
impl_jsonrpc_request!(CloseSessionRequest, CloseSessionResponse, "session/close");
