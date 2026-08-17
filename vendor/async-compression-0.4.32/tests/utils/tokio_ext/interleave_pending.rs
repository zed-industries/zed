use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct InterleavePending<T> {
    inner: T,
    pended: bool,
}

impl<T> InterleavePending<T> {
    pub(crate) fn new(inner: T) -> Self {
        Self {
            inner,
            pended: false,
        }
    }
}

impl<W: tokio::io::AsyncWrite + Unpin> tokio::io::AsyncWrite for InterleavePending<W> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        if self.pended {
            let next = Pin::new(&mut self.inner).poll_write(cx, buf);
            if next.is_ready() {
                self.pended = false;
            }
            next
        } else {
            cx.waker().wake_by_ref();
            self.pended = true;
            Poll::Pending
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        if self.pended {
            let next = Pin::new(&mut self.inner).poll_flush(cx);
            if next.is_ready() {
                self.pended = false;
            }
            next
        } else {
            cx.waker().wake_by_ref();
            self.pended = true;
            Poll::Pending
        }
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        if self.pended {
            let next = Pin::new(&mut self.inner).poll_shutdown(cx);
            if next.is_ready() {
                self.pended = false;
            }
            next
        } else {
            cx.waker().wake_by_ref();
            self.pended = true;
            Poll::Pending
        }
    }
}
