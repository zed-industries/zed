use crate::{Decoder, Encoder};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::io::Error;

const U64_LENGTH: usize = std::mem::size_of::<u64>();

/// A simple `Codec` implementation sending your data by prefixing it by its length.
///
/// # Example
///
/// This codec will most likely be used wrapped in another codec like so.
///
/// ```
/// use asynchronous_codec::{Decoder, Encoder, LengthCodec};
/// use bytes::{Bytes, BytesMut};
/// use std::io::{Error, ErrorKind};
///
/// pub struct MyStringCodec(LengthCodec);
///
/// impl Encoder for MyStringCodec {
///     type Item<'a> = String;
///     type Error = Error;
///
///     fn encode(&mut self, src: Self::Item<'_>, dst: &mut BytesMut) -> Result<(), Self::Error> {
///         let bytes = Bytes::from(src);
///         self.0.encode(bytes, dst)
///     }
/// }
///
/// impl Decoder for MyStringCodec {
///     type Item = String;
///     type Error = Error;
///
///     fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
///         match self.0.decode(src)? {
///             Some(bytes) => {
///                 match String::from_utf8(bytes.to_vec()) {
///                     Ok(string) => Ok(Some(string)),
///                     Err(e) => Err(Error::new(ErrorKind::InvalidData, e))
///                 }
///             },
///             None => Ok(None),
///         }
///     }
/// }
/// ```
pub struct LengthCodec;

impl Encoder for LengthCodec {
    type Item<'a> = Bytes;
    type Error = Error;

    fn encode(&mut self, src: Self::Item<'_>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.reserve(U64_LENGTH + src.len());
        dst.put_u64(src.len() as u64);
        dst.extend_from_slice(&src);
        Ok(())
    }
}

impl Decoder for LengthCodec {
    type Item = Bytes;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < U64_LENGTH {
            return Ok(None);
        }

        let mut len_bytes = [0u8; U64_LENGTH];
        len_bytes.copy_from_slice(&src[..U64_LENGTH]);
        let len = u64::from_be_bytes(len_bytes) as usize;

        if src.len() - U64_LENGTH >= len {
            // Skip the length header we already read.
            src.advance(U64_LENGTH);
            Ok(Some(src.split_to(len).freeze()))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod decode {
        use super::*;

        #[test]
        fn it_returns_bytes_withouth_length_header() {
            let mut codec = LengthCodec {};

            let mut src = BytesMut::with_capacity(5);
            src.put(&[0, 0, 0, 0, 0, 0, 0, 3u8, 1, 2, 3, 4][..]);
            let item = codec.decode(&mut src).unwrap();

            assert!(item == Some(Bytes::from(&[1u8, 2, 3][..])));
        }
    }
}
