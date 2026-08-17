//! JSON-RPC trait implementations for the experimental schema v2 namespace.

use crate::schema::v2;
use crate::{JsonRpcMessage, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse, UntypedMessage};

macro_rules! impl_v2_jsonrpc_request {
    ($req:ty, $resp:ty, $method:literal) => {
        impl JsonRpcMessage for $req {
            fn matches_method(method: &str) -> bool {
                method == $method
            }

            fn method(&self) -> &str {
                $method
            }

            fn to_untyped_message(&self) -> Result<UntypedMessage, crate::Error> {
                UntypedMessage::new($method, self)
            }

            fn parse_message(
                method: &str,
                params: &impl serde::Serialize,
            ) -> Result<Self, crate::Error> {
                if method != $method {
                    return Err(crate::Error::method_not_found());
                }
                crate::util::json_cast_params(params)
            }
        }

        impl JsonRpcRequest for $req {
            type Response = $resp;
        }

        impl JsonRpcResponse for $resp {
            fn into_json(self, _method: &str) -> Result<serde_json::Value, crate::Error> {
                serde_json::to_value(self).map_err(crate::Error::into_internal_error)
            }

            fn from_value(_method: &str, value: serde_json::Value) -> Result<Self, crate::Error> {
                crate::util::json_cast(value)
            }
        }
    };
}

macro_rules! impl_v2_jsonrpc_notification {
    ($notif:ty, $method:literal) => {
        impl JsonRpcMessage for $notif {
            fn matches_method(method: &str) -> bool {
                method == $method
            }

            fn method(&self) -> &str {
                $method
            }

            fn to_untyped_message(&self) -> Result<UntypedMessage, crate::Error> {
                UntypedMessage::new($method, self)
            }

            fn parse_message(
                method: &str,
                params: &impl serde::Serialize,
            ) -> Result<Self, crate::Error> {
                if method != $method {
                    return Err(crate::Error::method_not_found());
                }
                crate::util::json_cast_params(params)
            }
        }

        impl JsonRpcNotification for $notif {}
    };
}

