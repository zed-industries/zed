mod copy_buf;
mod interleave_pending;
mod limited;

pub use copy_buf::copy_buf;

pub trait AsyncWriteTestExt: tokio::io::AsyncWrite {
    fn interleave_pending_write(self) -> interleave_pending::InterleavePending<Self>
    where
        Self: Sized + Unpin,
    {
        interleave_pending::InterleavePending::new(self)
    }

    fn limited_write(self, limit: usize) -> limited::Limited<Self>
    where
        Self: Sized + Unpin,
    {
        limited::Limited::new(self, limit)
    }
}

impl<T: tokio::io::AsyncWrite> AsyncWriteTestExt for T {}
