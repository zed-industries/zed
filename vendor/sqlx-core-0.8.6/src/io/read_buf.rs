use bytes::{BufMut, BytesMut};

/// An extension for [`BufMut`] for getting a writeable buffer in safe code.
pub trait ReadBuf: BufMut {
    /// Get the full capacity of this buffer as a safely initialized slice.
    fn init_mut(&mut self) -> &mut [u8];
}

impl ReadBuf for &'_ mut [u8] {
    #[inline(always)]
    fn init_mut(&mut self) -> &mut [u8] {
        self
    }
}

impl ReadBuf for BytesMut {
    #[inline(always)]
    fn init_mut(&mut self) -> &mut [u8] {
        // `self.remaining_mut()` returns `usize::MAX - self.len()`
        let remaining = self.capacity() - self.len();

        // I'm hoping for most uses that this operation is elided by the optimizer.
        self.put_bytes(0, remaining);

        self
    }
}

#[test]
fn test_read_buf_bytes_mut() {
    let mut buf = BytesMut::with_capacity(8);
    buf.put_u32(0x12345678);

    assert_eq!(buf.init_mut(), [0x12, 0x34, 0x56, 0x78, 0, 0, 0, 0]);
}
