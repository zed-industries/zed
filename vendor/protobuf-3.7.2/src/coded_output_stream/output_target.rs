use std::fmt;
use std::io::Write;

/// Output buffer/writer for `CodedOutputStream`.
pub(crate) enum OutputTarget<'a> {
    Write(&'a mut dyn Write, Vec<u8>),
    Vec(&'a mut Vec<u8>),
    /// The buffer is passed as `&[u8]` to `CodedOutputStream` constructor
    /// and immediately converted to `buffer` field of `CodedOutputStream`,
    /// it is not needed to be stored here.
    /// Lifetime parameter of `CodedOutputStream` guarantees the buffer is valid
    /// during the lifetime of `CodedOutputStream`.
    Bytes,
}

impl<'a> fmt::Debug for OutputTarget<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OutputTarget::Write(_w, vec) => f
                .debug_struct("Write")
                .field("buf_len", &vec.len())
                .field("buf_cap", &vec.capacity())
                .finish_non_exhaustive(),
            OutputTarget::Vec(vec) => f
                .debug_struct("Vec")
                .field("len", &vec.len())
                .field("cap", &vec.capacity())
                .finish_non_exhaustive(),
            OutputTarget::Bytes => f.debug_tuple("Bytes").finish(),
        }
    }
}
