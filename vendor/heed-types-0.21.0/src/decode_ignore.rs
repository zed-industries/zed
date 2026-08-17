use heed_traits::BoxedError;

/// A convenient struct made to ignore the type when decoding it.
///
/// For example, it is appropriate to be used to count keys or to ensure that an
/// entry exists.
pub enum DecodeIgnore {}

impl heed_traits::BytesDecode<'_> for DecodeIgnore {
    type DItem = ();

    fn bytes_decode(_bytes: &[u8]) -> Result<Self::DItem, BoxedError> {
        Ok(())
    }
}
