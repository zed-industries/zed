use crate::buffer::Buffer;
use crate::format::Format;
use crate::sealed::Sealed;

/// Marker trait for number types that can be formatted without heap allocation (see [`Buffer`]).
///
/// This trait is sealed; so you may not implement it on your own types.
///
/// [`Buffer`]: struct.Buffer.html
pub trait ToFormattedStr: Sealed + Sized {
    #[doc(hidden)]
    fn read_to_buffer<F>(&self, buf: &mut Buffer, format: &F) -> usize
    where
        F: Format;
}