macro_rules! impl_v2_jsonrpc_request_enum {
    ($enum:ty {
        $( $(#[$meta:meta])* $variant:ident => $method:literal, )*
        [ext] $ext_variant:ident,
    }) => {
        impl JsonRpcMessage for $enum {
            fn matches_method(_method: &str) -> bool {
                true
            }

            fn method(&self) -> &str {
                match self {
                    $( $(#[$meta])* Self::$variant(_) => $method, )*
                    Self::$ext_variant(ext) => &ext.method,
                    _ => "_unknown",
                }
            }

            fn to_untyped_message(&self) -> Result<UntypedMessage, crate::Error> {
                UntypedMessage::new(self.method(), self)
            }

            fn parse_message(
                method: &str,
                params: &impl serde::Serialize,
            ) -> Result<Self, crate::Error> {
                match method {
                    $( $(#[$meta])* $method => crate::util::json_cast_params(params).map(Self::$variant), )*
                    _ => {
                        if method.starts_with('_') {
                            crate::util::json_cast_params(params).map(
                                |ext_req: v2::ExtRequest| {
                                    Self::$ext_variant(Box::new(v2::ExtRequest::new(
                                        method.to_string(),
                                        ext_req.params,
                                    )))
                                },
                            )
                        } else {
                            Err(crate::Error::method_not_found())
                        }
                    }
                }
            }
        }

        impl JsonRpcRequest for $enum {
            type Response = serde_json::Value;
        }
    };
}

macro_rules! impl_v2_jsonrpc_notification_enum {
    ($enum:ty {
        $( $(#[$meta:meta])* $variant:ident => $method:literal, )*
        [ext] $ext_variant:ident,
    }) => {
        impl JsonRpcMessage for $enum {
            fn matches_method(_method: &str) -> bool {
                true
            }

            fn method(&self) -> &str {
                match self {
                    $( $(#[$meta])* Self::$variant(_) => $method, )*
                    Self::$ext_variant(ext) => &ext.method,
                    _ => "_unknown",
                }
            }

            fn to_untyped_message(&self) -> Result<UntypedMessage, crate::Error> {
                UntypedMessage::new(self.method(), self)
            }

            fn parse_message(
                method: &str,
                params: &impl serde::Serialize,
            ) -> Result<Self, crate::Error> {
                match method {
                    $( $(#[$meta])* $method => crate::util::json_cast_params(params).map(Self::$variant), )*
                    _ => {
                        if method.starts_with('_') {
                            crate::util::json_cast_params(params).map(
                                |ext_notif: v2::ExtNotification| {
                                    Self::$ext_variant(Box::new(v2::ExtNotification::new(
                                        method.to_string(),
                                        ext_notif.params,
                                    )))
                                },
                            )
                        } else {
                            Err(crate::Error::method_not_found())
                        }
                    }
                }
            }
        }

        impl JsonRpcNotification for $enum {}
    };
}

macro_rules! impl_v2_jsonrpc_response_enum {
    ($enum:ty {
        $( $(#[$meta:meta])* $variant:ident => $method:literal, )*
        [ext] $ext_variant:ident,
    }) => {
        impl JsonRpcResponse for $enum {
            fn into_json(
                self,
                _method: &str,
            ) -> Result<serde_json::Value, crate::Error> {
                serde_json::to_value(self).map_err(crate::Error::into_internal_error)
            }

            fn from_value(
                method: &str,
                value: serde_json::Value,
            ) -> Result<Self, crate::Error> {
                match method {
                    $( $(#[$meta])* $method => crate::util::json_cast(value).map(Self::$variant), )*
                    _ => {
                        if method.starts_with('_') {
                            crate::util::json_cast(value).map(Self::$ext_variant)
                        } else {
                            Err(crate::Error::method_not_found())
                        }
                    }
                }
            }
        }
    };
}

impl_v2_jsonrpc_request!(v2::InitializeRequest, v2::InitializeResponse, "initialize");
impl_v2_jsonrpc_request!(v2::LoginAuthRequest, v2::LoginAuthResponse, "auth/login");
impl_v2_jsonrpc_request!(v2::LogoutAuthRequest, v2::LogoutAuthResponse, "auth/logout");
impl_v2_jsonrpc_request!(v2::NewSessionRequest, v2::NewSessionResponse, "session/new");
impl_v2_jsonrpc_request!(
    v2::ListSessionsRequest,
    v2::ListSessionsResponse,
    "session/list"
);
impl_v2_jsonrpc_request!(
    v2::DeleteSessionRequest,
    v2::DeleteSessionResponse,
    "session/delete"
);
#[cfg(feature = "unstable_session_fork")]
impl_v2_jsonrpc_request!(
    v2::ForkSessionRequest,
    v2::ForkSessionResponse,
    "session/fork"
);
impl_v2_jsonrpc_request!(
    v2::ResumeSessionRequest,
    v2::ResumeSessionResponse,
    "session/resume"
);
impl_v2_jsonrpc_request!(
    v2::CloseSessionRequest,
    v2::CloseSessionResponse,
    "session/close"
);
impl_v2_jsonrpc_request!(
    v2::SetSessionConfigOptionRequest,
    v2::SetSessionConfigOptionResponse,
    "session/set_config_option"
);
impl_v2_jsonrpc_request!(v2::PromptRequest, v2::PromptResponse, "session/prompt");
#[cfg(feature = "unstable_mcp_over_acp")]
impl_v2_jsonrpc_request!(v2::MessageMcpRequest, v2::MessageMcpResponse, "mcp/message");

impl_v2_jsonrpc_notification!(v2::CancelRequestNotification, "$/cancel_request");
impl_v2_jsonrpc_notification!(v2::CancelSessionNotification, "session/cancel");
#[cfg(feature = "unstable_mcp_over_acp")]
impl_v2_jsonrpc_notification!(v2::MessageMcpNotification, "mcp/message");

impl_v2_jsonrpc_request!(
    v2::RequestPermissionRequest,
    v2::RequestPermissionResponse,
    "session/request_permission"
);
#[cfg(feature = "unstable_elicitation")]
impl_v2_jsonrpc_request!(
    v2::CreateElicitationRequest,
    v2::CreateElicitationResponse,
    "elicitation/create"
);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_v2_jsonrpc_request!(v2::ConnectMcpRequest, v2::ConnectMcpResponse, "mcp/connect");
#[cfg(feature = "unstable_mcp_over_acp")]
impl_v2_jsonrpc_request!(
    v2::DisconnectMcpRequest,
    v2::DisconnectMcpResponse,
    "mcp/disconnect"
);

impl_v2_jsonrpc_notification!(v2::UpdateSessionNotification, "session/update");
#[cfg(feature = "unstable_elicitation")]
impl_v2_jsonrpc_notification!(v2::CompleteElicitationNotification, "elicitation/complete");

impl_jsonrpc_protocol_level_notification_enum!(v2::ProtocolLevelNotification {
    CancelRequestNotification => "$/cancel_request",
});

impl_v2_jsonrpc_request_enum!(v2::ClientRequest {
    InitializeRequest => "initialize",
    LoginAuthRequest => "auth/login",
    LogoutAuthRequest => "auth/logout",
    NewSessionRequest => "session/new",
    ListSessionsRequest => "session/list",
    DeleteSessionRequest => "session/delete",
    #[cfg(feature = "unstable_session_fork")]
    ForkSessionRequest => "session/fork",
    ResumeSessionRequest => "session/resume",
    CloseSessionRequest => "session/close",
    SetSessionConfigOptionRequest => "session/set_config_option",
    PromptRequest => "session/prompt",
    #[cfg(feature = "unstable_mcp_over_acp")]
    MessageMcpRequest => "mcp/message",
    [ext] ExtMethodRequest,
});

impl_v2_jsonrpc_response_enum!(v2::AgentResponse {
    InitializeResponse => "initialize",
    LoginAuthResponse => "auth/login",
    LogoutAuthResponse => "auth/logout",
    NewSessionResponse => "session/new",
    ListSessionsResponse => "session/list",
    DeleteSessionResponse => "session/delete",
    #[cfg(feature = "unstable_session_fork")]
    ForkSessionResponse => "session/fork",
    ResumeSessionResponse => "session/resume",
    CloseSessionResponse => "session/close",
    SetSessionConfigOptionResponse => "session/set_config_option",
    PromptResponse => "session/prompt",
    #[cfg(feature = "unstable_mcp_over_acp")]
    MessageMcpResponse => "mcp/message",
    [ext] ExtMethodResponse,
});

impl_v2_jsonrpc_notification_enum!(v2::ClientNotification {
    CancelSessionNotification => "session/cancel",
    #[cfg(feature = "unstable_mcp_over_acp")]
    MessageMcpNotification => "mcp/message",
    [ext] ExtNotification,
});

impl_v2_jsonrpc_request_enum!(v2::AgentRequest {
    RequestPermissionRequest => "session/request_permission",
    #[cfg(feature = "unstable_elicitation")]
    CreateElicitationRequest => "elicitation/create",
    #[cfg(feature = "unstable_mcp_over_acp")]
    ConnectMcpRequest => "mcp/connect",
    #[cfg(feature = "unstable_mcp_over_acp")]
    MessageMcpRequest => "mcp/message",
    #[cfg(feature = "unstable_mcp_over_acp")]
    DisconnectMcpRequest => "mcp/disconnect",
    [ext] ExtMethodRequest,
});

impl_v2_jsonrpc_response_enum!(v2::ClientResponse {
    RequestPermissionResponse => "session/request_permission",
    #[cfg(feature = "unstable_elicitation")]
    CreateElicitationResponse => "elicitation/create",
    #[cfg(feature = "unstable_mcp_over_acp")]
    ConnectMcpResponse => "mcp/connect",
    #[cfg(feature = "unstable_mcp_over_acp")]
    MessageMcpResponse => "mcp/message",
    #[cfg(feature = "unstable_mcp_over_acp")]
    DisconnectMcpResponse => "mcp/disconnect",
    [ext] ExtMethodResponse,
});

impl_v2_jsonrpc_notification_enum!(v2::AgentNotification {
    UpdateSessionNotification => "session/update",
    #[cfg(feature = "unstable_elicitation")]
    CompleteElicitationNotification => "elicitation/complete",
    #[cfg(feature = "unstable_mcp_over_acp")]
    MessageMcpNotification => "mcp/message",
    [ext] ExtNotification,
});
