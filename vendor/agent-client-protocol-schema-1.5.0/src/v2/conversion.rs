//! Explicit conversion helpers for experimenting with ACP v2 while SDKs still speak v1.
//!
//! The conversions below intentionally move values field-by-field and
//! variant-by-variant instead of serializing through JSON so v2 shape changes
//! have obvious edit points. Conversions use [`From`] when every source value
//! has a target representation and [`TryFrom`] when values outside the shared
//! protocol subset must be rejected instead of guessed, dropped, or defaulted.
//! One-to-many fan-out is represented as `TryFrom<Source> for Vec<Target>`.
//!
//! These helpers are convenience APIs for code that wants to share internal
//! ACP-shaped values while supporting both protocol versions. They are not a
//! protocol router: SDKs should choose the v1 or v2 implementation for a
//! connection before dispatching JSON-RPC messages.

use std::{
    collections::{BTreeMap, HashMap},
    convert::Infallible,
    fmt,
    hash::{BuildHasher, Hash},
    path::{Path, PathBuf},
    sync::Arc,
};

use serde_json::value::RawValue;

use crate::version::ProtocolVersion;

/// Result type returned by protocol try-conversion helpers.
pub type Result<T> = std::result::Result<T, ProtocolConversionError>;

/// Error returned when converting between v1 and v2 protocol type namespaces fails.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ProtocolConversionError {
    message: String,
}

impl ProtocolConversionError {
    /// Creates a conversion error with a human-readable message.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Returns the human-readable conversion error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ProtocolConversionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ProtocolConversionError {}

impl From<Infallible> for ProtocolConversionError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}

fn unknown_v2_enum_variant(type_name: &str, value: &str) -> ProtocolConversionError {
    ProtocolConversionError::new(format!(
        "v2 {type_name} variant `{value}` cannot be represented in v1"
    ))
}

fn removed_v1_enum_variant(type_name: &str, value: &str) -> ProtocolConversionError {
    ProtocolConversionError::new(format!(
        "v1 {type_name} variant `{value}` cannot be represented in v2"
    ))
}

fn unrepresentable_v1_field(type_name: &str, field: &str) -> ProtocolConversionError {
    ProtocolConversionError::new(format!(
        "v1 {type_name}.{field} cannot be represented in v2"
    ))
}

fn unrepresentable_v2_field(type_name: &str, field: &str) -> ProtocolConversionError {
    ProtocolConversionError::new(format!(
        "v2 {type_name}.{field} cannot be represented in v1"
    ))
}

fn reject_v2_marker_meta(type_name: &str, field: &str, meta: Option<&super::Meta>) -> Result<()> {
    if meta.is_some() {
        return Err(ProtocolConversionError::new(format!(
            "v2 {type_name}.{field} metadata cannot be represented in v1"
        )));
    }
    Ok(())
}

fn reject_v1_marker_meta(
    type_name: &str,
    field: &str,
    meta: Option<&crate::v1::Meta>,
) -> Result<()> {
    if meta.is_some() {
        return Err(ProtocolConversionError::new(format!(
            "v1 {type_name}.{field} metadata cannot be represented in v2"
        )));
    }
    Ok(())
}

const LEGACY_V1_PLAN_ID: &str = "main";

/// Converts a [`ProtocolConversionError`] into a v1 [`Error`](crate::v1::Error)
/// so callers can use `?` to bubble conversion failures through APIs that
/// already speak the v1 error type.
///
/// The conversion is mapped onto [`Error::internal_error`](crate::v1::Error::internal_error)
/// because a failed cross-version conversion always indicates a protocol
/// mismatch on this side of the wire rather than a client mistake.
impl From<ProtocolConversionError> for crate::v1::Error {
    fn from(error: ProtocolConversionError) -> Self {
        crate::v1::Error::internal_error().data(error.message)
    }
}

/// Mirror of the [v1 `From`](#impl-From%3CProtocolConversionError%3E-for-Error)
/// impl for the v2 [`Error`](crate::v2::Error) type.
impl From<ProtocolConversionError> for crate::v2::Error {
    fn from(error: ProtocolConversionError) -> Self {
        crate::v2::Error::internal_error().data(error.message)
    }
}

trait TryToV1 {
    /// The corresponding v1 type.
    type Output;

    /// Attempts to convert this value into the corresponding v1 type.
    ///
    /// # Errors
    ///
    /// Returns [`ProtocolConversionError`] when a value cannot be represented in v1.
    fn try_to_v1(self) -> Result<Self::Output>;
}

trait TryToV2 {
    /// The corresponding v2 draft type.
    type Output;

    /// Attempts to convert this value into the corresponding v2 draft type.
    ///
    /// # Errors
    ///
    /// Returns [`ProtocolConversionError`] when a value cannot be represented in v2.
    fn try_to_v2(self) -> Result<Self::Output>;
}

/// Attempts to convert a v2 draft value into the corresponding v1 value type.
///
/// Infallible [`From`] conversions also work with this helper through the
/// standard library's blanket [`TryFrom`] implementation.
///
/// # Errors
///
/// Returns [`ProtocolConversionError`] when a value cannot be represented in v1.
pub fn try_v2_to_v1<T, U>(value: T) -> Result<U>
where
    U: TryFrom<T>,
    ProtocolConversionError: From<<U as TryFrom<T>>::Error>,
{
    U::try_from(value).map_err(ProtocolConversionError::from)
}

/// Attempts to convert a v2 draft value into one or more corresponding v1 values.
///
/// This is a readability wrapper around `Vec::<U>::try_from(value)`.
///
/// One-to-many conversions are stateless. When a v2 update's semantics depend
/// on previously delivered updates, this helper can only reject cases that are
/// visible in the value being converted. For example, a content-bearing whole
/// message update can be emitted as v1 chunks, but v1 cannot represent the
/// replacement semantics if the v1 side has already received content for that
/// `messageId`.
///
/// # Errors
///
/// Returns [`ProtocolConversionError`] when a value cannot be represented in v1.
pub fn try_v2_to_v1_many<T, U>(value: T) -> Result<Vec<U>>
where
    Vec<U>: TryFrom<T>,
    ProtocolConversionError: From<<Vec<U> as TryFrom<T>>::Error>,
{
    Vec::<U>::try_from(value).map_err(ProtocolConversionError::from)
}

/// Attempts to convert a v1 value into the corresponding v2 draft value type.
///
/// Infallible [`From`] conversions also work with this helper through the
/// standard library's blanket [`TryFrom`] implementation.
///
/// # Errors
///
/// Returns [`ProtocolConversionError`] when a value cannot be represented in v2.
pub fn try_v1_to_v2<T, U>(value: T) -> Result<U>
where
    U: TryFrom<T>,
    ProtocolConversionError: From<<U as TryFrom<T>>::Error>,
{
    U::try_from(value).map_err(ProtocolConversionError::from)
}

macro_rules! impl_from_tuple_newtype {
    ($source:path => $target:path) => {
        impl From<$source> for $target {
            fn from(value: $source) -> Self {
                $target(value.0)
            }
        }
    };
}

macro_rules! impl_from_enum {
    ($source:ty => $target:ty { $($variant:ident),+ $(,)? }) => {
        impl From<$source> for $target {
            fn from(value: $source) -> Self {
                match value {
                    $(<$source>::$variant => <$target>::$variant,)+
                }
            }
        }
    };
}

macro_rules! impl_try_from_v2_to_v1 {
    ($source:ty => $target:ty) => {
        impl TryFrom<$source> for $target {
            type Error = ProtocolConversionError;

            fn try_from(value: $source) -> Result<Self> {
                value.try_to_v1()
            }
        }
    };
}

macro_rules! impl_try_from_v1_to_v2 {
    ($source:ty => $target:ty) => {
        impl TryFrom<$source> for $target {
            type Error = ProtocolConversionError;

            fn try_from(value: $source) -> Result<Self> {
                value.try_to_v2()
            }
        }
    };
}

impl_from_tuple_newtype!(super::SessionId => crate::v1::SessionId);
impl_from_tuple_newtype!(crate::v1::SessionId => super::SessionId);
impl_from_tuple_newtype!(super::MessageId => crate::v1::MessageId);
impl_from_tuple_newtype!(crate::v1::MessageId => super::MessageId);
#[cfg(not(feature = "unstable_plan_operations"))]
impl_try_from_v2_to_v1!(super::PlanUpdate => crate::v1::Plan);
#[cfg(feature = "unstable_plan_operations")]
impl_from_tuple_newtype!(super::PlanId => crate::v1::PlanId);
#[cfg(feature = "unstable_plan_operations")]
impl_from_tuple_newtype!(crate::v1::PlanId => super::PlanId);
#[cfg(feature = "unstable_plan_operations")]
impl_try_from_v2_to_v1!(super::PlanUpdate => crate::v1::PlanUpdate);
#[cfg(feature = "unstable_plan_operations")]
impl_try_from_v1_to_v2!(crate::v1::PlanUpdate => super::PlanUpdate);
#[cfg(feature = "unstable_plan_operations")]
impl_try_from_v2_to_v1!(super::PlanUpdateContent => crate::v1::PlanUpdateContent);
#[cfg(feature = "unstable_plan_operations")]
impl_try_from_v1_to_v2!(crate::v1::PlanUpdateContent => super::PlanUpdateContent);
#[cfg(feature = "unstable_plan_operations")]
impl_try_from_v2_to_v1!(super::PlanItems => crate::v1::PlanItems);
#[cfg(feature = "unstable_plan_operations")]
impl_try_from_v1_to_v2!(crate::v1::PlanItems => super::PlanItems);
#[cfg(feature = "unstable_plan_operations")]
impl_try_from_v2_to_v1!(super::PlanFile => crate::v1::PlanFile);
#[cfg(feature = "unstable_plan_operations")]
impl_try_from_v1_to_v2!(crate::v1::PlanFile => super::PlanFile);
#[cfg(feature = "unstable_plan_operations")]
impl_try_from_v2_to_v1!(super::PlanMarkdown => crate::v1::PlanMarkdown);
#[cfg(feature = "unstable_plan_operations")]
impl_try_from_v1_to_v2!(crate::v1::PlanMarkdown => super::PlanMarkdown);
#[cfg(feature = "unstable_plan_operations")]
impl_try_from_v2_to_v1!(super::PlanRemoved => crate::v1::PlanRemoved);
#[cfg(feature = "unstable_plan_operations")]
impl_try_from_v1_to_v2!(crate::v1::PlanRemoved => super::PlanRemoved);
impl_try_from_v2_to_v1!(super::PlanEntry => crate::v1::PlanEntry);
impl_try_from_v1_to_v2!(crate::v1::PlanEntry => super::PlanEntry);
impl_try_from_v2_to_v1!(super::PlanEntryPriority => crate::v1::PlanEntryPriority);
impl_from_enum!(crate::v1::PlanEntryPriority => super::PlanEntryPriority {
    High,
    Medium,
    Low,
});
impl_try_from_v2_to_v1!(super::PlanEntryStatus => crate::v1::PlanEntryStatus);
impl_from_enum!(crate::v1::PlanEntryStatus => super::PlanEntryStatus {
    Pending,
    InProgress,
    Completed,
});
impl_try_from_v2_to_v1!(super::CancelRequestNotification => crate::v1::CancelRequestNotification);
impl_try_from_v1_to_v2!(crate::v1::CancelRequestNotification => super::CancelRequestNotification);
impl_try_from_v2_to_v1!(super::ProtocolLevelNotification => crate::v1::ProtocolLevelNotification);
impl_try_from_v1_to_v2!(crate::v1::ProtocolLevelNotification => super::ProtocolLevelNotification);
impl_try_from_v1_to_v2!(crate::v1::SessionNotification => super::UpdateSessionNotification);
impl_try_from_v1_to_v2!(crate::v1::SessionUpdate => super::SessionUpdate);
impl_try_from_v2_to_v1!(super::ConfigOptionUpdate => crate::v1::ConfigOptionUpdate);
impl_try_from_v1_to_v2!(crate::v1::ConfigOptionUpdate => super::ConfigOptionUpdate);
impl_try_from_v2_to_v1!(super::SessionInfoUpdate => crate::v1::SessionInfoUpdate);
impl_try_from_v1_to_v2!(crate::v1::SessionInfoUpdate => super::SessionInfoUpdate);
impl_try_from_v2_to_v1!(super::UsageUpdate => crate::v1::UsageUpdate);
impl_try_from_v1_to_v2!(crate::v1::UsageUpdate => super::UsageUpdate);
impl_try_from_v2_to_v1!(super::Cost => crate::v1::Cost);
impl_try_from_v1_to_v2!(crate::v1::Cost => super::Cost);
impl_try_from_v2_to_v1!(super::ContentChunk => crate::v1::ContentChunk);
impl_try_from_v1_to_v2!(crate::v1::ContentChunk => super::ContentChunk);
impl_try_from_v2_to_v1!(super::AvailableCommandsUpdate => crate::v1::AvailableCommandsUpdate);
impl_try_from_v1_to_v2!(crate::v1::AvailableCommandsUpdate => super::AvailableCommandsUpdate);
impl_try_from_v2_to_v1!(super::AvailableCommand => crate::v1::AvailableCommand);
impl_try_from_v1_to_v2!(crate::v1::AvailableCommand => super::AvailableCommand);
impl_try_from_v2_to_v1!(super::AvailableCommandInput => crate::v1::AvailableCommandInput);
impl_try_from_v1_to_v2!(crate::v1::AvailableCommandInput => super::AvailableCommandInput);
impl_try_from_v2_to_v1!(super::TextCommandInput => crate::v1::UnstructuredCommandInput);
impl_try_from_v1_to_v2!(crate::v1::UnstructuredCommandInput => super::TextCommandInput);
impl_try_from_v2_to_v1!(super::RequestPermissionRequest => crate::v1::RequestPermissionRequest);
impl_try_from_v1_to_v2!(crate::v1::RequestPermissionRequest => super::RequestPermissionRequest);
impl_try_from_v2_to_v1!(super::PermissionOption => crate::v1::PermissionOption);
impl_try_from_v1_to_v2!(crate::v1::PermissionOption => super::PermissionOption);
impl_from_tuple_newtype!(super::PermissionOptionId => crate::v1::PermissionOptionId);
impl_from_tuple_newtype!(crate::v1::PermissionOptionId => super::PermissionOptionId);
impl_try_from_v2_to_v1!(super::PermissionOptionKind => crate::v1::PermissionOptionKind);
impl_from_enum!(crate::v1::PermissionOptionKind => super::PermissionOptionKind {
    AllowOnce,
    AllowAlways,
    RejectOnce,
    RejectAlways,
});
impl_try_from_v2_to_v1!(super::RequestPermissionResponse => crate::v1::RequestPermissionResponse);
impl_try_from_v1_to_v2!(crate::v1::RequestPermissionResponse => super::RequestPermissionResponse);
impl_try_from_v2_to_v1!(super::RequestPermissionOutcome => crate::v1::RequestPermissionOutcome);
impl_try_from_v1_to_v2!(crate::v1::RequestPermissionOutcome => super::RequestPermissionOutcome);
impl_try_from_v2_to_v1!(super::SelectedPermissionOutcome => crate::v1::SelectedPermissionOutcome);
impl_try_from_v1_to_v2!(crate::v1::SelectedPermissionOutcome => super::SelectedPermissionOutcome);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_try_from_v2_to_v1!(super::ConnectMcpRequest => crate::v1::ConnectMcpRequest);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_try_from_v1_to_v2!(crate::v1::ConnectMcpRequest => super::ConnectMcpRequest);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_try_from_v2_to_v1!(super::ConnectMcpResponse => crate::v1::ConnectMcpResponse);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_try_from_v1_to_v2!(crate::v1::ConnectMcpResponse => super::ConnectMcpResponse);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_try_from_v2_to_v1!(super::MessageMcpRequest => crate::v1::MessageMcpRequest);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_try_from_v1_to_v2!(crate::v1::MessageMcpRequest => super::MessageMcpRequest);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_try_from_v2_to_v1!(super::MessageMcpNotification => crate::v1::MessageMcpNotification);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_try_from_v1_to_v2!(crate::v1::MessageMcpNotification => super::MessageMcpNotification);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_try_from_v2_to_v1!(super::MessageMcpResponse => crate::v1::MessageMcpResponse);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_try_from_v1_to_v2!(crate::v1::MessageMcpResponse => super::MessageMcpResponse);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_try_from_v2_to_v1!(super::DisconnectMcpRequest => crate::v1::DisconnectMcpRequest);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_try_from_v1_to_v2!(crate::v1::DisconnectMcpRequest => super::DisconnectMcpRequest);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_try_from_v2_to_v1!(super::DisconnectMcpResponse => crate::v1::DisconnectMcpResponse);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_try_from_v1_to_v2!(crate::v1::DisconnectMcpResponse => super::DisconnectMcpResponse);
impl_try_from_v2_to_v1!(super::ClientCapabilities => crate::v1::ClientCapabilities);
impl_try_from_v1_to_v2!(crate::v1::ClientCapabilities => super::ClientCapabilities);
#[cfg(feature = "unstable_auth_methods")]
impl_try_from_v2_to_v1!(super::AuthCapabilities => crate::v1::AuthCapabilities);
#[cfg(feature = "unstable_auth_methods")]
impl_try_from_v1_to_v2!(crate::v1::AuthCapabilities => super::AuthCapabilities);
impl_try_from_v2_to_v1!(super::Error => crate::v1::Error);
impl_try_from_v1_to_v2!(crate::v1::Error => super::Error);
impl From<super::ErrorCode> for crate::v1::ErrorCode {
    fn from(value: super::ErrorCode) -> Self {
        i32::from(value).into()
    }
}

impl From<crate::v1::ErrorCode> for super::ErrorCode {
    fn from(value: crate::v1::ErrorCode) -> Self {
        i32::from(value).into()
    }
}

impl_try_from_v2_to_v1!(super::ExtRequest => crate::v1::ExtRequest);
impl_try_from_v1_to_v2!(crate::v1::ExtRequest => super::ExtRequest);
impl_try_from_v2_to_v1!(super::ExtResponse => crate::v1::ExtResponse);
impl_try_from_v1_to_v2!(crate::v1::ExtResponse => super::ExtResponse);
impl_try_from_v2_to_v1!(super::ExtNotification => crate::v1::ExtNotification);
impl_try_from_v1_to_v2!(crate::v1::ExtNotification => super::ExtNotification);
impl_try_from_v2_to_v1!(super::ToolCallUpdate => crate::v1::ToolCallUpdate);
impl_try_from_v1_to_v2!(crate::v1::ToolCall => super::ToolCallUpdate);
impl_try_from_v1_to_v2!(crate::v1::ToolCallUpdate => super::ToolCallUpdate);
impl_from_tuple_newtype!(super::ToolCallId => crate::v1::ToolCallId);
impl_from_tuple_newtype!(crate::v1::ToolCallId => super::ToolCallId);
impl_try_from_v2_to_v1!(super::ToolKind => crate::v1::ToolKind);
impl_from_enum!(crate::v1::ToolKind => super::ToolKind {
    Read,
    Edit,
    Delete,
    Move,
    Search,
    Execute,
    Think,
    Fetch,
    SwitchMode,
    Other,
});
impl_try_from_v2_to_v1!(super::ToolCallStatus => crate::v1::ToolCallStatus);
impl_from_enum!(crate::v1::ToolCallStatus => super::ToolCallStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
});
impl_try_from_v2_to_v1!(super::ToolCallContent => crate::v1::ToolCallContent);
impl_try_from_v1_to_v2!(crate::v1::ToolCallContent => super::ToolCallContent);
impl_try_from_v2_to_v1!(super::Content => crate::v1::Content);
impl_try_from_v1_to_v2!(crate::v1::Content => super::Content);
impl_try_from_v2_to_v1!(super::Diff => crate::v1::Diff);
impl_try_from_v1_to_v2!(crate::v1::Diff => super::Diff);
impl_try_from_v2_to_v1!(super::ToolCallLocation => crate::v1::ToolCallLocation);
impl_try_from_v1_to_v2!(crate::v1::ToolCallLocation => super::ToolCallLocation);
impl_try_from_v2_to_v1!(super::InitializeRequest => crate::v1::InitializeRequest);
impl_try_from_v1_to_v2!(crate::v1::InitializeRequest => super::InitializeRequest);
impl_try_from_v2_to_v1!(super::InitializeResponse => crate::v1::InitializeResponse);
impl_try_from_v1_to_v2!(crate::v1::InitializeResponse => super::InitializeResponse);
impl_try_from_v2_to_v1!(super::Implementation => crate::v1::Implementation);
impl_try_from_v1_to_v2!(crate::v1::Implementation => super::Implementation);
impl_try_from_v2_to_v1!(super::LoginAuthRequest => crate::v1::AuthenticateRequest);
impl_try_from_v1_to_v2!(crate::v1::AuthenticateRequest => super::LoginAuthRequest);
impl_try_from_v2_to_v1!(super::LoginAuthResponse => crate::v1::AuthenticateResponse);
impl_try_from_v1_to_v2!(crate::v1::AuthenticateResponse => super::LoginAuthResponse);
impl_try_from_v2_to_v1!(super::LogoutAuthRequest => crate::v1::LogoutRequest);
impl_try_from_v1_to_v2!(crate::v1::LogoutRequest => super::LogoutAuthRequest);
impl_try_from_v2_to_v1!(super::LogoutAuthResponse => crate::v1::LogoutResponse);
impl_try_from_v1_to_v2!(crate::v1::LogoutResponse => super::LogoutAuthResponse);
impl_try_from_v2_to_v1!(super::AgentAuthCapabilities => crate::v1::AgentAuthCapabilities);
impl_try_from_v1_to_v2!(crate::v1::AgentAuthCapabilities => super::AgentAuthCapabilities);
impl_from_tuple_newtype!(super::AuthMethodId => crate::v1::AuthMethodId);
impl_from_tuple_newtype!(crate::v1::AuthMethodId => super::AuthMethodId);
impl_try_from_v2_to_v1!(super::AuthMethod => crate::v1::AuthMethod);
impl_try_from_v1_to_v2!(crate::v1::AuthMethod => super::AuthMethod);
impl_try_from_v2_to_v1!(super::AuthMethodAgent => crate::v1::AuthMethodAgent);
impl_try_from_v1_to_v2!(crate::v1::AuthMethodAgent => super::AuthMethodAgent);
#[cfg(feature = "unstable_auth_methods")]
impl_try_from_v2_to_v1!(super::AuthMethodEnvVar => crate::v1::AuthMethodEnvVar);
#[cfg(feature = "unstable_auth_methods")]
impl_try_from_v1_to_v2!(crate::v1::AuthMethodEnvVar => super::AuthMethodEnvVar);
#[cfg(feature = "unstable_auth_methods")]
impl_try_from_v2_to_v1!(super::AuthEnvVar => crate::v1::AuthEnvVar);
#[cfg(feature = "unstable_auth_methods")]
impl_try_from_v1_to_v2!(crate::v1::AuthEnvVar => super::AuthEnvVar);
#[cfg(feature = "unstable_auth_methods")]
impl_try_from_v2_to_v1!(super::AuthMethodTerminal => crate::v1::AuthMethodTerminal);
#[cfg(feature = "unstable_auth_methods")]
impl_try_from_v1_to_v2!(crate::v1::AuthMethodTerminal => super::AuthMethodTerminal);
impl_try_from_v2_to_v1!(super::NewSessionRequest => crate::v1::NewSessionRequest);
impl_try_from_v1_to_v2!(crate::v1::NewSessionRequest => super::NewSessionRequest);
impl_try_from_v2_to_v1!(super::NewSessionResponse => crate::v1::NewSessionResponse);
impl_try_from_v1_to_v2!(crate::v1::NewSessionResponse => super::NewSessionResponse);
impl_try_from_v1_to_v2!(crate::v1::LoadSessionRequest => super::ResumeSessionRequest);
impl_try_from_v1_to_v2!(crate::v1::LoadSessionResponse => super::ResumeSessionResponse);
#[cfg(feature = "unstable_session_fork")]
impl_try_from_v2_to_v1!(super::ForkSessionRequest => crate::v1::ForkSessionRequest);
#[cfg(feature = "unstable_session_fork")]
impl_try_from_v1_to_v2!(crate::v1::ForkSessionRequest => super::ForkSessionRequest);
#[cfg(feature = "unstable_session_fork")]
impl_try_from_v2_to_v1!(super::ForkSessionResponse => crate::v1::ForkSessionResponse);
#[cfg(feature = "unstable_session_fork")]
impl_try_from_v1_to_v2!(crate::v1::ForkSessionResponse => super::ForkSessionResponse);
impl_try_from_v2_to_v1!(super::ResumeSessionRequest => crate::v1::ResumeSessionRequest);
impl_try_from_v1_to_v2!(crate::v1::ResumeSessionRequest => super::ResumeSessionRequest);
impl TryFrom<super::ResumeSessionRequest> for crate::v1::LoadSessionRequest {
    type Error = ProtocolConversionError;

