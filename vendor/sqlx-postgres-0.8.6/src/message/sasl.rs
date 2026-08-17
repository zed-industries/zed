use crate::io::BufMutExt;
use crate::message::{FrontendMessage, FrontendMessageFormat};
use sqlx_core::Error;
use std::num::Saturating;

pub struct SaslInitialResponse<'a> {
    pub response: &'a str,
    pub plus: bool,
}

impl SaslInitialResponse<'_> {
    #[inline(always)]
    fn selected_mechanism(&self) -> &'static str {
        if self.plus {
            "SCRAM-SHA-256-PLUS"
        } else {
            "SCRAM-SHA-256"
        }
    }
}

impl FrontendMessage for SaslInitialResponse<'_> {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::PasswordPolymorphic;

    #[inline(always)]
    fn body_size_hint(&self) -> Saturating<usize> {
        let mut size = Saturating(0);

        size += self.selected_mechanism().len();
        size += 1; // NUL terminator

        size += 4; // response_len
        size += self.response.len();

        size
    }

    fn encode_body(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        // name of the SASL authentication mechanism that the client selected
        buf.put_str_nul(self.selected_mechanism());

        let response_len = i32::try_from(self.response.len()).map_err(|_| {
            err_protocol!(
                "SASL Initial Response length too long for protcol: {}",
                self.response.len()
            )
        })?;

        buf.extend_from_slice(&response_len.to_be_bytes());
        buf.extend_from_slice(self.response.as_bytes());

        Ok(())
    }
}

pub struct SaslResponse<'a>(pub &'a str);

impl FrontendMessage for SaslResponse<'_> {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::PasswordPolymorphic;

    fn body_size_hint(&self) -> Saturating<usize> {
        Saturating(self.0.len())
    }

    fn encode_body(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        buf.extend(self.0.as_bytes());
        Ok(())
    }
}
