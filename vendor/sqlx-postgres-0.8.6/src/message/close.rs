use crate::io::{PgBufMutExt, PortalId, StatementId};
use crate::message::{FrontendMessage, FrontendMessageFormat};
use std::num::Saturating;

const CLOSE_PORTAL: u8 = b'P';
const CLOSE_STATEMENT: u8 = b'S';

#[derive(Debug)]
#[allow(dead_code)]
pub enum Close {
    Statement(StatementId),
    Portal(PortalId),
}

impl FrontendMessage for Close {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::Close;

    fn body_size_hint(&self) -> Saturating<usize> {
        // Either `CLOSE_PORTAL` or `CLOSE_STATEMENT`
        let mut size = Saturating(1);

        match self {
            Close::Statement(id) => size += id.name_len(),
            Close::Portal(id) => size += id.name_len(),
        }

        size
    }

    fn encode_body(&self, buf: &mut Vec<u8>) -> Result<(), crate::Error> {
        match self {
            Close::Statement(id) => {
                buf.push(CLOSE_STATEMENT);
                buf.put_statement_name(*id);
            }

            Close::Portal(id) => {
                buf.push(CLOSE_PORTAL);
                buf.put_portal_name(*id);
            }
        }

        Ok(())
    }
}
