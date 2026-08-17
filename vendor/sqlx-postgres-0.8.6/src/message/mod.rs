use sqlx_core::bytes::Bytes;
use std::num::Saturating;

use crate::error::Error;
use crate::io::PgBufMutExt;

mod authentication;
mod backend_key_data;
mod bind;
mod close;
mod command_complete;
mod copy;
mod data_row;
mod describe;
mod execute;
mod flush;
mod notification;
mod parameter_description;
mod parameter_status;
mod parse;
mod parse_complete;
mod password;
mod query;
mod ready_for_query;
mod response;
mod row_description;
mod sasl;
mod ssl_request;
mod startup;
mod sync;
mod terminate;

pub use authentication::{Authentication, AuthenticationSasl};
pub use backend_key_data::BackendKeyData;
pub use bind::Bind;
pub use close::Close;
pub use command_complete::CommandComplete;
pub use copy::{CopyData, CopyDone, CopyFail, CopyInResponse, CopyOutResponse, CopyResponseData};
pub use data_row::DataRow;
pub use describe::Describe;
pub use execute::Execute;
#[allow(unused_imports)]
pub use flush::Flush;
pub use notification::Notification;
pub use parameter_description::ParameterDescription;
pub use parameter_status::ParameterStatus;
pub use parse::Parse;
pub use parse_complete::ParseComplete;
pub use password::Password;
pub use query::Query;
pub use ready_for_query::{ReadyForQuery, TransactionStatus};
pub use response::{Notice, PgSeverity};
pub use row_description::RowDescription;
pub use sasl::{SaslInitialResponse, SaslResponse};
use sqlx_core::io::ProtocolEncode;
pub use ssl_request::SslRequest;
pub use startup::Startup;
pub use sync::Sync;
pub use terminate::Terminate;

// Note: we can't use the same enum for both frontend and backend message formats
// because there are duplicated format codes between them.
//
// For example, `Close` (frontend) and `CommandComplete` (backend) both use format code `C`.
// <https://www.postgresql.org/docs/current/protocol-message-formats.html>
#[derive(Debug, PartialOrd, PartialEq)]
#[repr(u8)]
pub enum FrontendMessageFormat {
    Bind = b'B',
    Close = b'C',
    CopyData = b'd',
    CopyDone = b'c',
    CopyFail = b'f',
    Describe = b'D',
    Execute = b'E',
    Flush = b'H',
    Parse = b'P',
    /// This message format is polymorphic. It's used for:
    ///
    /// * Plain password responses
    /// * MD5 password responses
    /// * SASL responses
    /// * GSSAPI/SSPI responses
    PasswordPolymorphic = b'p',
    Query = b'Q',
    Sync = b'S',
    Terminate = b'X',
}

#[derive(Debug, PartialOrd, PartialEq)]
#[repr(u8)]
pub enum BackendMessageFormat {
    Authentication,
    BackendKeyData,
    BindComplete,
    CloseComplete,
    CommandComplete,
    CopyData,
    CopyDone,
    CopyInResponse,
    CopyOutResponse,
    DataRow,
    EmptyQueryResponse,
    ErrorResponse,
    NoData,
    NoticeResponse,
    NotificationResponse,
    ParameterDescription,
    ParameterStatus,
    ParseComplete,
    PortalSuspended,
    ReadyForQuery,
    RowDescription,
}

#[derive(Debug)]
pub struct ReceivedMessage {
    pub format: BackendMessageFormat,
    pub contents: Bytes,
}

impl ReceivedMessage {
    #[inline]
    pub fn decode<T>(self) -> Result<T, Error>
    where
        T: BackendMessage,
    {
        if T::FORMAT != self.format {
            return Err(err_protocol!(
                "Postgres protocol error: expected {:?}, got {:?}",
                T::FORMAT,
                self.format
            ));
        }

        T::decode_body(self.contents).map_err(|e| match e {
            Error::Protocol(s) => {
                err_protocol!("Postgres protocol error (reading {:?}): {s}", self.format)
            }
            other => other,
        })
    }
}

impl BackendMessageFormat {
    pub fn try_from_u8(v: u8) -> Result<Self, Error> {
        // https://www.postgresql.org/docs/current/protocol-message-formats.html

        Ok(match v {
            b'1' => BackendMessageFormat::ParseComplete,
            b'2' => BackendMessageFormat::BindComplete,
            b'3' => BackendMessageFormat::CloseComplete,
            b'C' => BackendMessageFormat::CommandComplete,
            b'd' => BackendMessageFormat::CopyData,
            b'c' => BackendMessageFormat::CopyDone,
            b'G' => BackendMessageFormat::CopyInResponse,
            b'H' => BackendMessageFormat::CopyOutResponse,
            b'D' => BackendMessageFormat::DataRow,
            b'E' => BackendMessageFormat::ErrorResponse,
            b'I' => BackendMessageFormat::EmptyQueryResponse,
            b'A' => BackendMessageFormat::NotificationResponse,
            b'K' => BackendMessageFormat::BackendKeyData,
            b'N' => BackendMessageFormat::NoticeResponse,
            b'R' => BackendMessageFormat::Authentication,
            b'S' => BackendMessageFormat::ParameterStatus,
            b'T' => BackendMessageFormat::RowDescription,
            b'Z' => BackendMessageFormat::ReadyForQuery,
            b'n' => BackendMessageFormat::NoData,
            b's' => BackendMessageFormat::PortalSuspended,
            b't' => BackendMessageFormat::ParameterDescription,

            _ => return Err(err_protocol!("unknown message type: {:?}", v as char)),
        })
    }
}

pub(crate) trait FrontendMessage: Sized {
    /// The format prefix of this message.
    const FORMAT: FrontendMessageFormat;

    /// Return the amount of space, in bytes, to reserve in the buffer passed to [`Self::encode_body()`].
    fn body_size_hint(&self) -> Saturating<usize>;

    /// Encode this type as a Frontend message in the Postgres protocol.
    ///
    /// The implementation should *not* include `Self::FORMAT` or the length prefix.
    fn encode_body(&self, buf: &mut Vec<u8>) -> Result<(), Error>;

    #[inline(always)]
    #[cfg_attr(not(test), allow(dead_code))]
    fn encode_msg(self, buf: &mut Vec<u8>) -> Result<(), Error> {
        EncodeMessage(self).encode(buf)
    }
}

pub(crate) trait BackendMessage: Sized {
    /// The expected message format.
    ///
    /// <https://www.postgresql.org/docs/current/protocol-message-formats.html>
    const FORMAT: BackendMessageFormat;

    /// Decode this type from a Backend message in the Postgres protocol.
    ///
    /// The format code and length prefix have already been read and are not at the start of `bytes`.
    fn decode_body(buf: Bytes) -> Result<Self, Error>;
}

pub struct EncodeMessage<F>(pub F);

impl<F: FrontendMessage> ProtocolEncode<'_, ()> for EncodeMessage<F> {
    fn encode_with(&self, buf: &mut Vec<u8>, _context: ()) -> Result<(), Error> {
        let mut size_hint = self.0.body_size_hint();
        // plus format code and length prefix
        size_hint += 5;

        // don't panic if `size_hint` is ridiculous
        buf.try_reserve(size_hint.0).map_err(|e| {
            err_protocol!(
                "Postgres protocol: error allocating {} bytes for encoding message {:?}: {e}",
                size_hint.0,
                F::FORMAT,
            )
        })?;

        buf.push(F::FORMAT as u8);

        buf.put_length_prefixed(|buf| self.0.encode_body(buf))
    }
}
