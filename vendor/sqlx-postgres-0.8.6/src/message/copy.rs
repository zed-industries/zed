use crate::error::Result;
use crate::io::BufMutExt;
use crate::message::{
    BackendMessage, BackendMessageFormat, FrontendMessage, FrontendMessageFormat,
};
use sqlx_core::bytes::{Buf, Bytes};
use sqlx_core::Error;
use std::num::Saturating;
use std::ops::Deref;

/// The same structure is sent for both `CopyInResponse` and `CopyOutResponse`
pub struct CopyResponseData {
    pub format: i8,
    pub num_columns: i16,
    pub format_codes: Vec<i16>,
}

pub struct CopyInResponse(pub CopyResponseData);

#[allow(dead_code)]
pub struct CopyOutResponse(pub CopyResponseData);

pub struct CopyData<B>(pub B);

pub struct CopyFail {
    pub message: String,
}

pub struct CopyDone;

impl CopyResponseData {
    #[inline]
    fn decode(mut buf: Bytes) -> Result<Self> {
        let format = buf.get_i8();
        let num_columns = buf.get_i16();

        let format_codes = (0..num_columns).map(|_| buf.get_i16()).collect();

        Ok(CopyResponseData {
            format,
            num_columns,
            format_codes,
        })
    }
}

impl BackendMessage for CopyInResponse {
    const FORMAT: BackendMessageFormat = BackendMessageFormat::CopyInResponse;

    #[inline(always)]
    fn decode_body(buf: Bytes) -> std::result::Result<Self, Error> {
        Ok(Self(CopyResponseData::decode(buf)?))
    }
}

impl BackendMessage for CopyOutResponse {
    const FORMAT: BackendMessageFormat = BackendMessageFormat::CopyOutResponse;

    #[inline(always)]
    fn decode_body(buf: Bytes) -> std::result::Result<Self, Error> {
        Ok(Self(CopyResponseData::decode(buf)?))
    }
}

impl BackendMessage for CopyData<Bytes> {
    const FORMAT: BackendMessageFormat = BackendMessageFormat::CopyData;

    #[inline(always)]
    fn decode_body(buf: Bytes) -> std::result::Result<Self, Error> {
        Ok(Self(buf))
    }
}

impl<B: Deref<Target = [u8]>> FrontendMessage for CopyData<B> {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::CopyData;

    #[inline(always)]
    fn body_size_hint(&self) -> Saturating<usize> {
        Saturating(self.0.len())
    }

    #[inline(always)]
    fn encode_body(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        buf.extend_from_slice(&self.0);
        Ok(())
    }
}

impl FrontendMessage for CopyFail {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::CopyFail;

    #[inline(always)]
    fn body_size_hint(&self) -> Saturating<usize> {
        Saturating(self.message.len())
    }

    #[inline(always)]
    fn encode_body(&self, buf: &mut Vec<u8>) -> std::result::Result<(), Error> {
        buf.put_str_nul(&self.message);
        Ok(())
    }
}

impl CopyFail {
    #[inline(always)]
    pub fn new(msg: impl Into<String>) -> CopyFail {
        CopyFail {
            message: msg.into(),
        }
    }
}

impl FrontendMessage for CopyDone {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::CopyDone;
    #[inline(always)]
    fn body_size_hint(&self) -> Saturating<usize> {
        Saturating(0)
    }

    #[inline(always)]
    fn encode_body(&self, _buf: &mut Vec<u8>) -> std::result::Result<(), Error> {
        Ok(())
    }
}

impl BackendMessage for CopyDone {
    const FORMAT: BackendMessageFormat = BackendMessageFormat::CopyDone;

    #[inline(always)]
    fn decode_body(bytes: Bytes) -> std::result::Result<Self, Error> {
        if !bytes.is_empty() {
            // Not fatal but may indicate a protocol change
            tracing::debug!(
                "Postgres backend returned non-empty message for CopyDone: \"{}\"",
                bytes.escape_ascii()
            )
        }

        Ok(CopyDone)
    }
}
