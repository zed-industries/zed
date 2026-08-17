//! ACP protocol schema types and message implementations.
//!
//! This module contains all the types from the Agent-Client Protocol schema,
//! including requests, responses, notifications, and supporting types.
//! All types are re-exported flatly from this module.

// ---------------------------------------------------------------------------
// Macros for implementing JsonRpc traits on schema types
// ---------------------------------------------------------------------------

/// Implement `JsonRpcMessage`, `JsonRpcRequest`, and `JsonRpcResponse` for a
/// request/response pair from the schema crate.
///
/// ```ignore
/// impl_jsonrpc_request!(PromptRequest, PromptResponse, "session/prompt");
/// ```
macro_rules! impl_jsonrpc_request {
    ($req:ty, $resp:ty, $method:literal) => {
        impl $crate::JsonRpcMessage for $req {
            fn matches_method(method: &str) -> bool {
                method == $method
            }

            fn method(&self) -> &str {
                $method
            }

            fn to_untyped_message(&self) -> Result<$crate::UntypedMessage, $crate::Error> {
                $crate::UntypedMessage::new($method, self)
            }

            fn parse_message(
                method: &str,
                params: &impl serde::Serialize,
            ) -> Result<Self, $crate::Error> {
                if method != $method {
                    return Err($crate::Error::method_not_found());
                }
                $crate::util::json_cast_params(params)
            }
        }

        impl $crate::JsonRpcRequest for $req {
            type Response = $resp;
        }

        impl $crate::JsonRpcResponse for $resp {
            fn into_json(self, _method: &str) -> Result<serde_json::Value, $crate::Error> {
                serde_json::to_value(self).map_err($crate::Error::into_internal_error)
            }

            fn from_value(_method: &str, value: serde_json::Value) -> Result<Self, $crate::Error> {
                $crate::util::json_cast(&value)
            }
        }
    };
}

/// Implement `JsonRpcMessage` and `JsonRpcNotification` for a notification type
/// from the schema crate.
///
/// ```ignore
/// impl_jsonrpc_notification!(CancelNotification, "session/cancel");
/// ```
macro_rules! impl_jsonrpc_notification {
    ($notif:ty, $method:literal) => {
        impl $crate::JsonRpcMessage for $notif {
            fn matches_method(method: &str) -> bool {
                method == $method
            }

            fn method(&self) -> &str {
                $method
            }

            fn to_untyped_message(&self) -> Result<$crate::UntypedMessage, $crate::Error> {
                $crate::UntypedMessage::new($method, self)
            }

            fn parse_message(
                method: &str,
                params: &impl serde::Serialize,
            ) -> Result<Self, $crate::Error> {
                if method != $method {
                    return Err($crate::Error::method_not_found());
                }
                $crate::util::json_cast_params(params)
            }
        }

        impl $crate::JsonRpcNotification for $notif {}
    };
}

