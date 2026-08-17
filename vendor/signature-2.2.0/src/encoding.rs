//! Encoding support.

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Support for decoding/encoding signatures as bytes.
pub trait SignatureEncoding:
    Clone + Sized + for<'a> TryFrom<&'a [u8]> + TryInto<Self::Repr>
{
    /// Byte representation of a signature.
    type Repr: 'static + AsRef<[u8]> + Clone + Send + Sync;

    /// Encode signature as its byte representation.
    fn to_bytes(&self) -> Self::Repr {
        self.clone()
            .try_into()
            .ok()
            .expect("signature encoding error")
    }

    /// Encode signature as a byte vector.
    #[cfg(feature = "alloc")]
    fn to_vec(&self) -> Vec<u8> {
        self.to_bytes().as_ref().to_vec()
    }

    /// Get the length of this signature when encoded.
    fn encoded_len(&self) -> usize {
        self.to_bytes().as_ref().len()
    }
}