    fn try_from(value: super::ResumeSessionRequest) -> Result<Self> {
        v2_resume_session_request_into_v1_load(value)
    }
}
impl_try_from_v2_to_v1!(super::ResumeSessionResponse => crate::v1::ResumeSessionResponse);
impl_try_from_v1_to_v2!(crate::v1::ResumeSessionResponse => super::ResumeSessionResponse);
impl_try_from_v2_to_v1!(super::CloseSessionRequest => crate::v1::CloseSessionRequest);
impl_try_from_v1_to_v2!(crate::v1::CloseSessionRequest => super::CloseSessionRequest);
impl_try_from_v2_to_v1!(super::CloseSessionResponse => crate::v1::CloseSessionResponse);
impl_try_from_v1_to_v2!(crate::v1::CloseSessionResponse => super::CloseSessionResponse);
impl_try_from_v2_to_v1!(super::DeleteSessionRequest => crate::v1::DeleteSessionRequest);
impl_try_from_v1_to_v2!(crate::v1::DeleteSessionRequest => super::DeleteSessionRequest);
impl_try_from_v2_to_v1!(super::DeleteSessionResponse => crate::v1::DeleteSessionResponse);
impl_try_from_v1_to_v2!(crate::v1::DeleteSessionResponse => super::DeleteSessionResponse);
impl_try_from_v2_to_v1!(super::ListSessionsRequest => crate::v1::ListSessionsRequest);
impl_try_from_v1_to_v2!(crate::v1::ListSessionsRequest => super::ListSessionsRequest);
impl_try_from_v2_to_v1!(super::ListSessionsResponse => crate::v1::ListSessionsResponse);
impl_try_from_v1_to_v2!(crate::v1::ListSessionsResponse => super::ListSessionsResponse);
impl_try_from_v2_to_v1!(super::SessionInfo => crate::v1::SessionInfo);
impl_try_from_v1_to_v2!(crate::v1::SessionInfo => super::SessionInfo);
impl_from_tuple_newtype!(super::SessionConfigId => crate::v1::SessionConfigId);
impl_from_tuple_newtype!(crate::v1::SessionConfigId => super::SessionConfigId);
impl_from_tuple_newtype!(super::SessionConfigValueId => crate::v1::SessionConfigValueId);
impl_from_tuple_newtype!(crate::v1::SessionConfigValueId => super::SessionConfigValueId);
impl_from_tuple_newtype!(super::SessionConfigGroupId => crate::v1::SessionConfigGroupId);
impl_from_tuple_newtype!(crate::v1::SessionConfigGroupId => super::SessionConfigGroupId);
impl_try_from_v2_to_v1!(super::SessionConfigSelectOption => crate::v1::SessionConfigSelectOption);
impl_try_from_v1_to_v2!(crate::v1::SessionConfigSelectOption => super::SessionConfigSelectOption);
impl_try_from_v2_to_v1!(super::SessionConfigSelectGroup => crate::v1::SessionConfigSelectGroup);
impl_try_from_v1_to_v2!(crate::v1::SessionConfigSelectGroup => super::SessionConfigSelectGroup);
impl_try_from_v2_to_v1!(super::SessionConfigSelectOptions => crate::v1::SessionConfigSelectOptions);
impl_try_from_v1_to_v2!(crate::v1::SessionConfigSelectOptions => super::SessionConfigSelectOptions);
impl_try_from_v2_to_v1!(super::SessionConfigSelect => crate::v1::SessionConfigSelect);
impl_try_from_v1_to_v2!(crate::v1::SessionConfigSelect => super::SessionConfigSelect);
impl_try_from_v2_to_v1!(super::SessionConfigBoolean => crate::v1::SessionConfigBoolean);
impl_try_from_v1_to_v2!(crate::v1::SessionConfigBoolean => super::SessionConfigBoolean);
impl_try_from_v2_to_v1!(super::SessionConfigOptionCategory => crate::v1::SessionConfigOptionCategory);
impl_try_from_v1_to_v2!(crate::v1::SessionConfigOptionCategory => super::SessionConfigOptionCategory);
impl_try_from_v2_to_v1!(super::SessionConfigKind => crate::v1::SessionConfigKind);
impl_try_from_v1_to_v2!(crate::v1::SessionConfigKind => super::SessionConfigKind);
impl_try_from_v2_to_v1!(super::SessionConfigOption => crate::v1::SessionConfigOption);
impl_try_from_v1_to_v2!(crate::v1::SessionConfigOption => super::SessionConfigOption);
impl_try_from_v2_to_v1!(super::SessionConfigOptionValue => crate::v1::SessionConfigOptionValue);
impl_try_from_v1_to_v2!(crate::v1::SessionConfigOptionValue => super::SessionConfigOptionValue);
impl_try_from_v2_to_v1!(super::SetSessionConfigOptionRequest => crate::v1::SetSessionConfigOptionRequest);
impl_try_from_v1_to_v2!(crate::v1::SetSessionConfigOptionRequest => super::SetSessionConfigOptionRequest);
impl_try_from_v2_to_v1!(super::SetSessionConfigOptionResponse => crate::v1::SetSessionConfigOptionResponse);
impl_try_from_v1_to_v2!(crate::v1::SetSessionConfigOptionResponse => super::SetSessionConfigOptionResponse);
impl_try_from_v2_to_v1!(super::McpServer => crate::v1::McpServer);
impl_try_from_v1_to_v2!(crate::v1::McpServer => super::McpServer);
impl_try_from_v2_to_v1!(super::McpServerHttp => crate::v1::McpServerHttp);
impl_try_from_v1_to_v2!(crate::v1::McpServerHttp => super::McpServerHttp);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_from_tuple_newtype!(super::McpServerAcpId => crate::v1::McpServerAcpId);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_from_tuple_newtype!(crate::v1::McpServerAcpId => super::McpServerAcpId);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_from_tuple_newtype!(super::McpConnectionId => crate::v1::McpConnectionId);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_from_tuple_newtype!(crate::v1::McpConnectionId => super::McpConnectionId);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_try_from_v2_to_v1!(super::McpServerAcp => crate::v1::McpServerAcp);
#[cfg(feature = "unstable_mcp_over_acp")]
impl_try_from_v1_to_v2!(crate::v1::McpServerAcp => super::McpServerAcp);
impl_try_from_v2_to_v1!(super::McpServerStdio => crate::v1::McpServerStdio);
impl_try_from_v1_to_v2!(crate::v1::McpServerStdio => super::McpServerStdio);
impl_try_from_v2_to_v1!(super::EnvVariable => crate::v1::EnvVariable);
impl_try_from_v1_to_v2!(crate::v1::EnvVariable => super::EnvVariable);
impl_try_from_v2_to_v1!(super::HttpHeader => crate::v1::HttpHeader);
impl_try_from_v1_to_v2!(crate::v1::HttpHeader => super::HttpHeader);
impl_try_from_v2_to_v1!(super::PromptRequest => crate::v1::PromptRequest);
impl_try_from_v1_to_v2!(crate::v1::PromptRequest => super::PromptRequest);
impl_try_from_v2_to_v1!(super::PromptResponse => crate::v1::PromptResponse);
impl_try_from_v1_to_v2!(crate::v1::PromptResponse => super::PromptResponse);
impl_try_from_v2_to_v1!(super::StopReason => crate::v1::StopReason);
impl_from_enum!(crate::v1::StopReason => super::StopReason {
    EndTurn,
    MaxTokens,
    MaxTurnRequests,
    Refusal,
    Cancelled,
});
#[cfg(feature = "unstable_end_turn_token_usage")]
impl_try_from_v2_to_v1!(super::Usage => crate::v1::Usage);
#[cfg(feature = "unstable_end_turn_token_usage")]
impl_try_from_v1_to_v2!(crate::v1::Usage => super::Usage);
#[cfg(feature = "unstable_llm_providers")]
impl_try_from_v2_to_v1!(super::LlmProtocol => crate::v1::LlmProtocol);
#[cfg(feature = "unstable_llm_providers")]
impl_try_from_v1_to_v2!(crate::v1::LlmProtocol => super::LlmProtocol);
#[cfg(feature = "unstable_llm_providers")]
impl_try_from_v2_to_v1!(super::ProviderCurrentConfig => crate::v1::ProviderCurrentConfig);
#[cfg(feature = "unstable_llm_providers")]
impl_try_from_v1_to_v2!(crate::v1::ProviderCurrentConfig => super::ProviderCurrentConfig);
#[cfg(feature = "unstable_llm_providers")]
impl_from_tuple_newtype!(super::ProviderId => crate::v1::ProviderId);
#[cfg(feature = "unstable_llm_providers")]
impl_from_tuple_newtype!(crate::v1::ProviderId => super::ProviderId);
#[cfg(feature = "unstable_llm_providers")]
impl_try_from_v2_to_v1!(super::ProviderInfo => crate::v1::ProviderInfo);
#[cfg(feature = "unstable_llm_providers")]
impl_try_from_v1_to_v2!(crate::v1::ProviderInfo => super::ProviderInfo);
#[cfg(feature = "unstable_llm_providers")]
impl_try_from_v2_to_v1!(super::ListProvidersRequest => crate::v1::ListProvidersRequest);
#[cfg(feature = "unstable_llm_providers")]
impl_try_from_v1_to_v2!(crate::v1::ListProvidersRequest => super::ListProvidersRequest);
#[cfg(feature = "unstable_llm_providers")]
impl_try_from_v2_to_v1!(super::ListProvidersResponse => crate::v1::ListProvidersResponse);
#[cfg(feature = "unstable_llm_providers")]
impl_try_from_v1_to_v2!(crate::v1::ListProvidersResponse => super::ListProvidersResponse);
#[cfg(feature = "unstable_llm_providers")]
impl_try_from_v2_to_v1!(super::SetProviderRequest => crate::v1::SetProviderRequest);
#[cfg(feature = "unstable_llm_providers")]
impl_try_from_v1_to_v2!(crate::v1::SetProviderRequest => super::SetProviderRequest);
#[cfg(feature = "unstable_llm_providers")]
impl_try_from_v2_to_v1!(super::SetProviderResponse => crate::v1::SetProviderResponse);
#[cfg(feature = "unstable_llm_providers")]
impl_try_from_v1_to_v2!(crate::v1::SetProviderResponse => super::SetProviderResponse);
#[cfg(feature = "unstable_llm_providers")]
impl_try_from_v2_to_v1!(super::DisableProviderRequest => crate::v1::DisableProviderRequest);
#[cfg(feature = "unstable_llm_providers")]
impl_try_from_v1_to_v2!(crate::v1::DisableProviderRequest => super::DisableProviderRequest);
#[cfg(feature = "unstable_llm_providers")]
impl_try_from_v2_to_v1!(super::DisableProviderResponse => crate::v1::DisableProviderResponse);
#[cfg(feature = "unstable_llm_providers")]
impl_try_from_v1_to_v2!(crate::v1::DisableProviderResponse => super::DisableProviderResponse);
impl_try_from_v2_to_v1!(super::AgentCapabilities => crate::v1::AgentCapabilities);
impl_try_from_v1_to_v2!(crate::v1::AgentCapabilities => super::AgentCapabilities);
#[cfg(feature = "unstable_llm_providers")]
impl_try_from_v2_to_v1!(super::ProvidersCapabilities => crate::v1::ProvidersCapabilities);
#[cfg(feature = "unstable_llm_providers")]
impl_try_from_v1_to_v2!(crate::v1::ProvidersCapabilities => super::ProvidersCapabilities);
impl_try_from_v2_to_v1!(super::SessionDeleteCapabilities => crate::v1::SessionDeleteCapabilities);
impl_try_from_v1_to_v2!(crate::v1::SessionDeleteCapabilities => super::SessionDeleteCapabilities);
impl_try_from_v2_to_v1!(super::SessionAdditionalDirectoriesCapabilities => crate::v1::SessionAdditionalDirectoriesCapabilities);
impl_try_from_v1_to_v2!(crate::v1::SessionAdditionalDirectoriesCapabilities => super::SessionAdditionalDirectoriesCapabilities);
#[cfg(feature = "unstable_session_fork")]
impl_try_from_v2_to_v1!(super::SessionForkCapabilities => crate::v1::SessionForkCapabilities);
#[cfg(feature = "unstable_session_fork")]
impl_try_from_v1_to_v2!(crate::v1::SessionForkCapabilities => super::SessionForkCapabilities);
impl_try_from_v2_to_v1!(super::PromptCapabilities => crate::v1::PromptCapabilities);
impl_try_from_v1_to_v2!(crate::v1::PromptCapabilities => super::PromptCapabilities);
impl_try_from_v2_to_v1!(super::McpCapabilities => crate::v1::McpCapabilities);
impl_try_from_v1_to_v2!(crate::v1::McpCapabilities => super::McpCapabilities);
impl_try_from_v2_to_v1!(super::CancelSessionNotification => crate::v1::CancelNotification);
impl_try_from_v1_to_v2!(crate::v1::CancelNotification => super::CancelSessionNotification);
#[cfg(feature = "unstable_nes")]
impl_from_enum!(super::PositionEncodingKind => crate::v1::PositionEncodingKind {
    Utf16,
    Utf32,
    Utf8,
});
#[cfg(feature = "unstable_nes")]
impl_from_enum!(crate::v1::PositionEncodingKind => super::PositionEncodingKind {
    Utf16,
    Utf32,
    Utf8,
});
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::Position => crate::v1::Position);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::Position => super::Position);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::Range => crate::v1::Range);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::Range => super::Range);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesCapabilities => crate::v1::NesCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesCapabilities => super::NesCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesEventCapabilities => crate::v1::NesEventCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesEventCapabilities => super::NesEventCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesDocumentEventCapabilities => crate::v1::NesDocumentEventCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesDocumentEventCapabilities => super::NesDocumentEventCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesDocumentDidOpenCapabilities => crate::v1::NesDocumentDidOpenCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesDocumentDidOpenCapabilities => super::NesDocumentDidOpenCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesDocumentDidChangeCapabilities => crate::v1::NesDocumentDidChangeCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesDocumentDidChangeCapabilities => super::NesDocumentDidChangeCapabilities);
#[cfg(feature = "unstable_nes")]
impl_from_enum!(super::TextDocumentSyncKind => crate::v1::TextDocumentSyncKind {
    Full,
    Incremental,
});
#[cfg(feature = "unstable_nes")]
impl_from_enum!(crate::v1::TextDocumentSyncKind => super::TextDocumentSyncKind {
    Full,
    Incremental,
});
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesDocumentDidCloseCapabilities => crate::v1::NesDocumentDidCloseCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesDocumentDidCloseCapabilities => super::NesDocumentDidCloseCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesDocumentDidSaveCapabilities => crate::v1::NesDocumentDidSaveCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesDocumentDidSaveCapabilities => super::NesDocumentDidSaveCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesDocumentDidFocusCapabilities => crate::v1::NesDocumentDidFocusCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesDocumentDidFocusCapabilities => super::NesDocumentDidFocusCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesContextCapabilities => crate::v1::NesContextCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesContextCapabilities => super::NesContextCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesRecentFilesCapabilities => crate::v1::NesRecentFilesCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesRecentFilesCapabilities => super::NesRecentFilesCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesRelatedSnippetsCapabilities => crate::v1::NesRelatedSnippetsCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesRelatedSnippetsCapabilities => super::NesRelatedSnippetsCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesEditHistoryCapabilities => crate::v1::NesEditHistoryCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesEditHistoryCapabilities => super::NesEditHistoryCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesUserActionsCapabilities => crate::v1::NesUserActionsCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesUserActionsCapabilities => super::NesUserActionsCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesOpenFilesCapabilities => crate::v1::NesOpenFilesCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesOpenFilesCapabilities => super::NesOpenFilesCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesDiagnosticsCapabilities => crate::v1::NesDiagnosticsCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesDiagnosticsCapabilities => super::NesDiagnosticsCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::ClientNesCapabilities => crate::v1::ClientNesCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::ClientNesCapabilities => super::ClientNesCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesJumpCapabilities => crate::v1::NesJumpCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesJumpCapabilities => super::NesJumpCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesRenameCapabilities => crate::v1::NesRenameCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesRenameCapabilities => super::NesRenameCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesSearchAndReplaceCapabilities => crate::v1::NesSearchAndReplaceCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesSearchAndReplaceCapabilities => super::NesSearchAndReplaceCapabilities);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::DidOpenDocumentNotification => crate::v1::DidOpenDocumentNotification);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::DidOpenDocumentNotification => super::DidOpenDocumentNotification);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::DidChangeDocumentNotification => crate::v1::DidChangeDocumentNotification);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::DidChangeDocumentNotification => super::DidChangeDocumentNotification);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::TextDocumentContentChangeEvent => crate::v1::TextDocumentContentChangeEvent);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::TextDocumentContentChangeEvent => super::TextDocumentContentChangeEvent);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::DidCloseDocumentNotification => crate::v1::DidCloseDocumentNotification);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::DidCloseDocumentNotification => super::DidCloseDocumentNotification);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::DidSaveDocumentNotification => crate::v1::DidSaveDocumentNotification);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::DidSaveDocumentNotification => super::DidSaveDocumentNotification);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::DidFocusDocumentNotification => crate::v1::DidFocusDocumentNotification);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::DidFocusDocumentNotification => super::DidFocusDocumentNotification);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::StartNesRequest => crate::v1::StartNesRequest);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::StartNesRequest => super::StartNesRequest);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::WorkspaceFolder => crate::v1::WorkspaceFolder);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::WorkspaceFolder => super::WorkspaceFolder);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesRepository => crate::v1::NesRepository);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesRepository => super::NesRepository);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::StartNesResponse => crate::v1::StartNesResponse);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::StartNesResponse => super::StartNesResponse);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::CloseNesRequest => crate::v1::CloseNesRequest);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::CloseNesRequest => super::CloseNesRequest);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::CloseNesResponse => crate::v1::CloseNesResponse);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::CloseNesResponse => super::CloseNesResponse);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesTriggerKind => crate::v1::NesTriggerKind);
#[cfg(feature = "unstable_nes")]
impl_from_enum!(crate::v1::NesTriggerKind => super::NesTriggerKind {
    Automatic,
    Diagnostic,
    Manual,
});
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::SuggestNesRequest => crate::v1::SuggestNesRequest);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::SuggestNesRequest => super::SuggestNesRequest);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesSuggestContext => crate::v1::NesSuggestContext);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesSuggestContext => super::NesSuggestContext);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesRecentFile => crate::v1::NesRecentFile);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesRecentFile => super::NesRecentFile);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesRelatedSnippet => crate::v1::NesRelatedSnippet);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesRelatedSnippet => super::NesRelatedSnippet);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesExcerpt => crate::v1::NesExcerpt);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesExcerpt => super::NesExcerpt);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesEditHistoryEntry => crate::v1::NesEditHistoryEntry);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesEditHistoryEntry => super::NesEditHistoryEntry);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesUserAction => crate::v1::NesUserAction);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesUserAction => super::NesUserAction);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesOpenFile => crate::v1::NesOpenFile);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesOpenFile => super::NesOpenFile);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesDiagnostic => crate::v1::NesDiagnostic);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesDiagnostic => super::NesDiagnostic);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesDiagnosticSeverity => crate::v1::NesDiagnosticSeverity);
#[cfg(feature = "unstable_nes")]
impl_from_enum!(crate::v1::NesDiagnosticSeverity => super::NesDiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
});
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::SuggestNesResponse => crate::v1::SuggestNesResponse);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::SuggestNesResponse => super::SuggestNesResponse);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesSuggestion => crate::v1::NesSuggestion);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesSuggestion => super::NesSuggestion);
#[cfg(feature = "unstable_nes")]
impl_from_tuple_newtype!(super::NesSuggestionId => crate::v1::NesSuggestionId);
#[cfg(feature = "unstable_nes")]
impl_from_tuple_newtype!(crate::v1::NesSuggestionId => super::NesSuggestionId);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesEditSuggestion => crate::v1::NesEditSuggestion);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesEditSuggestion => super::NesEditSuggestion);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesTextEdit => crate::v1::NesTextEdit);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesTextEdit => super::NesTextEdit);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesJumpSuggestion => crate::v1::NesJumpSuggestion);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesJumpSuggestion => super::NesJumpSuggestion);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesRenameSuggestion => crate::v1::NesRenameSuggestion);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesRenameSuggestion => super::NesRenameSuggestion);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesSearchAndReplaceSuggestion => crate::v1::NesSearchAndReplaceSuggestion);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::NesSearchAndReplaceSuggestion => super::NesSearchAndReplaceSuggestion);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::AcceptNesNotification => crate::v1::AcceptNesNotification);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::AcceptNesNotification => super::AcceptNesNotification);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::RejectNesNotification => crate::v1::RejectNesNotification);
#[cfg(feature = "unstable_nes")]
impl_try_from_v1_to_v2!(crate::v1::RejectNesNotification => super::RejectNesNotification);
#[cfg(feature = "unstable_nes")]
impl_try_from_v2_to_v1!(super::NesRejectReason => crate::v1::NesRejectReason);
#[cfg(feature = "unstable_nes")]
impl_from_enum!(crate::v1::NesRejectReason => super::NesRejectReason {
    Rejected,
    Ignored,
    Replaced,
    Cancelled,
});
#[cfg(feature = "unstable_elicitation")]
impl_from_tuple_newtype!(super::ElicitationId => crate::v1::ElicitationId);
#[cfg(feature = "unstable_elicitation")]
impl_from_tuple_newtype!(crate::v1::ElicitationId => super::ElicitationId);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::StringFormat => crate::v1::StringFormat);
#[cfg(feature = "unstable_elicitation")]
impl_from_enum!(crate::v1::StringFormat => super::StringFormat {
    Email,
    Uri,
    Date,
    DateTime,
});
#[cfg(feature = "unstable_elicitation")]
impl_from_enum!(super::ElicitationSchemaType => crate::v1::ElicitationSchemaType {
    Object,
});
#[cfg(feature = "unstable_elicitation")]
impl_from_enum!(crate::v1::ElicitationSchemaType => super::ElicitationSchemaType {
    Object,
});
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::EnumOption => crate::v1::EnumOption);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::EnumOption => super::EnumOption);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::StringPropertySchema => crate::v1::StringPropertySchema);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::StringPropertySchema => super::StringPropertySchema);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::NumberPropertySchema => crate::v1::NumberPropertySchema);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::NumberPropertySchema => super::NumberPropertySchema);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::IntegerPropertySchema => crate::v1::IntegerPropertySchema);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::IntegerPropertySchema => super::IntegerPropertySchema);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::BooleanPropertySchema => crate::v1::BooleanPropertySchema);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::BooleanPropertySchema => super::BooleanPropertySchema);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::StringMultiSelectItems => crate::v1::StringMultiSelectItems);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::StringMultiSelectItems => super::StringMultiSelectItems);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::OtherMultiSelectItems => crate::v1::OtherMultiSelectItems);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::OtherMultiSelectItems => super::OtherMultiSelectItems);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::TitledMultiSelectItems => crate::v1::TitledMultiSelectItems);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::TitledMultiSelectItems => super::TitledMultiSelectItems);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::MultiSelectItems => crate::v1::MultiSelectItems);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::MultiSelectItems => super::MultiSelectItems);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::MultiSelectPropertySchema => crate::v1::MultiSelectPropertySchema);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::MultiSelectPropertySchema => super::MultiSelectPropertySchema);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::ElicitationPropertySchema => crate::v1::ElicitationPropertySchema);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::ElicitationPropertySchema => super::ElicitationPropertySchema);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::OtherElicitationPropertySchema => crate::v1::OtherElicitationPropertySchema);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::OtherElicitationPropertySchema => super::OtherElicitationPropertySchema);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::ElicitationSchema => crate::v1::ElicitationSchema);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::ElicitationSchema => super::ElicitationSchema);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::ElicitationCapabilities => crate::v1::ElicitationCapabilities);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::ElicitationCapabilities => super::ElicitationCapabilities);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::ElicitationFormCapabilities => crate::v1::ElicitationFormCapabilities);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::ElicitationFormCapabilities => super::ElicitationFormCapabilities);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::ElicitationUrlCapabilities => crate::v1::ElicitationUrlCapabilities);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::ElicitationUrlCapabilities => super::ElicitationUrlCapabilities);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::ElicitationScope => crate::v1::ElicitationScope);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::ElicitationScope => super::ElicitationScope);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::ElicitationSessionScope => crate::v1::ElicitationSessionScope);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::ElicitationSessionScope => super::ElicitationSessionScope);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::ElicitationRequestScope => crate::v1::ElicitationRequestScope);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::ElicitationRequestScope => super::ElicitationRequestScope);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::CreateElicitationRequest => crate::v1::CreateElicitationRequest);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::CreateElicitationRequest => super::CreateElicitationRequest);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::ElicitationMode => crate::v1::ElicitationMode);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::ElicitationMode => super::ElicitationMode);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::OtherElicitationMode => crate::v1::OtherElicitationMode);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::OtherElicitationMode => super::OtherElicitationMode);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::ElicitationFormMode => crate::v1::ElicitationFormMode);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::ElicitationFormMode => super::ElicitationFormMode);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::ElicitationUrlMode => crate::v1::ElicitationUrlMode);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::ElicitationUrlMode => super::ElicitationUrlMode);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::CreateElicitationResponse => crate::v1::CreateElicitationResponse);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::CreateElicitationResponse => super::CreateElicitationResponse);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::ElicitationAction => crate::v1::ElicitationAction);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::ElicitationAction => super::ElicitationAction);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::OtherElicitationAction => crate::v1::OtherElicitationAction);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::OtherElicitationAction => super::OtherElicitationAction);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::ElicitationAcceptAction => crate::v1::ElicitationAcceptAction);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::ElicitationAcceptAction => super::ElicitationAcceptAction);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::ElicitationContentValue => crate::v1::ElicitationContentValue);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::ElicitationContentValue => super::ElicitationContentValue);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v2_to_v1!(super::CompleteElicitationNotification => crate::v1::CompleteElicitationNotification);
#[cfg(feature = "unstable_elicitation")]
impl_try_from_v1_to_v2!(crate::v1::CompleteElicitationNotification => super::CompleteElicitationNotification);
impl_try_from_v2_to_v1!(super::ContentBlock => crate::v1::ContentBlock);
impl_try_from_v1_to_v2!(crate::v1::ContentBlock => super::ContentBlock);
impl_try_from_v2_to_v1!(super::TextContent => crate::v1::TextContent);
impl_try_from_v1_to_v2!(crate::v1::TextContent => super::TextContent);
impl_try_from_v2_to_v1!(super::ImageContent => crate::v1::ImageContent);
impl_try_from_v1_to_v2!(crate::v1::ImageContent => super::ImageContent);
impl_try_from_v2_to_v1!(super::AudioContent => crate::v1::AudioContent);
impl_try_from_v1_to_v2!(crate::v1::AudioContent => super::AudioContent);
impl_try_from_v2_to_v1!(super::EmbeddedResource => crate::v1::EmbeddedResource);
impl_try_from_v1_to_v2!(crate::v1::EmbeddedResource => super::EmbeddedResource);
impl_try_from_v2_to_v1!(super::EmbeddedResourceResource => crate::v1::EmbeddedResourceResource);
impl_try_from_v1_to_v2!(crate::v1::EmbeddedResourceResource => super::EmbeddedResourceResource);
impl_try_from_v2_to_v1!(super::TextResourceContents => crate::v1::TextResourceContents);
impl_try_from_v1_to_v2!(crate::v1::TextResourceContents => super::TextResourceContents);
impl_try_from_v2_to_v1!(super::BlobResourceContents => crate::v1::BlobResourceContents);
impl_try_from_v1_to_v2!(crate::v1::BlobResourceContents => super::BlobResourceContents);
impl_try_from_v2_to_v1!(super::ResourceLink => crate::v1::ResourceLink);
impl_try_from_v1_to_v2!(crate::v1::ResourceLink => super::ResourceLink);
impl_try_from_v2_to_v1!(super::Annotations => crate::v1::Annotations);
impl_try_from_v1_to_v2!(crate::v1::Annotations => super::Annotations);
impl_try_from_v2_to_v1!(super::Role => crate::v1::Role);
impl_from_enum!(crate::v1::Role => super::Role {
    Assistant,
    User,
});

macro_rules! identity_conversion {
    ($($ty:ty),* $(,)?) => {
        $(
            impl TryToV1 for $ty {
                type Output = Self;

                fn try_to_v1(self) -> Result<Self::Output> {
                    Ok(self)
                }
            }

            impl TryToV2 for $ty {
                type Output = Self;

                fn try_to_v2(self) -> Result<Self::Output> {
                    Ok(self)
                }
            }
        )*
    };
}

identity_conversion!(
    bool,
    f32,
    f64,
    i16,
    i32,
    i64,
    i8,
    isize,
    String,
    u16,
    u32,
    u64,
    u8,
    usize,
    &'static str,
    Arc<RawValue>,
    Arc<str>,
    PathBuf,
    ProtocolVersion,
    super::RequestId,
    serde_json::Map<String, serde_json::Value>,
    serde_json::Value,
);

impl<T> TryToV1 for Option<T>
where
    T: TryToV1,
{
    type Output = Option<T::Output>;
    fn try_to_v1(self) -> Result<Self::Output> {
        self.map(TryToV1::try_to_v1).transpose()
    }
}

impl<T> TryToV2 for Option<T>
where
    T: TryToV2,
{
    type Output = Option<T::Output>;
    fn try_to_v2(self) -> Result<Self::Output> {
        self.map(TryToV2::try_to_v2).transpose()
    }
}

impl<T> TryToV1 for Vec<T>
where
    T: TryToV1,
{
    type Output = Vec<T::Output>;
    fn try_to_v1(self) -> Result<Self::Output> {
        self.into_iter().map(TryToV1::try_to_v1).collect()
    }
}

fn option_vec_into_v2_default<T>(value: Option<Vec<T>>) -> Result<Vec<T::Output>>
where
    T: TryToV2,
{
    value.unwrap_or_default().try_to_v2()
}

impl<T> TryToV2 for Vec<T>
where
    T: TryToV2,
{
    type Output = Vec<T::Output>;
    fn try_to_v2(self) -> Result<Self::Output> {
        self.into_iter().map(TryToV2::try_to_v2).collect()
    }
}

impl<K, V> TryToV1 for BTreeMap<K, V>
where
    K: TryToV1,
    K::Output: Ord,
    V: TryToV1,
{
    type Output = BTreeMap<K::Output, V::Output>;
    fn try_to_v1(self) -> Result<Self::Output> {
        self.into_iter()
            .map(|(key, value)| Ok((key.try_to_v1()?, value.try_to_v1()?)))
            .collect()
    }
}

impl<K, V> TryToV2 for BTreeMap<K, V>
where
    K: TryToV2,
    K::Output: Ord,
    V: TryToV2,
{
    type Output = BTreeMap<K::Output, V::Output>;
    fn try_to_v2(self) -> Result<Self::Output> {
        self.into_iter()
            .map(|(key, value)| Ok((key.try_to_v2()?, value.try_to_v2()?)))
            .collect()
    }
}

impl<K, V, S> TryToV1 for HashMap<K, V, S>
where
    K: TryToV1,
    K::Output: Eq + Hash,
    V: TryToV1,
    S: BuildHasher,
{
    type Output = HashMap<K::Output, V::Output>;
    fn try_to_v1(self) -> Result<Self::Output> {
        self.into_iter()
            .map(|(key, value)| Ok((key.try_to_v1()?, value.try_to_v1()?)))
            .collect()
    }
}

impl<K, V, S> TryToV2 for HashMap<K, V, S>
where
    K: TryToV2,
    K::Output: Eq + Hash,
    V: TryToV2,
    S: BuildHasher,
{
    type Output = HashMap<K::Output, V::Output>;
    fn try_to_v2(self) -> Result<Self::Output> {
        self.into_iter()
            .map(|(key, value)| Ok((key.try_to_v2()?, value.try_to_v2()?)))
            .collect()
    }
}

impl<T> TryToV1 for crate::MaybeUndefined<T>
where
    T: TryToV1,
{
    type Output = crate::MaybeUndefined<T::Output>;
    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Undefined => crate::MaybeUndefined::Undefined,
            Self::Null => crate::MaybeUndefined::Null,
            Self::Value(value) => crate::MaybeUndefined::Value(value.try_to_v1()?),
        })
    }
}

impl<T> TryToV2 for crate::MaybeUndefined<T>
where
    T: TryToV2,
{
    type Output = crate::MaybeUndefined<T::Output>;
    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Undefined => crate::MaybeUndefined::Undefined,
            Self::Null => crate::MaybeUndefined::Null,
            Self::Value(value) => crate::MaybeUndefined::Value(value.try_to_v2()?),
        })
    }
}

impl TryToV1 for super::SessionId {
    type Output = crate::v1::SessionId;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(crate::v1::SessionId(self.0.try_to_v1()?))
    }
}

impl TryToV2 for crate::v1::SessionId {
    type Output = super::SessionId;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(super::SessionId(self.0.try_to_v2()?))
    }
}

impl TryToV1 for super::AbsolutePath {
    type Output = PathBuf;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(self.into_inner())
    }
}

impl TryToV1 for super::SessionListCursor {
    type Output = String;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(self.0.to_string())
    }
}

impl TryToV1 for super::MediaType {
    type Output = String;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(self.0.to_string())
    }
}

impl TryToV1 for super::MessageId {
    type Output = crate::v1::MessageId;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(crate::v1::MessageId(self.0.try_to_v1()?))
    }
}

impl TryToV2 for crate::v1::MessageId {
    type Output = super::MessageId;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(super::MessageId(self.0.try_to_v2()?))
    }
}

#[cfg(not(feature = "unstable_plan_operations"))]
impl TryToV1 for super::PlanUpdate {
    type Output = crate::v1::Plan;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { plan, meta } = self;
        Ok(match plan {
            super::PlanUpdateContent::Items(items) => {
                let super::PlanItems {
                    plan_id,
                    entries,
                    meta: items_meta,
                } = items;
                if plan_id.0.as_ref() != LEGACY_V1_PLAN_ID {
                    return Err(unrepresentable_v2_field("PlanItems", "planId"));
                }
                let meta = match (meta, items_meta) {
                    (Some(update_meta), Some(items_meta)) if update_meta != items_meta => {
                        return Err(ProtocolConversionError::new(
                            "v2 PlanUpdate and PlanItems metadata cannot both be represented in v1 Plan",
                        ));
                    }
                    (Some(meta), _) | (_, Some(meta)) => Some(meta),
                    (None, None) => None,
                };
                crate::v1::Plan {
                    entries: entries.try_to_v1()?,
                    meta: meta.try_to_v1()?,
                }
            }
            super::PlanUpdateContent::Other(value) => {
                return Err(unknown_v2_enum_variant("PlanUpdateContent", &value.type_));
            }
        })
    }
}

#[cfg(feature = "unstable_plan_operations")]
impl TryToV1 for super::PlanId {
    type Output = crate::v1::PlanId;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(crate::v1::PlanId(self.0.try_to_v1()?))
    }
}

#[cfg(feature = "unstable_plan_operations")]
impl TryToV2 for crate::v1::PlanId {
    type Output = super::PlanId;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(super::PlanId(self.0.try_to_v2()?))
    }
}

#[cfg(feature = "unstable_plan_operations")]
impl TryToV1 for super::PlanUpdate {
    type Output = crate::v1::PlanUpdate;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { plan, meta } = self;
        Ok(crate::v1::PlanUpdate {
            plan: plan.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_plan_operations")]
impl TryToV2 for crate::v1::PlanUpdate {
    type Output = super::PlanUpdate;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { plan, meta } = self;
        Ok(super::PlanUpdate {
            plan: plan.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_plan_operations")]
impl TryToV1 for super::PlanUpdateContent {
    type Output = crate::v1::PlanUpdateContent;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Items(value) => crate::v1::PlanUpdateContent::Items(value.try_to_v1()?),
            Self::File(value) => crate::v1::PlanUpdateContent::File(value.try_to_v1()?),
            Self::Markdown(value) => crate::v1::PlanUpdateContent::Markdown(value.try_to_v1()?),
            Self::Other(value) => {
                return Err(unknown_v2_enum_variant("PlanUpdateContent", &value.type_));
            }
        })
    }
}

#[cfg(feature = "unstable_plan_operations")]
impl TryToV2 for crate::v1::PlanUpdateContent {
    type Output = super::PlanUpdateContent;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Items(value) => super::PlanUpdateContent::Items(value.try_to_v2()?),
            Self::File(value) => super::PlanUpdateContent::File(value.try_to_v2()?),
            Self::Markdown(value) => super::PlanUpdateContent::Markdown(value.try_to_v2()?),
        })
    }
}

