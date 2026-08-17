use crate::io::{PortalId, StatementId};

pub trait PgBufMutExt {
    fn put_length_prefixed<F>(&mut self, f: F) -> Result<(), crate::Error>
    where
        F: FnOnce(&mut Vec<u8>) -> Result<(), crate::Error>;

    fn put_statement_name(&mut self, id: StatementId);

    fn put_portal_name(&mut self, id: PortalId);
}

impl PgBufMutExt for Vec<u8> {
    // writes a length-prefixed message, this is used when encoding nearly all messages as postgres
    // wants us to send the length of the often-variable-sized messages up front
    fn put_length_prefixed<F>(&mut self, write_contents: F) -> Result<(), crate::Error>
    where
        F: FnOnce(&mut Vec<u8>) -> Result<(), crate::Error>,
    {
        // reserve space to write the prefixed length
        let offset = self.len();
        self.extend(&[0; 4]);

        // write the main body of the message
        let write_result = write_contents(self);

        let size_result = write_result.and_then(|_| {
            let size = self.len() - offset;
            i32::try_from(size)
                .map_err(|_| err_protocol!("message size out of range for protocol: {size}"))
        });

        match size_result {
            Ok(size) => {
                // now calculate the size of what we wrote and set the length value
                self[offset..(offset + 4)].copy_from_slice(&size.to_be_bytes());
                Ok(())
            }
            Err(e) => {
                // Put the buffer back to where it was.
                self.truncate(offset);
                Err(e)
            }
        }
    }

    // writes a statement name by ID
    #[inline]
    fn put_statement_name(&mut self, id: StatementId) {
        id.put_name_with_nul(self);
    }

    // writes a portal name by ID
    #[inline]
    fn put_portal_name(&mut self, id: PortalId) {
        id.put_name_with_nul(self);
    }
}
