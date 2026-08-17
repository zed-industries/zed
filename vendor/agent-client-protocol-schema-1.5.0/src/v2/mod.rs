//! Agent Client Protocol version 2 draft types.
//!
//! **EXPERIMENTAL.** This module is gated behind the `unstable_protocol_v2`
//! feature, is not part of the [`unstable`] umbrella, and must be selected
//! explicitly with [`crate::ProtocolVersion::V2`]. The wire format is
//! currently identical to v1 (the default crate-root types) and the types here
//! exist only as a place to evolve v2 without disturbing the stable v1 API. The
//! wire format intentionally diverges from v1 as draft v2 RFDs land. Both the
//! type definitions and the [`conversion`] helpers may change at any time.
//!
//! [`unstable`]: https://docs.rs/crate/agent-client-protocol-schema/latest/features

mod agent;
mod client;
mod content;
pub mod conversion;
#[cfg(feature = "unstable_elicitation")]
mod elicitation;
mod error;
mod ext;
#[cfg(feature = "unstable_mcp_over_acp")]
mod mcp;
#[cfg(feature = "unstable_nes")]
mod nes;
mod plan;
mod protocol_level;
pub(crate) mod schema_util;
mod terminal;
mod tool_call;

pub use crate::rpc::{JsonRpcBatch, JsonRpcMessage, Notification, Request, RequestId};
pub use agent::*;
pub use client::*;
pub use content::*;
use derive_more::{Display, From};
#[cfg(feature = "unstable_elicitation")]
pub use elicitation::*;
pub use error::*;
pub use ext::*;
#[cfg(feature = "unstable_mcp_over_acp")]
pub use mcp::*;
#[cfg(feature = "unstable_nes")]
pub use nes::*;
pub use plan::*;
pub use protocol_level::*;
pub use serde_json::value::RawValue;
pub use terminal::*;
pub use tool_call::*;

/// JSON-RPC response envelope using this protocol version's error type.
pub type Response<Result> = crate::rpc::Response<Result, Error>;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
    sync::Arc,
};

/// An absolute filesystem path used by the protocol.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, From)]
#[serde(transparent)]
#[from(forward)]
#[non_exhaustive]
pub struct AbsolutePath(pub PathBuf);

impl AbsolutePath {
    /// Wraps a filesystem path as a typed [`AbsolutePath`].
    #[must_use]
    pub fn new(path: impl Into<Self>) -> Self {
        path.into()
    }

    /// Returns the wrapped filesystem path.
    #[must_use]
    pub fn into_inner(self) -> PathBuf {
        self.0
    }
}

impl AsRef<Path> for AbsolutePath {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

impl AsRef<OsStr> for AbsolutePath {
    fn as_ref(&self) -> &OsStr {
        self.0.as_os_str()
    }
}

macro_rules! impl_into_option_conversion {
    ($target:ty, $source:ty) => {
        impl crate::IntoOption<$target> for $source {
            fn into_option(self) -> Option<$target> {
                Some(self.into())
            }
        }
    };
}

macro_rules! impl_into_maybe_undefined_conversion {
    ($target:ty, $source:ty) => {
        impl crate::IntoMaybeUndefined<$target> for $source {
            fn into_maybe_undefined(self) -> crate::MaybeUndefined<$target> {
                crate::MaybeUndefined::Value(self.into())
            }
        }
    };
}

impl_into_option_conversion!(AbsolutePath, PathBuf);
impl_into_option_conversion!(AbsolutePath, OsString);
impl_into_option_conversion!(AbsolutePath, String);
impl_into_option_conversion!(AbsolutePath, Box<Path>);
impl_into_option_conversion!(AbsolutePath, Cow<'_, Path>);
impl_into_maybe_undefined_conversion!(AbsolutePath, PathBuf);
impl_into_maybe_undefined_conversion!(AbsolutePath, OsString);
impl_into_maybe_undefined_conversion!(AbsolutePath, String);
impl_into_maybe_undefined_conversion!(AbsolutePath, Box<Path>);
impl_into_maybe_undefined_conversion!(AbsolutePath, Cow<'_, Path>);

impl<T: ?Sized + AsRef<OsStr>> crate::IntoOption<AbsolutePath> for &T {
    fn into_option(self) -> Option<AbsolutePath> {
        Some(self.into())
    }
}

impl<T: ?Sized + AsRef<OsStr>> crate::IntoMaybeUndefined<AbsolutePath> for &T {
    fn into_maybe_undefined(self) -> crate::MaybeUndefined<AbsolutePath> {
        crate::MaybeUndefined::Value(self.into())
    }
}
/// A unique identifier for a conversation session between a client and agent.
///
/// Sessions maintain their own context, conversation history, and state,
/// allowing multiple independent interactions with the same agent.
///
/// See protocol docs: [Session ID](https://agentclientprotocol.com/protocol/session-setup#session-id)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Display, From)]
#[serde(transparent)]
#[from(forward)]
#[non_exhaustive]
pub struct SessionId(pub Arc<str>);

impl SessionId {
    /// Wraps a protocol string as a typed [`SessionId`].
    #[must_use]
    pub fn new(id: impl Into<Self>) -> Self {
        id.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_newtype_builders_remain_ergonomic() {
        let request = NewSessionRequest::new("/workspace")
            .additional_directories(["/workspace/shared", "/workspace/docs"]);
        assert_eq!(request.cwd, AbsolutePath::new("/workspace"));
        assert_eq!(
            <AbsolutePath as AsRef<Path>>::as_ref(&request.cwd),
            Path::new("/workspace")
        );
        assert_eq!(
            <AbsolutePath as AsRef<OsStr>>::as_ref(&request.cwd),
            OsStr::new("/workspace")
        );

        let list = ListSessionsRequest::new()
            .cwd("/workspace")
            .cursor("next-page");
        assert_eq!(list.cursor, Some(SessionListCursor::new("next-page")));
        assert_eq!(
            <SessionListCursor as AsRef<str>>::as_ref(list.cursor.as_ref().unwrap()),
            "next-page"
        );

        let image = ImageContent::new("aGVsbG8=", "image/png");
        assert_eq!(image.mime_type, MediaType::new("image/png"));
        assert_eq!(
            <MediaType as AsRef<str>>::as_ref(&image.mime_type),
            "image/png"
        );

        let session_id_source = String::from("session-1");
        let session_id = SessionId::new(session_id_source.as_str());
        assert_eq!(SessionId::new(session_id).to_string(), "session-1");

        let os_path = OsString::from("/workspace");
        assert_eq!(
            ListSessionsRequest::new().cwd(&os_path).cwd,
            Some(AbsolutePath::new(&os_path))
        );
        drop(TerminalUpdate::new("terminal-1").cwd(&os_path));

        drop(ListSessionsRequest::new().cwd(None).cursor(None));
        drop(ListSessionsResponse::new(Vec::new()).next_cursor(None));
        drop(Icon::new("https://example.com/icon.png").mime_type(None));
        drop(TerminalUpdate::new("terminal-1").cwd(None));
    }
}
