//! The [DSIG](https://learn.microsoft.com/en-us/typography/opentype/spec/dsig) table

include!("../../generated/generated_dsig.rs");

impl SignatureRecord {
    /// The signature block enclosed within this record.
    ///
    /// The `data` argument should be retrieved from the parent table
    /// By calling its `offset_data` method.
    ///
    /// Only format 1 is recognised and read successfully.
    pub fn signature_block<'a>(
        &self,
        data: FontData<'a>,
    ) -> Result<SignatureBlockFormat1<'a>, ReadError> {
        match self.format() {
            1 => {
                let signature = self
                    .signature_block_offset()
                    .resolve::<SignatureBlockFormat1>(data)?;

                // Check that the inner block's size matches our length field.
                let actual_len =
                    u32::try_from(signature.compute_len()).map_err(|_| ReadError::OutOfBounds)?;

                if self.length() == actual_len {
                    Ok(signature)
                } else {
                    Err(ReadError::MalformedData("ambiguous DSIG signature length"))
                }
            }
            unknown => Err(ReadError::InvalidFormat(unknown.into())),
        }
    }
}

impl SignatureBlockFormat1<'_> {
    /// Return the exact byte length of this table.
    fn compute_len(&self) -> usize {
        // This format is contiguous, and includes no trailing data.
        self.min_byte_range().len()
    }
}

#[cfg(test)]
mod tests {
    use font_test_data::bebuffer::BeBuffer;

    use super::{Dsig, PermissionFlags};
    use crate::{FontData, FontRead, ReadError};

    /// An empty, dummy DSIG, as inserted by fonttools.
    /// See <https://github.com/fonttools/fonttools/blob/ec716f11851f8d5a04e3f535b53219d97001482a/Lib/fontTools/fontBuilder.py#L823-L833>.
    #[test]
    fn test_empty() {
        let buf = BeBuffer::new()
            .push(0x1u32) // version
            .push(0x0u16) // numSignatures
            .push(0x0u16); // flags

        let dsig = Dsig::read(FontData::new(buf.data())).unwrap();
        assert_eq!(dsig.version(), 1);
        assert_eq!(dsig.signature_records().len(), 0);
        assert_eq!(dsig.flags(), PermissionFlags::empty());
    }

    // A DSIG with a single entry. For ease-of-testing, we use 0xDEADBEEF
    // instead of a full PKCS#7 packet, as it would not be this crate's
    // responsibility to validate it anyway.
    #[test]
    fn test_beef() {
        let buf = BeBuffer::new()
            .push(0x1_u32) // DsigHeader.version
            .push(0x1_u16) // DsigHeader.numSignatures
            .push(0x1_u16) // DsigHeader.flags
            .push(0x1_u32) // SignatureRecord.format
            .push(0xC_u32) // SignatureRecord.length
            .push(0x14_u32) // SignatureRecord.signatureBlockOffset
            .push(0x0_u16) // SignatureBlockFormat1.reserved1
            .push(0x0_u16) // SignatureBlockFormat1.reserved2
            .push(0x4_u32) // SignatureBlockFormat1.signatureLength
            .push(0xDEADBEEF_u32); // SignatureBlockFormat1.signature

        let data = FontData::new(buf.data());

        let dsig = Dsig::read(data).unwrap();

        assert_eq!(dsig.version(), 1);
        assert_eq!(dsig.signature_records().len(), 1);
        assert_eq!(dsig.flags(), PermissionFlags::CANNOT_BE_RESIGNED);

        let record = dsig.signature_records()[0];
        assert_eq!(record.format(), 1);

        let block = record.signature_block(data).unwrap();
        assert_eq!(block.signature(), &[0xDE, 0xAD, 0xBE, 0xEF]);
    }

    // A DSIG with a single entry, but in a format we do not recognise; this
    // should fail to read.
    #[test]
    fn test_unknown_format() {
        let buf = BeBuffer::new()
            .push(0x1_u32) // DsigHeader.version
            .push(0x1_u16) // DsigHeader.numSignatures
            .push(0x1_u16) // DsigHeader.flags
            .push(0x2_u32) // SignatureRecord.format, BUT AN UNRECOGNISED ONE
            .push(0xC_u32) // SignatureRecord.length
            .push(0x14_u32) // SignatureRecord.signatureBlockOffset
            .push(0x0_u16) // SignatureBlockFormat1.reserved1
            .push(0x0_u16) // SignatureBlockFormat1.reserved2
            .push(0x4_u32) // SignatureBlockFormat1.signatureLength
            .push(0xDEADBEEF_u32); // SignatureBlockFormat1.signature

        let data = FontData::new(buf.data());

        // Load the first record.
        let dsig = Dsig::read(data).unwrap();

        assert_eq!(dsig.signature_records().len(), 1);
        let record = dsig.signature_records()[0];

        // Assert that it is the unknown format '2', and that it fails to be read.
        assert_eq!(record.format(), 2);

        let block_attempt = record.signature_block(data);
        assert_eq!(block_attempt.err(), Some(ReadError::InvalidFormat(2)));
    }

    // A DSIG with a single entry, whose inner block has a different length to
    // that which the outer record's length field prescribes; this is ambiguous,
    // and so should fail to read.
    #[test]
    fn test_lying_length() {
        let buf = BeBuffer::new()
            .push(0x1_u32) // DsigHeader.version
            .push(0x1_u16) // DsigHeader.numSignatures
            .push(0x1_u16) // DsigHeader.flags
            .push(0x1_u32) // SignatureRecord.format
            .push(0xB_u32) // SignatureRecord.length, BUT ONE LESS THAN THE INNER LENGTH REQUIRES
            .push(0x14_u32) // SignatureRecord.signatureBlockOffset
            .push(0x0_u16) // SignatureBlockFormat1.reserved1
            .push(0x0_u16) // SignatureBlockFormat1.reserved2
            .push(0x4_u32) // SignatureBlockFormat1.signatureLength
            .push(0xDEADBEEF_u32); // SignatureBlockFormat1.signature

        let data = FontData::new(buf.data());

        // Load the first record.
        let dsig = Dsig::read(data).unwrap();

        assert_eq!(dsig.signature_records().len(), 1);
        let record = dsig.signature_records()[0];

        // Assert that we yield an error, because the inner length requires more
        // bytes than the outer length prescribes.
        let block_attempt = record.signature_block(data);
        assert_eq!(
            block_attempt.err(),
            Some(ReadError::MalformedData("ambiguous DSIG signature length"))
        );
    }
}
