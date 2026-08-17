use crate::runtime::driver::op::{CancelData, Cancellable, Completable, CqeResult, Op};
use crate::util::as_ref::OwnedBuf;

use io_uring::{opcode, types};
use std::io::{self, Error};
use std::os::fd::{AsRawFd, OwnedFd};

#[derive(Debug)]
pub(crate) struct Write {
    buf: OwnedBuf,
    fd: OwnedFd,
}

impl Completable for Write {
    type Output = (io::Result<u32>, OwnedBuf, OwnedFd);
    fn complete(self, cqe: CqeResult) -> Self::Output {
        (cqe.result, self.buf, self.fd)
    }

    fn complete_with_error(self, err: Error) -> Self::Output {
        (Err(err), self.buf, self.fd)
    }
}

impl Cancellable for Write {
    fn cancel(self) -> CancelData {
        CancelData::Write(self)
    }
}

impl Op<Write> {
    /// Issue a write that starts at `buf_offset` within `buf` and writes some bytes
    /// into `file` at `file_offset`.
    pub(crate) fn write_at(
        fd: OwnedFd,
        buf: OwnedBuf,
        buf_offset: usize,
        file_offset: u64,
    ) -> io::Result<Self> {
        // There is a cap on how many bytes we can write in a single uring write operation.
        // ref: https://github.com/axboe/liburing/discussions/497
        let len = u32::try_from(buf.as_ref().len() - buf_offset).unwrap_or(u32::MAX);

        let ptr = buf.as_ref()[buf_offset..buf_offset + len as usize].as_ptr();

        let sqe = opcode::Write::new(types::Fd(fd.as_raw_fd()), ptr, len)
            .offset(file_offset)
            .build();

        // SAFETY: parameters of the entry, such as `fd` and `buf`, are valid
        // until this operation completes.
        let op = unsafe { Op::new(sqe, Write { buf, fd }) };
        Ok(op)
    }
}
