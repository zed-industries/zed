use crate::io::BufMutExt;
use crate::io::{PgBufMutExt, StatementId};
use crate::message::{FrontendMessage, FrontendMessageFormat};
use crate::types::Oid;
use sqlx_core::Error;
use std::num::Saturating;

#[derive(Debug)]
pub struct Parse<'a> {
    /// The ID of the destination prepared statement.
    pub statement: StatementId,

    /// The query string to be parsed.
    pub query: &'a str,

    /// The parameter data types specified (could be zero). Note that this is not an
    /// indication of the number of parameters that might appear in the query string,
    /// only the number that the frontend wants to pre-specify types for.
    pub param_types: &'a [Oid],
}

impl FrontendMessage for Parse<'_> {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::Parse;

    fn body_size_hint(&self) -> Saturating<usize> {
        let mut size = Saturating(0);

        size += self.statement.name_len();

        size += self.query.len();
        size += 1; // NUL terminator

        size += 2; // param_types_len

        // `param_types`
        size += self.param_types.len().saturating_mul(4);

        size
    }

    fn encode_body(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        buf.put_statement_name(self.statement);

        buf.put_str_nul(self.query);

        // Note: actually interpreted as unsigned
        // https://github.com/launchbadge/sqlx/issues/3464
        let param_types_len = u16::try_from(self.param_types.len()).map_err(|_| {
            err_protocol!(
                "param_types.len() too large for binary protocol: {}",
                self.param_types.len()
            )
        })?;

        buf.extend(param_types_len.to_be_bytes());

        for &oid in self.param_types {
            buf.extend(oid.0.to_be_bytes());
        }

        Ok(())
    }
}

#[test]
fn test_encode_parse() {
    const EXPECTED: &[u8] = b"P\0\0\0\x26sqlx_s_1234567890\0SELECT $1\0\0\x01\0\0\0\x19";

    let mut buf = Vec::new();
    let m = Parse {
        statement: StatementId::TEST_VAL,
        query: "SELECT $1",
        param_types: &[Oid(25)],
    };

    m.encode_msg(&mut buf).unwrap();

    assert_eq!(buf, EXPECTED);
}

#[test]
fn test_encode_parse_unnamed_statement() {
    const EXPECTED: &[u8] = b"P\0\0\0\x15\0SELECT $1\0\0\x01\0\0\0\x19";

    let mut buf = Vec::new();
    let m = Parse {
        statement: StatementId::UNNAMED,
        query: "SELECT $1",
        param_types: &[Oid(25)],
    };

    m.encode_msg(&mut buf).unwrap();

    assert_eq!(buf, EXPECTED);
}