#[cfg(feature = "unstable_plan_operations")]
impl TryToV1 for super::PlanItems {
    type Output = crate::v1::PlanItems;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            plan_id,
            entries,
            meta,
        } = self;
        Ok(crate::v1::PlanItems {
            plan_id: plan_id.try_to_v1()?,
            entries: entries.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_plan_operations")]
impl TryToV2 for crate::v1::PlanItems {
    type Output = super::PlanItems;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            plan_id,
            entries,
            meta,
        } = self;
        Ok(super::PlanItems {
            plan_id: plan_id.try_to_v2()?,
            entries: entries.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_plan_operations")]
impl TryToV1 for super::PlanFile {
    type Output = crate::v1::PlanFile;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { plan_id, uri, meta } = self;
        Ok(crate::v1::PlanFile {
            plan_id: plan_id.try_to_v1()?,
            uri: uri.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_plan_operations")]
impl TryToV2 for crate::v1::PlanFile {
    type Output = super::PlanFile;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { plan_id, uri, meta } = self;
        Ok(super::PlanFile {
            plan_id: plan_id.try_to_v2()?,
            uri: uri.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_plan_operations")]
impl TryToV1 for super::PlanMarkdown {
    type Output = crate::v1::PlanMarkdown;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            plan_id,
            content,
            meta,
        } = self;
        Ok(crate::v1::PlanMarkdown {
            plan_id: plan_id.try_to_v1()?,
            content: content.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_plan_operations")]
impl TryToV2 for crate::v1::PlanMarkdown {
    type Output = super::PlanMarkdown;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            plan_id,
            content,
            meta,
        } = self;
        Ok(super::PlanMarkdown {
            plan_id: plan_id.try_to_v2()?,
            content: content.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_plan_operations")]
impl TryToV1 for super::PlanRemoved {
    type Output = crate::v1::PlanRemoved;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { plan_id, meta } = self;
        Ok(crate::v1::PlanRemoved {
            plan_id: plan_id.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_plan_operations")]
impl TryToV2 for crate::v1::PlanRemoved {
    type Output = super::PlanRemoved;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { plan_id, meta } = self;
        Ok(super::PlanRemoved {
            plan_id: plan_id.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::PlanEntry {
    type Output = crate::v1::PlanEntry;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            content,
            priority,
            status,
            meta,
        } = self;
        Ok(crate::v1::PlanEntry {
            content: content.try_to_v1()?,
            priority: priority.try_to_v1()?,
            status: status.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::PlanEntry {
    type Output = super::PlanEntry;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            content,
            priority,
            status,
            meta,
        } = self;
        Ok(super::PlanEntry {
            content: content.try_to_v2()?,
            priority: priority.try_to_v2()?,
            status: status.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::PlanEntryPriority {
    type Output = crate::v1::PlanEntryPriority;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::High => crate::v1::PlanEntryPriority::High,
            Self::Medium => crate::v1::PlanEntryPriority::Medium,
            Self::Low => crate::v1::PlanEntryPriority::Low,
            Self::Other(value) => {
                return Err(unknown_v2_enum_variant("PlanEntryPriority", &value));
            }
        })
    }
}

impl TryToV2 for crate::v1::PlanEntryPriority {
    type Output = super::PlanEntryPriority;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::High => super::PlanEntryPriority::High,
            Self::Medium => super::PlanEntryPriority::Medium,
            Self::Low => super::PlanEntryPriority::Low,
        })
    }
}

impl TryToV1 for super::PlanEntryStatus {
    type Output = crate::v1::PlanEntryStatus;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Pending => crate::v1::PlanEntryStatus::Pending,
            Self::InProgress => crate::v1::PlanEntryStatus::InProgress,
            Self::Completed => crate::v1::PlanEntryStatus::Completed,
            Self::Other(value) => return Err(unknown_v2_enum_variant("PlanEntryStatus", &value)),
        })
    }
}

impl TryToV2 for crate::v1::PlanEntryStatus {
    type Output = super::PlanEntryStatus;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Pending => super::PlanEntryStatus::Pending,
            Self::InProgress => super::PlanEntryStatus::InProgress,
            Self::Completed => super::PlanEntryStatus::Completed,
        })
    }
}

impl TryToV1 for super::CancelRequestNotification {
    type Output = crate::v1::CancelRequestNotification;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { request_id, meta } = self;
        Ok(crate::v1::CancelRequestNotification {
            request_id: request_id.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::CancelRequestNotification {
    type Output = super::CancelRequestNotification;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { request_id, meta } = self;
        Ok(super::CancelRequestNotification {
            request_id: request_id.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::ProtocolLevelNotification {
    type Output = crate::v1::ProtocolLevelNotification;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::CancelRequestNotification(value) => {
                crate::v1::ProtocolLevelNotification::CancelRequestNotification(value.try_to_v1()?)
            }
        })
    }
}

impl TryToV2 for crate::v1::ProtocolLevelNotification {
    type Output = super::ProtocolLevelNotification;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::CancelRequestNotification(value) => {
                super::ProtocolLevelNotification::CancelRequestNotification(value.try_to_v2()?)
            }
        })
    }
}

impl TryFrom<super::UpdateSessionNotification> for Vec<crate::v1::SessionNotification> {
    type Error = ProtocolConversionError;

    fn try_from(value: super::UpdateSessionNotification) -> Result<Self> {
        let super::UpdateSessionNotification {
            session_id,
            update,
            meta,
        } = value;
        let session_id = session_id.try_to_v1()?;
        let meta = meta.try_to_v1()?;
        Vec::<crate::v1::SessionUpdate>::try_from(update)?
            .into_iter()
            .map(|update| {
                Ok(crate::v1::SessionNotification {
                    session_id: session_id.clone(),
                    update,
                    meta: meta.clone(),
                })
            })
            .collect()
    }
}

impl TryToV2 for crate::v1::SessionNotification {
    type Output = super::UpdateSessionNotification;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            session_id,
            update,
            meta,
        } = self;
        Ok(super::UpdateSessionNotification {
            session_id: session_id.try_to_v2()?,
            update: update.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryFrom<super::SessionUpdate> for Vec<crate::v1::SessionUpdate> {
    type Error = ProtocolConversionError;

    fn try_from(value: super::SessionUpdate) -> Result<Self> {
        Ok(match value {
            super::SessionUpdate::UserMessageChunk(value) => {
                vec![crate::v1::SessionUpdate::UserMessageChunk(
                    value.try_to_v1()?,
                )]
            }
            super::SessionUpdate::UserMessage(value) => v2_message_update_into_v1_chunks(
                "user_message",
                value.message_id,
                value.content,
                value.meta,
                crate::v1::SessionUpdate::UserMessageChunk,
            )?,
            super::SessionUpdate::AgentMessageChunk(value) => {
                vec![crate::v1::SessionUpdate::AgentMessageChunk(
                    value.try_to_v1()?,
                )]
            }
            super::SessionUpdate::AgentMessage(value) => v2_message_update_into_v1_chunks(
                "agent_message",
                value.message_id,
                value.content,
                value.meta,
                crate::v1::SessionUpdate::AgentMessageChunk,
            )?,
            super::SessionUpdate::AgentThoughtChunk(value) => {
                vec![crate::v1::SessionUpdate::AgentThoughtChunk(
                    value.try_to_v1()?,
                )]
            }
            super::SessionUpdate::AgentThought(value) => v2_message_update_into_v1_chunks(
                "agent_thought",
                value.message_id,
                value.content,
                value.meta,
                crate::v1::SessionUpdate::AgentThoughtChunk,
            )?,
            super::SessionUpdate::StateUpdate(_) => {
                return Err(ProtocolConversionError::new(
                    "v2 SessionUpdate variant `state_update` cannot be represented in v1 because v1 reports completion in the session/prompt response",
                ));
            }
            super::SessionUpdate::ToolCallContentChunk(_) => {
                return Err(ProtocolConversionError::new(
                    "v2 SessionUpdate variant `tool_call_content_chunk` cannot be represented in v1 because v1 tool-call content updates replace content instead of appending",
                ));
            }
            super::SessionUpdate::ToolCallUpdate(value) => {
                vec![crate::v1::SessionUpdate::ToolCallUpdate(value.try_to_v1()?)]
            }
            super::SessionUpdate::TerminalUpdate(_) => {
                return Err(ProtocolConversionError::new(
                    "v2 SessionUpdate variant `terminal_update` cannot be represented in v1",
                ));
            }
            super::SessionUpdate::TerminalOutputChunk(_) => {
                return Err(ProtocolConversionError::new(
                    "v2 SessionUpdate variant `terminal_output_chunk` cannot be represented in v1",
                ));
            }
            #[cfg(feature = "unstable_plan_operations")]
            super::SessionUpdate::PlanUpdate(value) => {
                vec![crate::v1::SessionUpdate::PlanUpdate(value.try_to_v1()?)]
            }
            #[cfg(not(feature = "unstable_plan_operations"))]
            super::SessionUpdate::PlanUpdate(value) => {
                vec![crate::v1::SessionUpdate::Plan(value.try_to_v1()?)]
            }
            #[cfg(feature = "unstable_plan_operations")]
            super::SessionUpdate::PlanRemoved(value) => {
                vec![crate::v1::SessionUpdate::PlanRemoved(value.try_to_v1()?)]
            }
            super::SessionUpdate::AvailableCommandsUpdate(value) => {
                vec![crate::v1::SessionUpdate::AvailableCommandsUpdate(
                    value.try_to_v1()?,
                )]
            }
            super::SessionUpdate::ConfigOptionUpdate(value) => {
                vec![crate::v1::SessionUpdate::ConfigOptionUpdate(
                    value.try_to_v1()?,
                )]
            }
            super::SessionUpdate::SessionInfoUpdate(value) => {
                vec![crate::v1::SessionUpdate::SessionInfoUpdate(
                    value.try_to_v1()?,
                )]
            }
            super::SessionUpdate::UsageUpdate(value) => {
                vec![crate::v1::SessionUpdate::UsageUpdate(value.try_to_v1()?)]
            }
            super::SessionUpdate::Other(value) => {
                return Err(unknown_v2_enum_variant(
                    "SessionUpdate",
                    &value.session_update,
                ));
            }
        })
    }
}

fn v2_message_update_into_v1_chunks(
    variant: &str,
    message_id: super::MessageId,
    content: crate::MaybeUndefined<Vec<super::ContentBlock>>,
    meta: crate::MaybeUndefined<super::Meta>,
    wrap: impl Fn(crate::v1::ContentChunk) -> crate::v1::SessionUpdate,
) -> Result<Vec<crate::v1::SessionUpdate>> {
    let content = match content {
        crate::MaybeUndefined::Value(content) if !content.is_empty() => content,
        crate::MaybeUndefined::Value(_) => {
            return Err(ProtocolConversionError::new(format!(
                "v2 SessionUpdate variant `{variant}` with empty content cannot be represented in v1 chunks"
            )));
        }
        crate::MaybeUndefined::Null => {
            return Err(ProtocolConversionError::new(format!(
                "v2 SessionUpdate variant `{variant}` with null content cannot be represented in v1 chunks"
            )));
        }
        crate::MaybeUndefined::Undefined => {
            return Err(ProtocolConversionError::new(format!(
                "v2 SessionUpdate variant `{variant}` without content cannot be represented in v1 chunks"
            )));
        }
    };
    let message_id = message_id.try_to_v1()?;
    let meta = match meta {
        crate::MaybeUndefined::Value(meta) => Some(meta.try_to_v1()?),
        crate::MaybeUndefined::Null => {
            return Err(ProtocolConversionError::new(format!(
                "v2 SessionUpdate variant `{variant}` with null _meta cannot be represented in v1 chunks"
            )));
        }
        crate::MaybeUndefined::Undefined => None,
    };

    content
        .into_iter()
        .map(|content| {
            Ok(wrap(crate::v1::ContentChunk {
                content: content.try_to_v1()?,
                message_id: Some(message_id.clone()),
                meta: meta.clone(),
            }))
        })
        .collect()
}

impl TryToV2 for crate::v1::SessionUpdate {
    type Output = super::SessionUpdate;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::UserMessageChunk(value) => {
                super::SessionUpdate::UserMessageChunk(value.try_to_v2()?)
            }
            Self::AgentMessageChunk(value) => {
                super::SessionUpdate::AgentMessageChunk(value.try_to_v2()?)
            }
            Self::AgentThoughtChunk(value) => {
                super::SessionUpdate::AgentThoughtChunk(value.try_to_v2()?)
            }
            Self::ToolCall(value) => super::SessionUpdate::ToolCallUpdate(value.try_to_v2()?),
            Self::ToolCallUpdate(value) => super::SessionUpdate::ToolCallUpdate(value.try_to_v2()?),
            Self::Plan(value) => {
                let crate::v1::Plan { entries, meta } = value;
                super::SessionUpdate::PlanUpdate(super::PlanUpdate {
                    plan: super::PlanUpdateContent::items(LEGACY_V1_PLAN_ID, entries.try_to_v2()?),
                    meta: meta.try_to_v2()?,
                })
            }
            #[cfg(feature = "unstable_plan_operations")]
            Self::PlanUpdate(value) => super::SessionUpdate::PlanUpdate(value.try_to_v2()?),
            #[cfg(feature = "unstable_plan_operations")]
            Self::PlanRemoved(value) => super::SessionUpdate::PlanRemoved(value.try_to_v2()?),
            Self::AvailableCommandsUpdate(value) => {
                super::SessionUpdate::AvailableCommandsUpdate(value.try_to_v2()?)
            }
            Self::CurrentModeUpdate(_) => {
                return Err(removed_v1_enum_variant(
                    "SessionUpdate",
                    "current_mode_update",
                ));
            }
            Self::ConfigOptionUpdate(value) => {
                super::SessionUpdate::ConfigOptionUpdate(value.try_to_v2()?)
            }
            Self::SessionInfoUpdate(value) => {
                super::SessionUpdate::SessionInfoUpdate(value.try_to_v2()?)
            }
            Self::UsageUpdate(value) => super::SessionUpdate::UsageUpdate(value.try_to_v2()?),
        })
    }
}

impl TryToV1 for super::ConfigOptionUpdate {
    type Output = crate::v1::ConfigOptionUpdate;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            config_options,
            meta,
        } = self;
        Ok(crate::v1::ConfigOptionUpdate {
            config_options: config_options.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::ConfigOptionUpdate {
    type Output = super::ConfigOptionUpdate;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            config_options,
            meta,
        } = self;
        Ok(super::ConfigOptionUpdate {
            config_options: config_options.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::SessionInfoUpdate {
    type Output = crate::v1::SessionInfoUpdate;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            title,
            updated_at,
            meta,
        } = self;
        Ok(crate::v1::SessionInfoUpdate {
            title: title.try_to_v1()?,
            updated_at: updated_at.try_to_v1()?,
            meta: maybe_undefined_meta_into_v1_option("SessionInfoUpdate", meta)?,
        })
    }
}

impl TryToV2 for crate::v1::SessionInfoUpdate {
    type Output = super::SessionInfoUpdate;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            title,
            updated_at,
            meta,
        } = self;
        Ok(super::SessionInfoUpdate {
            title: title.try_to_v2()?,
            updated_at: updated_at.try_to_v2()?,
            meta: option_into_v2_maybe_undefined(meta)?,
        })
    }
}

impl TryToV1 for super::UsageUpdate {
    type Output = crate::v1::UsageUpdate;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            used,
            size,
            cost,
            meta,
        } = self;
        Ok(crate::v1::UsageUpdate {
            used: used.try_to_v1()?,
            size: size.try_to_v1()?,
            cost: cost.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::UsageUpdate {
    type Output = super::UsageUpdate;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            used,
            size,
            cost,
            meta,
        } = self;
        Ok(super::UsageUpdate {
            used: used.try_to_v2()?,
            size: size.try_to_v2()?,
            cost: cost.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::Cost {
    type Output = crate::v1::Cost;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            amount,
            currency,
            meta,
        } = self;
        Ok(crate::v1::Cost {
            amount: amount.try_to_v1()?,
            currency: currency.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::Cost {
    type Output = super::Cost;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            amount,
            currency,
            meta,
        } = self;
        Ok(super::Cost {
            amount: amount.try_to_v2()?,
            currency: currency.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::ContentChunk {
    type Output = crate::v1::ContentChunk;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            content,
            message_id,
            meta,
        } = self;
        Ok(crate::v1::ContentChunk {
            content: content.try_to_v1()?,
            message_id: Some(message_id.try_to_v1()?),
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::ContentChunk {
    type Output = super::ContentChunk;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            content,
            message_id,
            meta,
        } = self;
        Ok(super::ContentChunk {
            content: content.try_to_v2()?,
            message_id: message_id
                .ok_or_else(|| {
                    ProtocolConversionError::new(
                        "v1 ContentChunk without messageId cannot be represented in v2",
                    )
                })?
                .try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::AvailableCommandsUpdate {
    type Output = crate::v1::AvailableCommandsUpdate;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            available_commands,
            meta,
        } = self;
        Ok(crate::v1::AvailableCommandsUpdate {
            available_commands: available_commands.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::AvailableCommandsUpdate {
    type Output = super::AvailableCommandsUpdate;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            available_commands,
            meta,
        } = self;
        Ok(super::AvailableCommandsUpdate {
            available_commands: available_commands.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::AvailableCommand {
    type Output = crate::v1::AvailableCommand;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            name,
            description,
            input,
            meta,
        } = self;
        Ok(crate::v1::AvailableCommand {
            name: name.try_to_v1()?,
            description: description.try_to_v1()?,
            input: input.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::AvailableCommand {
    type Output = super::AvailableCommand;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            name,
            description,
            input,
            meta,
        } = self;
        Ok(super::AvailableCommand {
            name: name.try_to_v2()?,
            description: description.try_to_v2()?,
            input: input.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::AvailableCommandInput {
    type Output = crate::v1::AvailableCommandInput;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Text(value) => crate::v1::AvailableCommandInput::Unstructured(value.try_to_v1()?),
            Self::Other(value) => {
                return Err(unknown_v2_enum_variant(
                    "AvailableCommandInput",
                    &value.type_,
                ));
            }
        })
    }
}

impl TryToV2 for crate::v1::AvailableCommandInput {
    type Output = super::AvailableCommandInput;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Unstructured(value) => super::AvailableCommandInput::Text(value.try_to_v2()?),
        })
    }
}

impl TryToV1 for super::TextCommandInput {
    type Output = crate::v1::UnstructuredCommandInput;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { hint, meta } = self;
        Ok(crate::v1::UnstructuredCommandInput {
            hint: hint.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::UnstructuredCommandInput {
    type Output = super::TextCommandInput;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { hint, meta } = self;
        Ok(super::TextCommandInput {
            hint: hint.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::RequestPermissionRequest {
    type Output = crate::v1::RequestPermissionRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            session_id,
            title,
            description,
            subject,
            options,
            meta,
        } = self;
        if description.is_some() {
            return Err(unrepresentable_v2_field(
                "RequestPermissionRequest",
                "description",
            ));
        }
        let Some(subject) = subject else {
            return Err(ProtocolConversionError::new(
                "v2 RequestPermissionRequest without `subject` cannot be represented in v1",
            ));
        };
        let tool_call = match subject {
            super::RequestPermissionSubject::ToolCall(subject) => {
                let super::ToolCallPermissionSubject { tool_call } = *subject;
                tool_call
            }
            super::RequestPermissionSubject::Command(_) => {
                return Err(ProtocolConversionError::new(
                    "v2 RequestPermissionSubject variant `command` cannot be represented in v1",
                ));
            }
            super::RequestPermissionSubject::Other(subject) => {
                return Err(unknown_v2_enum_variant(
                    "RequestPermissionSubject",
                    &subject.type_,
                ));
            }
        };
        if tool_call.title.value().map(String::as_str) != Some(title.as_str()) {
            return Err(ProtocolConversionError::new(
                "v2 RequestPermissionRequest.title cannot be represented in v1 unless it matches the tool-call title",
            ));
        }
        Ok(crate::v1::RequestPermissionRequest {
            session_id: session_id.try_to_v1()?,
            tool_call: tool_call.try_to_v1()?,
            options: options.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::RequestPermissionRequest {
    type Output = super::RequestPermissionRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            session_id,
            tool_call,
            options,
            meta,
        } = self;
        let Some(title) = tool_call
            .fields
            .title
            .clone()
            .filter(|title| !title.is_empty())
        else {
            return Err(ProtocolConversionError::new(
                "v1 RequestPermissionRequest without a tool-call title cannot be represented in v2",
            ));
        };
        Ok(super::RequestPermissionRequest {
            session_id: session_id.try_to_v2()?,
            title,
            description: None,
            subject: Some(super::RequestPermissionSubject::from(
                tool_call.try_to_v2()?,
            )),
            options: options.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::PermissionOption {
    type Output = crate::v1::PermissionOption;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            option_id,
            name,
            kind,
            meta,
        } = self;
        Ok(crate::v1::PermissionOption {
            option_id: option_id.try_to_v1()?,
            name: name.try_to_v1()?,
            kind: kind.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::PermissionOption {
    type Output = super::PermissionOption;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            option_id,
            name,
            kind,
            meta,
        } = self;
        Ok(super::PermissionOption {
            option_id: option_id.try_to_v2()?,
            name: name.try_to_v2()?,
            kind: kind.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::PermissionOptionId {
    type Output = crate::v1::PermissionOptionId;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(crate::v1::PermissionOptionId(self.0.try_to_v1()?))
    }
}

impl TryToV2 for crate::v1::PermissionOptionId {
    type Output = super::PermissionOptionId;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(super::PermissionOptionId(self.0.try_to_v2()?))
    }
}

impl TryToV1 for super::PermissionOptionKind {
    type Output = crate::v1::PermissionOptionKind;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::AllowOnce => crate::v1::PermissionOptionKind::AllowOnce,
            Self::AllowAlways => crate::v1::PermissionOptionKind::AllowAlways,
            Self::RejectOnce => crate::v1::PermissionOptionKind::RejectOnce,
            Self::RejectAlways => crate::v1::PermissionOptionKind::RejectAlways,
            Self::Other(value) => {
                return Err(unknown_v2_enum_variant("PermissionOptionKind", &value));
            }
        })
    }
}

impl TryToV2 for crate::v1::PermissionOptionKind {
    type Output = super::PermissionOptionKind;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::AllowOnce => super::PermissionOptionKind::AllowOnce,
            Self::AllowAlways => super::PermissionOptionKind::AllowAlways,
            Self::RejectOnce => super::PermissionOptionKind::RejectOnce,
            Self::RejectAlways => super::PermissionOptionKind::RejectAlways,
        })
    }
}

impl TryToV1 for super::RequestPermissionResponse {
    type Output = crate::v1::RequestPermissionResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { outcome, meta } = self;
        Ok(crate::v1::RequestPermissionResponse {
            outcome: outcome.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::RequestPermissionResponse {
    type Output = super::RequestPermissionResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { outcome, meta } = self;
        Ok(super::RequestPermissionResponse {
            outcome: outcome.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::RequestPermissionOutcome {
    type Output = crate::v1::RequestPermissionOutcome;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Cancelled => crate::v1::RequestPermissionOutcome::Cancelled,
            Self::Selected(value) => {
                crate::v1::RequestPermissionOutcome::Selected(value.try_to_v1()?)
            }
            Self::Other(value) => {
                return Err(unknown_v2_enum_variant(
                    "RequestPermissionOutcome",
                    &value.outcome,
                ));
            }
        })
    }
}

impl TryToV2 for crate::v1::RequestPermissionOutcome {
    type Output = super::RequestPermissionOutcome;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Cancelled => super::RequestPermissionOutcome::Cancelled,
            Self::Selected(value) => super::RequestPermissionOutcome::Selected(value.try_to_v2()?),
        })
    }
}

impl TryToV1 for super::SelectedPermissionOutcome {
    type Output = crate::v1::SelectedPermissionOutcome;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { option_id, meta } = self;
        Ok(crate::v1::SelectedPermissionOutcome {
            option_id: option_id.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::SelectedPermissionOutcome {
    type Output = super::SelectedPermissionOutcome;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { option_id, meta } = self;
        Ok(super::SelectedPermissionOutcome {
            option_id: option_id.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl TryToV1 for super::ConnectMcpRequest {
    type Output = crate::v1::ConnectMcpRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { server_id, meta } = self;
        Ok(crate::v1::ConnectMcpRequest {
            server_id: server_id.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl TryToV2 for crate::v1::ConnectMcpRequest {
    type Output = super::ConnectMcpRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { server_id, meta } = self;
        Ok(super::ConnectMcpRequest {
            server_id: server_id.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl TryToV1 for super::ConnectMcpResponse {
    type Output = crate::v1::ConnectMcpResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            connection_id,
            meta,
        } = self;
        Ok(crate::v1::ConnectMcpResponse {
            connection_id: connection_id.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl TryToV2 for crate::v1::ConnectMcpResponse {
    type Output = super::ConnectMcpResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            connection_id,
            meta,
        } = self;
        Ok(super::ConnectMcpResponse {
            connection_id: connection_id.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl TryToV1 for super::MessageMcpRequest {
    type Output = crate::v1::MessageMcpRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            connection_id,
            method,
            params,
            meta,
        } = self;
        Ok(crate::v1::MessageMcpRequest {
            connection_id: connection_id.try_to_v1()?,
            method: method.try_to_v1()?,
            params: params.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl TryToV2 for crate::v1::MessageMcpRequest {
    type Output = super::MessageMcpRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            connection_id,
            method,
            params,
            meta,
        } = self;
        Ok(super::MessageMcpRequest {
            connection_id: connection_id.try_to_v2()?,
            method: method.try_to_v2()?,
            params: params.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl TryToV1 for super::MessageMcpNotification {
    type Output = crate::v1::MessageMcpNotification;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            connection_id,
            method,
            params,
            meta,
        } = self;
        Ok(crate::v1::MessageMcpNotification {
            connection_id: connection_id.try_to_v1()?,
            method: method.try_to_v1()?,
            params: params.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl TryToV2 for crate::v1::MessageMcpNotification {
    type Output = super::MessageMcpNotification;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            connection_id,
            method,
            params,
            meta,
        } = self;
        Ok(super::MessageMcpNotification {
            connection_id: connection_id.try_to_v2()?,
            method: method.try_to_v2()?,
            params: params.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl TryToV1 for super::MessageMcpResponse {
    type Output = crate::v1::MessageMcpResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self(result) = self;
        Ok(crate::v1::MessageMcpResponse::new(result.try_to_v1()?))
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl TryToV2 for crate::v1::MessageMcpResponse {
    type Output = super::MessageMcpResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self(result) = self;
        Ok(super::MessageMcpResponse::new(result.try_to_v2()?))
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl TryToV1 for super::DisconnectMcpRequest {
    type Output = crate::v1::DisconnectMcpRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            connection_id,
            meta,
        } = self;
        Ok(crate::v1::DisconnectMcpRequest {
            connection_id: connection_id.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl TryToV2 for crate::v1::DisconnectMcpRequest {
    type Output = super::DisconnectMcpRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            connection_id,
            meta,
        } = self;
        Ok(super::DisconnectMcpRequest {
            connection_id: connection_id.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl TryToV1 for super::DisconnectMcpResponse {
    type Output = crate::v1::DisconnectMcpResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::DisconnectMcpResponse {
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl TryToV2 for crate::v1::DisconnectMcpResponse {
    type Output = super::DisconnectMcpResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::DisconnectMcpResponse {
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::ClientCapabilities {
    type Output = crate::v1::ClientCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            #[cfg(feature = "unstable_auth_methods")]
            auth,
            #[cfg(feature = "unstable_elicitation")]
            elicitation,
            #[cfg(feature = "unstable_nes")]
            nes,
            #[cfg(feature = "unstable_nes")]
            position_encodings,
            meta,
        } = self;
        Ok(crate::v1::ClientCapabilities {
            fs: crate::v1::FileSystemCapabilities::default(),
            terminal: false,
            session: Some(
                crate::v1::ClientSessionCapabilities::new().config_options(
                    crate::v1::SessionConfigOptionsCapabilities::new()
                        .boolean(crate::v1::BooleanConfigOptionCapabilities::new()),
                ),
            ),
            #[cfg(feature = "unstable_plan_operations")]
            plan: None,
            #[cfg(feature = "unstable_auth_methods")]
            auth: auth
                .map(TryToV1::try_to_v1)
                .transpose()?
                .unwrap_or_default(),
            #[cfg(feature = "unstable_elicitation")]
            elicitation: elicitation.try_to_v1()?,
            #[cfg(feature = "unstable_nes")]
            nes: nes.try_to_v1()?,
            #[cfg(feature = "unstable_nes")]
            position_encodings: position_encodings.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::ClientCapabilities {
    type Output = super::ClientCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            fs,
            terminal,
            session,
            #[cfg(feature = "unstable_plan_operations")]
            plan,
            #[cfg(feature = "unstable_auth_methods")]
            auth,
            #[cfg(feature = "unstable_elicitation")]
            elicitation,
            #[cfg(feature = "unstable_nes")]
            nes,
            #[cfg(feature = "unstable_nes")]
            position_encodings,
            meta,
        } = self;
        if fs != crate::v1::FileSystemCapabilities::default() {
            return Err(unrepresentable_v1_field("ClientCapabilities", "fs"));
        }
        if terminal {
            return Err(unrepresentable_v1_field("ClientCapabilities", "terminal"));
        }
        v1_client_session_capabilities_representable_in_v2(session)?;
        #[cfg(feature = "unstable_plan_operations")]
        if plan.is_some() {
            return Err(unrepresentable_v1_field("ClientCapabilities", "plan"));
        }

        Ok(super::ClientCapabilities {
            #[cfg(feature = "unstable_auth_methods")]
            auth: v1_client_auth_capabilities_into_v2_option(auth)?,
            #[cfg(feature = "unstable_elicitation")]
            elicitation: elicitation.try_to_v2()?,
            #[cfg(feature = "unstable_nes")]
            nes: nes.try_to_v2()?,
            #[cfg(feature = "unstable_nes")]
            position_encodings: position_encodings.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

fn v1_client_session_capabilities_representable_in_v2(
    session: Option<crate::v1::ClientSessionCapabilities>,
) -> Result<()> {
    let Some(session) = session else {
        return Ok(());
    };
    let crate::v1::ClientSessionCapabilities {
        config_options,
        meta,
    } = session;
    reject_v1_marker_meta("ClientCapabilities", "session", meta.as_ref())?;

    let Some(config_options) = config_options else {
        return Ok(());
    };
    let crate::v1::SessionConfigOptionsCapabilities { boolean, meta } = config_options;
    reject_v1_marker_meta("ClientCapabilities.session", "configOptions", meta.as_ref())?;

    if let Some(boolean) = boolean {
        reject_v1_marker_meta(
            "ClientCapabilities.session.configOptions",
            "boolean",
            boolean.meta.as_ref(),
        )?;
    }

    Ok(())
}

#[cfg(feature = "unstable_auth_methods")]
fn v1_client_auth_capabilities_into_v2_option(
    auth: crate::v1::AuthCapabilities,
) -> Result<Option<super::AuthCapabilities>> {
    let crate::v1::AuthCapabilities { terminal, meta } = auth;
    if !terminal && meta.is_none() {
        return Ok(None);
    }
    Ok(Some(super::AuthCapabilities {
        terminal: terminal.then(super::TerminalAuthCapabilities::new),
        meta: meta.try_to_v2()?,
    }))
}

#[cfg(feature = "unstable_auth_methods")]
impl TryToV1 for super::AuthCapabilities {
    type Output = crate::v1::AuthCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { terminal, meta } = self;
        if let Some(terminal) = &terminal {
            reject_v2_marker_meta("AuthCapabilities", "terminal", terminal.meta.as_ref())?;
        }
        Ok(crate::v1::AuthCapabilities {
            terminal: terminal.is_some(),
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_auth_methods")]
impl TryToV2 for crate::v1::AuthCapabilities {
    type Output = super::AuthCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { terminal, meta } = self;
        Ok(super::AuthCapabilities {
            terminal: terminal.then(super::TerminalAuthCapabilities::new),
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::Error {
    type Output = crate::v1::Error;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            code,
            message,
            data,
        } = self;
        Ok(crate::v1::Error {
            code: code.try_to_v1()?,
            message: message.try_to_v1()?,
            data: data.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::Error {
    type Output = super::Error;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            code,
            message,
            data,
        } = self;
        Ok(super::Error {
            code: code.try_to_v2()?,
            message: message.try_to_v2()?,
            data: data.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::ErrorCode {
    type Output = crate::v1::ErrorCode;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(i32::from(self).into())
    }
}

impl TryToV2 for crate::v1::ErrorCode {
    type Output = super::ErrorCode;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(i32::from(self).into())
    }
}

impl TryToV1 for super::ExtRequest {
    type Output = crate::v1::ExtRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { method, params } = self;
        Ok(crate::v1::ExtRequest {
            method: method.try_to_v1()?,
            params: params.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::ExtRequest {
    type Output = super::ExtRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { method, params } = self;
        Ok(super::ExtRequest {
            method: method.try_to_v2()?,
            params: params.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::ExtResponse {
    type Output = crate::v1::ExtResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(crate::v1::ExtResponse(self.0.try_to_v1()?))
    }
}

impl TryToV2 for crate::v1::ExtResponse {
    type Output = super::ExtResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(super::ExtResponse(self.0.try_to_v2()?))
    }
}

impl TryToV1 for super::ExtNotification {
    type Output = crate::v1::ExtNotification;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { method, params } = self;
        Ok(crate::v1::ExtNotification {
            method: method.try_to_v1()?,
            params: params.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::ExtNotification {
    type Output = super::ExtNotification;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { method, params } = self;
        Ok(super::ExtNotification {
            method: method.try_to_v2()?,
            params: params.try_to_v2()?,
        })
    }
}

fn maybe_undefined_value_into_v1_option<T>(
    context: &str,
    value: crate::MaybeUndefined<T>,
) -> Result<Option<T::Output>>
where
    T: TryToV1,
{
    match value {
        crate::MaybeUndefined::Value(value) => Ok(Some(value.try_to_v1()?)),
        crate::MaybeUndefined::Null => Err(ProtocolConversionError::new(format!(
            "v2 {context} with null value cannot be represented in v1"
        ))),
        crate::MaybeUndefined::Undefined => Ok(None),
    }
}

fn maybe_undefined_vec_into_v1_option<T>(
    context: &str,
    value: crate::MaybeUndefined<Vec<T>>,
) -> Result<Option<Vec<T::Output>>>
where
    T: TryToV1,
{
    match value {
        crate::MaybeUndefined::Value(value) => Ok(Some(value.try_to_v1()?)),
        crate::MaybeUndefined::Null => Err(ProtocolConversionError::new(format!(
            "v2 {context} with null collection cannot be represented in v1"
        ))),
        crate::MaybeUndefined::Undefined => Ok(None),
    }
}

fn maybe_undefined_meta_into_v1_option(
    context: &str,
    value: crate::MaybeUndefined<super::Meta>,
) -> Result<Option<crate::v1::Meta>> {
    match value {
        crate::MaybeUndefined::Value(value) => Ok(Some(value.try_to_v1()?)),
        crate::MaybeUndefined::Null => Err(ProtocolConversionError::new(format!(
            "v2 {context} with null _meta cannot be represented in v1"
        ))),
        crate::MaybeUndefined::Undefined => Ok(None),
    }
}

fn option_into_v2_maybe_undefined<T>(value: Option<T>) -> Result<crate::MaybeUndefined<T::Output>>
where
    T: TryToV2,
{
    match value {
        Some(value) => Ok(crate::MaybeUndefined::Value(value.try_to_v2()?)),
        None => Ok(crate::MaybeUndefined::Undefined),
    }
}

fn option_vec_into_v2_maybe_undefined<T>(
    value: Option<Vec<T>>,
) -> Result<crate::MaybeUndefined<Vec<T::Output>>>
where
    T: TryToV2,
{
    match value {
        Some(value) => Ok(crate::MaybeUndefined::Value(value.try_to_v2()?)),
        None => Ok(crate::MaybeUndefined::Undefined),
    }
}

fn vec_into_v2_maybe_undefined<T>(value: Vec<T>) -> Result<crate::MaybeUndefined<Vec<T::Output>>>
where
    T: TryToV2,
{
    if value.is_empty() {
        Ok(crate::MaybeUndefined::Undefined)
    } else {
        Ok(crate::MaybeUndefined::Value(value.try_to_v2()?))
    }
}

impl TryToV1 for super::ToolCallUpdate {
    type Output = crate::v1::ToolCallUpdate;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            tool_call_id,
            title,
            kind,
            status,
            content,
            locations,
            raw_input,
            raw_output,
            meta,
        } = self;
        Ok(crate::v1::ToolCallUpdate {
            tool_call_id: tool_call_id.try_to_v1()?,
            fields: crate::v1::ToolCallUpdateFields {
                kind: maybe_undefined_value_into_v1_option("ToolCallUpdate.kind", kind)?,
                status: maybe_undefined_value_into_v1_option("ToolCallUpdate.status", status)?,
                title: maybe_undefined_value_into_v1_option("ToolCallUpdate.title", title)?,
                content: maybe_undefined_vec_into_v1_option("ToolCallUpdate.content", content)?,
                locations: maybe_undefined_vec_into_v1_option(
                    "ToolCallUpdate.locations",
                    locations,
                )?,
                raw_input: maybe_undefined_value_into_v1_option(
                    "ToolCallUpdate.rawInput",
                    raw_input,
                )?,
                raw_output: maybe_undefined_value_into_v1_option(
                    "ToolCallUpdate.rawOutput",
                    raw_output,
                )?,
            },
            meta: maybe_undefined_meta_into_v1_option("ToolCallUpdate", meta)?,
        })
    }
}

impl TryToV2 for crate::v1::ToolCall {
    type Output = super::ToolCallUpdate;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            tool_call_id,
            title,
            kind,
            status,
            content,
            locations,
            raw_input,
            raw_output,
            meta,
        } = self;
        Ok(super::ToolCallUpdate {
            tool_call_id: tool_call_id.try_to_v2()?,
            title: crate::MaybeUndefined::Value(title.try_to_v2()?),
            kind: if matches!(kind, crate::v1::ToolKind::Other) {
                crate::MaybeUndefined::Undefined
            } else {
                crate::MaybeUndefined::Value(kind.try_to_v2()?)
            },
            status: if matches!(status, crate::v1::ToolCallStatus::Pending) {
                crate::MaybeUndefined::Undefined
            } else {
                crate::MaybeUndefined::Value(status.try_to_v2()?)
            },
            content: vec_into_v2_maybe_undefined(content)?,
            locations: vec_into_v2_maybe_undefined(locations)?,
            raw_input: option_into_v2_maybe_undefined(raw_input)?,
            raw_output: option_into_v2_maybe_undefined(raw_output)?,
            meta: option_into_v2_maybe_undefined(meta)?,
        })
    }
}

impl TryToV2 for crate::v1::ToolCallUpdate {
    type Output = super::ToolCallUpdate;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            tool_call_id,
            fields,
            meta,
        } = self;
        let crate::v1::ToolCallUpdateFields {
            kind,
            status,
            title,
            content,
            locations,
            raw_input,
            raw_output,
        } = fields;
        Ok(super::ToolCallUpdate {
            tool_call_id: tool_call_id.try_to_v2()?,
            kind: option_into_v2_maybe_undefined(kind)?,
            status: option_into_v2_maybe_undefined(status)?,
            title: option_into_v2_maybe_undefined(title)?,
            content: option_vec_into_v2_maybe_undefined(content)?,
            locations: option_vec_into_v2_maybe_undefined(locations)?,
            raw_input: option_into_v2_maybe_undefined(raw_input)?,
            raw_output: option_into_v2_maybe_undefined(raw_output)?,
            meta: option_into_v2_maybe_undefined(meta)?,
        })
    }
}

impl TryToV1 for super::ToolCallId {
    type Output = crate::v1::ToolCallId;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(crate::v1::ToolCallId(self.0.try_to_v1()?))
    }
}

impl TryToV2 for crate::v1::ToolCallId {
    type Output = super::ToolCallId;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(super::ToolCallId(self.0.try_to_v2()?))
    }
}

impl TryToV1 for super::ToolKind {
    type Output = crate::v1::ToolKind;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Read => crate::v1::ToolKind::Read,
            Self::Edit => crate::v1::ToolKind::Edit,
            Self::Delete => crate::v1::ToolKind::Delete,
            Self::Move => crate::v1::ToolKind::Move,
            Self::Search => crate::v1::ToolKind::Search,
            Self::Execute => crate::v1::ToolKind::Execute,
            Self::Think => crate::v1::ToolKind::Think,
            Self::Fetch => crate::v1::ToolKind::Fetch,
            Self::SwitchMode => crate::v1::ToolKind::SwitchMode,
            Self::Other => crate::v1::ToolKind::Other,
            Self::Unknown(value) => return Err(unknown_v2_enum_variant("ToolKind", &value)),
        })
    }
}

impl TryToV2 for crate::v1::ToolKind {
    type Output = super::ToolKind;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Read => super::ToolKind::Read,
            Self::Edit => super::ToolKind::Edit,
            Self::Delete => super::ToolKind::Delete,
            Self::Move => super::ToolKind::Move,
            Self::Search => super::ToolKind::Search,
            Self::Execute => super::ToolKind::Execute,
            Self::Think => super::ToolKind::Think,
            Self::Fetch => super::ToolKind::Fetch,
            Self::SwitchMode => super::ToolKind::SwitchMode,
            Self::Other => super::ToolKind::Other,
        })
    }
}

impl TryToV1 for super::ToolCallStatus {
    type Output = crate::v1::ToolCallStatus;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Pending => crate::v1::ToolCallStatus::Pending,
            Self::InProgress => crate::v1::ToolCallStatus::InProgress,
            Self::Completed => crate::v1::ToolCallStatus::Completed,
            Self::Failed => crate::v1::ToolCallStatus::Failed,
            Self::Other(value) => return Err(unknown_v2_enum_variant("ToolCallStatus", &value)),
        })
    }
}

impl TryToV2 for crate::v1::ToolCallStatus {
    type Output = super::ToolCallStatus;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Pending => super::ToolCallStatus::Pending,
            Self::InProgress => super::ToolCallStatus::InProgress,
            Self::Completed => super::ToolCallStatus::Completed,
            Self::Failed => super::ToolCallStatus::Failed,
        })
    }
}

impl TryToV1 for super::ToolCallContent {
    type Output = crate::v1::ToolCallContent;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Content(value) => crate::v1::ToolCallContent::Content(value.try_to_v1()?),
            Self::Diff(value) => crate::v1::ToolCallContent::Diff(value.try_to_v1()?),
            Self::Terminal(_) => {
                return Err(ProtocolConversionError::new(
                    "v2 ToolCallContent variant `terminal` cannot be represented in v1 because v1 terminal content refers to a client-created terminal",
                ));
            }
            Self::Other(value) => {
                return Err(unknown_v2_enum_variant("ToolCallContent", &value.type_));
            }
        })
    }
}

impl TryToV2 for crate::v1::ToolCallContent {
    type Output = super::ToolCallContent;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Content(value) => super::ToolCallContent::Content(Box::new(value.try_to_v2()?)),
            Self::Diff(value) => super::ToolCallContent::Diff(value.try_to_v2()?),
            Self::Terminal(_) => {
                return Err(removed_v1_enum_variant("ToolCallContent", "terminal"));
            }
        })
    }
}

impl TryToV1 for super::Content {
    type Output = crate::v1::Content;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { content, meta } = self;
        Ok(crate::v1::Content {
            content: content.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::Content {
    type Output = super::Content;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { content, meta } = self;
        Ok(super::Content {
            content: content.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::Diff {
    type Output = crate::v1::Diff;

    fn try_to_v1(self) -> Result<Self::Output> {
        Err(ProtocolConversionError::new(
            "v2 Diff cannot be represented in v1 because v1 requires oldText/newText while v2 carries Git --patch text and structured changes",
        ))
    }
}

impl TryToV2 for crate::v1::Diff {
    type Output = super::Diff;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            path,
            old_text,
            new_text,
            meta,
        } = self;
        let path = path.try_to_v2()?;
        let old_text = old_text.try_to_v2()?;
        let new_text = new_text.try_to_v2()?;
        let change = if old_text.is_some() {
            super::DiffChange::modify(path.clone()).file_type(super::DiffFileType::Text)
        } else {
            super::DiffChange::add(path.clone()).file_type(super::DiffFileType::Text)
        };
        let patch_text = full_file_git_patch(&path, old_text.as_deref(), &new_text);

        Ok(super::Diff::patch(patch_text, vec![change]).meta(meta.try_to_v2()?))
    }
}

fn full_file_git_patch(path: &Path, old_text: Option<&str>, new_text: &str) -> String {
    let path = path.to_string_lossy();
    let old = old_text.unwrap_or_default();
    let original_filename = if old_text.is_some() {
        path.to_string()
    } else {
        "/dev/null".to_string()
    };

    let mut options = diffy::DiffOptions::new();
    options
        .set_original_filename(original_filename)
        .set_modified_filename(path.to_string());

    let mut patch_text = format!("diff --git {path} {path}\n");
    if old_text.is_none() {
        patch_text.push_str("new file mode 100644\n");
    }
    patch_text.push_str(&options.create_patch(old, new_text).to_string());
    patch_text
}

impl TryToV1 for super::ToolCallLocation {
    type Output = crate::v1::ToolCallLocation;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { path, line, meta } = self;
        Ok(crate::v1::ToolCallLocation {
            path: path.try_to_v1()?,
            line: line.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::ToolCallLocation {
    type Output = super::ToolCallLocation;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { path, line, meta } = self;
        Ok(super::ToolCallLocation {
            path: super::AbsolutePath::new(path),
            line: line.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::InitializeRequest {
    type Output = crate::v1::InitializeRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            protocol_version,
            capabilities,
            info,
            meta,
        } = self;
        Ok(crate::v1::InitializeRequest {
            protocol_version: protocol_version.try_to_v1()?,
            client_capabilities: capabilities.try_to_v1()?,
            client_info: Some(info.try_to_v1()?),
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::InitializeRequest {
    type Output = super::InitializeRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            protocol_version,
            client_capabilities,
            client_info,
            meta,
        } = self;
        let info = match client_info {
            Some(client_info) => client_info.try_to_v2()?,
            None => {
                return Err(ProtocolConversionError::new(
                    "v1 InitializeRequest without `clientInfo` cannot be represented in v2",
                ));
            }
        };
        Ok(super::InitializeRequest {
            protocol_version: protocol_version.try_to_v2()?,
            capabilities: client_capabilities.try_to_v2()?,
            info,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::InitializeResponse {
    type Output = crate::v1::InitializeResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            protocol_version,
            capabilities: agent_capabilities,
            auth_methods,
            info,
            meta,
        } = self;
        let advertises_auth = !auth_methods.is_empty();
        let mut agent_capabilities = agent_capabilities.try_to_v1()?;
        if advertises_auth {
            agent_capabilities.auth.logout = Some(crate::v1::LogoutCapabilities::new());
        }
        Ok(crate::v1::InitializeResponse {
            protocol_version: protocol_version.try_to_v1()?,
            agent_capabilities,
            auth_methods: auth_methods.try_to_v1()?,
            agent_info: Some(info.try_to_v1()?),
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::InitializeResponse {
    type Output = super::InitializeResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            protocol_version,
            mut agent_capabilities,
            auth_methods,
            agent_info,
            meta,
        } = self;
        let info = match agent_info {
            Some(agent_info) => agent_info.try_to_v2()?,
            None => {
                return Err(ProtocolConversionError::new(
                    "v1 InitializeResponse without `agentInfo` cannot be represented in v2",
                ));
            }
        };
        let advertises_auth = !auth_methods.is_empty();
        match (advertises_auth, agent_capabilities.auth.logout.take()) {
            (true, Some(logout)) => {
                reject_v1_marker_meta(
                    "InitializeResponse.agentCapabilities.auth",
                    "logout",
                    logout.meta.as_ref(),
                )?;
            }
            (true, None) => {
                return Err(ProtocolConversionError::new(
                    "v1 InitializeResponse with non-empty `authMethods` and no \
                     `agentCapabilities.auth.logout` cannot be represented in v2",
                ));
            }
            (false, Some(_)) => {
                return Err(ProtocolConversionError::new(
                    "v1 InitializeResponse with `agentCapabilities.auth.logout` and empty \
                     `authMethods` cannot be represented in v2",
                ));
            }
            (false, None) => {}
        }
        Ok(super::InitializeResponse {
            protocol_version: protocol_version.try_to_v2()?,
            capabilities: agent_capabilities.try_to_v2()?,
            auth_methods: auth_methods.try_to_v2()?,
            info,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::Implementation {
    type Output = crate::v1::Implementation;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            name,
            title,
            version,
            meta,
        } = self;
        Ok(crate::v1::Implementation {
            name: name.try_to_v1()?,
            title: title.try_to_v1()?,
            version: version.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::Implementation {
    type Output = super::Implementation;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            name,
            title,
            version,
            meta,
        } = self;
        Ok(super::Implementation {
            name: name.try_to_v2()?,
            title: title.try_to_v2()?,
            version: version.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::LoginAuthRequest {
    type Output = crate::v1::AuthenticateRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { method_id, meta } = self;
        Ok(crate::v1::AuthenticateRequest {
            method_id: method_id.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::AuthenticateRequest {
    type Output = super::LoginAuthRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { method_id, meta } = self;
        Ok(super::LoginAuthRequest {
            method_id: method_id.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::LoginAuthResponse {
    type Output = crate::v1::AuthenticateResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::AuthenticateResponse {
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::AuthenticateResponse {
    type Output = super::LoginAuthResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::LoginAuthResponse {
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::LogoutAuthRequest {
    type Output = crate::v1::LogoutRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::LogoutRequest {
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::LogoutRequest {
    type Output = super::LogoutAuthRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::LogoutAuthRequest {
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::LogoutAuthResponse {
    type Output = crate::v1::LogoutResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::LogoutResponse {
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::LogoutResponse {
    type Output = super::LogoutAuthResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::LogoutAuthResponse {
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::AgentAuthCapabilities {
    type Output = crate::v1::AgentAuthCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::AgentAuthCapabilities {
            logout: None,
            meta: meta.try_to_v1()?,
        })
    }
}

fn v1_agent_auth_capabilities_into_v2_option(
    auth: crate::v1::AgentAuthCapabilities,
) -> Result<Option<super::AgentAuthCapabilities>> {
    let crate::v1::AgentAuthCapabilities { logout, meta } = auth;
    if logout.is_some() {
        return Err(unrepresentable_v1_field("AgentAuthCapabilities", "logout"));
    }
    if meta.is_none() {
        return Ok(None);
    }
    Ok(Some(super::AgentAuthCapabilities {
        meta: meta.try_to_v2()?,
    }))
}

impl TryToV2 for crate::v1::AgentAuthCapabilities {
    type Output = super::AgentAuthCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { logout, meta } = self;
        if logout.is_some() {
            return Err(unrepresentable_v1_field("AgentAuthCapabilities", "logout"));
        }
        Ok(super::AgentAuthCapabilities {
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::AuthMethodId {
    type Output = crate::v1::AuthMethodId;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(crate::v1::AuthMethodId(self.0.try_to_v1()?))
    }
}

impl TryToV2 for crate::v1::AuthMethodId {
    type Output = super::AuthMethodId;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(super::AuthMethodId(self.0.try_to_v2()?))
    }
}

impl TryToV1 for super::AuthMethod {
    type Output = crate::v1::AuthMethod;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            #[cfg(feature = "unstable_auth_methods")]
            Self::EnvVar(value) => crate::v1::AuthMethod::EnvVar(value.try_to_v1()?),
            #[cfg(feature = "unstable_auth_methods")]
            Self::Terminal(value) => crate::v1::AuthMethod::Terminal(value.try_to_v1()?),
            Self::Other(value) => {
                return Err(unknown_v2_enum_variant("AuthMethod", &value.type_));
            }
            Self::Agent(value) => crate::v1::AuthMethod::Agent(value.try_to_v1()?),
        })
    }
}

impl TryToV2 for crate::v1::AuthMethod {
    type Output = super::AuthMethod;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            #[cfg(feature = "unstable_auth_methods")]
            Self::EnvVar(value) => super::AuthMethod::EnvVar(value.try_to_v2()?),
            #[cfg(feature = "unstable_auth_methods")]
            Self::Terminal(value) => super::AuthMethod::Terminal(value.try_to_v2()?),
            Self::Agent(value) => super::AuthMethod::Agent(value.try_to_v2()?),
        })
    }
}

impl TryToV1 for super::AuthMethodAgent {
    type Output = crate::v1::AuthMethodAgent;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            method_id,
            name,
            description,
            meta,
        } = self;
        Ok(crate::v1::AuthMethodAgent {
            id: method_id.try_to_v1()?,
            name: name.try_to_v1()?,
            description: description.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::AuthMethodAgent {
    type Output = super::AuthMethodAgent;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            id,
            name,
            description,
            meta,
        } = self;
        Ok(super::AuthMethodAgent {
            method_id: id.try_to_v2()?,
            name: name.try_to_v2()?,
            description: description.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_auth_methods")]
impl TryToV1 for super::AuthMethodEnvVar {
    type Output = crate::v1::AuthMethodEnvVar;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            method_id,
            name,
            description,
            vars,
            link,
            meta,
        } = self;
        Ok(crate::v1::AuthMethodEnvVar {
            id: method_id.try_to_v1()?,
            name: name.try_to_v1()?,
            description: description.try_to_v1()?,
            vars: vars.try_to_v1()?,
            link: link.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_auth_methods")]
impl TryToV2 for crate::v1::AuthMethodEnvVar {
    type Output = super::AuthMethodEnvVar;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            id,
            name,
            description,
            vars,
            link,
            meta,
        } = self;
        Ok(super::AuthMethodEnvVar {
            method_id: id.try_to_v2()?,
            name: name.try_to_v2()?,
            description: description.try_to_v2()?,
            vars: vars.try_to_v2()?,
            link: link.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_auth_methods")]
impl TryToV1 for super::AuthEnvVar {
    type Output = crate::v1::AuthEnvVar;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            name,
            label,
            secret,
            optional,
            meta,
        } = self;
        Ok(crate::v1::AuthEnvVar {
            name: name.try_to_v1()?,
            label: label.try_to_v1()?,
            secret: secret.try_to_v1()?,
            optional: optional.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_auth_methods")]
impl TryToV2 for crate::v1::AuthEnvVar {
    type Output = super::AuthEnvVar;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            name,
            label,
            secret,
            optional,
            meta,
        } = self;
        Ok(super::AuthEnvVar {
            name: name.try_to_v2()?,
            label: label.try_to_v2()?,
            secret: secret.try_to_v2()?,
            optional: optional.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_auth_methods")]
impl TryToV1 for super::AuthMethodTerminal {
    type Output = crate::v1::AuthMethodTerminal;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            method_id,
            name,
            description,
            args,
            env,
            meta,
        } = self;
        let env = env
            .into_iter()
            .map(|env_var| {
                let super::EnvVariable { name, value, meta } = env_var;
                if meta.is_some() {
                    return Err(ProtocolConversionError::new(
                        "v2 AuthMethodTerminal env variable `_meta` cannot be represented in v1",
                    ));
                }
                Ok((name.try_to_v1()?, value.try_to_v1()?))
            })
            .try_fold(HashMap::new(), |mut env, item| {
                let (name, value) = item?;
                if env.insert(name.clone(), value).is_some() {
                    return Err(ProtocolConversionError::new(format!(
                        "v2 AuthMethodTerminal env variable `{name}` is duplicated and cannot be represented in v1",
                    )));
                }
                Ok(env)
            })?;
        Ok(crate::v1::AuthMethodTerminal {
            id: method_id.try_to_v1()?,
            name: name.try_to_v1()?,
            description: description.try_to_v1()?,
            args: args.try_to_v1()?,
            env,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_auth_methods")]
impl TryToV2 for crate::v1::AuthMethodTerminal {
    type Output = super::AuthMethodTerminal;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            id,
            name,
            description,
            args,
            env,
            meta,
        } = self;
        let mut env = env
            .into_iter()
            .map(|(name, value)| {
                Ok(super::EnvVariable::new(
                    name.try_to_v2()?,
                    value.try_to_v2()?,
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        env.sort_by(|left, right| left.name.cmp(&right.name));
        Ok(super::AuthMethodTerminal {
            method_id: id.try_to_v2()?,
            name: name.try_to_v2()?,
            description: description.try_to_v2()?,
            args: args.try_to_v2()?,
            env,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::NewSessionRequest {
    type Output = crate::v1::NewSessionRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            cwd,
            additional_directories,
            mcp_servers,
            meta,
        } = self;
        Ok(crate::v1::NewSessionRequest {
            cwd: cwd.try_to_v1()?,
            additional_directories: additional_directories.try_to_v1()?,
            mcp_servers: mcp_servers.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::NewSessionRequest {
    type Output = super::NewSessionRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            cwd,
            additional_directories,
            mcp_servers,
            meta,
        } = self;
        Ok(super::NewSessionRequest {
            cwd: super::AbsolutePath::new(cwd),
            additional_directories: additional_directories
                .into_iter()
                .map(super::AbsolutePath::new)
                .collect(),
            mcp_servers: mcp_servers.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::NewSessionResponse {
    type Output = crate::v1::NewSessionResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            session_id,
            config_options,
            meta,
        } = self;
        Ok(crate::v1::NewSessionResponse {
            session_id: session_id.try_to_v1()?,
            modes: None,
            config_options: Some(config_options.try_to_v1()?),
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::NewSessionResponse {
    type Output = super::NewSessionResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            session_id,
            modes,
            config_options,
            meta,
        } = self;
        if modes.is_some() {
            return Err(unrepresentable_v1_field("NewSessionResponse", "modes"));
        }
        Ok(super::NewSessionResponse {
            session_id: session_id.try_to_v2()?,
            config_options: option_vec_into_v2_default(config_options)?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV2 for crate::v1::LoadSessionRequest {
    type Output = super::ResumeSessionRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            mcp_servers,
            cwd,
            additional_directories,
            session_id,
            meta,
        } = self;
        Ok(super::ResumeSessionRequest {
            mcp_servers: mcp_servers.try_to_v2()?,
            cwd: super::AbsolutePath::new(cwd),
            additional_directories: additional_directories
                .into_iter()
                .map(super::AbsolutePath::new)
                .collect(),
            session_id: session_id.try_to_v2()?,
            replay_from: Some(super::ReplayFrom::Start(super::ReplayFromStart::new())),
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV2 for crate::v1::LoadSessionResponse {
    type Output = super::ResumeSessionResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            modes,
            config_options,
            meta,
        } = self;
        if modes.is_some() {
            return Err(unrepresentable_v1_field("LoadSessionResponse", "modes"));
        }
        Ok(super::ResumeSessionResponse {
            config_options: option_vec_into_v2_default(config_options)?,
            meta: meta.try_to_v2()?,
        })
    }
}

fn v2_resume_session_request_into_v1_load(
    request: super::ResumeSessionRequest,
) -> Result<crate::v1::LoadSessionRequest> {
    let super::ResumeSessionRequest {
        session_id,
        cwd,
        additional_directories,
        mcp_servers,
        replay_from,
        meta,
    } = request;
    match replay_from {
        Some(super::ReplayFrom::Start(_)) => {}
        Some(super::ReplayFrom::Other(other)) => {
            return Err(unknown_v2_enum_variant("ReplayFrom", &other.type_));
        }
        None => {
            return Err(ProtocolConversionError::new(
                "v2 ResumeSessionRequest without `replayFrom: start` maps to v1 session/resume, not v1 session/load",
            ));
        }
    }
    Ok(crate::v1::LoadSessionRequest {
        mcp_servers: mcp_servers.try_to_v1()?,
        cwd: cwd.try_to_v1()?,
        additional_directories: additional_directories.try_to_v1()?,
        session_id: session_id.try_to_v1()?,
        meta: meta.try_to_v1()?,
    })
}

fn unsupported_replay_from_for_v1_resume(
    replay_from: super::ReplayFrom,
) -> ProtocolConversionError {
    match replay_from {
        super::ReplayFrom::Start(_) => ProtocolConversionError::new(
            "v2 ResumeSessionRequest `replayFrom: start` maps to v1 session/load, not v1 session/resume",
        ),
        super::ReplayFrom::Other(other) => unknown_v2_enum_variant("ReplayFrom", &other.type_),
    }
}

#[cfg(feature = "unstable_session_fork")]
impl TryToV1 for super::ForkSessionRequest {
    type Output = crate::v1::ForkSessionRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            session_id,
            cwd,
            additional_directories,
            mcp_servers,
            meta,
        } = self;
        Ok(crate::v1::ForkSessionRequest {
            session_id: session_id.try_to_v1()?,
            cwd: cwd.try_to_v1()?,
            additional_directories: additional_directories.try_to_v1()?,
            mcp_servers: mcp_servers.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_session_fork")]
impl TryToV2 for crate::v1::ForkSessionRequest {
    type Output = super::ForkSessionRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            session_id,
            cwd,
            additional_directories,
            mcp_servers,
            meta,
        } = self;
        Ok(super::ForkSessionRequest {
            session_id: session_id.try_to_v2()?,
            cwd: super::AbsolutePath::new(cwd),
            additional_directories: additional_directories
                .into_iter()
                .map(super::AbsolutePath::new)
                .collect(),
            mcp_servers: mcp_servers.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_session_fork")]
impl TryToV1 for super::ForkSessionResponse {
    type Output = crate::v1::ForkSessionResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            session_id,
            config_options,
            meta,
        } = self;
        Ok(crate::v1::ForkSessionResponse {
            session_id: session_id.try_to_v1()?,
            modes: None,
            config_options: Some(config_options.try_to_v1()?),
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_session_fork")]
impl TryToV2 for crate::v1::ForkSessionResponse {
    type Output = super::ForkSessionResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            session_id,
            modes,
            config_options,
            meta,
        } = self;
        if modes.is_some() {
            return Err(unrepresentable_v1_field("ForkSessionResponse", "modes"));
        }
        Ok(super::ForkSessionResponse {
            session_id: session_id.try_to_v2()?,
            config_options: option_vec_into_v2_default(config_options)?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::ResumeSessionRequest {
    type Output = crate::v1::ResumeSessionRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            session_id,
            cwd,
            additional_directories,
            mcp_servers,
            replay_from,
            meta,
        } = self;
        if let Some(replay_from) = replay_from {
            return Err(unsupported_replay_from_for_v1_resume(replay_from));
        }
        Ok(crate::v1::ResumeSessionRequest {
            session_id: session_id.try_to_v1()?,
            cwd: cwd.try_to_v1()?,
            additional_directories: additional_directories.try_to_v1()?,
            mcp_servers: mcp_servers.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::ResumeSessionRequest {
    type Output = super::ResumeSessionRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            session_id,
            cwd,
            additional_directories,
            mcp_servers,
            meta,
        } = self;
        Ok(super::ResumeSessionRequest {
            session_id: session_id.try_to_v2()?,
            cwd: super::AbsolutePath::new(cwd),
            additional_directories: additional_directories
                .into_iter()
                .map(super::AbsolutePath::new)
                .collect(),
            mcp_servers: mcp_servers.try_to_v2()?,
            replay_from: None,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::ResumeSessionResponse {
    type Output = crate::v1::ResumeSessionResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            config_options,
            meta,
        } = self;
        Ok(crate::v1::ResumeSessionResponse {
            modes: None,
            config_options: Some(config_options.try_to_v1()?),
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::ResumeSessionResponse {
    type Output = super::ResumeSessionResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            modes,
            config_options,
            meta,
        } = self;
        if modes.is_some() {
            return Err(unrepresentable_v1_field("ResumeSessionResponse", "modes"));
        }
        Ok(super::ResumeSessionResponse {
            config_options: option_vec_into_v2_default(config_options)?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::CloseSessionRequest {
    type Output = crate::v1::CloseSessionRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { session_id, meta } = self;
        Ok(crate::v1::CloseSessionRequest {
            session_id: session_id.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::CloseSessionRequest {
    type Output = super::CloseSessionRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { session_id, meta } = self;
        Ok(super::CloseSessionRequest {
            session_id: session_id.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::CloseSessionResponse {
    type Output = crate::v1::CloseSessionResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::CloseSessionResponse {
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::CloseSessionResponse {
    type Output = super::CloseSessionResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::CloseSessionResponse {
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::DeleteSessionRequest {
    type Output = crate::v1::DeleteSessionRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { session_id, meta } = self;
        Ok(crate::v1::DeleteSessionRequest {
            session_id: session_id.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::DeleteSessionRequest {
    type Output = super::DeleteSessionRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { session_id, meta } = self;
        Ok(super::DeleteSessionRequest {
            session_id: session_id.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::DeleteSessionResponse {
    type Output = crate::v1::DeleteSessionResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::DeleteSessionResponse {
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::DeleteSessionResponse {
    type Output = super::DeleteSessionResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::DeleteSessionResponse {
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::ListSessionsRequest {
    type Output = crate::v1::ListSessionsRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { cwd, cursor, meta } = self;
        Ok(crate::v1::ListSessionsRequest {
            cwd: cwd.try_to_v1()?,
            cursor: cursor.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::ListSessionsRequest {
    type Output = super::ListSessionsRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { cwd, cursor, meta } = self;
        Ok(super::ListSessionsRequest {
            cwd: cwd.map(super::AbsolutePath::new),
            cursor: cursor.map(super::SessionListCursor::new),
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::ListSessionsResponse {
    type Output = crate::v1::ListSessionsResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            sessions,
            next_cursor,
            meta,
        } = self;
        Ok(crate::v1::ListSessionsResponse {
            sessions: sessions.try_to_v1()?,
            next_cursor: next_cursor.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::ListSessionsResponse {
    type Output = super::ListSessionsResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            sessions,
            next_cursor,
            meta,
        } = self;
        Ok(super::ListSessionsResponse {
            sessions: sessions.try_to_v2()?,
            next_cursor: next_cursor.map(super::SessionListCursor::new),
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::SessionInfo {
    type Output = crate::v1::SessionInfo;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            session_id,
            cwd,
            additional_directories,
            title,
            updated_at,
            meta,
        } = self;
        Ok(crate::v1::SessionInfo {
            session_id: session_id.try_to_v1()?,
            cwd: cwd.try_to_v1()?,
            additional_directories: additional_directories.try_to_v1()?,
            title: title.try_to_v1()?,
            updated_at: updated_at.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::SessionInfo {
    type Output = super::SessionInfo;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            session_id,
            cwd,
            additional_directories,
            title,
            updated_at,
            meta,
        } = self;
        Ok(super::SessionInfo {
            session_id: session_id.try_to_v2()?,
            cwd: super::AbsolutePath::new(cwd),
            additional_directories: additional_directories
                .into_iter()
                .map(super::AbsolutePath::new)
                .collect(),
            title: title.try_to_v2()?,
            updated_at: updated_at.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::SessionConfigId {
    type Output = crate::v1::SessionConfigId;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(crate::v1::SessionConfigId(self.0.try_to_v1()?))
    }
}

impl TryToV2 for crate::v1::SessionConfigId {
    type Output = super::SessionConfigId;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(super::SessionConfigId(self.0.try_to_v2()?))
    }
}

impl TryToV1 for super::SessionConfigValueId {
    type Output = crate::v1::SessionConfigValueId;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(crate::v1::SessionConfigValueId(self.0.try_to_v1()?))
    }
}

impl TryToV2 for crate::v1::SessionConfigValueId {
    type Output = super::SessionConfigValueId;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(super::SessionConfigValueId(self.0.try_to_v2()?))
    }
}

impl TryToV1 for super::SessionConfigGroupId {
    type Output = crate::v1::SessionConfigGroupId;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(crate::v1::SessionConfigGroupId(self.0.try_to_v1()?))
    }
}

impl TryToV2 for crate::v1::SessionConfigGroupId {
    type Output = super::SessionConfigGroupId;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(super::SessionConfigGroupId(self.0.try_to_v2()?))
    }
}

impl TryToV1 for super::SessionConfigSelectOption {
    type Output = crate::v1::SessionConfigSelectOption;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            value,
            name,
            description,
            meta,
        } = self;
        Ok(crate::v1::SessionConfigSelectOption {
            value: value.try_to_v1()?,
            name: name.try_to_v1()?,
            description: description.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::SessionConfigSelectOption {
    type Output = super::SessionConfigSelectOption;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            value,
            name,
            description,
            meta,
        } = self;
        Ok(super::SessionConfigSelectOption {
            value: value.try_to_v2()?,
            name: name.try_to_v2()?,
            description: description.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::SessionConfigSelectGroup {
    type Output = crate::v1::SessionConfigSelectGroup;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            group_id,
            name,
            options,
            meta,
        } = self;
        Ok(crate::v1::SessionConfigSelectGroup {
            group: group_id.try_to_v1()?,
            name: name.try_to_v1()?,
            options: options.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::SessionConfigSelectGroup {
    type Output = super::SessionConfigSelectGroup;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            group,
            name,
            options,
            meta,
        } = self;
        Ok(super::SessionConfigSelectGroup {
            group_id: group.try_to_v2()?,
            name: name.try_to_v2()?,
            options: options.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::SessionConfigSelectOptions {
    type Output = crate::v1::SessionConfigSelectOptions;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Ungrouped(value) => {
                crate::v1::SessionConfigSelectOptions::Ungrouped(value.try_to_v1()?)
            }
            Self::Grouped(value) => {
                crate::v1::SessionConfigSelectOptions::Grouped(value.try_to_v1()?)
            }
        })
    }
}

impl TryToV2 for crate::v1::SessionConfigSelectOptions {
    type Output = super::SessionConfigSelectOptions;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Ungrouped(value) => {
                super::SessionConfigSelectOptions::Ungrouped(value.try_to_v2()?)
            }
            Self::Grouped(value) => super::SessionConfigSelectOptions::Grouped(value.try_to_v2()?),
        })
    }
}

impl TryToV1 for super::SessionConfigSelect {
    type Output = crate::v1::SessionConfigSelect;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            current_value,
            options,
        } = self;
        Ok(crate::v1::SessionConfigSelect {
            current_value: current_value.try_to_v1()?,
            options: options.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::SessionConfigSelect {
    type Output = super::SessionConfigSelect;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            current_value,
            options,
        } = self;
        Ok(super::SessionConfigSelect {
            current_value: current_value.try_to_v2()?,
            options: options.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::SessionConfigBoolean {
    type Output = crate::v1::SessionConfigBoolean;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { current_value } = self;
        Ok(crate::v1::SessionConfigBoolean {
            current_value: current_value.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::SessionConfigBoolean {
    type Output = super::SessionConfigBoolean;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { current_value } = self;
        Ok(super::SessionConfigBoolean {
            current_value: current_value.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::SessionConfigOptionCategory {
    type Output = crate::v1::SessionConfigOptionCategory;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Mode => crate::v1::SessionConfigOptionCategory::Mode,
            Self::Model => crate::v1::SessionConfigOptionCategory::Model,
            Self::ModelConfig => crate::v1::SessionConfigOptionCategory::ModelConfig,
            Self::ThoughtLevel => crate::v1::SessionConfigOptionCategory::ThoughtLevel,
            Self::Other(value) => crate::v1::SessionConfigOptionCategory::Other(value.try_to_v1()?),
        })
    }
}

impl TryToV2 for crate::v1::SessionConfigOptionCategory {
    type Output = super::SessionConfigOptionCategory;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Mode => super::SessionConfigOptionCategory::Mode,
            Self::Model => super::SessionConfigOptionCategory::Model,
            Self::ModelConfig => super::SessionConfigOptionCategory::ModelConfig,
            Self::ThoughtLevel => super::SessionConfigOptionCategory::ThoughtLevel,
            Self::Other(value) => super::SessionConfigOptionCategory::Other(value.try_to_v2()?),
        })
    }
}

impl TryToV1 for super::SessionConfigKind {
    type Output = crate::v1::SessionConfigKind;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Select(value) => crate::v1::SessionConfigKind::Select(value.try_to_v1()?),
            Self::Boolean(value) => crate::v1::SessionConfigKind::Boolean(value.try_to_v1()?),
            Self::Other(value) => {
                return Err(unknown_v2_enum_variant("SessionConfigKind", &value.type_));
            }
        })
    }
}

impl TryToV2 for crate::v1::SessionConfigKind {
    type Output = super::SessionConfigKind;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Select(value) => super::SessionConfigKind::Select(value.try_to_v2()?),
            Self::Boolean(value) => super::SessionConfigKind::Boolean(value.try_to_v2()?),
        })
    }
}

impl TryToV1 for super::SessionConfigOption {
    type Output = crate::v1::SessionConfigOption;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            config_id,
            name,
            description,
            category,
            kind,
            meta,
        } = self;
        Ok(crate::v1::SessionConfigOption {
            id: config_id.try_to_v1()?,
            name: name.try_to_v1()?,
            description: description.try_to_v1()?,
            category: category.try_to_v1()?,
            kind: kind.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::SessionConfigOption {
    type Output = super::SessionConfigOption;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            id,
            name,
            description,
            category,
            kind,
            meta,
        } = self;
        Ok(super::SessionConfigOption {
            config_id: id.try_to_v2()?,
            name: name.try_to_v2()?,
            description: description.try_to_v2()?,
            category: category.try_to_v2()?,
            kind: kind.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::SessionConfigOptionValue {
    type Output = crate::v1::SessionConfigOptionValue;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Id { value } => crate::v1::SessionConfigOptionValue::ValueId {
                value: value.try_to_v1()?,
            },
            Self::Boolean { value } => crate::v1::SessionConfigOptionValue::Boolean {
                value: value.try_to_v1()?,
            },
            Self::Other(value) => {
                return Err(unknown_v2_enum_variant(
                    "SessionConfigOptionValue",
                    &value.type_,
                ));
            }
        })
    }
}

impl TryToV2 for crate::v1::SessionConfigOptionValue {
    type Output = super::SessionConfigOptionValue;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Boolean { value } => super::SessionConfigOptionValue::Boolean {
                value: value.try_to_v2()?,
            },
            Self::ValueId { value } => super::SessionConfigOptionValue::Id {
                value: value.try_to_v2()?,
            },
        })
    }
}

impl TryToV1 for super::SetSessionConfigOptionRequest {
    type Output = crate::v1::SetSessionConfigOptionRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            session_id,
            config_id,
            value,
            meta,
        } = self;
        Ok(crate::v1::SetSessionConfigOptionRequest {
            session_id: session_id.try_to_v1()?,
            config_id: config_id.try_to_v1()?,
            value: value.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::SetSessionConfigOptionRequest {
    type Output = super::SetSessionConfigOptionRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            session_id,
            config_id,
            value,
            meta,
        } = self;
        Ok(super::SetSessionConfigOptionRequest {
            session_id: session_id.try_to_v2()?,
            config_id: config_id.try_to_v2()?,
            value: value.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::SetSessionConfigOptionResponse {
    type Output = crate::v1::SetSessionConfigOptionResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            config_options,
            meta,
        } = self;
        Ok(crate::v1::SetSessionConfigOptionResponse {
            config_options: config_options.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::SetSessionConfigOptionResponse {
    type Output = super::SetSessionConfigOptionResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            config_options,
            meta,
        } = self;
        Ok(super::SetSessionConfigOptionResponse {
            config_options: config_options.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::McpServer {
    type Output = crate::v1::McpServer;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Http(value) => crate::v1::McpServer::Http(value.try_to_v1()?),
            #[cfg(feature = "unstable_mcp_over_acp")]
            Self::Acp(value) => crate::v1::McpServer::Acp(value.try_to_v1()?),
            Self::Stdio(value) => crate::v1::McpServer::Stdio(value.try_to_v1()?),
            Self::Other(value) => return Err(unknown_v2_enum_variant("McpServer", &value.type_)),
        })
    }
}

impl TryToV2 for crate::v1::McpServer {
    type Output = super::McpServer;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Http(value) => super::McpServer::Http(value.try_to_v2()?),
            Self::Sse(_) => return Err(removed_v1_enum_variant("McpServer", "sse")),
            #[cfg(feature = "unstable_mcp_over_acp")]
            Self::Acp(value) => super::McpServer::Acp(value.try_to_v2()?),
            Self::Stdio(value) => super::McpServer::Stdio(value.try_to_v2()?),
        })
    }
}

impl TryToV1 for super::McpServerHttp {
    type Output = crate::v1::McpServerHttp;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            name,
            url,
            headers,
            meta,
        } = self;
        Ok(crate::v1::McpServerHttp {
            name: name.try_to_v1()?,
            url: url.try_to_v1()?,
            headers: headers.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::McpServerHttp {
    type Output = super::McpServerHttp;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            name,
            url,
            headers,
            meta,
        } = self;
        Ok(super::McpServerHttp {
            name: name.try_to_v2()?,
            url: url.try_to_v2()?,
            headers: headers.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl TryToV1 for super::McpServerAcpId {
    type Output = crate::v1::McpServerAcpId;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(crate::v1::McpServerAcpId(self.0.try_to_v1()?))
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl TryToV2 for crate::v1::McpServerAcpId {
    type Output = super::McpServerAcpId;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(super::McpServerAcpId(self.0.try_to_v2()?))
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl TryToV1 for super::McpConnectionId {
    type Output = crate::v1::McpConnectionId;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(crate::v1::McpConnectionId(self.0.try_to_v1()?))
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl TryToV2 for crate::v1::McpConnectionId {
    type Output = super::McpConnectionId;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(super::McpConnectionId(self.0.try_to_v2()?))
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl TryToV1 for super::McpServerAcp {
    type Output = crate::v1::McpServerAcp;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            name,
            server_id,
            meta,
        } = self;
        Ok(crate::v1::McpServerAcp {
            name: name.try_to_v1()?,
            server_id: server_id.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_mcp_over_acp")]
impl TryToV2 for crate::v1::McpServerAcp {
    type Output = super::McpServerAcp;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            name,
            server_id,
            meta,
        } = self;
        Ok(super::McpServerAcp {
            name: name.try_to_v2()?,
            server_id: server_id.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::McpServerStdio {
    type Output = crate::v1::McpServerStdio;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            name,
            command,
            args,
            env,
            meta,
        } = self;
        Ok(crate::v1::McpServerStdio {
            name: name.try_to_v1()?,
            command: command.try_to_v1()?,
            args: args.try_to_v1()?,
            env: env.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::McpServerStdio {
    type Output = super::McpServerStdio;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            name,
            command,
            args,
            env,
            meta,
        } = self;
        Ok(super::McpServerStdio {
            name: name.try_to_v2()?,
            command: super::AbsolutePath::new(command),
            args: args.try_to_v2()?,
            env: env.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::EnvVariable {
    type Output = crate::v1::EnvVariable;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { name, value, meta } = self;
        Ok(crate::v1::EnvVariable {
            name: name.try_to_v1()?,
            value: value.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::EnvVariable {
    type Output = super::EnvVariable;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { name, value, meta } = self;
        Ok(super::EnvVariable {
            name: name.try_to_v2()?,
            value: value.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::HttpHeader {
    type Output = crate::v1::HttpHeader;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { name, value, meta } = self;
        Ok(crate::v1::HttpHeader {
            name: name.try_to_v1()?,
            value: value.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::HttpHeader {
    type Output = super::HttpHeader;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { name, value, meta } = self;
        Ok(super::HttpHeader {
            name: name.try_to_v2()?,
            value: value.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::PromptRequest {
    type Output = crate::v1::PromptRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            session_id,
            prompt,
            meta,
        } = self;
        Ok(crate::v1::PromptRequest {
            session_id: session_id.try_to_v1()?,
            prompt: prompt.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::PromptRequest {
    type Output = super::PromptRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            session_id,
            prompt,
            meta,
        } = self;
        Ok(super::PromptRequest {
            session_id: session_id.try_to_v2()?,
            prompt: prompt.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::PromptResponse {
    type Output = crate::v1::PromptResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        Err(ProtocolConversionError::new(
            "v2 PromptResponse cannot be represented in v1 because v2 reports completion with state_update session updates",
        ))
    }
}

impl TryToV2 for crate::v1::PromptResponse {
    type Output = super::PromptResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        Err(ProtocolConversionError::new(
            "v1 PromptResponse cannot be represented in v2 by itself because v2 reports completion with state_update session updates",
        ))
    }
}

impl TryToV1 for super::StopReason {
    type Output = crate::v1::StopReason;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::EndTurn => crate::v1::StopReason::EndTurn,
            Self::MaxTokens => crate::v1::StopReason::MaxTokens,
            Self::MaxTurnRequests => crate::v1::StopReason::MaxTurnRequests,
            Self::Refusal => crate::v1::StopReason::Refusal,
            Self::Cancelled => crate::v1::StopReason::Cancelled,
            Self::Other(value) => return Err(unknown_v2_enum_variant("StopReason", &value)),
        })
    }
}

impl TryToV2 for crate::v1::StopReason {
    type Output = super::StopReason;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::EndTurn => super::StopReason::EndTurn,
            Self::MaxTokens => super::StopReason::MaxTokens,
            Self::MaxTurnRequests => super::StopReason::MaxTurnRequests,
            Self::Refusal => super::StopReason::Refusal,
            Self::Cancelled => super::StopReason::Cancelled,
        })
    }
}

#[cfg(feature = "unstable_end_turn_token_usage")]
impl TryToV1 for super::Usage {
    type Output = crate::v1::Usage;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            total_tokens,
            input_tokens,
            output_tokens,
            thought_tokens,
            cached_read_tokens,
            cached_write_tokens,
            meta,
        } = self;
        Ok(crate::v1::Usage {
            total_tokens: total_tokens.try_to_v1()?,
            input_tokens: input_tokens.try_to_v1()?,
            output_tokens: output_tokens.try_to_v1()?,
            thought_tokens: thought_tokens.try_to_v1()?,
            cached_read_tokens: cached_read_tokens.try_to_v1()?,
            cached_write_tokens: cached_write_tokens.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_end_turn_token_usage")]
impl TryToV2 for crate::v1::Usage {
    type Output = super::Usage;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            total_tokens,
            input_tokens,
            output_tokens,
            thought_tokens,
            cached_read_tokens,
            cached_write_tokens,
            meta,
        } = self;
        Ok(super::Usage {
            total_tokens: total_tokens.try_to_v2()?,
            input_tokens: input_tokens.try_to_v2()?,
            output_tokens: output_tokens.try_to_v2()?,
            thought_tokens: thought_tokens.try_to_v2()?,
            cached_read_tokens: cached_read_tokens.try_to_v2()?,
            cached_write_tokens: cached_write_tokens.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV1 for super::LlmProtocol {
    type Output = crate::v1::LlmProtocol;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Anthropic => crate::v1::LlmProtocol::Anthropic,
            Self::OpenAi => crate::v1::LlmProtocol::OpenAi,
            Self::Azure => crate::v1::LlmProtocol::Azure,
            Self::Vertex => crate::v1::LlmProtocol::Vertex,
            Self::Bedrock => crate::v1::LlmProtocol::Bedrock,
            Self::Other(value) => crate::v1::LlmProtocol::Other(value.try_to_v1()?),
        })
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV2 for crate::v1::LlmProtocol {
    type Output = super::LlmProtocol;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Anthropic => super::LlmProtocol::Anthropic,
            Self::OpenAi => super::LlmProtocol::OpenAi,
            Self::Azure => super::LlmProtocol::Azure,
            Self::Vertex => super::LlmProtocol::Vertex,
            Self::Bedrock => super::LlmProtocol::Bedrock,
            Self::Other(value) => super::LlmProtocol::Other(value.try_to_v2()?),
        })
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV1 for super::ProviderCurrentConfig {
    type Output = crate::v1::ProviderCurrentConfig;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            api_type,
            base_url,
            meta,
        } = self;
        Ok(crate::v1::ProviderCurrentConfig {
            api_type: api_type.try_to_v1()?,
            base_url: base_url.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV2 for crate::v1::ProviderCurrentConfig {
    type Output = super::ProviderCurrentConfig;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            api_type,
            base_url,
            meta,
        } = self;
        Ok(super::ProviderCurrentConfig {
            api_type: api_type.try_to_v2()?,
            base_url: base_url.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV1 for super::ProviderId {
    type Output = crate::v1::ProviderId;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(crate::v1::ProviderId(self.0.try_to_v1()?))
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV2 for crate::v1::ProviderId {
    type Output = super::ProviderId;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(super::ProviderId(self.0.try_to_v2()?))
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV1 for super::ProviderInfo {
    type Output = crate::v1::ProviderInfo;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            provider_id,
            supported,
            required,
            current,
            meta,
        } = self;
        Ok(crate::v1::ProviderInfo {
            provider_id: provider_id.try_to_v1()?,
            supported: supported.try_to_v1()?,
            required: required.try_to_v1()?,
            current: current.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV2 for crate::v1::ProviderInfo {
    type Output = super::ProviderInfo;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            provider_id,
            supported,
            required,
            current,
            meta,
        } = self;
        Ok(super::ProviderInfo {
            provider_id: provider_id.try_to_v2()?,
            supported: supported.try_to_v2()?,
            required: required.try_to_v2()?,
            current: current.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV1 for super::ListProvidersRequest {
    type Output = crate::v1::ListProvidersRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::ListProvidersRequest {
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV2 for crate::v1::ListProvidersRequest {
    type Output = super::ListProvidersRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::ListProvidersRequest {
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV1 for super::ListProvidersResponse {
    type Output = crate::v1::ListProvidersResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { providers, meta } = self;
        Ok(crate::v1::ListProvidersResponse {
            providers: providers.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV2 for crate::v1::ListProvidersResponse {
    type Output = super::ListProvidersResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { providers, meta } = self;
        Ok(super::ListProvidersResponse {
            providers: providers.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV1 for super::SetProviderRequest {
    type Output = crate::v1::SetProviderRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            provider_id,
            api_type,
            base_url,
            headers,
            meta,
        } = self;
        Ok(crate::v1::SetProviderRequest {
            provider_id: provider_id.try_to_v1()?,
            api_type: api_type.try_to_v1()?,
            base_url: base_url.try_to_v1()?,
            headers: headers.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV2 for crate::v1::SetProviderRequest {
    type Output = super::SetProviderRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            provider_id,
            api_type,
            base_url,
            headers,
            meta,
        } = self;
        Ok(super::SetProviderRequest {
            provider_id: provider_id.try_to_v2()?,
            api_type: api_type.try_to_v2()?,
            base_url: base_url.try_to_v2()?,
            headers: headers.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV1 for super::SetProviderResponse {
    type Output = crate::v1::SetProviderResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::SetProviderResponse {
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV2 for crate::v1::SetProviderResponse {
    type Output = super::SetProviderResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::SetProviderResponse {
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV1 for super::DisableProviderRequest {
    type Output = crate::v1::DisableProviderRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { provider_id, meta } = self;
        Ok(crate::v1::DisableProviderRequest {
            provider_id: provider_id.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV2 for crate::v1::DisableProviderRequest {
    type Output = super::DisableProviderRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { provider_id, meta } = self;
        Ok(super::DisableProviderRequest {
            provider_id: provider_id.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV1 for super::DisableProviderResponse {
    type Output = crate::v1::DisableProviderResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::DisableProviderResponse {
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV2 for crate::v1::DisableProviderResponse {
    type Output = super::DisableProviderResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::DisableProviderResponse {
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::AgentCapabilities {
    type Output = crate::v1::AgentCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            session,
            auth,
            #[cfg(feature = "unstable_llm_providers")]
            providers,
            #[cfg(feature = "unstable_nes")]
            nes,
            #[cfg(feature = "unstable_nes")]
            position_encoding,
            meta,
        } = self;
        let Some(session) = session else {
            return Err(ProtocolConversionError::new(
                "v2 AgentCapabilities without `session` cannot be represented in v1",
            ));
        };
        let V1SessionCapabilityParts {
            session_capabilities,
            prompt_capabilities,
            load_session,
            mcp_capabilities,
        } = session.try_into_v1_parts()?;

        Ok(crate::v1::AgentCapabilities {
            load_session: load_session.try_to_v1()?,
            prompt_capabilities,
            mcp_capabilities,
            session_capabilities,
            auth: auth
                .map(TryToV1::try_to_v1)
                .transpose()?
                .unwrap_or_default(),
            #[cfg(feature = "unstable_llm_providers")]
            providers: providers.try_to_v1()?,
            #[cfg(feature = "unstable_nes")]
            nes: nes.try_to_v1()?,
            #[cfg(feature = "unstable_nes")]
            position_encoding: position_encoding.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::AgentCapabilities {
    type Output = super::AgentCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            load_session,
            prompt_capabilities,
            mcp_capabilities,
            session_capabilities,
            auth,
            #[cfg(feature = "unstable_llm_providers")]
            providers,
            #[cfg(feature = "unstable_nes")]
            nes,
            #[cfg(feature = "unstable_nes")]
            position_encoding,
            meta,
        } = self;
        let session = super::SessionCapabilities::from_v1(
            session_capabilities,
            prompt_capabilities,
            load_session,
            mcp_capabilities,
        )?;

        Ok(super::AgentCapabilities {
            session: Some(session),
            auth: v1_agent_auth_capabilities_into_v2_option(auth)?,
            #[cfg(feature = "unstable_llm_providers")]
            providers: providers.try_to_v2()?,
            #[cfg(feature = "unstable_nes")]
            nes: nes.try_to_v2()?,
            #[cfg(feature = "unstable_nes")]
            position_encoding: position_encoding.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV1 for super::ProvidersCapabilities {
    type Output = crate::v1::ProvidersCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::ProvidersCapabilities {
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_llm_providers")]
impl TryToV2 for crate::v1::ProvidersCapabilities {
    type Output = super::ProvidersCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::ProvidersCapabilities {
            meta: meta.try_to_v2()?,
        })
    }
}

/// The v1 capability fields represented by v2 `SessionCapabilities`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct V1SessionCapabilityParts {
    /// Session-specific v1 capabilities.
    pub session_capabilities: crate::v1::SessionCapabilities,
    /// Prompt capabilities for v1 `session/prompt` requests.
    pub prompt_capabilities: crate::v1::PromptCapabilities,
    /// Whether v1 `session/load` is supported.
    pub load_session: bool,
    /// MCP capabilities for v1 session lifecycle requests.
    pub mcp_capabilities: crate::v1::McpCapabilities,
}

impl super::SessionCapabilities {
    /// Splits v2 session capabilities into the v1 agent capability fields that
    /// v2 groups under `session`.
    ///
    /// This is useful when shared internal capability construction produces a
    /// v2 [`SessionCapabilities`](super::SessionCapabilities) value but a v1
    /// implementation still needs the corresponding top-level v1 fields.
    ///
    /// # Errors
    ///
    /// Returns [`ProtocolConversionError`] when a nested v2 capability cannot
    /// be represented by the v1 capability fields.
    pub fn try_into_v1_parts(self) -> Result<V1SessionCapabilityParts> {
        let Self {
            prompt,
            mcp,
            delete,
            additional_directories,
            #[cfg(feature = "unstable_session_fork")]
            fork,
            meta,
        } = self;

        Ok(V1SessionCapabilityParts {
            session_capabilities: crate::v1::SessionCapabilities {
                list: Some(crate::v1::SessionListCapabilities::new()),
                delete: delete.try_to_v1()?,
                additional_directories: additional_directories.try_to_v1()?,
                #[cfg(feature = "unstable_session_fork")]
                fork: fork.try_to_v1()?,
                resume: Some(crate::v1::SessionResumeCapabilities::new()),
                close: Some(crate::v1::SessionCloseCapabilities::new()),
                meta: meta.try_to_v1()?,
            },
            prompt_capabilities: prompt.unwrap_or_default().try_to_v1()?,
            load_session: true,
            mcp_capabilities: mcp.unwrap_or_default().try_to_v1()?,
        })
    }

    /// Builds v2 draft session capabilities from the v1 agent capability fields
    /// that now live under `session` in v2.
    ///
    /// # Errors
    ///
    /// Returns [`ProtocolConversionError`] when any of the supplied v1
    /// capability fields cannot be represented in v2.
    pub fn from_v1(
        session_capabilities: crate::v1::SessionCapabilities,
        prompt_capabilities: crate::v1::PromptCapabilities,
        load_session: bool,
        mcp_capabilities: crate::v1::McpCapabilities,
    ) -> Result<Self> {
        if !load_session {
            return Err(unrepresentable_v1_field("AgentCapabilities", "loadSession"));
        }
        let crate::v1::SessionCapabilities {
            list,
            delete,
            additional_directories,
            #[cfg(feature = "unstable_session_fork")]
            fork,
            resume,
            close,
            meta,
        } = session_capabilities;

        let Some(list) = list else {
            return Err(unrepresentable_v1_field("SessionCapabilities", "list"));
        };
        reject_v1_marker_meta("SessionCapabilities", "list", list.meta.as_ref())?;

        let Some(resume) = resume else {
            return Err(unrepresentable_v1_field("SessionCapabilities", "resume"));
        };
        reject_v1_marker_meta("SessionCapabilities", "resume", resume.meta.as_ref())?;

        let Some(close) = close else {
            return Err(unrepresentable_v1_field("SessionCapabilities", "close"));
        };
        reject_v1_marker_meta("SessionCapabilities", "close", close.meta.as_ref())?;

        Ok(super::SessionCapabilities {
            prompt: Some(prompt_capabilities.try_to_v2()?),
            mcp: Some(mcp_capabilities.try_to_v2()?),
            delete: delete.try_to_v2()?,
            additional_directories: additional_directories.try_to_v2()?,
            #[cfg(feature = "unstable_session_fork")]
            fork: fork.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryFrom<super::SessionCapabilities> for V1SessionCapabilityParts {
    type Error = ProtocolConversionError;

    fn try_from(value: super::SessionCapabilities) -> Result<Self> {
        value.try_into_v1_parts()
    }
}

impl TryToV1 for super::SessionDeleteCapabilities {
    type Output = crate::v1::SessionDeleteCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::SessionDeleteCapabilities {
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::SessionDeleteCapabilities {
    type Output = super::SessionDeleteCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::SessionDeleteCapabilities {
            meta: meta.try_to_v2()?,
        })
    }
}
impl TryToV1 for super::SessionAdditionalDirectoriesCapabilities {
    type Output = crate::v1::SessionAdditionalDirectoriesCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::SessionAdditionalDirectoriesCapabilities {
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::SessionAdditionalDirectoriesCapabilities {
    type Output = super::SessionAdditionalDirectoriesCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::SessionAdditionalDirectoriesCapabilities {
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_session_fork")]
impl TryToV1 for super::SessionForkCapabilities {
    type Output = crate::v1::SessionForkCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::SessionForkCapabilities {
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_session_fork")]
impl TryToV2 for crate::v1::SessionForkCapabilities {
    type Output = super::SessionForkCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::SessionForkCapabilities {
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::PromptCapabilities {
    type Output = crate::v1::PromptCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            image,
            audio,
            embedded_context,
            meta,
        } = self;
        if let Some(image) = &image {
            reject_v2_marker_meta("PromptCapabilities", "image", image.meta.as_ref())?;
        }
        if let Some(audio) = &audio {
            reject_v2_marker_meta("PromptCapabilities", "audio", audio.meta.as_ref())?;
        }
        if let Some(embedded_context) = &embedded_context {
            reject_v2_marker_meta(
                "PromptCapabilities",
                "embeddedContext",
                embedded_context.meta.as_ref(),
            )?;
        }
        Ok(crate::v1::PromptCapabilities {
            image: image.is_some(),
            audio: audio.is_some(),
            embedded_context: embedded_context.is_some(),
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::PromptCapabilities {
    type Output = super::PromptCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            image,
            audio,
            embedded_context,
            meta,
        } = self;
        Ok(super::PromptCapabilities {
            image: image.then(super::PromptImageCapabilities::new),
            audio: audio.then(super::PromptAudioCapabilities::new),
            embedded_context: embedded_context.then(super::PromptEmbeddedContextCapabilities::new),
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::McpCapabilities {
    type Output = crate::v1::McpCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            stdio,
            http,
            #[cfg(feature = "unstable_mcp_over_acp")]
            acp,
            meta,
        } = self;
        if let Some(stdio) = &stdio {
            reject_v2_marker_meta("McpCapabilities", "stdio", stdio.meta.as_ref())?;
        }
        if let Some(http) = &http {
            reject_v2_marker_meta("McpCapabilities", "http", http.meta.as_ref())?;
        }
        #[cfg(feature = "unstable_mcp_over_acp")]
        if let Some(acp) = &acp {
            reject_v2_marker_meta("McpCapabilities", "acp", acp.meta.as_ref())?;
        }
        Ok(crate::v1::McpCapabilities {
            http: http.is_some(),
            sse: false,
            #[cfg(feature = "unstable_mcp_over_acp")]
            acp: acp.is_some(),
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::McpCapabilities {
    type Output = super::McpCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            http,
            sse,
            #[cfg(feature = "unstable_mcp_over_acp")]
            acp,
            meta,
        } = self;
        if sse {
            return Err(unrepresentable_v1_field("McpCapabilities", "sse"));
        }
        Ok(super::McpCapabilities {
            stdio: Some(super::McpStdioCapabilities::new()),
            http: http.then(super::McpHttpCapabilities::new),
            #[cfg(feature = "unstable_mcp_over_acp")]
            acp: acp.then(super::McpAcpCapabilities::new),
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::CancelSessionNotification {
    type Output = crate::v1::CancelNotification;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { session_id, meta } = self;
        Ok(crate::v1::CancelNotification {
            session_id: session_id.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::CancelNotification {
    type Output = super::CancelSessionNotification;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { session_id, meta } = self;
        Ok(super::CancelSessionNotification {
            session_id: session_id.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::PositionEncodingKind {
    type Output = crate::v1::PositionEncodingKind;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Utf16 => crate::v1::PositionEncodingKind::Utf16,
            Self::Utf32 => crate::v1::PositionEncodingKind::Utf32,
            Self::Utf8 => crate::v1::PositionEncodingKind::Utf8,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::PositionEncodingKind {
    type Output = super::PositionEncodingKind;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Utf16 => super::PositionEncodingKind::Utf16,
            Self::Utf32 => super::PositionEncodingKind::Utf32,
            Self::Utf8 => super::PositionEncodingKind::Utf8,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::Position {
    type Output = crate::v1::Position;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            line,
            character,
            meta,
        } = self;
        Ok(crate::v1::Position {
            line: line.try_to_v1()?,
            character: character.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::Position {
    type Output = super::Position;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            line,
            character,
            meta,
        } = self;
        Ok(super::Position {
            line: line.try_to_v2()?,
            character: character.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::Range {
    type Output = crate::v1::Range;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { start, end, meta } = self;
        Ok(crate::v1::Range {
            start: start.try_to_v1()?,
            end: end.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::Range {
    type Output = super::Range;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { start, end, meta } = self;
        Ok(super::Range {
            start: start.try_to_v2()?,
            end: end.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesCapabilities {
    type Output = crate::v1::NesCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            events,
            context,
            meta,
        } = self;
        Ok(crate::v1::NesCapabilities {
            events: events.try_to_v1()?,
            context: context.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesCapabilities {
    type Output = super::NesCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            events,
            context,
            meta,
        } = self;
        Ok(super::NesCapabilities {
            events: events.try_to_v2()?,
            context: context.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesEventCapabilities {
    type Output = crate::v1::NesEventCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { document, meta } = self;
        Ok(crate::v1::NesEventCapabilities {
            document: document.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesEventCapabilities {
    type Output = super::NesEventCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { document, meta } = self;
        Ok(super::NesEventCapabilities {
            document: document.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesDocumentEventCapabilities {
    type Output = crate::v1::NesDocumentEventCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            did_open,
            did_change,
            did_close,
            did_save,
            did_focus,
            meta,
        } = self;
        Ok(crate::v1::NesDocumentEventCapabilities {
            did_open: did_open.try_to_v1()?,
            did_change: did_change.try_to_v1()?,
            did_close: did_close.try_to_v1()?,
            did_save: did_save.try_to_v1()?,
            did_focus: did_focus.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesDocumentEventCapabilities {
    type Output = super::NesDocumentEventCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            did_open,
            did_change,
            did_close,
            did_save,
            did_focus,
            meta,
        } = self;
        Ok(super::NesDocumentEventCapabilities {
            did_open: did_open.try_to_v2()?,
            did_change: did_change.try_to_v2()?,
            did_close: did_close.try_to_v2()?,
            did_save: did_save.try_to_v2()?,
            did_focus: did_focus.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesDocumentDidOpenCapabilities {
    type Output = crate::v1::NesDocumentDidOpenCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::NesDocumentDidOpenCapabilities {
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesDocumentDidOpenCapabilities {
    type Output = super::NesDocumentDidOpenCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::NesDocumentDidOpenCapabilities {
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesDocumentDidChangeCapabilities {
    type Output = crate::v1::NesDocumentDidChangeCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { sync_kind, meta } = self;
        Ok(crate::v1::NesDocumentDidChangeCapabilities {
            sync_kind: sync_kind.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesDocumentDidChangeCapabilities {
    type Output = super::NesDocumentDidChangeCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { sync_kind, meta } = self;
        Ok(super::NesDocumentDidChangeCapabilities {
            sync_kind: sync_kind.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::TextDocumentSyncKind {
    type Output = crate::v1::TextDocumentSyncKind;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Full => crate::v1::TextDocumentSyncKind::Full,
            Self::Incremental => crate::v1::TextDocumentSyncKind::Incremental,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::TextDocumentSyncKind {
    type Output = super::TextDocumentSyncKind;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Full => super::TextDocumentSyncKind::Full,
            Self::Incremental => super::TextDocumentSyncKind::Incremental,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesDocumentDidCloseCapabilities {
    type Output = crate::v1::NesDocumentDidCloseCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::NesDocumentDidCloseCapabilities {
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesDocumentDidCloseCapabilities {
    type Output = super::NesDocumentDidCloseCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::NesDocumentDidCloseCapabilities {
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesDocumentDidSaveCapabilities {
    type Output = crate::v1::NesDocumentDidSaveCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::NesDocumentDidSaveCapabilities {
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesDocumentDidSaveCapabilities {
    type Output = super::NesDocumentDidSaveCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::NesDocumentDidSaveCapabilities {
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesDocumentDidFocusCapabilities {
    type Output = crate::v1::NesDocumentDidFocusCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::NesDocumentDidFocusCapabilities {
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesDocumentDidFocusCapabilities {
    type Output = super::NesDocumentDidFocusCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::NesDocumentDidFocusCapabilities {
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesContextCapabilities {
    type Output = crate::v1::NesContextCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            recent_files,
            related_snippets,
            edit_history,
            user_actions,
            open_files,
            diagnostics,
            meta,
        } = self;
        Ok(crate::v1::NesContextCapabilities {
            recent_files: recent_files.try_to_v1()?,
            related_snippets: related_snippets.try_to_v1()?,
            edit_history: edit_history.try_to_v1()?,
            user_actions: user_actions.try_to_v1()?,
            open_files: open_files.try_to_v1()?,
            diagnostics: diagnostics.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesContextCapabilities {
    type Output = super::NesContextCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            recent_files,
            related_snippets,
            edit_history,
            user_actions,
            open_files,
            diagnostics,
            meta,
        } = self;
        Ok(super::NesContextCapabilities {
            recent_files: recent_files.try_to_v2()?,
            related_snippets: related_snippets.try_to_v2()?,
            edit_history: edit_history.try_to_v2()?,
            user_actions: user_actions.try_to_v2()?,
            open_files: open_files.try_to_v2()?,
            diagnostics: diagnostics.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesRecentFilesCapabilities {
    type Output = crate::v1::NesRecentFilesCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { max_count, meta } = self;
        Ok(crate::v1::NesRecentFilesCapabilities {
            max_count: max_count.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesRecentFilesCapabilities {
    type Output = super::NesRecentFilesCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { max_count, meta } = self;
        Ok(super::NesRecentFilesCapabilities {
            max_count: max_count.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesRelatedSnippetsCapabilities {
    type Output = crate::v1::NesRelatedSnippetsCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::NesRelatedSnippetsCapabilities {
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesRelatedSnippetsCapabilities {
    type Output = super::NesRelatedSnippetsCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::NesRelatedSnippetsCapabilities {
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesEditHistoryCapabilities {
    type Output = crate::v1::NesEditHistoryCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { max_count, meta } = self;
        Ok(crate::v1::NesEditHistoryCapabilities {
            max_count: max_count.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesEditHistoryCapabilities {
    type Output = super::NesEditHistoryCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { max_count, meta } = self;
        Ok(super::NesEditHistoryCapabilities {
            max_count: max_count.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesUserActionsCapabilities {
    type Output = crate::v1::NesUserActionsCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { max_count, meta } = self;
        Ok(crate::v1::NesUserActionsCapabilities {
            max_count: max_count.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesUserActionsCapabilities {
    type Output = super::NesUserActionsCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { max_count, meta } = self;
        Ok(super::NesUserActionsCapabilities {
            max_count: max_count.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesOpenFilesCapabilities {
    type Output = crate::v1::NesOpenFilesCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::NesOpenFilesCapabilities {
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesOpenFilesCapabilities {
    type Output = super::NesOpenFilesCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::NesOpenFilesCapabilities {
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesDiagnosticsCapabilities {
    type Output = crate::v1::NesDiagnosticsCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::NesDiagnosticsCapabilities {
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesDiagnosticsCapabilities {
    type Output = super::NesDiagnosticsCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::NesDiagnosticsCapabilities {
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::ClientNesCapabilities {
    type Output = crate::v1::ClientNesCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            jump,
            rename,
            search_and_replace,
            meta,
        } = self;
        Ok(crate::v1::ClientNesCapabilities {
            jump: jump.try_to_v1()?,
            rename: rename.try_to_v1()?,
            search_and_replace: search_and_replace.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::ClientNesCapabilities {
    type Output = super::ClientNesCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            jump,
            rename,
            search_and_replace,
            meta,
        } = self;
        Ok(super::ClientNesCapabilities {
            jump: jump.try_to_v2()?,
            rename: rename.try_to_v2()?,
            search_and_replace: search_and_replace.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesJumpCapabilities {
    type Output = crate::v1::NesJumpCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::NesJumpCapabilities {
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesJumpCapabilities {
    type Output = super::NesJumpCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::NesJumpCapabilities {
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesRenameCapabilities {
    type Output = crate::v1::NesRenameCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::NesRenameCapabilities {
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesRenameCapabilities {
    type Output = super::NesRenameCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::NesRenameCapabilities {
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesSearchAndReplaceCapabilities {
    type Output = crate::v1::NesSearchAndReplaceCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::NesSearchAndReplaceCapabilities {
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesSearchAndReplaceCapabilities {
    type Output = super::NesSearchAndReplaceCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::NesSearchAndReplaceCapabilities {
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::DidOpenDocumentNotification {
    type Output = crate::v1::DidOpenDocumentNotification;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            session_id,
            uri,
            language_id,
            version,
            text,
            meta,
        } = self;
        Ok(crate::v1::DidOpenDocumentNotification {
            session_id: session_id.try_to_v1()?,
            uri: uri.try_to_v1()?,
            language_id: language_id.try_to_v1()?,
            version: version.try_to_v1()?,
            text: text.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::DidOpenDocumentNotification {
    type Output = super::DidOpenDocumentNotification;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            session_id,
            uri,
            language_id,
            version,
            text,
            meta,
        } = self;
        Ok(super::DidOpenDocumentNotification {
            session_id: session_id.try_to_v2()?,
            uri: uri.try_to_v2()?,
            language_id: language_id.try_to_v2()?,
            version: version.try_to_v2()?,
            text: text.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::DidChangeDocumentNotification {
    type Output = crate::v1::DidChangeDocumentNotification;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            session_id,
            uri,
            version,
            content_changes,
            meta,
        } = self;
        Ok(crate::v1::DidChangeDocumentNotification {
            session_id: session_id.try_to_v1()?,
            uri: uri.try_to_v1()?,
            version: version.try_to_v1()?,
            content_changes: content_changes.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::DidChangeDocumentNotification {
    type Output = super::DidChangeDocumentNotification;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            session_id,
            uri,
            version,
            content_changes,
            meta,
        } = self;
        Ok(super::DidChangeDocumentNotification {
            session_id: session_id.try_to_v2()?,
            uri: uri.try_to_v2()?,
            version: version.try_to_v2()?,
            content_changes: content_changes.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::TextDocumentContentChangeEvent {
    type Output = crate::v1::TextDocumentContentChangeEvent;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { range, text, meta } = self;
        Ok(crate::v1::TextDocumentContentChangeEvent {
            range: range.try_to_v1()?,
            text: text.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::TextDocumentContentChangeEvent {
    type Output = super::TextDocumentContentChangeEvent;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { range, text, meta } = self;
        Ok(super::TextDocumentContentChangeEvent {
            range: range.try_to_v2()?,
            text: text.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::DidCloseDocumentNotification {
    type Output = crate::v1::DidCloseDocumentNotification;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            session_id,
            uri,
            meta,
        } = self;
        Ok(crate::v1::DidCloseDocumentNotification {
            session_id: session_id.try_to_v1()?,
            uri: uri.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::DidCloseDocumentNotification {
    type Output = super::DidCloseDocumentNotification;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            session_id,
            uri,
            meta,
        } = self;
        Ok(super::DidCloseDocumentNotification {
            session_id: session_id.try_to_v2()?,
            uri: uri.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::DidSaveDocumentNotification {
    type Output = crate::v1::DidSaveDocumentNotification;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            session_id,
            uri,
            meta,
        } = self;
        Ok(crate::v1::DidSaveDocumentNotification {
            session_id: session_id.try_to_v1()?,
            uri: uri.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::DidSaveDocumentNotification {
    type Output = super::DidSaveDocumentNotification;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            session_id,
            uri,
            meta,
        } = self;
        Ok(super::DidSaveDocumentNotification {
            session_id: session_id.try_to_v2()?,
            uri: uri.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::DidFocusDocumentNotification {
    type Output = crate::v1::DidFocusDocumentNotification;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            session_id,
            uri,
            version,
            position,
            visible_range,
            meta,
        } = self;
        Ok(crate::v1::DidFocusDocumentNotification {
            session_id: session_id.try_to_v1()?,
            uri: uri.try_to_v1()?,
            version: version.try_to_v1()?,
            position: position.try_to_v1()?,
            visible_range: visible_range.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::DidFocusDocumentNotification {
    type Output = super::DidFocusDocumentNotification;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            session_id,
            uri,
            version,
            position,
            visible_range,
            meta,
        } = self;
        Ok(super::DidFocusDocumentNotification {
            session_id: session_id.try_to_v2()?,
            uri: uri.try_to_v2()?,
            version: version.try_to_v2()?,
            position: position.try_to_v2()?,
            visible_range: visible_range.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::StartNesRequest {
    type Output = crate::v1::StartNesRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            workspace_uri,
            workspace_folders,
            repository,
            meta,
        } = self;
        Ok(crate::v1::StartNesRequest {
            workspace_uri: workspace_uri.try_to_v1()?,
            workspace_folders: workspace_folders.try_to_v1()?,
            repository: repository.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::StartNesRequest {
    type Output = super::StartNesRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            workspace_uri,
            workspace_folders,
            repository,
            meta,
        } = self;
        Ok(super::StartNesRequest {
            workspace_uri: workspace_uri.try_to_v2()?,
            workspace_folders: workspace_folders.try_to_v2()?,
            repository: repository.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::WorkspaceFolder {
    type Output = crate::v1::WorkspaceFolder;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { uri, name, meta } = self;
        Ok(crate::v1::WorkspaceFolder {
            uri: uri.try_to_v1()?,
            name: name.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::WorkspaceFolder {
    type Output = super::WorkspaceFolder;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { uri, name, meta } = self;
        Ok(super::WorkspaceFolder {
            uri: uri.try_to_v2()?,
            name: name.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesRepository {
    type Output = crate::v1::NesRepository;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            name,
            owner,
            remote_url,
            meta,
        } = self;
        Ok(crate::v1::NesRepository {
            name: name.try_to_v1()?,
            owner: owner.try_to_v1()?,
            remote_url: remote_url.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesRepository {
    type Output = super::NesRepository;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            name,
            owner,
            remote_url,
            meta,
        } = self;
        Ok(super::NesRepository {
            name: name.try_to_v2()?,
            owner: owner.try_to_v2()?,
            remote_url: remote_url.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::StartNesResponse {
    type Output = crate::v1::StartNesResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { session_id, meta } = self;
        Ok(crate::v1::StartNesResponse {
            session_id: session_id.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::StartNesResponse {
    type Output = super::StartNesResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { session_id, meta } = self;
        Ok(super::StartNesResponse {
            session_id: session_id.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::CloseNesRequest {
    type Output = crate::v1::CloseNesRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { session_id, meta } = self;
        Ok(crate::v1::CloseNesRequest {
            session_id: session_id.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::CloseNesRequest {
    type Output = super::CloseNesRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { session_id, meta } = self;
        Ok(super::CloseNesRequest {
            session_id: session_id.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::CloseNesResponse {
    type Output = crate::v1::CloseNesResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::CloseNesResponse {
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::CloseNesResponse {
    type Output = super::CloseNesResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::CloseNesResponse {
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesTriggerKind {
    type Output = crate::v1::NesTriggerKind;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Automatic => crate::v1::NesTriggerKind::Automatic,
            Self::Diagnostic => crate::v1::NesTriggerKind::Diagnostic,
            Self::Manual => crate::v1::NesTriggerKind::Manual,
            Self::Other(value) => return Err(unknown_v2_enum_variant("NesTriggerKind", &value)),
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesTriggerKind {
    type Output = super::NesTriggerKind;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Automatic => super::NesTriggerKind::Automatic,
            Self::Diagnostic => super::NesTriggerKind::Diagnostic,
            Self::Manual => super::NesTriggerKind::Manual,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::SuggestNesRequest {
    type Output = crate::v1::SuggestNesRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            session_id,
            uri,
            version,
            position,
            selection,
            trigger_kind,
            context,
            meta,
        } = self;
        Ok(crate::v1::SuggestNesRequest {
            session_id: session_id.try_to_v1()?,
            uri: uri.try_to_v1()?,
            version: version.try_to_v1()?,
            position: position.try_to_v1()?,
            selection: selection.try_to_v1()?,
            trigger_kind: trigger_kind.try_to_v1()?,
            context: context.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::SuggestNesRequest {
    type Output = super::SuggestNesRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            session_id,
            uri,
            version,
            position,
            selection,
            trigger_kind,
            context,
            meta,
        } = self;
        Ok(super::SuggestNesRequest {
            session_id: session_id.try_to_v2()?,
            uri: uri.try_to_v2()?,
            version: version.try_to_v2()?,
            position: position.try_to_v2()?,
            selection: selection.try_to_v2()?,
            trigger_kind: trigger_kind.try_to_v2()?,
            context: context.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesSuggestContext {
    type Output = crate::v1::NesSuggestContext;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            recent_files,
            related_snippets,
            edit_history,
            user_actions,
            open_files,
            diagnostics,
            meta,
        } = self;
        Ok(crate::v1::NesSuggestContext {
            recent_files: recent_files.try_to_v1()?,
            related_snippets: related_snippets.try_to_v1()?,
            edit_history: edit_history.try_to_v1()?,
            user_actions: user_actions.try_to_v1()?,
            open_files: open_files.try_to_v1()?,
            diagnostics: diagnostics.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesSuggestContext {
    type Output = super::NesSuggestContext;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            recent_files,
            related_snippets,
            edit_history,
            user_actions,
            open_files,
            diagnostics,
            meta,
        } = self;
        Ok(super::NesSuggestContext {
            recent_files: recent_files.try_to_v2()?,
            related_snippets: related_snippets.try_to_v2()?,
            edit_history: edit_history.try_to_v2()?,
            user_actions: user_actions.try_to_v2()?,
            open_files: open_files.try_to_v2()?,
            diagnostics: diagnostics.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesRecentFile {
    type Output = crate::v1::NesRecentFile;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            uri,
            language_id,
            text,
            meta,
        } = self;
        Ok(crate::v1::NesRecentFile {
            uri: uri.try_to_v1()?,
            language_id: language_id.try_to_v1()?,
            text: text.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesRecentFile {
    type Output = super::NesRecentFile;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            uri,
            language_id,
            text,
            meta,
        } = self;
        Ok(super::NesRecentFile {
            uri: uri.try_to_v2()?,
            language_id: language_id.try_to_v2()?,
            text: text.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesRelatedSnippet {
    type Output = crate::v1::NesRelatedSnippet;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            uri,
            excerpts,
            meta,
        } = self;
        Ok(crate::v1::NesRelatedSnippet {
            uri: uri.try_to_v1()?,
            excerpts: excerpts.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesRelatedSnippet {
    type Output = super::NesRelatedSnippet;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            uri,
            excerpts,
            meta,
        } = self;
        Ok(super::NesRelatedSnippet {
            uri: uri.try_to_v2()?,
            excerpts: excerpts.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesExcerpt {
    type Output = crate::v1::NesExcerpt;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            start_line,
            end_line,
            text,
            meta,
        } = self;
        Ok(crate::v1::NesExcerpt {
            start_line: start_line.try_to_v1()?,
            end_line: end_line.try_to_v1()?,
            text: text.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesExcerpt {
    type Output = super::NesExcerpt;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            start_line,
            end_line,
            text,
            meta,
        } = self;
        Ok(super::NesExcerpt {
            start_line: start_line.try_to_v2()?,
            end_line: end_line.try_to_v2()?,
            text: text.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesEditHistoryEntry {
    type Output = crate::v1::NesEditHistoryEntry;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { uri, diff, meta } = self;
        Ok(crate::v1::NesEditHistoryEntry {
            uri: uri.try_to_v1()?,
            diff: diff.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesEditHistoryEntry {
    type Output = super::NesEditHistoryEntry;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { uri, diff, meta } = self;
        Ok(super::NesEditHistoryEntry {
            uri: uri.try_to_v2()?,
            diff: diff.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesUserAction {
    type Output = crate::v1::NesUserAction;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            action,
            uri,
            position,
            timestamp_ms,
            meta,
        } = self;
        Ok(crate::v1::NesUserAction {
            action: action.try_to_v1()?,
            uri: uri.try_to_v1()?,
            position: position.try_to_v1()?,
            timestamp_ms: timestamp_ms.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesUserAction {
    type Output = super::NesUserAction;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            action,
            uri,
            position,
            timestamp_ms,
            meta,
        } = self;
        Ok(super::NesUserAction {
            action: action.try_to_v2()?,
            uri: uri.try_to_v2()?,
            position: position.try_to_v2()?,
            timestamp_ms: timestamp_ms.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesOpenFile {
    type Output = crate::v1::NesOpenFile;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            uri,
            language_id,
            visible_range,
            last_focused_ms,
            meta,
        } = self;
        Ok(crate::v1::NesOpenFile {
            uri: uri.try_to_v1()?,
            language_id: language_id.try_to_v1()?,
            visible_range: visible_range.try_to_v1()?,
            last_focused_ms: last_focused_ms.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesOpenFile {
    type Output = super::NesOpenFile;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            uri,
            language_id,
            visible_range,
            last_focused_ms,
            meta,
        } = self;
        Ok(super::NesOpenFile {
            uri: uri.try_to_v2()?,
            language_id: language_id.try_to_v2()?,
            visible_range: visible_range.try_to_v2()?,
            last_focused_ms: last_focused_ms.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesDiagnostic {
    type Output = crate::v1::NesDiagnostic;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            uri,
            range,
            severity,
            message,
            meta,
        } = self;
        Ok(crate::v1::NesDiagnostic {
            uri: uri.try_to_v1()?,
            range: range.try_to_v1()?,
            severity: severity.try_to_v1()?,
            message: message.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesDiagnostic {
    type Output = super::NesDiagnostic;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            uri,
            range,
            severity,
            message,
            meta,
        } = self;
        Ok(super::NesDiagnostic {
            uri: uri.try_to_v2()?,
            range: range.try_to_v2()?,
            severity: severity.try_to_v2()?,
            message: message.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesDiagnosticSeverity {
    type Output = crate::v1::NesDiagnosticSeverity;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Error => crate::v1::NesDiagnosticSeverity::Error,
            Self::Warning => crate::v1::NesDiagnosticSeverity::Warning,
            Self::Information => crate::v1::NesDiagnosticSeverity::Information,
            Self::Hint => crate::v1::NesDiagnosticSeverity::Hint,
            Self::Other(value) => {
                return Err(unknown_v2_enum_variant("NesDiagnosticSeverity", &value));
            }
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesDiagnosticSeverity {
    type Output = super::NesDiagnosticSeverity;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Error => super::NesDiagnosticSeverity::Error,
            Self::Warning => super::NesDiagnosticSeverity::Warning,
            Self::Information => super::NesDiagnosticSeverity::Information,
            Self::Hint => super::NesDiagnosticSeverity::Hint,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::SuggestNesResponse {
    type Output = crate::v1::SuggestNesResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { suggestions, meta } = self;
        Ok(crate::v1::SuggestNesResponse {
            suggestions: suggestions.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::SuggestNesResponse {
    type Output = super::SuggestNesResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { suggestions, meta } = self;
        Ok(super::SuggestNesResponse {
            suggestions: suggestions.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesSuggestion {
    type Output = crate::v1::NesSuggestion;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Edit(value) => crate::v1::NesSuggestion::Edit(value.try_to_v1()?),
            Self::Jump(value) => crate::v1::NesSuggestion::Jump(value.try_to_v1()?),
            Self::Rename(value) => crate::v1::NesSuggestion::Rename(value.try_to_v1()?),
            Self::SearchAndReplace(value) => {
                crate::v1::NesSuggestion::SearchAndReplace(value.try_to_v1()?)
            }
            Self::Other(value) => {
                return Err(unknown_v2_enum_variant("NesSuggestion", &value.kind));
            }
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesSuggestion {
    type Output = super::NesSuggestion;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Edit(value) => super::NesSuggestion::Edit(value.try_to_v2()?),
            Self::Jump(value) => super::NesSuggestion::Jump(value.try_to_v2()?),
            Self::Rename(value) => super::NesSuggestion::Rename(value.try_to_v2()?),
            Self::SearchAndReplace(value) => {
                super::NesSuggestion::SearchAndReplace(value.try_to_v2()?)
            }
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesSuggestionId {
    type Output = crate::v1::NesSuggestionId;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(crate::v1::NesSuggestionId(self.0.try_to_v1()?))
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesSuggestionId {
    type Output = super::NesSuggestionId;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(super::NesSuggestionId(self.0.try_to_v2()?))
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesEditSuggestion {
    type Output = crate::v1::NesEditSuggestion;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            suggestion_id,
            uri,
            edits,
            cursor_position,
            meta,
        } = self;
        Ok(crate::v1::NesEditSuggestion {
            id: suggestion_id.try_to_v1()?,
            uri: uri.try_to_v1()?,
            edits: edits.try_to_v1()?,
            cursor_position: cursor_position.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesEditSuggestion {
    type Output = super::NesEditSuggestion;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            id,
            uri,
            edits,
            cursor_position,
            meta,
        } = self;
        Ok(super::NesEditSuggestion {
            suggestion_id: id.try_to_v2()?,
            uri: uri.try_to_v2()?,
            edits: edits.try_to_v2()?,
            cursor_position: cursor_position.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesTextEdit {
    type Output = crate::v1::NesTextEdit;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            range,
            new_text,
            meta,
        } = self;
        Ok(crate::v1::NesTextEdit {
            range: range.try_to_v1()?,
            new_text: new_text.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesTextEdit {
    type Output = super::NesTextEdit;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            range,
            new_text,
            meta,
        } = self;
        Ok(super::NesTextEdit {
            range: range.try_to_v2()?,
            new_text: new_text.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesJumpSuggestion {
    type Output = crate::v1::NesJumpSuggestion;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            suggestion_id,
            uri,
            position,
            meta,
        } = self;
        Ok(crate::v1::NesJumpSuggestion {
            id: suggestion_id.try_to_v1()?,
            uri: uri.try_to_v1()?,
            position: position.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesJumpSuggestion {
    type Output = super::NesJumpSuggestion;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            id,
            uri,
            position,
            meta,
        } = self;
        Ok(super::NesJumpSuggestion {
            suggestion_id: id.try_to_v2()?,
            uri: uri.try_to_v2()?,
            position: position.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesRenameSuggestion {
    type Output = crate::v1::NesRenameSuggestion;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            suggestion_id,
            uri,
            position,
            new_name,
            meta,
        } = self;
        Ok(crate::v1::NesRenameSuggestion {
            id: suggestion_id.try_to_v1()?,
            uri: uri.try_to_v1()?,
            position: position.try_to_v1()?,
            new_name: new_name.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesRenameSuggestion {
    type Output = super::NesRenameSuggestion;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            id,
            uri,
            position,
            new_name,
            meta,
        } = self;
        Ok(super::NesRenameSuggestion {
            suggestion_id: id.try_to_v2()?,
            uri: uri.try_to_v2()?,
            position: position.try_to_v2()?,
            new_name: new_name.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesSearchAndReplaceSuggestion {
    type Output = crate::v1::NesSearchAndReplaceSuggestion;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            suggestion_id,
            uri,
            search,
            replace,
            is_regex,
            meta,
        } = self;
        Ok(crate::v1::NesSearchAndReplaceSuggestion {
            id: suggestion_id.try_to_v1()?,
            uri: uri.try_to_v1()?,
            search: search.try_to_v1()?,
            replace: replace.try_to_v1()?,
            is_regex: is_regex.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesSearchAndReplaceSuggestion {
    type Output = super::NesSearchAndReplaceSuggestion;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            id,
            uri,
            search,
            replace,
            is_regex,
            meta,
        } = self;
        Ok(super::NesSearchAndReplaceSuggestion {
            suggestion_id: id.try_to_v2()?,
            uri: uri.try_to_v2()?,
            search: search.try_to_v2()?,
            replace: replace.try_to_v2()?,
            is_regex: is_regex.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::AcceptNesNotification {
    type Output = crate::v1::AcceptNesNotification;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            session_id,
            suggestion_id,
            meta,
        } = self;
        Ok(crate::v1::AcceptNesNotification {
            session_id: session_id.try_to_v1()?,
            id: suggestion_id.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::AcceptNesNotification {
    type Output = super::AcceptNesNotification;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            session_id,
            id,
            meta,
        } = self;
        Ok(super::AcceptNesNotification {
            session_id: session_id.try_to_v2()?,
            suggestion_id: id.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::RejectNesNotification {
    type Output = crate::v1::RejectNesNotification;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            session_id,
            suggestion_id,
            reason,
            meta,
        } = self;
        Ok(crate::v1::RejectNesNotification {
            session_id: session_id.try_to_v1()?,
            id: suggestion_id.try_to_v1()?,
            reason: reason.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::RejectNesNotification {
    type Output = super::RejectNesNotification;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            session_id,
            id,
            reason,
            meta,
        } = self;
        Ok(super::RejectNesNotification {
            session_id: session_id.try_to_v2()?,
            suggestion_id: id.try_to_v2()?,
            reason: reason.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV1 for super::NesRejectReason {
    type Output = crate::v1::NesRejectReason;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Rejected => crate::v1::NesRejectReason::Rejected,
            Self::Ignored => crate::v1::NesRejectReason::Ignored,
            Self::Replaced => crate::v1::NesRejectReason::Replaced,
            Self::Cancelled => crate::v1::NesRejectReason::Cancelled,
            Self::Other(value) => return Err(unknown_v2_enum_variant("NesRejectReason", &value)),
        })
    }
}

#[cfg(feature = "unstable_nes")]
impl TryToV2 for crate::v1::NesRejectReason {
    type Output = super::NesRejectReason;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Rejected => super::NesRejectReason::Rejected,
            Self::Ignored => super::NesRejectReason::Ignored,
            Self::Replaced => super::NesRejectReason::Replaced,
            Self::Cancelled => super::NesRejectReason::Cancelled,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::ElicitationId {
    type Output = crate::v1::ElicitationId;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(crate::v1::ElicitationId(self.0.try_to_v1()?))
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::ElicitationId {
    type Output = super::ElicitationId;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(super::ElicitationId(self.0.try_to_v2()?))
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::StringFormat {
    type Output = crate::v1::StringFormat;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Email => crate::v1::StringFormat::Email,
            Self::Uri => crate::v1::StringFormat::Uri,
            Self::Date => crate::v1::StringFormat::Date,
            Self::DateTime => crate::v1::StringFormat::DateTime,
            Self::Other(value) => return Err(unknown_v2_enum_variant("StringFormat", &value)),
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::StringFormat {
    type Output = super::StringFormat;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Email => super::StringFormat::Email,
            Self::Uri => super::StringFormat::Uri,
            Self::Date => super::StringFormat::Date,
            Self::DateTime => super::StringFormat::DateTime,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::ElicitationSchemaType {
    type Output = crate::v1::ElicitationSchemaType;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Object => crate::v1::ElicitationSchemaType::Object,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::ElicitationSchemaType {
    type Output = super::ElicitationSchemaType;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Object => super::ElicitationSchemaType::Object,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::EnumOption {
    type Output = crate::v1::EnumOption;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            value,
            title,
            description,
            meta,
        } = self;
        Ok(crate::v1::EnumOption {
            value: value.try_to_v1()?,
            title: title.try_to_v1()?,
            description: description.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::EnumOption {
    type Output = super::EnumOption;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            value,
            title,
            description,
            meta,
        } = self;
        Ok(super::EnumOption {
            value: value.try_to_v2()?,
            title: title.try_to_v2()?,
            description: description.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::StringPropertySchema {
    type Output = crate::v1::StringPropertySchema;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            title,
            description,
            min_length,
            max_length,
            pattern,
            format,
            default,
            enum_values,
            one_of,
            meta,
        } = self;
        Ok(crate::v1::StringPropertySchema {
            title: title.try_to_v1()?,
            description: description.try_to_v1()?,
            min_length: min_length.try_to_v1()?,
            max_length: max_length.try_to_v1()?,
            pattern: pattern.try_to_v1()?,
            format: format.try_to_v1()?,
            default: default.try_to_v1()?,
            enum_values: enum_values.try_to_v1()?,
            one_of: one_of.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::StringPropertySchema {
    type Output = super::StringPropertySchema;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            title,
            description,
            min_length,
            max_length,
            pattern,
            format,
            default,
            enum_values,
            one_of,
            meta,
        } = self;
        Ok(super::StringPropertySchema {
            title: title.try_to_v2()?,
            description: description.try_to_v2()?,
            min_length: min_length.try_to_v2()?,
            max_length: max_length.try_to_v2()?,
            pattern: pattern.try_to_v2()?,
            format: format.try_to_v2()?,
            default: default.try_to_v2()?,
            enum_values: enum_values.try_to_v2()?,
            one_of: one_of.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::NumberPropertySchema {
    type Output = crate::v1::NumberPropertySchema;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            title,
            description,
            minimum,
            maximum,
            default,
            meta,
        } = self;
        Ok(crate::v1::NumberPropertySchema {
            title: title.try_to_v1()?,
            description: description.try_to_v1()?,
            minimum: minimum.try_to_v1()?,
            maximum: maximum.try_to_v1()?,
            default: default.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::NumberPropertySchema {
    type Output = super::NumberPropertySchema;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            title,
            description,
            minimum,
            maximum,
            default,
            meta,
        } = self;
        Ok(super::NumberPropertySchema {
            title: title.try_to_v2()?,
            description: description.try_to_v2()?,
            minimum: minimum.try_to_v2()?,
            maximum: maximum.try_to_v2()?,
            default: default.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::IntegerPropertySchema {
    type Output = crate::v1::IntegerPropertySchema;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            title,
            description,
            minimum,
            maximum,
            default,
            meta,
        } = self;
        Ok(crate::v1::IntegerPropertySchema {
            title: title.try_to_v1()?,
            description: description.try_to_v1()?,
            minimum: minimum.try_to_v1()?,
            maximum: maximum.try_to_v1()?,
            default: default.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::IntegerPropertySchema {
    type Output = super::IntegerPropertySchema;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            title,
            description,
            minimum,
            maximum,
            default,
            meta,
        } = self;
        Ok(super::IntegerPropertySchema {
            title: title.try_to_v2()?,
            description: description.try_to_v2()?,
            minimum: minimum.try_to_v2()?,
            maximum: maximum.try_to_v2()?,
            default: default.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::BooleanPropertySchema {
    type Output = crate::v1::BooleanPropertySchema;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            title,
            description,
            default,
            meta,
        } = self;
        Ok(crate::v1::BooleanPropertySchema {
            title: title.try_to_v1()?,
            description: description.try_to_v1()?,
            default: default.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::BooleanPropertySchema {
    type Output = super::BooleanPropertySchema;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            title,
            description,
            default,
            meta,
        } = self;
        Ok(super::BooleanPropertySchema {
            title: title.try_to_v2()?,
            description: description.try_to_v2()?,
            default: default.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::StringMultiSelectItems {
    type Output = crate::v1::StringMultiSelectItems;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { values, meta } = self;
        Ok(crate::v1::StringMultiSelectItems {
            values: values.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::StringMultiSelectItems {
    type Output = super::StringMultiSelectItems;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { values, meta } = self;
        Ok(super::StringMultiSelectItems {
            values: values.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::OtherMultiSelectItems {
    type Output = crate::v1::OtherMultiSelectItems;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { type_, fields } = self;
        Ok(crate::v1::OtherMultiSelectItems {
            type_: type_.try_to_v1()?,
            fields: fields.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::OtherMultiSelectItems {
    type Output = super::OtherMultiSelectItems;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { type_, fields } = self;
        Ok(super::OtherMultiSelectItems {
            type_: type_.try_to_v2()?,
            fields: fields.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::TitledMultiSelectItems {
    type Output = crate::v1::TitledMultiSelectItems;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { options, meta } = self;
        Ok(crate::v1::TitledMultiSelectItems {
            options: options.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::TitledMultiSelectItems {
    type Output = super::TitledMultiSelectItems;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { options, meta } = self;
        Ok(super::TitledMultiSelectItems {
            options: options.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::MultiSelectItems {
    type Output = crate::v1::MultiSelectItems;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::String(value) => crate::v1::MultiSelectItems::String(value.try_to_v1()?),
            Self::Other(value) => crate::v1::MultiSelectItems::Other(value.try_to_v1()?),
            Self::Titled(value) => crate::v1::MultiSelectItems::Titled(value.try_to_v1()?),
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::MultiSelectItems {
    type Output = super::MultiSelectItems;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::String(value) => super::MultiSelectItems::String(value.try_to_v2()?),
            Self::Other(value) => super::MultiSelectItems::Other(value.try_to_v2()?),
            Self::Titled(value) => super::MultiSelectItems::Titled(value.try_to_v2()?),
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::MultiSelectPropertySchema {
    type Output = crate::v1::MultiSelectPropertySchema;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            title,
            description,
            min_items,
            max_items,
            items,
            default,
            meta,
        } = self;
        Ok(crate::v1::MultiSelectPropertySchema {
            title: title.try_to_v1()?,
            description: description.try_to_v1()?,
            min_items: min_items.try_to_v1()?,
            max_items: max_items.try_to_v1()?,
            items: items.try_to_v1()?,
            default: default.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::MultiSelectPropertySchema {
    type Output = super::MultiSelectPropertySchema;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            title,
            description,
            min_items,
            max_items,
            items,
            default,
            meta,
        } = self;
        Ok(super::MultiSelectPropertySchema {
            title: title.try_to_v2()?,
            description: description.try_to_v2()?,
            min_items: min_items.try_to_v2()?,
            max_items: max_items.try_to_v2()?,
            items: items.try_to_v2()?,
            default: default.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::ElicitationPropertySchema {
    type Output = crate::v1::ElicitationPropertySchema;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::String(value) => crate::v1::ElicitationPropertySchema::String(value.try_to_v1()?),
            Self::Number(value) => crate::v1::ElicitationPropertySchema::Number(value.try_to_v1()?),
            Self::Integer(value) => {
                crate::v1::ElicitationPropertySchema::Integer(value.try_to_v1()?)
            }
            Self::Boolean(value) => {
                crate::v1::ElicitationPropertySchema::Boolean(value.try_to_v1()?)
            }
            Self::Array(value) => crate::v1::ElicitationPropertySchema::Array(value.try_to_v1()?),
            Self::Other(value) => crate::v1::ElicitationPropertySchema::Other(value.try_to_v1()?),
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::ElicitationPropertySchema {
    type Output = super::ElicitationPropertySchema;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::String(value) => super::ElicitationPropertySchema::String(value.try_to_v2()?),
            Self::Number(value) => super::ElicitationPropertySchema::Number(value.try_to_v2()?),
            Self::Integer(value) => super::ElicitationPropertySchema::Integer(value.try_to_v2()?),
            Self::Boolean(value) => super::ElicitationPropertySchema::Boolean(value.try_to_v2()?),
            Self::Array(value) => super::ElicitationPropertySchema::Array(value.try_to_v2()?),
            Self::Other(value) => super::ElicitationPropertySchema::Other(value.try_to_v2()?),
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::OtherElicitationPropertySchema {
    type Output = crate::v1::OtherElicitationPropertySchema;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { type_, fields } = self;
        Ok(crate::v1::OtherElicitationPropertySchema {
            type_: type_.try_to_v1()?,
            fields: fields.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::OtherElicitationPropertySchema {
    type Output = super::OtherElicitationPropertySchema;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { type_, fields } = self;
        Ok(super::OtherElicitationPropertySchema {
            type_: type_.try_to_v2()?,
            fields: fields.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::ElicitationSchema {
    type Output = crate::v1::ElicitationSchema;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            type_,
            title,
            properties,
            required,
            description,
            meta,
        } = self;
        Ok(crate::v1::ElicitationSchema {
            type_: type_.try_to_v1()?,
            title: title.try_to_v1()?,
            properties: properties.try_to_v1()?,
            required: required.try_to_v1()?,
            description: description.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::ElicitationSchema {
    type Output = super::ElicitationSchema;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            type_,
            title,
            properties,
            required,
            description,
            meta,
        } = self;
        Ok(super::ElicitationSchema {
            type_: type_.try_to_v2()?,
            title: title.try_to_v2()?,
            properties: properties.try_to_v2()?,
            required: required.try_to_v2()?,
            description: description.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::ElicitationCapabilities {
    type Output = crate::v1::ElicitationCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { form, url, meta } = self;
        Ok(crate::v1::ElicitationCapabilities {
            form: form.try_to_v1()?,
            url: url.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::ElicitationCapabilities {
    type Output = super::ElicitationCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { form, url, meta } = self;
        Ok(super::ElicitationCapabilities {
            form: form.try_to_v2()?,
            url: url.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::ElicitationFormCapabilities {
    type Output = crate::v1::ElicitationFormCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::ElicitationFormCapabilities {
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::ElicitationFormCapabilities {
    type Output = super::ElicitationFormCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::ElicitationFormCapabilities {
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::ElicitationUrlCapabilities {
    type Output = crate::v1::ElicitationUrlCapabilities;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(crate::v1::ElicitationUrlCapabilities {
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::ElicitationUrlCapabilities {
    type Output = super::ElicitationUrlCapabilities;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { meta } = self;
        Ok(super::ElicitationUrlCapabilities {
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::ElicitationScope {
    type Output = crate::v1::ElicitationScope;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Session(value) => crate::v1::ElicitationScope::Session(value.try_to_v1()?),
            Self::Request(value) => crate::v1::ElicitationScope::Request(value.try_to_v1()?),
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::ElicitationScope {
    type Output = super::ElicitationScope;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Session(value) => super::ElicitationScope::Session(value.try_to_v2()?),
            Self::Request(value) => super::ElicitationScope::Request(value.try_to_v2()?),
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::ElicitationSessionScope {
    type Output = crate::v1::ElicitationSessionScope;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            session_id,
            tool_call_id,
        } = self;
        Ok(crate::v1::ElicitationSessionScope {
            session_id: session_id.try_to_v1()?,
            tool_call_id: tool_call_id.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::ElicitationSessionScope {
    type Output = super::ElicitationSessionScope;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            session_id,
            tool_call_id,
        } = self;
        Ok(super::ElicitationSessionScope {
            session_id: session_id.try_to_v2()?,
            tool_call_id: tool_call_id.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::ElicitationRequestScope {
    type Output = crate::v1::ElicitationRequestScope;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { request_id } = self;
        Ok(crate::v1::ElicitationRequestScope {
            request_id: request_id.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::ElicitationRequestScope {
    type Output = super::ElicitationRequestScope;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { request_id } = self;
        Ok(super::ElicitationRequestScope {
            request_id: request_id.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::CreateElicitationRequest {
    type Output = crate::v1::CreateElicitationRequest;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            mode,
            message,
            meta,
        } = self;
        Ok(crate::v1::CreateElicitationRequest {
            mode: mode.try_to_v1()?,
            message: message.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::CreateElicitationRequest {
    type Output = super::CreateElicitationRequest;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            mode,
            message,
            meta,
        } = self;
        Ok(super::CreateElicitationRequest {
            mode: mode.try_to_v2()?,
            message: message.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::ElicitationMode {
    type Output = crate::v1::ElicitationMode;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Form(value) => crate::v1::ElicitationMode::Form(value.try_to_v1()?),
            Self::Url(value) => crate::v1::ElicitationMode::Url(value.try_to_v1()?),
            Self::Other(value) => crate::v1::ElicitationMode::Other(value.try_to_v1()?),
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::ElicitationMode {
    type Output = super::ElicitationMode;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Form(value) => super::ElicitationMode::Form(value.try_to_v2()?),
            Self::Url(value) => super::ElicitationMode::Url(value.try_to_v2()?),
            Self::Other(value) => super::ElicitationMode::Other(value.try_to_v2()?),
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::OtherElicitationMode {
    type Output = crate::v1::OtherElicitationMode;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            mode,
            scope,
            fields,
        } = self;
        Ok(crate::v1::OtherElicitationMode {
            mode: mode.try_to_v1()?,
            scope: scope.try_to_v1()?,
            fields: fields.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::OtherElicitationMode {
    type Output = super::OtherElicitationMode;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            mode,
            scope,
            fields,
        } = self;
        Ok(super::OtherElicitationMode {
            mode: mode.try_to_v2()?,
            scope: scope.try_to_v2()?,
            fields: fields.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::ElicitationFormMode {
    type Output = crate::v1::ElicitationFormMode;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            scope,
            requested_schema,
        } = self;
        Ok(crate::v1::ElicitationFormMode {
            scope: scope.try_to_v1()?,
            requested_schema: requested_schema.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::ElicitationFormMode {
    type Output = super::ElicitationFormMode;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            scope,
            requested_schema,
        } = self;
        Ok(super::ElicitationFormMode {
            scope: scope.try_to_v2()?,
            requested_schema: requested_schema.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::ElicitationUrlMode {
    type Output = crate::v1::ElicitationUrlMode;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            scope,
            elicitation_id,
            url,
        } = self;
        Ok(crate::v1::ElicitationUrlMode {
            scope: scope.try_to_v1()?,
            elicitation_id: elicitation_id.try_to_v1()?,
            url: url.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::ElicitationUrlMode {
    type Output = super::ElicitationUrlMode;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            scope,
            elicitation_id,
            url,
        } = self;
        Ok(super::ElicitationUrlMode {
            scope: scope.try_to_v2()?,
            elicitation_id: elicitation_id.try_to_v2()?,
            url: url.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::CreateElicitationResponse {
    type Output = crate::v1::CreateElicitationResponse;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { action, meta } = self;
        Ok(crate::v1::CreateElicitationResponse {
            action: action.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::CreateElicitationResponse {
    type Output = super::CreateElicitationResponse;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { action, meta } = self;
        Ok(super::CreateElicitationResponse {
            action: action.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::ElicitationAction {
    type Output = crate::v1::ElicitationAction;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Accept(value) => crate::v1::ElicitationAction::Accept(value.try_to_v1()?),
            Self::Decline => crate::v1::ElicitationAction::Decline,
            Self::Cancel => crate::v1::ElicitationAction::Cancel,
            Self::Other(value) => crate::v1::ElicitationAction::Other(value.try_to_v1()?),
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::ElicitationAction {
    type Output = super::ElicitationAction;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Accept(value) => super::ElicitationAction::Accept(value.try_to_v2()?),
            Self::Decline => super::ElicitationAction::Decline,
            Self::Cancel => super::ElicitationAction::Cancel,
            Self::Other(value) => super::ElicitationAction::Other(value.try_to_v2()?),
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::OtherElicitationAction {
    type Output = crate::v1::OtherElicitationAction;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { action, fields } = self;
        Ok(crate::v1::OtherElicitationAction {
            action: action.try_to_v1()?,
            fields: fields.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::OtherElicitationAction {
    type Output = super::OtherElicitationAction;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { action, fields } = self;
        Ok(super::OtherElicitationAction {
            action: action.try_to_v2()?,
            fields: fields.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::ElicitationAcceptAction {
    type Output = crate::v1::ElicitationAcceptAction;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self { content } = self;
        Ok(crate::v1::ElicitationAcceptAction {
            content: content.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::ElicitationAcceptAction {
    type Output = super::ElicitationAcceptAction;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self { content } = self;
        Ok(super::ElicitationAcceptAction {
            content: content.try_to_v2()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::ElicitationContentValue {
    type Output = crate::v1::ElicitationContentValue;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::String(value) => crate::v1::ElicitationContentValue::String(value.try_to_v1()?),
            Self::Integer(value) => crate::v1::ElicitationContentValue::Integer(value.try_to_v1()?),
            Self::Number(value) => crate::v1::ElicitationContentValue::Number(value.try_to_v1()?),
            Self::Boolean(value) => crate::v1::ElicitationContentValue::Boolean(value.try_to_v1()?),
            Self::StringArray(value) => {
                crate::v1::ElicitationContentValue::StringArray(value.try_to_v1()?)
            }
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::ElicitationContentValue {
    type Output = super::ElicitationContentValue;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::String(value) => super::ElicitationContentValue::String(value.try_to_v2()?),
            Self::Integer(value) => super::ElicitationContentValue::Integer(value.try_to_v2()?),
            Self::Number(value) => super::ElicitationContentValue::Number(value.try_to_v2()?),
            Self::Boolean(value) => super::ElicitationContentValue::Boolean(value.try_to_v2()?),
            Self::StringArray(value) => {
                super::ElicitationContentValue::StringArray(value.try_to_v2()?)
            }
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV1 for super::CompleteElicitationNotification {
    type Output = crate::v1::CompleteElicitationNotification;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            elicitation_id,
            meta,
        } = self;
        Ok(crate::v1::CompleteElicitationNotification {
            elicitation_id: elicitation_id.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

#[cfg(feature = "unstable_elicitation")]
impl TryToV2 for crate::v1::CompleteElicitationNotification {
    type Output = super::CompleteElicitationNotification;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            elicitation_id,
            meta,
        } = self;
        Ok(super::CompleteElicitationNotification {
            elicitation_id: elicitation_id.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::ContentBlock {
    type Output = crate::v1::ContentBlock;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Text(value) => crate::v1::ContentBlock::Text(value.try_to_v1()?),
            Self::Image(value) => crate::v1::ContentBlock::Image(value.try_to_v1()?),
            Self::Audio(value) => crate::v1::ContentBlock::Audio(value.try_to_v1()?),
            Self::ResourceLink(value) => crate::v1::ContentBlock::ResourceLink(value.try_to_v1()?),
            Self::Resource(value) => crate::v1::ContentBlock::Resource(value.try_to_v1()?),
            Self::Other(value) => {
                return Err(unknown_v2_enum_variant("ContentBlock", &value.type_));
            }
        })
    }
}

impl TryToV2 for crate::v1::ContentBlock {
    type Output = super::ContentBlock;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Text(value) => super::ContentBlock::Text(value.try_to_v2()?),
            Self::Image(value) => super::ContentBlock::Image(value.try_to_v2()?),
            Self::Audio(value) => super::ContentBlock::Audio(value.try_to_v2()?),
            Self::ResourceLink(value) => super::ContentBlock::ResourceLink(value.try_to_v2()?),
            Self::Resource(value) => super::ContentBlock::Resource(value.try_to_v2()?),
        })
    }
}

impl TryToV1 for super::TextContent {
    type Output = crate::v1::TextContent;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            annotations,
            text,
            meta,
        } = self;
        Ok(crate::v1::TextContent {
            annotations: annotations.try_to_v1()?,
            text: text.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::TextContent {
    type Output = super::TextContent;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            annotations,
            text,
            meta,
        } = self;
        Ok(super::TextContent {
            annotations: annotations.try_to_v2()?,
            text: text.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::ImageContent {
    type Output = crate::v1::ImageContent;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            annotations,
            data,
            mime_type,
            uri,
            meta,
        } = self;
        Ok(crate::v1::ImageContent {
            annotations: annotations.try_to_v1()?,
            data: data.try_to_v1()?,
            mime_type: mime_type.try_to_v1()?,
            uri: uri.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::ImageContent {
    type Output = super::ImageContent;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            annotations,
            data,
            mime_type,
            uri,
            meta,
        } = self;
        Ok(super::ImageContent {
            annotations: annotations.try_to_v2()?,
            data: data.try_to_v2()?,
            mime_type: super::MediaType::new(mime_type),
            uri: uri.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::AudioContent {
    type Output = crate::v1::AudioContent;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            annotations,
            data,
            mime_type,
            meta,
        } = self;
        Ok(crate::v1::AudioContent {
            annotations: annotations.try_to_v1()?,
            data: data.try_to_v1()?,
            mime_type: mime_type.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::AudioContent {
    type Output = super::AudioContent;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            annotations,
            data,
            mime_type,
            meta,
        } = self;
        Ok(super::AudioContent {
            annotations: annotations.try_to_v2()?,
            data: data.try_to_v2()?,
            mime_type: super::MediaType::new(mime_type),
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::EmbeddedResource {
    type Output = crate::v1::EmbeddedResource;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            annotations,
            resource,
            meta,
        } = self;
        Ok(crate::v1::EmbeddedResource {
            annotations: annotations.try_to_v1()?,
            resource: resource.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::EmbeddedResource {
    type Output = super::EmbeddedResource;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            annotations,
            resource,
            meta,
        } = self;
        Ok(super::EmbeddedResource {
            annotations: annotations.try_to_v2()?,
            resource: resource.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::EmbeddedResourceResource {
    type Output = crate::v1::EmbeddedResourceResource;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::TextResourceContents(value) => {
                crate::v1::EmbeddedResourceResource::TextResourceContents(value.try_to_v1()?)
            }
            Self::BlobResourceContents(value) => {
                crate::v1::EmbeddedResourceResource::BlobResourceContents(value.try_to_v1()?)
            }
        })
    }
}

impl TryToV2 for crate::v1::EmbeddedResourceResource {
    type Output = super::EmbeddedResourceResource;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::TextResourceContents(value) => {
                super::EmbeddedResourceResource::TextResourceContents(value.try_to_v2()?)
            }
            Self::BlobResourceContents(value) => {
                super::EmbeddedResourceResource::BlobResourceContents(value.try_to_v2()?)
            }
        })
    }
}

impl TryToV1 for super::TextResourceContents {
    type Output = crate::v1::TextResourceContents;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            mime_type,
            text,
            uri,
            meta,
        } = self;
        Ok(crate::v1::TextResourceContents {
            mime_type: mime_type.try_to_v1()?,
            text: text.try_to_v1()?,
            uri: uri.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::TextResourceContents {
    type Output = super::TextResourceContents;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            mime_type,
            text,
            uri,
            meta,
        } = self;
        Ok(super::TextResourceContents {
            mime_type: mime_type.map(super::MediaType::new),
            text: text.try_to_v2()?,
            uri: uri.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::BlobResourceContents {
    type Output = crate::v1::BlobResourceContents;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            blob,
            mime_type,
            uri,
            meta,
        } = self;
        Ok(crate::v1::BlobResourceContents {
            blob: blob.try_to_v1()?,
            mime_type: mime_type.try_to_v1()?,
            uri: uri.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::BlobResourceContents {
    type Output = super::BlobResourceContents;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            blob,
            mime_type,
            uri,
            meta,
        } = self;
        Ok(super::BlobResourceContents {
            blob: blob.try_to_v2()?,
            mime_type: mime_type.map(super::MediaType::new),
            uri: uri.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::ResourceLink {
    type Output = crate::v1::ResourceLink;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            annotations,
            description,
            icons,
            mime_type,
            name,
            size,
            title,
            uri,
            meta,
        } = self;

        if matches!(icons.as_ref(), Some(icons) if !icons.is_empty()) {
            return Err(ProtocolConversionError::new(
                "v2 ResourceLink.icons cannot be represented in v1",
            ));
        }

        Ok(crate::v1::ResourceLink {
            annotations: annotations.try_to_v1()?,
            description: description.try_to_v1()?,
            mime_type: mime_type.try_to_v1()?,
            name: name.try_to_v1()?,
            size: size.try_to_v1()?,
            title: title.try_to_v1()?,
            uri: uri.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::ResourceLink {
    type Output = super::ResourceLink;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            annotations,
            description,
            mime_type,
            name,
            size,
            title,
            uri,
            meta,
        } = self;
        Ok(super::ResourceLink {
            annotations: annotations.try_to_v2()?,
            description: description.try_to_v2()?,
            icons: None,
            mime_type: mime_type.map(super::MediaType::new),
            name: name.try_to_v2()?,
            size: size.try_to_v2()?,
            title: title.try_to_v2()?,
            uri: uri.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::Annotations {
    type Output = crate::v1::Annotations;

    fn try_to_v1(self) -> Result<Self::Output> {
        let Self {
            audience,
            last_modified,
            priority,
            meta,
        } = self;
        Ok(crate::v1::Annotations {
            audience: audience.try_to_v1()?,
            last_modified: last_modified.try_to_v1()?,
            priority: priority.try_to_v1()?,
            meta: meta.try_to_v1()?,
        })
    }
}

impl TryToV2 for crate::v1::Annotations {
    type Output = super::Annotations;

    fn try_to_v2(self) -> Result<Self::Output> {
        let Self {
            audience,
            last_modified,
            priority,
            meta,
        } = self;
        Ok(super::Annotations {
            audience: audience.try_to_v2()?,
            last_modified: last_modified.try_to_v2()?,
            priority: priority.try_to_v2()?,
            meta: meta.try_to_v2()?,
        })
    }
}

impl TryToV1 for super::Role {
    type Output = crate::v1::Role;

    fn try_to_v1(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Assistant => crate::v1::Role::Assistant,
            Self::User => crate::v1::Role::User,
            Self::Other(value) => return Err(unknown_v2_enum_variant("Role", &value)),
        })
    }
}

impl TryToV2 for crate::v1::Role {
    type Output = super::Role;

    fn try_to_v2(self) -> Result<Self::Output> {
        Ok(match self {
            Self::Assistant => super::Role::Assistant,
            Self::User => super::Role::User,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{v1, v2};

    /// Round-trip a v1 value through v2 and back, asserting equality.
    ///
    /// This catches dropped fields, renamed fields, and missing enum
    /// variants in either direction of the conversion as soon as v1 and v2
    /// diverge.
    fn assert_v1_round_trip<T1, T2>(value: T1)
    where
        T1: TryToV2<Output = T2> + Clone + std::fmt::Debug + PartialEq,
        T2: TryFrom<T1>,
        ProtocolConversionError: From<<T2 as TryFrom<T1>>::Error>,
        T1: TryFrom<T2>,
        ProtocolConversionError: From<<T1 as TryFrom<T2>>::Error>,
    {
        let original = value.clone();
        let as_v2: T2 = try_v1_to_v2(value).expect("v1 -> v2 conversion failed");
        let back: T1 = try_v2_to_v1(as_v2).expect("v2 -> v1 conversion failed");
        assert_eq!(
            original, back,
            "value did not survive v1 -> v2 -> v1 round trip"
        );
    }

    /// Round-trip a v2 value through v1 and back, asserting equality.
    fn assert_v2_round_trip<T2, T1>(value: T2)
    where
        T2: TryToV1<Output = T1> + Clone + std::fmt::Debug + PartialEq,
        T1: TryFrom<T2>,
        ProtocolConversionError: From<<T1 as TryFrom<T2>>::Error>,
        T2: TryFrom<T1>,
        ProtocolConversionError: From<<T2 as TryFrom<T1>>::Error>,
    {
        let original = value.clone();
        let as_v1: T1 = try_v2_to_v1(value).expect("v2 -> v1 conversion failed");
        let back: T2 = try_v1_to_v2(as_v1).expect("v1 -> v2 conversion failed");
        assert_eq!(
            original, back,
            "value did not survive v2 -> v1 -> v2 round trip"
        );
    }

    /// While v1 and v2 are structurally identical, JSON produced by either
    /// module must be byte-equal after a conversion. This is a cheap insurance
    /// against accidental field renames or shape drift in conversions.
    ///
    /// Starts from a v1 value, converts forward to v2 and asserts JSON
    /// equality, then converts back to v1 and asserts JSON equality again.
    /// Catches shape drift in both directions of the conversion.
    fn assert_json_eq_after_v1_to_v2<T1, T2>(value: T1)
    where
        T1: TryToV2<Output = T2> + serde::Serialize + Clone,
        T2: TryFrom<T1> + serde::Serialize,
        ProtocolConversionError: From<<T2 as TryFrom<T1>>::Error>,
        T1: TryFrom<T2> + serde::Serialize,
        ProtocolConversionError: From<<T1 as TryFrom<T2>>::Error>,
    {
        let v1_json = serde_json::to_value(&value).expect("v1 serialize");
        let as_v2: T2 = try_v1_to_v2(value).expect("v1 -> v2 conversion");
        let v2_json = serde_json::to_value(&as_v2).expect("v2 serialize");
        assert_eq!(
            v1_json, v2_json,
            "JSON shape diverged after v1 -> v2 conversion"
        );

        let back_to_v1: T1 = try_v2_to_v1(as_v2).expect("v2 -> v1 conversion");
        let v1_json_after =
            serde_json::to_value(&back_to_v1).expect("v1 serialize after round trip");
        assert_eq!(
            v2_json, v1_json_after,
            "JSON shape diverged after v2 -> v1 conversion"
        );
    }

    /// Mirror of [`assert_json_eq_after_v1_to_v2`] that starts from a v2 value.
    /// Useful when the natural starting point is a v2 type (for example when
    /// the type only exists today in v2's draft API surface).
    fn assert_json_eq_after_v2_to_v1<T2, T1>(value: T2)
    where
        T2: TryToV1<Output = T1> + serde::Serialize + Clone,
        T1: TryFrom<T2> + serde::Serialize,
        ProtocolConversionError: From<<T1 as TryFrom<T2>>::Error>,
        T2: TryFrom<T1> + serde::Serialize,
        ProtocolConversionError: From<<T2 as TryFrom<T1>>::Error>,
    {
        let v2_json = serde_json::to_value(&value).expect("v2 serialize");
        let as_v1: T1 = try_v2_to_v1(value).expect("v2 -> v1 conversion");
        let v1_json = serde_json::to_value(&as_v1).expect("v1 serialize");
        assert_eq!(
            v2_json, v1_json,
            "JSON shape diverged after v2 -> v1 conversion"
        );

        let back_to_v2: T2 = try_v1_to_v2(as_v1).expect("v1 -> v2 conversion");
        let v2_json_after =
            serde_json::to_value(&back_to_v2).expect("v2 serialize after round trip");
        assert_eq!(
            v1_json, v2_json_after,
            "JSON shape diverged after v1 -> v2 conversion"
        );
    }

    fn assert_v2_to_v1_error<T>(value: T, expected: &str)
    where
        T: TryToV1,
        T::Output: TryFrom<T> + std::fmt::Debug,
        ProtocolConversionError: From<<T::Output as TryFrom<T>>::Error>,
    {
        let error = try_v2_to_v1::<T, T::Output>(value).unwrap_err();
        assert_eq!(error.message(), expected);
    }

    fn assert_v2_session_update_to_v1_error(value: v2::SessionUpdate, expected: &str) {
        let error = try_v2_to_v1_many::<_, v1::SessionUpdate>(value).unwrap_err();
        assert_eq!(error.message(), expected);
    }

    fn assert_v1_to_v2_error<T>(value: T, expected: &str)
    where
        T: TryToV2,
        T::Output: TryFrom<T> + std::fmt::Debug,
        ProtocolConversionError: From<<T::Output as TryFrom<T>>::Error>,
    {
        let error = try_v1_to_v2::<T, T::Output>(value).unwrap_err();
        assert_eq!(error.message(), expected);
    }

    fn v1_baseline_agent_capabilities() -> v1::AgentCapabilities {
        v1::AgentCapabilities::new()
            .load_session(true)
            .session_capabilities(
                v1::SessionCapabilities::new()
                    .list(v1::SessionListCapabilities::new())
                    .resume(v1::SessionResumeCapabilities::new())
                    .close(v1::SessionCloseCapabilities::new()),
            )
    }

    fn v2_baseline_agent_capabilities() -> v2::AgentCapabilities {
        v2::AgentCapabilities::new().session(v2::SessionCapabilities::new())
    }

    fn v1_test_auth_method() -> v1::AuthMethod {
        v1::AuthMethod::Agent(v1::AuthMethodAgent::new("agent", "Agent"))
    }

    fn v2_test_auth_method() -> v2::AuthMethod {
        v2::AuthMethod::Agent(v2::AuthMethodAgent::new("agent", "Agent"))
    }

    #[test]
    fn infallible_leaf_conversions_support_from_and_try_helpers() {
        let v1_session: v1::SessionId = v2::SessionId::new("sess").into();
        assert_eq!(v1_session, v1::SessionId::new("sess"));

        let v2_session: v2::SessionId =
            try_v1_to_v2(v1_session).expect("infallible v1 session id -> v2");
        assert_eq!(v2_session, v2::SessionId::new("sess"));

        let v2_status: v2::ToolCallStatus = v1::ToolCallStatus::Completed.into();
        assert_eq!(v2_status, v2::ToolCallStatus::Completed);

        let v1_code: v1::ErrorCode = v2::ErrorCode::Other(-32099).into();
        assert_eq!(v1_code, v1::ErrorCode::Other(-32099));
    }

    #[test]
    fn round_trips_session_config_option_categories() {
        for category in [
            v1::SessionConfigOptionCategory::Mode,
            v1::SessionConfigOptionCategory::Model,
            v1::SessionConfigOptionCategory::ModelConfig,
            v1::SessionConfigOptionCategory::ThoughtLevel,
            v1::SessionConfigOptionCategory::Other("_custom_category".to_string()),
        ] {
            assert_v1_round_trip::<v1::SessionConfigOptionCategory, v2::SessionConfigOptionCategory>(
                category,
            );
        }

        for category in [
            v2::SessionConfigOptionCategory::Mode,
            v2::SessionConfigOptionCategory::Model,
            v2::SessionConfigOptionCategory::ModelConfig,
            v2::SessionConfigOptionCategory::ThoughtLevel,
            v2::SessionConfigOptionCategory::Other("_custom_category".to_string()),
        ] {
            assert_v2_round_trip::<v2::SessionConfigOptionCategory, v1::SessionConfigOptionCategory>(
                category,
            );
        }
    }

    #[test]
    fn converts_v2_initialize_request_to_v1_without_serde() {
        let request = v2::InitializeRequest::new(
            ProtocolVersion::V2,
            v2::Implementation::new("test-client", "1.0.0"),
        );

        let converted: v1::InitializeRequest = try_v2_to_v1(request).unwrap();

        assert_eq!(converted.protocol_version, ProtocolVersion::V2);
        assert_eq!(
            converted
                .client_info
                .as_ref()
                .map(|info| info.name.as_str()),
            Some("test-client")
        );
    }

    #[test]
    fn v1_initialize_request_without_client_info_does_not_convert_to_v2() {
        let request = v1::InitializeRequest::new(ProtocolVersion::V1);

        assert_v1_to_v2_error(
            request,
            "v1 InitializeRequest without `clientInfo` cannot be represented in v2",
        );
    }

    #[test]
    fn v1_initialize_response_without_agent_info_does_not_convert_to_v2() {
        let response = v1::InitializeResponse::new(ProtocolVersion::V1);

        assert_v1_to_v2_error(
            response,
            "v1 InitializeResponse without `agentInfo` cannot be represented in v2",
        );
    }

    #[test]
    fn round_trips_initialize_request() {
        let client_capabilities = v1::ClientCapabilities::new().session(
            v1::ClientSessionCapabilities::new().config_options(
                v1::SessionConfigOptionsCapabilities::new()
                    .boolean(v1::BooleanConfigOptionCapabilities::new()),
            ),
        );

        let request = v1::InitializeRequest::new(ProtocolVersion::V1)
            .client_capabilities(client_capabilities)
            .client_info(v1::Implementation::new("test-client", "1.0.0").title("Test Client"));

        assert_v1_round_trip::<v1::InitializeRequest, v2::InitializeRequest>(request.clone());
        let converted: v2::InitializeRequest =
            try_v1_to_v2(request).expect("v1 -> v2 conversion failed");
        let converted_capabilities =
            serde_json::to_value(&converted.capabilities).expect("v2 serialize");
        assert_eq!(converted_capabilities.get("fs"), None);
        assert_eq!(converted_capabilities.get("terminal"), None);
        let converted_json = serde_json::to_value(&converted).expect("v2 serialize");
        assert_eq!(converted_json.get("clientInfo"), None);
        assert_eq!(converted_json.get("implementation"), None);
        assert!(converted_json.get("info").is_some());
    }

    #[test]
    fn round_trips_initialize_response() {
        let response = v1::InitializeResponse::new(ProtocolVersion::V1)
            .agent_capabilities(
                v1_baseline_agent_capabilities()
                    .auth(v1::AgentAuthCapabilities::new().logout(v1::LogoutCapabilities::new())),
            )
            .auth_methods(vec![v1_test_auth_method()])
            .agent_info(v1::Implementation::new("test-agent", "2.0.0").title("Test Agent"));
        assert_v1_round_trip::<v1::InitializeResponse, v2::InitializeResponse>(response.clone());
        let converted: v2::InitializeResponse =
            try_v1_to_v2(response).expect("v1 -> v2 conversion failed");
        let converted_json = serde_json::to_value(&converted).expect("v2 serialize");
        assert_eq!(converted_json.get("agentCapabilities"), None);
        assert!(converted_json.get("capabilities").is_some());
        assert_eq!(converted_json.get("agentInfo"), None);
        assert_eq!(converted_json.get("implementation"), None);
        assert!(converted_json.get("info").is_some());
        assert_eq!(converted_json.pointer("/capabilities/loadSession"), None);
    }

    #[test]
    fn required_v2_session_methods_convert_to_v1_capability_markers() {
        let v1_capabilities = v1::AgentCapabilities::new()
            .load_session(true)
            .session_capabilities(
                v1::SessionCapabilities::new()
                    .list(v1::SessionListCapabilities::new())
                    .resume(v1::SessionResumeCapabilities::new())
                    .close(v1::SessionCloseCapabilities::new()),
            );

        let v2_capabilities: v2::AgentCapabilities =
            try_v1_to_v2(v1_capabilities).expect("v1 -> v2 conversion");
        let session = v2_capabilities
            .session
            .as_ref()
            .expect("v1 capabilities imply v2 session support");
        assert!(session.delete.is_none());
        let v2_json = serde_json::to_value(&v2_capabilities).expect("v2 serialize");
        assert_eq!(v2_json.get("loadSession"), None);
        assert_eq!(v2_json.pointer("/session/load"), None);
        assert_eq!(v2_json.pointer("/session/list"), None);
        assert_eq!(v2_json.pointer("/session/resume"), None);
        assert_eq!(v2_json.pointer("/session/close"), None);

        let v1_after: v1::AgentCapabilities =
            try_v2_to_v1(v2_capabilities).expect("v2 -> v1 conversion");
        assert!(v1_after.load_session);
        assert!(v1_after.session_capabilities.list.is_some());
        assert!(v1_after.session_capabilities.resume.is_some());
        assert!(v1_after.session_capabilities.close.is_some());

        assert_v1_to_v2_error(
            v1::AgentCapabilities::new().load_session(true),
            "v1 SessionCapabilities.list cannot be represented in v2",
        );
        assert_v1_to_v2_error(
            v1::AgentCapabilities::new().session_capabilities(
                v1::SessionCapabilities::new()
                    .list(v1::SessionListCapabilities::new())
                    .resume(v1::SessionResumeCapabilities::new())
                    .close(v1::SessionCloseCapabilities::new()),
            ),
            "v1 AgentCapabilities.loadSession cannot be represented in v2",
        );
    }

    #[test]
    fn v2_agent_capabilities_without_session_do_not_convert_to_v1() {
        let error = v1::AgentCapabilities::try_from(v2::AgentCapabilities::new()).unwrap_err();
        assert_eq!(
            error.message(),
            "v2 AgentCapabilities without `session` cannot be represented in v1"
        );
    }

    #[test]
    fn initialize_auth_methods_control_logout_conversion() {
        let auth_meta =
            serde_json::Map::from_iter([("extension".to_string(), serde_json::Value::Bool(true))]);
        let without_auth = v2::InitializeResponse::new(
            ProtocolVersion::V2,
            v2::Implementation::new("test-agent", "2.0.0"),
        )
        .capabilities(
            v2_baseline_agent_capabilities()
                .auth(v2::AgentAuthCapabilities::new().meta(auth_meta.clone())),
        );
        let v1_without_auth: v1::InitializeResponse =
            try_v2_to_v1(without_auth).expect("v2 without auth methods -> v1");
        assert!(v1_without_auth.auth_methods.is_empty());
        assert!(v1_without_auth.agent_capabilities.auth.logout.is_none());
        assert_eq!(
            v1_without_auth.agent_capabilities.auth.meta.as_ref(),
            Some(&auth_meta)
        );

        let v2_without_auth: v2::InitializeResponse =
            try_v1_to_v2(v1_without_auth).expect("v1 without auth methods -> v2");
        assert!(v2_without_auth.auth_methods.is_empty());
        assert_eq!(
            v2_without_auth.capabilities.auth.and_then(|auth| auth.meta),
            Some(auth_meta)
        );

        let with_auth = v2::InitializeResponse::new(
            ProtocolVersion::V2,
            v2::Implementation::new("test-agent", "2.0.0"),
        )
        .capabilities(v2_baseline_agent_capabilities())
        .auth_methods(vec![v2_test_auth_method()]);
        let v1_with_auth: v1::InitializeResponse =
            try_v2_to_v1(with_auth).expect("v2 with auth methods -> v1");
        assert_eq!(v1_with_auth.auth_methods.len(), 1);
        assert!(v1_with_auth.agent_capabilities.auth.logout.is_some());

        let v1_with_auth = v1::InitializeResponse::new(ProtocolVersion::V1)
            .agent_capabilities(
                v1_baseline_agent_capabilities()
                    .auth(v1::AgentAuthCapabilities::new().logout(v1::LogoutCapabilities::new())),
            )
            .auth_methods(vec![v1_test_auth_method()])
            .agent_info(v1::Implementation::new("test-agent", "2.0.0"));
        let v2_with_auth: v2::InitializeResponse =
            try_v1_to_v2(v1_with_auth).expect("v1 with auth methods -> v2");
        assert_eq!(v2_with_auth.auth_methods.len(), 1);
    }

    #[test]
    fn mismatched_v1_auth_methods_and_logout_marker_do_not_convert_to_v2() {
        let methods_without_logout = v1::InitializeResponse::new(ProtocolVersion::V1)
            .agent_capabilities(v1_baseline_agent_capabilities())
            .auth_methods(vec![v1_test_auth_method()])
            .agent_info(v1::Implementation::new("test-agent", "2.0.0"));
        assert_v1_to_v2_error(
            methods_without_logout,
            "v1 InitializeResponse with non-empty `authMethods` and no \
             `agentCapabilities.auth.logout` cannot be represented in v2",
        );

        let logout_without_methods = v1::InitializeResponse::new(ProtocolVersion::V1)
            .agent_capabilities(
                v1_baseline_agent_capabilities()
                    .auth(v1::AgentAuthCapabilities::new().logout(v1::LogoutCapabilities::new())),
            )
            .agent_info(v1::Implementation::new("test-agent", "2.0.0"));
        assert_v1_to_v2_error(
            logout_without_methods,
            "v1 InitializeResponse with `agentCapabilities.auth.logout` and empty \
             `authMethods` cannot be represented in v2",
        );

        let logout_meta =
            serde_json::Map::from_iter([("marker".to_string(), serde_json::Value::Bool(true))]);
        let logout_marker_with_meta = v1::InitializeResponse::new(ProtocolVersion::V1)
            .agent_capabilities(
                v1_baseline_agent_capabilities().auth(
                    v1::AgentAuthCapabilities::new()
                        .logout(v1::LogoutCapabilities::new().meta(logout_meta)),
                ),
            )
            .auth_methods(vec![v1_test_auth_method()])
            .agent_info(v1::Implementation::new("test-agent", "2.0.0"));
        assert_v1_to_v2_error(
            logout_marker_with_meta,
            "v1 InitializeResponse.agentCapabilities.auth.logout metadata cannot be represented \
             in v2",
        );
    }

    #[test]
    fn agent_auth_capabilities_do_not_encode_logout_support() {
        let v2_auth = v2::AgentAuthCapabilities::new();
        let v2_json = serde_json::to_value(&v2_auth).expect("v2 serialize");
        assert_eq!(v2_json.get("logout"), None);

        let v1_auth: v1::AgentAuthCapabilities =
            try_v2_to_v1(v2_auth).expect("v2 -> v1 conversion");
        assert!(v1_auth.logout.is_none());

        let v2_auth: v2::AgentAuthCapabilities =
            try_v1_to_v2(v1::AgentAuthCapabilities::new()).expect("v1 -> v2 conversion");
        let v2_json = serde_json::to_value(&v2_auth).expect("v2 serialize");
        assert_eq!(v2_json.get("logout"), None);

        assert_v1_to_v2_error(
            v1::AgentAuthCapabilities::new().logout(v1::LogoutCapabilities::new()),
            "v1 AgentAuthCapabilities.logout cannot be represented in v2",
        );
    }

    #[test]
    fn v2_session_capabilities_convert_to_v1_agent_capability_parts() {
        let parts = v2::SessionCapabilities::new()
            .prompt(v2::PromptCapabilities::new().image(v2::PromptImageCapabilities::new()))
            .mcp(v2::McpCapabilities::new().http(v2::McpHttpCapabilities::new()))
            .try_into_v1_parts()
            .expect("v2 session capabilities -> v1 parts");

        assert!(parts.session_capabilities.list.is_some());
        assert!(parts.session_capabilities.resume.is_some());
        assert!(parts.session_capabilities.close.is_some());
        assert!(parts.prompt_capabilities.image);
        assert!(parts.load_session);
        assert!(parts.mcp_capabilities.http);
    }

    #[test]
    fn v1_prompt_capability_bools_convert_to_v2_objects() {
        let v1_capabilities = v1::PromptCapabilities::new()
            .image(true)
            .audio(true)
            .embedded_context(true);

        let v2_capabilities: v2::PromptCapabilities =
            try_v1_to_v2(v1_capabilities).expect("v1 -> v2 conversion");
        let v2_json = serde_json::to_value(&v2_capabilities).expect("v2 serialize");
        assert_eq!(v2_json.pointer("/image"), Some(&serde_json::json!({})));
        assert_eq!(v2_json.pointer("/audio"), Some(&serde_json::json!({})));
        assert_eq!(
            v2_json.pointer("/embeddedContext"),
            Some(&serde_json::json!({}))
        );

        let v1_after: v1::PromptCapabilities =
            try_v2_to_v1(v2_capabilities).expect("v2 -> v1 conversion");
        assert!(v1_after.image);
        assert!(v1_after.audio);
        assert!(v1_after.embedded_context);
    }

    #[test]
    fn v1_mcp_capabilities_convert_to_v2_transport_objects() {
        let v1_capabilities = v1::McpCapabilities::new().http(true);

        let v2_capabilities: v2::McpCapabilities =
            try_v1_to_v2(v1_capabilities).expect("v1 -> v2 conversion");
        let v2_json = serde_json::to_value(&v2_capabilities).expect("v2 serialize");
        assert_eq!(v2_json.pointer("/stdio"), Some(&serde_json::json!({})));
        assert_eq!(v2_json.pointer("/http"), Some(&serde_json::json!({})));
        assert_eq!(v2_json.pointer("/sse"), None);

        let v1_after: v1::McpCapabilities =
            try_v2_to_v1(v2_capabilities).expect("v2 -> v1 conversion");
        assert!(v1_after.http);
        assert!(!v1_after.sse);

        assert_v1_to_v2_error(
            v1::McpCapabilities::new().sse(true),
            "v1 McpCapabilities.sse cannot be represented in v2",
        );
    }

    #[cfg(feature = "unstable_mcp_over_acp")]
    #[test]
    fn v1_mcp_acp_capability_bool_converts_to_v2_object() {
        let v1_capabilities = v1::McpCapabilities::new().acp(true);

        let v2_capabilities: v2::McpCapabilities =
            try_v1_to_v2(v1_capabilities).expect("v1 -> v2 conversion");
        let v2_json = serde_json::to_value(&v2_capabilities).expect("v2 serialize");
        assert_eq!(v2_json.pointer("/acp"), Some(&serde_json::json!({})));

        let v1_after: v1::McpCapabilities =
            try_v2_to_v1(v2_capabilities).expect("v2 -> v1 conversion");
        assert!(v1_after.acp);
    }

    #[cfg(feature = "unstable_auth_methods")]
    #[test]
    fn v1_auth_terminal_capability_bool_converts_to_v2_object() {
        let v1_capabilities = v1::AuthCapabilities::new().terminal(true);

        let v2_capabilities: v2::AuthCapabilities =
            try_v1_to_v2(v1_capabilities).expect("v1 -> v2 conversion");
        let v2_json = serde_json::to_value(&v2_capabilities).expect("v2 serialize");
        assert_eq!(v2_json.pointer("/terminal"), Some(&serde_json::json!({})));

        let v1_after: v1::AuthCapabilities =
            try_v2_to_v1(v2_capabilities).expect("v2 -> v1 conversion");
        assert!(v1_after.terminal);
    }

    #[cfg(feature = "unstable_auth_methods")]
    #[test]
    fn auth_method_terminal_env_converts_between_map_and_variable_array() {
        let mut env = HashMap::new();
        env.insert("TERM".to_string(), "xterm-256color".to_string());
        env.insert("API_KEY".to_string(), "secret".to_string());

        let v1_method = v1::AuthMethodTerminal::new("tui-auth", "Terminal Auth").env(env);
        let v2_method: v2::AuthMethodTerminal =
            try_v1_to_v2(v1_method).expect("v1 -> v2 conversion");
        let v2_json = serde_json::to_value(&v2_method).expect("v2 serialize");
        assert_eq!(
            v2_json.pointer("/env"),
            Some(&serde_json::json!([
                {
                    "name": "API_KEY",
                    "value": "secret"
                },
                {
                    "name": "TERM",
                    "value": "xterm-256color"
                }
            ]))
        );

        let v1_after: v1::AuthMethodTerminal =
            try_v2_to_v1(v2_method).expect("v2 -> v1 conversion");
        assert_eq!(
            v1_after.env.get("TERM").map(String::as_str),
            Some("xterm-256color")
        );
        assert_eq!(
            v1_after.env.get("API_KEY").map(String::as_str),
            Some("secret")
        );
    }

    #[cfg(feature = "unstable_auth_methods")]
    #[test]
    fn auth_method_terminal_duplicate_env_names_do_not_convert_to_v1() {
        let v2_method = v2::AuthMethodTerminal::new("tui-auth", "Terminal Auth").env(vec![
            v2::EnvVariable::new("TERM", "xterm"),
            v2::EnvVariable::new("TERM", "xterm-256color"),
        ]);

        assert_v2_to_v1_error(
            v2_method,
            "v2 AuthMethodTerminal env variable `TERM` is duplicated and cannot be represented in v1",
        );
    }

    #[test]
    fn v1_client_fs_and_terminal_capabilities_do_not_convert_to_v2() {
        assert_v1_to_v2_error(
            v1::ClientCapabilities::new().fs(v1::FileSystemCapabilities::new()
                .read_text_file(true)
                .write_text_file(true)),
            "v1 ClientCapabilities.fs cannot be represented in v2",
        );
        assert_v1_to_v2_error(
            v1::ClientCapabilities::new().terminal(true),
            "v1 ClientCapabilities.terminal cannot be represented in v2",
        );
    }

    #[test]
    fn v2_client_capabilities_default_to_v1_boolean_config_option_support() {
        let v2_capabilities = v2::ClientCapabilities::new();

        let v1_capabilities: v1::ClientCapabilities =
            try_v2_to_v1(v2_capabilities).expect("v2 -> v1 conversion");

        assert!(
            v1_capabilities
                .session
                .and_then(|session| session.config_options)
                .and_then(|config_options| config_options.boolean)
                .is_some()
        );
    }

    #[test]
    fn v1_terminal_tool_call_content_does_not_convert_to_v2() {
        assert_v1_to_v2_error(
            v1::ToolCallContent::Terminal(v1::Terminal::new("term")),
            "v1 ToolCallContent variant `terminal` cannot be represented in v2",
        );
    }

    #[test]
    fn v1_mcp_sse_transport_does_not_convert_to_v2() {
        assert_v1_to_v2_error(
            v1::McpServer::Sse(v1::McpServerSse::new("events", "https://example.com/sse")),
            "v1 McpServer variant `sse` cannot be represented in v2",
        );
    }

    #[test]
    fn v2_unknown_mcp_transport_does_not_convert_to_v1() {
        assert_v2_to_v1_error(
            v2::McpServer::Other(v2::OtherMcpServer::new("websocket", BTreeMap::default())),
            "v2 McpServer variant `websocket` cannot be represented in v1",
        );
    }

    #[test]
    fn round_trips_new_session_request_with_mcp_variants() {
        let request = v1::NewSessionRequest::new("/workspace").mcp_servers(vec![
            v1::McpServer::Stdio(v1::McpServerStdio::new("local", "/usr/bin/mcp")),
            v1::McpServer::Http(v1::McpServerHttp::new("remote", "https://example.com")),
        ]);

        assert_v1_round_trip::<v1::NewSessionRequest, v2::NewSessionRequest>(request.clone());

        let v2_request: v2::NewSessionRequest = try_v1_to_v2(request).expect("v1 -> v2 conversion");
        assert_eq!(
            serde_json::to_value(&v2_request).expect("v2 serialize"),
            serde_json::json!({
                "cwd": "/workspace",
                "mcpServers": [
                    {
                        "type": "stdio",
                        "name": "local",
                        "command": "/usr/bin/mcp"
                    },
                    {
                        "type": "http",
                        "name": "remote",
                        "url": "https://example.com"
                    }
                ]
            })
        );
    }

    #[test]
    fn round_trips_prompt_request_with_content_variants() {
        let prompt = vec![
            v1::ContentBlock::Text(v1::TextContent::new("hello")),
            v1::ContentBlock::Image(v1::ImageContent::new("data", "image/png")),
            v1::ContentBlock::ResourceLink(v1::ResourceLink::new("file.txt", "file:///file.txt")),
        ];
        let request = v1::PromptRequest::new("sess_1", prompt);
        assert_v1_round_trip::<v1::PromptRequest, v2::PromptRequest>(request.clone());
        assert_json_eq_after_v1_to_v2::<v1::PromptRequest, v2::PromptRequest>(request);
    }

    #[cfg(feature = "unstable_elicitation")]
    #[test]
    fn round_trips_elicitation_property_schema_unknown_type() {
        let v1_schema = v1::ElicitationSchema::new().property(
            "location",
            v1::ElicitationPropertySchema::Other(v1::OtherElicitationPropertySchema::new(
                "_location",
                std::collections::BTreeMap::from([(
                    "precision".to_string(),
                    serde_json::json!("city"),
                )]),
            )),
            false,
        );

        assert_v1_round_trip::<v1::ElicitationSchema, v2::ElicitationSchema>(v1_schema.clone());
        assert_json_eq_after_v1_to_v2::<v1::ElicitationSchema, v2::ElicitationSchema>(v1_schema);

        let v2_schema = v2::ElicitationSchema::new().property(
            "location",
            v2::ElicitationPropertySchema::Other(v2::OtherElicitationPropertySchema::new(
                "_location",
                std::collections::BTreeMap::from([(
                    "precision".to_string(),
                    serde_json::json!("city"),
                )]),
            )),
            false,
        );

        assert_v2_round_trip::<v2::ElicitationSchema, v1::ElicitationSchema>(v2_schema);
    }

    #[cfg(feature = "unstable_elicitation")]
    #[test]
    fn round_trips_multi_select_items_unknown_type() {
        let v1_items = v1::MultiSelectItems::Other(v1::OtherMultiSelectItems::new(
            "_token",
            std::collections::BTreeMap::from([
                ("format".to_string(), serde_json::json!("workspace")),
                (
                    "anyOf".to_string(),
                    serde_json::json!([{ "const": "repo", "title": "Repository" }]),
                ),
            ]),
        ));

        assert_v1_round_trip::<v1::MultiSelectItems, v2::MultiSelectItems>(v1_items.clone());
        assert_json_eq_after_v1_to_v2::<v1::MultiSelectItems, v2::MultiSelectItems>(v1_items);

        let v2_items = v2::MultiSelectItems::Other(v2::OtherMultiSelectItems::new(
            "_token",
            std::collections::BTreeMap::from([
                ("format".to_string(), serde_json::json!("workspace")),
                (
                    "anyOf".to_string(),
                    serde_json::json!([{ "const": "repo", "title": "Repository" }]),
                ),
            ]),
        ));

        assert_v2_round_trip::<v2::MultiSelectItems, v1::MultiSelectItems>(v2_items);
    }

    #[cfg(feature = "unstable_elicitation")]
    #[test]
    fn round_trips_elicitation_mode_unknown_type() {
        let v1_request = v1::CreateElicitationRequest::new(
            v1::OtherElicitationMode::new(
                "_browser",
                v1::ElicitationRequestScope::new(v1::RequestId::Number(42)),
                std::collections::BTreeMap::from([(
                    "target".to_string(),
                    serde_json::json!("login"),
                )]),
            ),
            "Open a browser window",
        );

        assert_v1_round_trip::<v1::CreateElicitationRequest, v2::CreateElicitationRequest>(
            v1_request.clone(),
        );
        assert_json_eq_after_v1_to_v2::<v1::CreateElicitationRequest, v2::CreateElicitationRequest>(
            v1_request,
        );

        let v2_request = v2::CreateElicitationRequest::new(
            v2::OtherElicitationMode::new(
                "_browser",
                v2::ElicitationRequestScope::new(v2::RequestId::Number(42)),
                std::collections::BTreeMap::from([(
                    "target".to_string(),
                    serde_json::json!("login"),
                )]),
            ),
            "Open a browser window",
        );

        assert_v2_round_trip::<v2::CreateElicitationRequest, v1::CreateElicitationRequest>(
            v2_request,
        );
    }

    #[cfg(feature = "unstable_elicitation")]
    #[test]
    fn round_trips_elicitation_action_unknown_type() {
        let v1_response = v1::CreateElicitationResponse::new(v1::OtherElicitationAction::new(
            "_defer",
            std::collections::BTreeMap::from([
                ("reason".to_string(), serde_json::json!("waiting")),
                ("retryAfterMs".to_string(), serde_json::json!(1000)),
            ]),
        ));

        assert_v1_round_trip::<v1::CreateElicitationResponse, v2::CreateElicitationResponse>(
            v1_response.clone(),
        );
        assert_json_eq_after_v1_to_v2::<v1::CreateElicitationResponse, v2::CreateElicitationResponse>(
            v1_response,
        );

        let v2_response = v2::CreateElicitationResponse::new(v2::OtherElicitationAction::new(
            "_defer",
            std::collections::BTreeMap::from([
                ("reason".to_string(), serde_json::json!("waiting")),
                ("retryAfterMs".to_string(), serde_json::json!(1000)),
            ]),
        ));

        assert_v2_round_trip::<v2::CreateElicitationResponse, v1::CreateElicitationResponse>(
            v2_response,
        );
    }

    #[test]
    fn prompt_responses_do_not_convert_across_v1_v2_lifecycle_boundary() {
        assert_v2_to_v1_error(
            v2::PromptResponse::new(),
            "v2 PromptResponse cannot be represented in v1 because v2 reports completion with state_update session updates",
        );
        assert_v1_to_v2_error(
            v1::PromptResponse::new(v1::StopReason::EndTurn),
            "v1 PromptResponse cannot be represented in v2 by itself because v2 reports completion with state_update session updates",
        );
    }

    #[test]
    fn v1_tool_call_converts_to_v2_upsert_with_diff_and_locations_one_way() {
        let tool_call = v1::ToolCall::new("tc_1", "editing files")
            .kind(v1::ToolKind::Edit)
            .status(v1::ToolCallStatus::InProgress)
            .content(vec![v1::ToolCallContent::Diff(
                v1::Diff::new("/path", "new contents").old_text("old contents"),
            )])
            .locations(vec![v1::ToolCallLocation::new("/path").line(42)])
            .raw_input(serde_json::json!({"foo": "bar"}))
            .raw_output(serde_json::json!({"ok": true}));

        let converted: v2::ToolCallUpdate = try_v1_to_v2(tool_call).expect("v1 -> v2 conversion");
        assert_eq!(
            serde_json::to_value(&converted).expect("v2 serialize"),
            serde_json::json!({
                "toolCallId": "tc_1",
                "title": "editing files",
                "kind": "edit",
                "status": "in_progress",
                "content": [
                    {
                        "type": "diff",
                        "changes": [
                            {
                                "operation": "modify",
                                "path": "/path",
                                "fileType": "text"
                            }
                        ],
                        "patch": {
                            "format": "git_patch",
                            "text": "diff --git /path /path\n--- /path\n+++ /path\n@@ -1 +1 @@\n-old contents\n\\ No newline at end of file\n+new contents\n\\ No newline at end of file\n"
                        }
                    }
                ],
                "locations": [
                    {
                        "path": "/path",
                        "line": 42
                    }
                ],
                "rawInput": {
                    "foo": "bar"
                },
                "rawOutput": {
                    "ok": true
                }
            })
        );

        assert_v2_to_v1_error(
            converted,
            "v2 Diff cannot be represented in v1 because v1 requires oldText/newText while v2 carries Git --patch text and structured changes",
        );
    }

    #[test]
    fn v1_tool_call_update_round_trips_through_v2_tool_call_update_upsert() {
        let update = v1::ToolCallUpdate::new(
            "tc",
            v1::ToolCallUpdateFields::new()
                .status(v1::ToolCallStatus::Completed)
                .content(Vec::new()),
        );

        assert_v1_round_trip::<v1::ToolCallUpdate, v2::ToolCallUpdate>(update.clone());
        assert_json_eq_after_v1_to_v2::<v1::ToolCallUpdate, v2::ToolCallUpdate>(update);
    }

    #[test]
    fn v2_entity_meta_null_does_not_convert_to_v1() {
        assert_v2_to_v1_error(
            v2::SessionInfoUpdate::new().meta(None::<v2::Meta>),
            "v2 SessionInfoUpdate with null _meta cannot be represented in v1",
        );
        assert_v2_to_v1_error(
            v2::ToolCallUpdate::new("tc").meta(None::<v2::Meta>),
            "v2 ToolCallUpdate with null _meta cannot be represented in v1",
        );
    }

    #[test]
    fn round_trips_session_notification_for_unchanged_update_kinds() {
        fn content_chunk(text: &str, message_id: &str) -> v1::ContentChunk {
            let chunk = v1::ContentChunk::new(v1::ContentBlock::Text(v1::TextContent::new(text)));
            chunk.message_id(message_id)
        }

        let cases: Vec<v1::SessionUpdate> = vec![
            v1::SessionUpdate::UserMessageChunk(content_chunk("u", "msg_user")),
            v1::SessionUpdate::AgentMessageChunk(content_chunk("a", "msg_agent")),
            v1::SessionUpdate::AgentThoughtChunk(content_chunk("t", "msg_thought")),
            v1::SessionUpdate::SessionInfoUpdate(v1::SessionInfoUpdate::new().title("hi")),
            v1::SessionUpdate::UsageUpdate(
                v1::UsageUpdate::new(53_000, 200_000).cost(v1::Cost::new(0.045, "USD")),
            ),
        ];
        for update in cases {
            let notification = v1::SessionNotification::new("sess", update);
            let original_json = serde_json::to_value(&notification).expect("v1 serialize");
            let as_v2: v2::UpdateSessionNotification =
                try_v1_to_v2(notification.clone()).expect("v1 -> v2 conversion");
            let v2_json = serde_json::to_value(&as_v2).expect("v2 serialize");
            assert_eq!(
                original_json, v2_json,
                "JSON shape diverged after v1 -> v2 conversion"
            );

            let back = try_v2_to_v1_many(as_v2).expect("v2 -> v1 conversion");
            assert_eq!(back, vec![notification]);
            let back_json = serde_json::to_value(&back[0]).expect("v1 serialize after round trip");
            assert_eq!(
                original_json, back_json,
                "JSON shape diverged after v2 -> v1 conversion"
            );
        }
    }

    #[test]
    fn v1_tool_call_session_updates_convert_to_unified_v2_tool_call_update() {
        let create = v1::SessionNotification::new(
            "sess",
            v1::SessionUpdate::ToolCall(v1::ToolCall::new("tc", "title")),
        );
        let create_v2: v2::UpdateSessionNotification =
            try_v1_to_v2(create).expect("v1 -> v2 conversion");
        assert!(matches!(
            create_v2.update,
            v2::SessionUpdate::ToolCallUpdate(_)
        ));
        assert_eq!(
            serde_json::to_value(&create_v2).expect("v2 serialize"),
            serde_json::json!({
                "sessionId": "sess",
                "update": {
                    "sessionUpdate": "tool_call_update",
                    "toolCallId": "tc",
                    "title": "title"
                }
            })
        );

        let update = v1::SessionNotification::new(
            "sess",
            v1::SessionUpdate::ToolCallUpdate(v1::ToolCallUpdate::new(
                "tc",
                v1::ToolCallUpdateFields::new().status(v1::ToolCallStatus::Completed),
            )),
        );
        let update_v2: v2::UpdateSessionNotification =
            try_v1_to_v2(update).expect("v1 -> v2 conversion");
        assert!(matches!(
            update_v2.update,
            v2::SessionUpdate::ToolCallUpdate(_)
        ));
        assert_eq!(
            serde_json::to_value(&update_v2).expect("v2 serialize"),
            serde_json::json!({
                "sessionId": "sess",
                "update": {
                    "sessionUpdate": "tool_call_update",
                    "toolCallId": "tc",
                    "status": "completed"
                }
            })
        );
    }

    #[test]
    fn v2_full_messages_convert_to_v1_message_chunks() {
        let mut meta = v2::Meta::new();
        meta.insert("source".to_string(), serde_json::json!("full"));

        let chunks = try_v2_to_v1_many(v2::SessionUpdate::UserMessage(
            v2::UserMessage::new("msg_user")
                .content(vec![
                    v2::ContentBlock::Text(v2::TextContent::new("hello")),
                    v2::ContentBlock::Text(v2::TextContent::new("world")),
                ])
                .meta(meta.clone()),
        ))
        .expect("v2 -> v1 conversion");
        assert_eq!(
            chunks,
            vec![
                v1::SessionUpdate::UserMessageChunk(
                    v1::ContentChunk::new(v1::ContentBlock::Text(v1::TextContent::new("hello")))
                        .message_id("msg_user")
                        .meta(meta.clone())
                ),
                v1::SessionUpdate::UserMessageChunk(
                    v1::ContentChunk::new(v1::ContentBlock::Text(v1::TextContent::new("world")))
                        .message_id("msg_user")
                        .meta(meta)
                ),
            ]
        );

        assert_eq!(
            try_v2_to_v1_many(v2::SessionUpdate::AgentMessage(
                v2::AgentMessage::new("msg_agent")
                    .content(vec![v2::ContentBlock::Text(v2::TextContent::new("hello"))])
            ))
            .expect("v2 -> v1 conversion"),
            vec![v1::SessionUpdate::AgentMessageChunk(
                v1::ContentChunk::new(v1::ContentBlock::Text(v1::TextContent::new("hello")))
                    .message_id("msg_agent")
            )]
        );

        assert_eq!(
            try_v2_to_v1_many(v2::SessionUpdate::AgentThought(
                v2::AgentThought::new("msg_thought").content(vec![v2::ContentBlock::Text(
                    v2::TextContent::new("thinking")
                )])
            ))
            .expect("v2 -> v1 conversion"),
            vec![v1::SessionUpdate::AgentThoughtChunk(
                v1::ContentChunk::new(v1::ContentBlock::Text(v1::TextContent::new("thinking")))
                    .message_id("msg_thought")
            )]
        );
    }

    #[test]
    fn v2_full_message_session_notification_fans_out_to_v1_chunk_notifications() {
        let notification = v2::UpdateSessionNotification::new(
            "sess",
            v2::SessionUpdate::AgentMessage(v2::AgentMessage::new("msg_agent").content(vec![
                v2::ContentBlock::Text(v2::TextContent::new("hello")),
                v2::ContentBlock::Text(v2::TextContent::new("world")),
            ])),
        );

        let notifications =
            Vec::<v1::SessionNotification>::try_from(notification).expect("v2 -> v1 conversion");
        assert_eq!(
            notifications,
            vec![
                v1::SessionNotification::new(
                    "sess",
                    v1::SessionUpdate::AgentMessageChunk(
                        v1::ContentChunk::new(v1::ContentBlock::Text(v1::TextContent::new(
                            "hello"
                        )))
                        .message_id("msg_agent")
                    )
                ),
                v1::SessionNotification::new(
                    "sess",
                    v1::SessionUpdate::AgentMessageChunk(
                        v1::ContentChunk::new(v1::ContentBlock::Text(v1::TextContent::new(
                            "world"
                        )))
                        .message_id("msg_agent")
                    )
                ),
            ]
        );
    }

    #[test]
    fn v2_message_patches_and_clears_do_not_convert_to_v1_chunks() {
        assert_v2_session_update_to_v1_error(
            v2::SessionUpdate::AgentMessage(v2::AgentMessage::new("msg_agent")),
            "v2 SessionUpdate variant `agent_message` without content cannot be represented in v1 chunks",
        );

        assert_v2_session_update_to_v1_error(
            v2::SessionUpdate::AgentMessage(
                v2::AgentMessage::new("msg_agent").content(None::<Vec<v2::ContentBlock>>),
            ),
            "v2 SessionUpdate variant `agent_message` with null content cannot be represented in v1 chunks",
        );

        assert_v2_session_update_to_v1_error(
            v2::SessionUpdate::AgentMessage(
                v2::AgentMessage::new("msg_agent").content(Vec::<v2::ContentBlock>::new()),
            ),
            "v2 SessionUpdate variant `agent_message` with empty content cannot be represented in v1 chunks",
        );

        assert_v2_session_update_to_v1_error(
            v2::SessionUpdate::AgentMessage(
                v2::AgentMessage::new("msg_agent")
                    .content(vec![v2::ContentBlock::Text(v2::TextContent::new("hello"))])
                    .meta(None::<v2::Meta>),
            ),
            "v2 SessionUpdate variant `agent_message` with null _meta cannot be represented in v1 chunks",
        );
    }

    #[test]
    fn v2_tool_call_content_chunk_does_not_convert_to_v1_replacement_update() {
        assert_v2_session_update_to_v1_error(
            v2::SessionUpdate::ToolCallContentChunk(v2::ToolCallContentChunk::new(
                "tc_1",
                v2::ContentBlock::Text(v2::TextContent::new("partial output")),
            )),
            "v2 SessionUpdate variant `tool_call_content_chunk` cannot be represented in v1 because v1 tool-call content updates replace content instead of appending",
        );
    }

    #[test]
    fn v2_terminal_session_updates_do_not_convert_to_v1() {
        assert_v2_session_update_to_v1_error(
            v2::SessionUpdate::TerminalUpdate(v2::TerminalUpdate::new("term_1")),
            "v2 SessionUpdate variant `terminal_update` cannot be represented in v1",
        );
        assert_v2_session_update_to_v1_error(
            v2::SessionUpdate::TerminalOutputChunk(v2::TerminalOutputChunk::new(
                "term_1", "dGVzdAo=",
            )),
            "v2 SessionUpdate variant `terminal_output_chunk` cannot be represented in v1",
        );
    }

    #[test]
    fn v1_content_chunk_without_message_id_does_not_convert_to_v2() {
        assert_v1_to_v2_error(
            v1::ContentChunk::new(v1::ContentBlock::Text(v1::TextContent::new("missing"))),
            "v1 ContentChunk without messageId cannot be represented in v2",
        );
    }

    #[test]
    fn v1_plan_session_update_converts_to_v2_item_plan_update() {
        let update = v1::SessionUpdate::Plan(v1::Plan::new(vec![v1::PlanEntry::new(
            "step",
            v1::PlanEntryPriority::High,
            v1::PlanEntryStatus::InProgress,
        )]));

        let as_v2: v2::SessionUpdate = try_v1_to_v2(update.clone()).unwrap();
        assert_eq!(
            serde_json::to_value(&as_v2).unwrap(),
            serde_json::json!({
                "sessionUpdate": "plan_update",
                "plan": {
                    "type": "items",
                    "planId": LEGACY_V1_PLAN_ID,
                    "entries": [
                        {
                            "content": "step",
                            "priority": "high",
                            "status": "in_progress"
                        }
                    ]
                }
            })
        );

        let back = try_v2_to_v1_many(as_v2).unwrap();
        #[cfg(not(feature = "unstable_plan_operations"))]
        assert_eq!(back, vec![update]);
        #[cfg(feature = "unstable_plan_operations")]
        assert!(matches!(
            back.as_slice(),
            [v1::SessionUpdate::PlanUpdate(_)]
        ));
    }

    #[test]
    fn unknown_v2_session_update_does_not_convert_to_v1() {
        let update = v2::SessionUpdate::Other(v2::OtherSessionUpdate::new(
            "_status_badge",
            std::collections::BTreeMap::new(),
        ));

        assert_v2_session_update_to_v1_error(
            update,
            "v2 SessionUpdate variant `_status_badge` cannot be represented in v1",
        );
    }

    #[test]
    fn v2_state_update_does_not_convert_to_v1() {
        assert_v2_session_update_to_v1_error(
            v2::SessionUpdate::StateUpdate(v2::StateUpdate::Idle(
                v2::IdleStateUpdate::new().stop_reason(v2::StopReason::EndTurn),
            )),
            "v2 SessionUpdate variant `state_update` cannot be represented in v1 because v1 reports completion in the session/prompt response",
        );
    }

    #[test]
    fn v1_current_mode_update_does_not_convert_to_v2() {
        assert_v1_to_v2_error(
            v1::SessionUpdate::CurrentModeUpdate(v1::CurrentModeUpdate::new("ask")),
            "v1 SessionUpdate variant `current_mode_update` cannot be represented in v2",
        );
    }

    #[test]
    fn v1_session_response_modes_do_not_convert_to_v2() {
        assert_v1_to_v2_error(
            v1::NewSessionResponse::new("sess").modes(v1::SessionModeState::new(
                "ask",
                vec![v1::SessionMode::new("ask", "Ask")],
            )),
            "v1 NewSessionResponse.modes cannot be represented in v2",
        );
    }

    #[test]
    fn v1_session_response_missing_config_options_becomes_empty_v2_vec() {
        let new_response: v2::NewSessionResponse =
            try_v1_to_v2(v1::NewSessionResponse::new("sess")).unwrap();
        assert!(new_response.config_options.is_empty());

        let load_response: v2::ResumeSessionResponse =
            try_v1_to_v2(v1::LoadSessionResponse::new()).unwrap();
        assert!(load_response.config_options.is_empty());

        let resume_response: v2::ResumeSessionResponse =
            try_v1_to_v2(v1::ResumeSessionResponse::new()).unwrap();
        assert!(resume_response.config_options.is_empty());

        #[cfg(feature = "unstable_session_fork")]
        {
            let fork_response: v2::ForkSessionResponse =
                try_v1_to_v2(v1::ForkSessionResponse::new("fork")).unwrap();
            assert!(fork_response.config_options.is_empty());
        }
    }

    #[test]
    fn v1_load_session_request_converts_to_v2_resume_replay_from_start() {
        let v2_request: v2::ResumeSessionRequest =
            try_v1_to_v2(v1::LoadSessionRequest::new("sess", "/workspace/project")).unwrap();
        assert!(matches!(
            v2_request.replay_from,
            Some(v2::ReplayFrom::Start(_))
        ));

        let v1_load: v1::LoadSessionRequest = try_v2_to_v1(v2_request).unwrap();
        assert_eq!(v1_load.session_id, v1::SessionId::new("sess"));
        assert_eq!(v1_load.cwd, PathBuf::from("/workspace/project"));
    }

    #[test]
    fn v2_resume_without_replay_maps_to_v1_resume_request() {
        let v1_request: v1::ResumeSessionRequest =
            try_v2_to_v1(v2::ResumeSessionRequest::new("sess", "/workspace/project")).unwrap();
        assert_eq!(v1_request.session_id, v1::SessionId::new("sess"));
        assert_eq!(v1_request.cwd, PathBuf::from("/workspace/project"));
    }

    #[test]
    fn v2_session_response_converts_to_v1_without_mode_state() {
        let response: v1::NewSessionResponse =
            try_v2_to_v1(v2::NewSessionResponse::new("sess")).unwrap();

        assert!(response.modes.is_none());
        assert!(matches!(
            response.config_options,
            Some(config_options) if config_options.is_empty()
        ));
    }

    #[test]
    fn v2_tool_call_update_unrepresentable_fields_do_not_convert_to_v1() {
        let update = v2::ToolCallUpdate::new("tc")
            .kind(v2::ToolKind::Unknown("_future_kind".to_string()))
            .status(v2::ToolCallStatus::Other("_paused".to_string()))
            .content(vec![
                v2::ToolCallContent::Other(v2::OtherToolCallContent::new(
                    "_chart",
                    BTreeMap::default(),
                )),
                v2::ToolCallContent::Diff(v2::Diff::patch(
                    "diff --git /tmp/file.txt /tmp/file.txt\n",
                    vec![v2::DiffChange::modify("/tmp/file.txt")],
                )),
            ]);

        assert_v2_to_v1_error(
            update,
            "v2 ToolKind variant `_future_kind` cannot be represented in v1",
        );
    }

    #[test]
    fn v2_collection_conversion_fails_on_unrepresentable_items() {
        let response = v2::InitializeResponse::new(
            ProtocolVersion::V2,
            v2::Implementation::new("test-agent", "2.0.0"),
        )
        .capabilities(v2::AgentCapabilities::new().session(v2::SessionCapabilities::new()))
        .auth_methods(vec![
            v2::AuthMethod::Other(v2::OtherAuthMethod::new(
                "_oauth",
                "oauth",
                "OAuth",
                BTreeMap::default(),
            )),
            v2::AuthMethod::Agent(v2::AuthMethodAgent::new("agent", "Agent")),
        ]);
        assert_v2_to_v1_error(
            response,
            "v2 AuthMethod variant `_oauth` cannot be represented in v1",
        );

        let config_update = v2::ConfigOptionUpdate::new(vec![
            v2::SessionConfigOption::select(
                "mode",
                "Mode",
                "ask",
                vec![v2::SessionConfigSelectOption::new("ask", "Ask")],
            ),
            v2::SessionConfigOption::new(
                "future",
                "Future",
                v2::SessionConfigKind::Other(v2::OtherSessionConfigKind::new(
                    "_slider",
                    BTreeMap::default(),
                )),
            ),
        ]);
        assert_v2_to_v1_error(
            config_update,
            "v2 SessionConfigKind variant `_slider` cannot be represented in v1",
        );
    }

    #[test]
    fn v2_optional_fields_fail_on_unrepresentable_nested_values() {
        let command = v2::AvailableCommand::new("review", "Review changes").input(
            v2::AvailableCommandInput::Other(v2::OtherAvailableCommandInput::new(
                "_choices",
                BTreeMap::default(),
            )),
        );
        assert_v2_to_v1_error(
            v2::AvailableCommandsUpdate::new(vec![command]),
            "v2 AvailableCommandInput variant `_choices` cannot be represented in v1",
        );

        let content = v2::TextContent::new("hello").annotations(
            v2::Annotations::new()
                .audience(vec![v2::Role::Other("_critic".to_string()), v2::Role::User]),
        );
        assert_v2_to_v1_error(
            content,
            "v2 Role variant `_critic` cannot be represented in v1",
        );
    }

    #[test]
    fn available_command_input_conversion_adds_v2_discriminator() {
        let input = v1::AvailableCommandInput::Unstructured(v1::UnstructuredCommandInput::new(
            "Describe changes",
        ));

        let v2_input: v2::AvailableCommandInput = try_v1_to_v2(input.clone()).unwrap();
        assert_eq!(
            serde_json::to_value(&v2_input).unwrap(),
            serde_json::json!({
                "type": "text",
                "hint": "Describe changes"
            })
        );

        let v1_input: v1::AvailableCommandInput = try_v2_to_v1(v2_input).unwrap();
        assert_eq!(v1_input, input);
        assert_eq!(
            serde_json::to_value(v1_input).unwrap(),
            serde_json::json!({
                "hint": "Describe changes"
            })
        );
    }

    #[test]
    fn v2_plan_entries_fail_on_unrepresentable_items_inside_vectors() {
        let update = v2::PlanUpdate::new(v2::PlanUpdateContent::items(
            "main",
            vec![
                v2::PlanEntry::new(
                    "keep",
                    v2::PlanEntryPriority::High,
                    v2::PlanEntryStatus::Pending,
                ),
                v2::PlanEntry::new(
                    "drop",
                    v2::PlanEntryPriority::Other("_critical".to_string()),
                    v2::PlanEntryStatus::Pending,
                ),
            ],
        ));

        assert_v2_to_v1_error(
            update,
            "v2 PlanEntryPriority variant `_critical` cannot be represented in v1",
        );
    }

    #[test]
    fn v1_tool_call_update_conversion_fails_on_unrepresentable_vec_items() {
        let update = v1::ToolCallUpdate::new(
            "tc",
            v1::ToolCallUpdateFields::new().content(vec![
                v1::ToolCallContent::Terminal(v1::Terminal::new("term")),
                v1::ToolCallContent::Diff(v1::Diff::new("/tmp/file.txt", "new")),
            ]),
        );

        assert_v1_to_v2_error(
            update,
            "v1 ToolCallContent variant `terminal` cannot be represented in v2",
        );
    }

    #[test]
    fn v2_terminal_content_does_not_convert_to_v1_client_terminal_content() {
        assert_v2_to_v1_error(
            v2::ToolCallContent::Terminal(v2::Terminal::new("term_1")),
            "v2 ToolCallContent variant `terminal` cannot be represented in v1 because v1 terminal content refers to a client-created terminal",
        );
    }

    #[test]
    fn v2_command_permission_subject_does_not_convert_to_v1() {
        assert_v2_to_v1_error(
            v2::RequestPermissionRequest::new("session-id", "Run cargo test?", Vec::new()).subject(
                v2::RequestPermissionSubject::from(v2::CommandPermissionSubject::new(
                    "cargo test",
                    "/workspace/project",
                )),
            ),
            "v2 RequestPermissionSubject variant `command` cannot be represented in v1",
        );
    }

    #[test]
    fn v2_resource_link_icons_do_not_convert_to_v1() {
        assert_v2_to_v1_error(
            v2::ResourceLink::new("file.txt", "file:///file.txt")
                .icons(vec![v2::Icon::new("https://example.com/icon.png")]),
            "v2 ResourceLink.icons cannot be represented in v1",
        );
    }

    #[test]
    fn unknown_v2_raw_fallbacks_do_not_convert_to_v1() {
        assert_v2_to_v1_error(
            v2::ContentBlock::Other(v2::OtherContentBlock::new(
                "_widget",
                std::collections::BTreeMap::new(),
            )),
            "v2 ContentBlock variant `_widget` cannot be represented in v1",
        );
        assert_v2_to_v1_error(
            v2::ToolCallContent::Other(v2::OtherToolCallContent::new(
                "_chart",
                std::collections::BTreeMap::new(),
            )),
            "v2 ToolCallContent variant `_chart` cannot be represented in v1",
        );
        assert_v2_to_v1_error(
            v2::AvailableCommandInput::Other(v2::OtherAvailableCommandInput::new(
                "_choices",
                std::collections::BTreeMap::new(),
            )),
            "v2 AvailableCommandInput variant `_choices` cannot be represented in v1",
        );
        assert_v2_to_v1_error(
            v2::RequestPermissionRequest::new("session-id", "Permission requested", Vec::new())
                .subject(v2::RequestPermissionSubject::Other(
                    v2::OtherRequestPermissionSubject::new(
                        "_review",
                        std::collections::BTreeMap::new(),
                    ),
                )),
            "v2 RequestPermissionSubject variant `_review` cannot be represented in v1",
        );
        assert_v2_to_v1_error(
            v2::RequestPermissionRequest::new("session-id", "Permission requested", Vec::new()),
            "v2 RequestPermissionRequest without `subject` cannot be represented in v1",
        );
        assert_v2_to_v1_error(
            v2::RequestPermissionOutcome::Other(v2::OtherRequestPermissionOutcome::new(
                "_defer",
                std::collections::BTreeMap::new(),
            )),
            "v2 RequestPermissionOutcome variant `_defer` cannot be represented in v1",
        );
        assert_v2_to_v1_error(
            v2::SessionConfigKind::Other(v2::OtherSessionConfigKind::new(
                "_slider",
                std::collections::BTreeMap::new(),
            )),
            "v2 SessionConfigKind variant `_slider` cannot be represented in v1",
        );
        assert_v2_to_v1_error(
            v2::AuthMethod::Other(v2::OtherAuthMethod::new(
                "_oauth",
                "oauth",
                "OAuth",
                std::collections::BTreeMap::new(),
            )),
            "v2 AuthMethod variant `_oauth` cannot be represented in v1",
        );
        assert_v2_to_v1_error(
            v2::PlanUpdate::new(v2::PlanUpdateContent::Other(
                v2::OtherPlanUpdateContent::new(
                    "_timeline",
                    "plan-1",
                    std::collections::BTreeMap::new(),
                ),
            )),
            "v2 PlanUpdateContent variant `_timeline` cannot be represented in v1",
        );
        #[cfg(feature = "unstable_nes")]
        assert_v2_to_v1_error(
            v2::NesSuggestion::Other(v2::OtherNesSuggestion::new(
                "_preview",
                "preview-1",
                std::collections::BTreeMap::new(),
            )),
            "v2 NesSuggestion variant `_preview` cannot be represented in v1",
        );
    }

    #[test]
    fn round_trips_request_permission_outcomes() {
        let cancelled = v1::RequestPermissionResponse::new(v1::RequestPermissionOutcome::Cancelled);
        assert_v1_round_trip::<v1::RequestPermissionResponse, v2::RequestPermissionResponse>(
            cancelled,
        );

        let selected = v1::RequestPermissionResponse::new(v1::RequestPermissionOutcome::Selected(
            v1::SelectedPermissionOutcome::new("opt_1"),
        ));
        assert_v1_round_trip::<v1::RequestPermissionResponse, v2::RequestPermissionResponse>(
            selected,
        );
    }

    #[test]
    fn converts_v1_request_permission_request_with_required_v2_title() {
        let titled = v1::RequestPermissionRequest::new(
            "session-id",
            v1::ToolCallUpdate::new("call_1", v1::ToolCallUpdateFields::new().title("Read file")),
            Vec::new(),
        );

        let converted: v2::RequestPermissionRequest = try_v1_to_v2(titled).unwrap();
        assert_eq!(converted.title, "Read file");
        let Some(v2::RequestPermissionSubject::ToolCall(subject)) = converted.subject else {
            panic!("expected tool-call permission subject");
        };
        assert_eq!(subject.tool_call.tool_call_id.to_string(), "call_1");

        let fallback = v1::RequestPermissionRequest::new(
            "session-id",
            v1::ToolCallUpdate::new("call_2", v1::ToolCallUpdateFields::new()),
            Vec::new(),
        );

        assert_v1_to_v2_error(
            fallback,
            "v1 RequestPermissionRequest without a tool-call title cannot be represented in v2",
        );
    }

    #[test]
    fn round_trips_error_with_data_payload() {
        let err = v1::Error::invalid_params().data(serde_json::json!({
            "reason": "missing field",
            "field": "sessionId",
        }));
        assert_v1_round_trip::<v1::Error, v2::Error>(err);
    }

    #[test]
    fn round_trips_v2_value_back_through_v1() {
        // Same coverage but starting from v2, to exercise v2 -> v1 conversion.
        let request = v2::PromptRequest::new(
            "sess_2",
            vec![v2::ContentBlock::Text(v2::TextContent::new("hi"))],
        );
        assert_v2_round_trip::<v2::PromptRequest, v1::PromptRequest>(request.clone());
        assert_json_eq_after_v2_to_v1::<v2::PromptRequest, v1::PromptRequest>(request);
    }

    #[test]
    fn protocol_version_constants_remain_explicit() {
        assert_eq!(ProtocolVersion::V1.as_u16(), 1);
        assert_eq!(ProtocolVersion::V2.as_u16(), 2);
    }

    /// `?` bubbles a [`ProtocolConversionError`] into a [`v1::Error`] without
    /// loss of message, mapped onto the internal-error code.
    #[test]
    fn protocol_conversion_error_maps_into_v1_error() {
        fn run() -> std::result::Result<(), v1::Error> {
            // Synthesize a conversion error so we don't have to wait for v2
            // to actually diverge before exercising the `?` path.
            Err(ProtocolConversionError::new("missing required field"))?;
            unreachable!();
        }

        let err = run().unwrap_err();
        assert_eq!(err.code, v1::ErrorCode::InternalError);
        assert_eq!(
            err.data,
            Some(serde_json::Value::String(
                "missing required field".to_string()
            ))
        );
    }

    /// Mirror of the v1 test for the v2 [`Error`] type.
    #[test]
    fn protocol_conversion_error_maps_into_v2_error() {
        fn run() -> std::result::Result<(), v2::Error> {
            Err(ProtocolConversionError::new("missing required field"))?;
            unreachable!();
        }

        let err = run().unwrap_err();
        assert_eq!(err.code, v2::ErrorCode::InternalError);
        assert_eq!(
            err.data,
            Some(serde_json::Value::String(
                "missing required field".to_string()
            ))
        );
    }
}
