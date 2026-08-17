use crate::message::{BackendMessage, BackendMessageFormat};
use sqlx_core::bytes::Bytes;
use sqlx_core::Error;

pub struct ParseComplete;

impl BackendMessage for ParseComplete {
    const FORMAT: BackendMessageFormat = BackendMessageFormat::ParseComplete;

    fn decode_body(_bytes: Bytes) -> Result<Self, Error> {
        Ok(ParseComplete)
    }
}
