use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};

pub(crate) trait AsyncBufWrite {
    /// Attempt to return an internal buffer to write to, flushing data out to the inner reader if
    /// it is full.
    ///
    /// On success, returns `Poll::Ready(Ok(buf))`.
    ///
    /// If the buffer is full and cannot be flushed, the method returns `Poll::Pending` and
    /// arranges for the current task context (`cx`) to receive a notification when the object
    /// becomes readable or is closed.
    fn poll_partial_flush_buf(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<&mut [u8]>>;

    /// Tells this buffer that `amt` bytes have been written to its buffer, so they should be
    /// written out to the underlying IO when possible.
    ///
    /// This function is a lower-level call. It needs to be paired with the `poll_flush_buf` method to
    /// function properly. This function does not perform any I/O, it simply informs this object
    /// that some amount of its buffer, returned from `poll_flush_buf`, has been written to and should
    /// be sent. As such, this function may do odd things if `poll_flush_buf` isn't
    /// called before calling it.
    ///
    /// The `amt` must be `<=` the number of bytes in the buffer returned by `poll_flush_buf`.
    fn produce(self: Pin<&mut Self>, amt: usize);
}
