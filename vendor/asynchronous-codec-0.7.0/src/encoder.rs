use super::fuse::Fuse;
use bytes::BytesMut;
use std::io::Error;

/// Encoding of messages as bytes, for use with `FramedWrite`.
pub trait Encoder {
    /// The type of items consumed by `encode`
    type Item<'a>;
    /// The type of encoding errors.
    type Error: From<Error>;

    /// Encodes an item into the `BytesMut` provided by dst.
    fn encode(&mut self, item: Self::Item<'_>, dst: &mut BytesMut) -> Result<(), Self::Error>;
}

impl<T, U: Encoder> Encoder for Fuse<T, U> {
    type Item<'a> = U::Item<'a>;
    type Error = U::Error;

    fn encode(&mut self, item: Self::Item<'_>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        self.u.encode(item, dst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FramedWrite;
    use futures::executor::block_on;
    use futures_util::SinkExt;

    #[test]
    fn can_use_borrowed_data() {
        let mut buf = Vec::new();

        let mut write = FramedWrite::new(&mut buf, BorrowedCodec);
        block_on(write.send(&[1, 2, 3, 4])).unwrap();

        assert_eq!(buf, vec![1, 2, 3, 4])
    }

    struct BorrowedCodec;

    impl Encoder for BorrowedCodec {
        type Item<'a> = &'a [u8];
        type Error = Error;

        fn encode(&mut self, item: Self::Item<'_>, dst: &mut BytesMut) -> Result<(), Self::Error> {
            dst.extend_from_slice(item);

            Ok(())
        }
    }
}
