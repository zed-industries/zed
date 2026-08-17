pub(crate) mod decode;
pub(crate) mod encode;
pub(crate) mod generic;

/// Encoded varint message is not longer than 10 bytes.
pub(crate) const MAX_VARINT_ENCODED_LEN: usize = 10;
pub(crate) const MAX_VARINT32_ENCODED_LEN: usize = 5;
