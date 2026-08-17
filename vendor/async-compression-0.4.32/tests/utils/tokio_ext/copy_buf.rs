use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use futures::ready;
use tokio::io::{AsyncBufRead, AsyncWrite};

pub fn copy_buf<R, W>(reader: R, writer: &mut W) -> CopyBuf<'_, R, W>
where
    R: AsyncBufRead + Unpin,
    W: AsyncWrite + Unpin + ?Sized,
{
    CopyBuf {
        reader,
        writer,
        amt: 0,
    }
}

#[derive(Debug)]
pub struct CopyBuf<'a, R, W: ?Sized> {
    reader: R,
    writer: &'a mut W,
    amt: u64,
}

impl<R, W> Future for CopyBuf<'_, R, W>
where
    R: AsyncBufRead + Unpin,
    W: AsyncWrite + Unpin + ?Sized,
{
    type Output = std::io::Result<u64>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        loop {
            let buffer = ready!(Pin::new(&mut this.reader).poll_fill_buf(cx))?;
            if buffer.is_empty() {
                ready!(Pin::new(&mut this.writer).poll_flush(cx))?;
                return Poll::Ready(Ok(this.amt));
            }

            let i = ready!(Pin::new(&mut this.writer).poll_write(cx, buffer))?;
            if i == 0 {
                return Poll::Ready(Err(std::io::ErrorKind::WriteZero.into()));
            }
            this.amt += i as u64;
            Pin::new(&mut this.reader).consume(i);
        }
    }
}
