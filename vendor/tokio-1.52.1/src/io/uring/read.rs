use crate::io::blocking::Buf;
use crate::io::uring::utils::{ArcFd, UringFd};
use crate::runtime::driver::op::{CancelData, Cancellable, Completable, CqeResult, Op};

use io_uring::{opcode, types};
use std::fmt;
use std::io::{self, Error};
use std::os::fd::OwnedFd;

/// Trait for buffers that can be used with io-uring read operations.
pub(crate) trait ReadBuffer: Send + 'static {
    /// Prepare the buffer for a read operation.
    /// Returns a pointer and length for the io-uring SQE.
    fn uring_read_prepare(&mut self, max_len: usize) -> (*mut u8, u32);

    /// Complete a read of `n` bytes.
    ///
    /// # Safety
    ///
    /// The caller must ensure the kernel wrote exactly `n` bytes
    /// into the buffer at the pointer returned by `uring_read_prepare`.
    unsafe fn uring_read_complete(&mut self, n: u32);
}

impl ReadBuffer for Vec<u8> {
    fn uring_read_prepare(&mut self, max_len: usize) -> (*mut u8, u32) {
        assert!(self.spare_capacity_mut().len() >= max_len);
        let ptr = self.spare_capacity_mut().as_mut_ptr().cast();
        (ptr, max_len as u32)
    }

    unsafe fn uring_read_complete(&mut self, n: u32) {
        // SAFETY: the kernel wrote `n` bytes into spare capacity starting
        // at the old self.len(), so self.len() + n bytes are now initialized.
        unsafe { self.set_len(self.len() + n as usize) };
    }
}

impl ReadBuffer for Buf {
    fn uring_read_prepare(&mut self, max_len: usize) -> (*mut u8, u32) {
        self.prepare_uring_read(max_len)
    }

    unsafe fn uring_read_complete(&mut self, n: u32) {
        // SAFETY: caller guarantees kernel wrote exactly n bytes.
        unsafe { self.complete_uring_read(n as usize) };
    }
}

pub(crate) struct Read<B, F = ArcFd> {
    fd: F,
    buf: B,
}

impl<B: fmt::Debug, F> fmt::Debug for Read<B, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Read")
            .field("buf", &self.buf)
            .finish_non_exhaustive()
    }
}

impl<B: ReadBuffer, F: UringFd> Completable for Read<B, F> {
    type Output = (io::Result<u32>, F, B);

    fn complete(self, cqe: CqeResult) -> Self::Output {
        let mut buf = self.buf;
        if let Ok(len) = cqe.result {
            // SAFETY: kernel wrote exactly `len` bytes into the prepared buffer.
            unsafe { buf.uring_read_complete(len) };
        }
        (cqe.result, self.fd, buf)
    }

    fn complete_with_error(self, err: Error) -> Self::Output {
        (Err(err), self.fd, self.buf)
    }
}

impl Cancellable for Read<Vec<u8>, OwnedFd> {
    fn cancel(self) -> CancelData {
        CancelData::ReadVec(self)
    }
}

impl Cancellable for Read<Buf, ArcFd> {
    fn cancel(self) -> CancelData {
        CancelData::ReadBuf(self)
    }
}

impl<B, F> Op<Read<B, F>>
where
    B: ReadBuffer + fmt::Debug,
    F: UringFd,
    Read<B, F>: Cancellable,
{
    /// Submit a read operation via io-uring.
    ///
    /// `max_len` is the maximum number of bytes to read.
    /// `offset` is the file offset; use `u64::MAX` for the current cursor.
    pub(crate) fn read_at(fd: F, mut buf: B, max_len: usize, offset: u64) -> Self {
        let (ptr, len) = buf.uring_read_prepare(max_len);

        let sqe = opcode::Read::new(types::Fd(UringFd::as_raw_fd(&fd)), ptr, len)
            .offset(offset)
            .build();

        // SAFETY: `fd` and `buf`, which owns the heap buffer, are moved into `Read`,
        // which is held by the `Op` for the entire duration of the io-uring operation.
        // The buffer pointer remains valid because Vec heap data doesn't move.
        unsafe { Op::new(sqe, Read { fd, buf }) }
    }
}
