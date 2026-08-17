use std::cmp;
use std::mem::MaybeUninit;

use crate::misc::maybe_uninit_write_slice;

#[derive(Debug)]
pub(crate) struct InputBuf<'a> {
    // Invariants: `0 <= pos_within_buf <= limit_within_buf <= buf.len()`.
    buf: &'a [u8],
    pos_within_buf: usize,
    limit_within_buf: usize,
}

impl<'a> InputBuf<'a> {
    #[inline]
    pub(crate) fn assertions(&self) {
        debug_assert!(self.pos_within_buf <= self.limit_within_buf);
        debug_assert!(self.limit_within_buf <= self.buf.len());
    }

    pub(crate) fn empty() -> InputBuf<'a> {
        InputBuf {
            buf: &[],
            pos_within_buf: 0,
            limit_within_buf: 0,
        }
    }

    pub(crate) fn from_bytes(buf: &'a [u8]) -> InputBuf<'a> {
        InputBuf {
            buf,
            pos_within_buf: 0,
            limit_within_buf: buf.len(),
        }
    }

    pub(crate) unsafe fn from_bytes_ignore_lifetime(buf: &[u8]) -> InputBuf<'a> {
        let buf = &*(buf as *const [u8]);
        Self::from_bytes(buf)
    }

    pub(crate) fn update_limit(&mut self, limit: u64) {
        let limit_within_buf = cmp::min(self.buf.len() as u64, limit);
        assert!(limit_within_buf >= self.pos_within_buf as u64);
        self.limit_within_buf = limit_within_buf as usize;
    }

    pub(crate) fn pos_within_buf(&self) -> usize {
        self.pos_within_buf
    }

    #[inline(always)]
    pub(crate) fn remaining_in_buf(&self) -> &'a [u8] {
        // SAFETY: Invariants.
        unsafe {
            self.buf
                .get_unchecked(self.pos_within_buf..self.limit_within_buf)
        }
    }

    #[inline(always)]
    pub(crate) fn consume(&mut self, amt: usize) {
        assert!(amt <= self.remaining_in_buf().len());
        self.pos_within_buf += amt;
    }

    #[inline(always)]
    pub(crate) fn read_byte(&mut self) -> Option<u8> {
        let r = self.remaining_in_buf().first().copied();
        if let Some(..) = r {
            self.pos_within_buf += 1;
        }
        r
    }

    pub(crate) fn read_bytes<'b>(&mut self, dest: &'b mut [MaybeUninit<u8>]) -> &'b mut [u8] {
        // This panics if this has not enough data.
        let r = maybe_uninit_write_slice(dest, &self.remaining_in_buf()[..dest.len()]);
        self.pos_within_buf += r.len();
        r
    }
}
