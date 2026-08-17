use std::num::Saturating;

use sqlx_core::Error;

use crate::io::{PgBufMutExt, PortalId};
use crate::message::{FrontendMessage, FrontendMessageFormat};

pub struct Execute {
    /// The id of the portal to execute.
    pub portal: PortalId,

    /// Maximum number of rows to return, if portal contains a query
    /// that returns rows (ignored otherwise). Zero denotes “no limit”.
    pub limit: u32,
}

impl FrontendMessage for Execute {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::Execute;

    fn body_size_hint(&self) -> Saturating<usize> {
        let mut size = Saturating(0);

        size += self.portal.name_len();
        size += 2; // limit

        size
    }

    fn encode_body(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        buf.put_portal_name(self.portal);
        buf.extend(&self.limit.to_be_bytes());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::io::PortalId;
    use crate::message::FrontendMessage;

    use super::Execute;

    #[test]
    fn test_encode_execute_named_portal() {
        const EXPECTED: &[u8] = b"E\0\0\0\x1Asqlx_p_1234567890\0\0\0\0\x02";

        let mut buf = Vec::new();
        let m = Execute {
            portal: PortalId::TEST_VAL,
            limit: 2,
        };

        m.encode_msg(&mut buf).unwrap();

        assert_eq!(buf, EXPECTED);
    }

    #[test]
    fn test_encode_execute_unnamed_portal() {
        const EXPECTED: &[u8] = b"E\0\0\0\x09\0\x49\x96\x02\xD2";

        let mut buf = Vec::new();
        let m = Execute {
            portal: PortalId::UNNAMED,
            limit: 1234567890,
        };

        m.encode_msg(&mut buf).unwrap();

        assert_eq!(buf, EXPECTED);
    }
}