/// Implement `JsonRpcMessage` and `JsonRpcRequest` for an enum that dispatches
/// across multiple request types, with an extension method fallback.
///
/// Variants can optionally have `#[cfg(...)]` attributes for conditional compilation.
///
/// ```ignore
/// impl_jsonrpc_request_enum!(ClientRequest {
///     InitializeRequest => "initialize",
///     PromptRequest => "session/prompt",
///     [ext] ExtMethodRequest,
/// });
/// ```
macro_rules! impl_jsonrpc_request_enum {
    ($enum:ty {
        $( $(#[$meta:meta])* $variant:ident => $method:literal, )*
        [ext] $ext_variant:ident,
    }) => {
        impl $crate::JsonRpcMessage for $enum {
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

            fn to_untyped_message(&self) -> Result<$crate::UntypedMessage, $crate::Error> {
                $crate::UntypedMessage::new(self.method(), self)
            }

            fn parse_message(
                method: &str,
                params: &impl serde::Serialize,
            ) -> Result<Self, $crate::Error> {
                match method {
                    $( $(#[$meta])* $method => $crate::util::json_cast_params(params).map(Self::$variant), )*
                    _ => {
                        if let Some(custom_method) = method.strip_prefix('_') {
                            $crate::util::json_cast_params(params).map(
                                |ext_req: $crate::schema::v1::ExtRequest| {
                                    Self::$ext_variant($crate::schema::v1::ExtRequest::new(
                                        custom_method.to_string(),
                                        ext_req.params,
                                    ))
                                },
                            )
                        } else {
                            Err($crate::Error::method_not_found())
                        }
                    }
                }
            }
        }

        impl $crate::JsonRpcRequest for $enum {
            type Response = serde_json::Value;
        }
    };
}

/// Implement `JsonRpcMessage` and `JsonRpcNotification` for an enum that
/// dispatches across multiple notification types, with an extension fallback.
///
/// Variants can optionally have `#[cfg(...)]` attributes for conditional compilation.
///
/// ```ignore
/// impl_jsonrpc_notification_enum!(AgentNotification {
///     SessionNotification => "session/update",
///     [ext] ExtNotification,
/// });
/// ```
macro_rules! impl_jsonrpc_notification_enum {
    ($enum:ty {
        $( $(#[$meta:meta])* $variant:ident => $method:literal, )*
        [ext] $ext_variant:ident,
    }) => {
        impl $crate::JsonRpcMessage for $enum {
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

            fn to_untyped_message(&self) -> Result<$crate::UntypedMessage, $crate::Error> {
                $crate::UntypedMessage::new(self.method(), self)
            }

            fn parse_message(
                method: &str,
                params: &impl serde::Serialize,
            ) -> Result<Self, $crate::Error> {
                match method {
                    $( $(#[$meta])* $method => $crate::util::json_cast_params(params).map(Self::$variant), )*
                    _ => {
                        if let Some(custom_method) = method.strip_prefix('_') {
                            $crate::util::json_cast_params(params).map(
                                |ext_notif: $crate::schema::v1::ExtNotification| {
                                    Self::$ext_variant($crate::schema::v1::ExtNotification::new(
                                        custom_method.to_string(),
                                        ext_notif.params,
                                    ))
                                },
                            )
                        } else {
                            Err($crate::Error::method_not_found())
                        }
                    }
                }
            }
        }

        impl $crate::JsonRpcNotification for $enum {}
    };
}

/// Implement `JsonRpcMessage` and `JsonRpcNotification` for a protocol-level
/// notification enum (`$/`-prefixed methods), shared between the v1 and v2
/// schema namespaces.
///
/// The incoming side (`matches_method`, `parse_message`) only recognizes the
/// methods listed in the macro invocation: when the schema crate adds a
/// protocol-level notification, list it here to parse it. The outgoing side
/// (`method`, `to_untyped_message`) instead delegates to the schema enum's
/// inherent `method()` and untagged serialization, which cover every variant,
/// so unlisted variants still serialize with the correct method name.
///
/// ```ignore
/// impl_jsonrpc_protocol_level_notification_enum!(ProtocolLevelNotification {
///     CancelRequestNotification => "$/cancel_request",
/// });
/// ```
macro_rules! impl_jsonrpc_protocol_level_notification_enum {
    ($enum:ty {
        $( $variant:ident => $method:literal, )*
    }) => {
        impl $crate::JsonRpcMessage for $enum {
            fn matches_method(method: &str) -> bool {
                matches!(method, $( $method )|*)
            }

            fn method(&self) -> &str {
                // Resolves to the schema enum's *inherent* `method()` (path
                // syntax prefers inherent items over trait items), which
                // matches its variants exhaustively: the enum is only
                // non-exhaustive downstream.
                <$enum>::method(self)
            }

            fn to_untyped_message(&self) -> Result<$crate::UntypedMessage, $crate::Error> {
                // The schema enum is `#[serde(untagged)]`, so serializing the
                // enum serializes the inner notification.
                $crate::UntypedMessage::new(<$enum>::method(self), self)
            }

            fn parse_message(
                method: &str,
                params: &impl serde::Serialize,
            ) -> Result<Self, $crate::Error> {
                match method {
                    $( $method => $crate::util::json_cast_params(params).map(Self::$variant), )*
                    _ => Err($crate::Error::method_not_found()),
                }
            }
        }

        impl $crate::JsonRpcNotification for $enum {}
    };
}

/// Implement `JsonRpcResponse` for an enum that dispatches across multiple
/// response types, with an extension method fallback.
macro_rules! impl_jsonrpc_response_enum {
    ($enum:ty {
        $( $(#[$meta:meta])* $variant:ident => $method:literal, )*
        [ext] $ext_variant:ident,
    }) => {
        impl $crate::JsonRpcResponse for $enum {
            fn into_json(
                self,
                _method: &str,
            ) -> Result<serde_json::Value, $crate::Error> {
                serde_json::to_value(self).map_err($crate::Error::into_internal_error)
            }

            fn from_value(
                method: &str,
                value: serde_json::Value,
            ) -> Result<Self, $crate::Error> {
                match method {
                    $( $(#[$meta])* $method => $crate::util::json_cast(value).map(Self::$variant), )*
                    _ => {
                        if method.starts_with('_') {
                            $crate::util::json_cast(value).map(Self::$ext_variant)
                        } else {
                            Err($crate::Error::method_not_found())
                        }
                    }
                }
            }
        }
    };
}

// Internal organization
mod agent_to_client;
mod client_to_agent;
mod enum_impls;
#[cfg(feature = "unstable_mcp_over_acp")]
mod mcp;
mod protocol_level;
mod proxy_protocol;
#[cfg(feature = "unstable_protocol_v2")]
mod v2_impls;

/// Agent Client Protocol v1 schema types.
pub mod v1 {
    pub use agent_client_protocol_schema::v1::*;
}

/// Agent Client Protocol v2 draft schema types.
#[cfg(feature = "unstable_protocol_v2")]
pub mod v2 {
    pub use agent_client_protocol_schema::v2::*;
}

pub use agent_client_protocol_schema::{
    IntoMaybeUndefined, IntoOption, MaybeUndefined, ProtocolVersion,
};

// Re-export SDK-local proxy protocol types flatly.
pub use proxy_protocol::*;
