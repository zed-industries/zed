use std::fmt;
use std::fmt::Formatter;
use std::mem::MaybeUninit;
use std::slice;

use crate::misc::maybe_uninit_write_slice;

pub(crate) struct OutputBuffer {
    // Actual buffer is owned by `OutputTarget`,
    // and here we alias the buffer so access to the buffer is branchless:
    // access does not require switch by actual target type: `&[], `Vec`, `Write` etc.
    // We don't access the actual buffer in `OutputTarget` except when
    // we initialize `buffer` field here.
    buffer: *mut [MaybeUninit<u8>],
    /// Position within the buffer.
    /// Always correct.
    pos_within_buf: usize,
}

impl fmt::Debug for OutputBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("OutputBuffer")
            .field("buffer.len", &self.buffer().len())
            .field("pos_within_buf", &self.pos_within_buf)
            .finish()
    }
}

impl OutputBuffer {
    #[inline]
    pub(crate) fn new(buffer: *mut [MaybeUninit<u8>]) -> OutputBuffer {
        Self {
            buffer,
            pos_within_buf: 0,
        }
    }

    /// Whole buffer: written data + unwritten data.
    #[inline]
    pub(crate) fn buffer(&self) -> &[MaybeUninit<u8>] {
        unsafe { &*self.buffer }
    }

    #[inline]
    fn buffer_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        unsafe { &mut *self.buffer }
    }

    #[inline]
    pub(crate) fn pos_within_buf(&self) -> usize {
        self.pos_within_buf
    }

    #[inline]
    pub(crate) fn filled(&self) -> &[u8] {
        // SAFETY: This type invariant is data is filled up to `pos_within_buf`.
        unsafe { slice::from_raw_parts_mut(self.buffer as *mut u8, self.pos_within_buf) }
    }

    #[inline]
    pub(crate) fn unfilled(&mut self) -> &mut [MaybeUninit<u8>] {
        // SAFETY: This type invariant is `pos_within_buf` is smaller than buffer length.
        let pos_within_buf = self.pos_within_buf;
        unsafe { self.buffer_mut().get_unchecked_mut(pos_within_buf..) }
    }

    #[inline]
    pub(crate) fn unfilled_len(&self) -> usize {
        self.buffer().len() - self.pos_within_buf
    }

    #[inline]
    pub(crate) unsafe fn advance(&mut self, n: usize) {
        debug_assert!(n <= self.unfilled_len());
        self.pos_within_buf += n;
    }

    #[inline]
    pub(crate) fn rewind(&mut self) {
        self.pos_within_buf = 0;
    }

    #[inline]
    pub(crate) fn replace_buffer_keep_pos(&mut self, buffer: *mut [MaybeUninit<u8>]) {
        unsafe {
            assert!(self.pos_within_buf <= (&*buffer).len());
        }
        self.buffer = buffer;
    }

    #[inline]
    pub(crate) unsafe fn write_byte(&mut self, b: u8) {
        debug_assert!(self.unfilled_len() >= 1);
        // SAFETY: caller is responsible for ensuring that byte fits in the buffer.
        let pos_within_buf = self.pos_within_buf;
        self.buffer_mut().get_unchecked_mut(pos_within_buf).write(b);
        self.pos_within_buf += 1;
    }

    #[inline]
    pub(crate) unsafe fn write_bytes(&mut self, bytes: &[u8]) {
        debug_assert!(self.unfilled_len() >= bytes.len());
        let bottom = self.pos_within_buf as usize;
        let top = bottom + (bytes.len() as usize);
        // SAFETY: caller is responsible for ensuring that `bytes` fits in the buffer.
        let buffer = self.buffer_mut().get_unchecked_mut(bottom..top);
        maybe_uninit_write_slice(buffer, bytes);
        self.pos_within_buf += bytes.len();
    }
}
