use crate::io::{PgBufMutExt, PortalId, StatementId};
use crate::message::{FrontendMessage, FrontendMessageFormat};
use sqlx_core::Error;
use std::num::Saturating;

const DESCRIBE_PORTAL: u8 = b'P';
const DESCRIBE_STATEMENT: u8 = b'S';

/// Note: will emit both a RowDescription and a ParameterDescription message
#[derive(Debug)]
#[allow(dead_code)]
pub enum Describe {
    Statement(StatementId),
    Portal(PortalId),
}

impl FrontendMessage for Describe {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::Describe;

    fn body_size_hint(&self) -> Saturating<usize> {
        // Either `DESCRIBE_PORTAL` or `DESCRIBE_STATEMENT`
        let mut size = Saturating(1);

        match self {
            Describe::Statement(id) => size += id.name_len(),
            Describe::Portal(id) => size += id.name_len(),
        }

        size
    }

    fn encode_body(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        match self {
            // #[likely]
            Describe::Statement(id) => {
                buf.push(DESCRIBE_STATEMENT);
                buf.put_statement_name(*id);
            }

            Describe::Portal(id) => {
                buf.push(DESCRIBE_PORTAL);
                buf.put_portal_name(*id);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::message::FrontendMessage;

    use super::{Describe, PortalId, StatementId};

    #[test]
    fn test_encode_describe_portal() {
        const EXPECTED: &[u8] = b"D\0\0\0\x17Psqlx_p_1234567890\0";

        let mut buf = Vec::new();
        let m = Describe::Portal(PortalId::TEST_VAL);

        m.encode_msg(&mut buf).unwrap();

        assert_eq!(buf, EXPECTED);
    }

    #[test]
    fn test_encode_describe_unnamed_portal() {
        const EXPECTED: &[u8] = b"D\0\0\0\x06P\0";

        let mut buf = Vec::new();
        let m = Describe::Portal(PortalId::UNNAMED);

        m.encode_msg(&mut buf).unwrap();

        assert_eq!(buf, EXPECTED);
    }

    #[test]
    fn test_encode_describe_statement() {
        const EXPECTED: &[u8] = b"D\0\0\0\x17Ssqlx_s_1234567890\0";

        let mut buf = Vec::new();
        let m = Describe::Statement(StatementId::TEST_VAL);

        m.encode_msg(&mut buf).unwrap();

        assert_eq!(buf, EXPECTED);
    }

    #[test]
    fn test_encode_describe_unnamed_statement() {
        const EXPECTED: &[u8] = b"D\0\0\0\x06S\0";

        let mut buf = Vec::new();
        let m = Describe::Statement(StatementId::UNNAMED);

        m.encode_msg(&mut buf).unwrap();

        assert_eq!(buf, EXPECTED);
    }
}
