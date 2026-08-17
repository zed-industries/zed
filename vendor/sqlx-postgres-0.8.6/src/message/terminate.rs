use crate::message::{FrontendMessage, FrontendMessageFormat};
use sqlx_core::Error;
use std::num::Saturating;

pub struct Terminate;

impl FrontendMessage for Terminate {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::Terminate;

    #[inline(always)]
    fn body_size_hint(&self) -> Saturating<usize> {
        Saturating(0)
    }

    #[inline(always)]
    fn encode_body(&self, _buf: &mut Vec<u8>) -> Result<(), Error> {
        Ok(())
    }
}
