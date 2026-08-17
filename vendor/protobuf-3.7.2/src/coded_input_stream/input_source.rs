#[cfg(feature = "bytes")]
use bytes::Bytes;

use crate::coded_input_stream::buf_read_or_reader::BufReadOrReader;

/// Hold all possible combinations of input source
#[derive(Debug)]
pub(crate) enum InputSource<'a> {
    Read(BufReadOrReader<'a>),
    #[allow(dead_code)] // Keep the field to clarify we logically hold the reference.
    Slice(&'a [u8]),
    #[cfg(feature = "bytes")]
    Bytes(&'a Bytes),
}
