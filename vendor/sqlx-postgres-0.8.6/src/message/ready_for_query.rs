use sqlx_core::bytes::Bytes;

use crate::error::Error;
use crate::message::{BackendMessage, BackendMessageFormat};

#[derive(Debug)]
#[repr(u8)]
pub enum TransactionStatus {
    /// Not in a transaction block.
    Idle = b'I',

    /// In a transaction block.
    Transaction = b'T',

    /// In a _failed_ transaction block. Queries will be rejected until block is ended.
    Error = b'E',
}

#[derive(Debug)]
pub struct ReadyForQuery {
    pub transaction_status: TransactionStatus,
}

impl BackendMessage for ReadyForQuery {
    const FORMAT: BackendMessageFormat = BackendMessageFormat::ReadyForQuery;

    fn decode_body(buf: Bytes) -> Result<Self, Error> {
        let status = match buf[0] {
            b'I' => TransactionStatus::Idle,
            b'T' => TransactionStatus::Transaction,
            b'E' => TransactionStatus::Error,

            status => {
                return Err(err_protocol!(
                    "unknown transaction status: {:?}",
                    status as char
                ));
            }
        };

        Ok(Self {
            transaction_status: status,
        })
    }
}

#[test]
fn test_decode_ready_for_query() -> Result<(), Error> {
    const DATA: &[u8] = b"E";

    let m = ReadyForQuery::decode_body(Bytes::from_static(DATA))?;

    assert!(matches!(m.transaction_status, TransactionStatus::Error));

    Ok(())
}
