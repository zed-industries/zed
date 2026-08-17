use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};

use crate::codecs::Encode;
use crate::core::util::PartialBuffer;
use crate::futures::write::{AsyncBufWrite, BufWriter};
use futures_core::ready;
use futures_io::{AsyncBufRead, AsyncRead, AsyncWrite, IoSliceMut};
use pin_project_lite::pin_project;

#[derive(Debug)]
enum State {
    Encoding,
    Finishing,
    Done,
}

pin_project! {
    #[derive(Debug)]
    pub struct Encoder<W, E> {
        #[pin]
        writer: BufWriter<W>,
        encoder: E,
        state: State,
    }
}

impl<W, E> Encoder<W, E> {
    pub fn get_ref(&self) -> &W {
        self.writer.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut W {
        self.writer.get_mut()
    }

    pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut W> {
        self.project().writer.get_pin_mut()
    }

    pub(crate) fn get_encoder_ref(&self) -> &E {
        &self.encoder
    }

    pub fn into_inner(self) -> W {
        self.writer.into_inner()
    }
}

impl<W: AsyncWrite, E: Encode> Encoder<W, E> {
    pub fn new(writer: W, encoder: E) -> Self {
        Self {
            writer: BufWriter::new(writer),
            encoder,
            state: State::Encoding,
        }
    }

    pub fn with_capacity(writer: W, encoder: E, cap: usize) -> Self {
        Self {
            writer: BufWriter::with_capacity(cap, writer),
            encoder,
            state: State::Encoding,
        }
    }

    fn do_poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        input: &mut PartialBuffer<&[u8]>,
    ) -> Poll<io::Result<()>> {
        let mut this = self.project();

        loop {
            let output = ready!(this.writer.as_mut().poll_partial_flush_buf(cx))?;
            let mut output = PartialBuffer::new(output);

            *this.state = match this.state {
                State::Encoding => {
                    this.encoder.encode(input, &mut output)?;
                    State::Encoding
                }

                State::Finishing | State::Done => {
                    return Poll::Ready(Err(io::Error::other("Write after close")))
                }
            };

            let produced = output.written().len();
            this.writer.as_mut().produce(produced);

            if input.unwritten().is_empty() {
                return Poll::Ready(Ok(()));
            }
        }
    }

    fn do_poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let mut this = self.project();

        loop {
            let output = ready!(this.writer.as_mut().poll_partial_flush_buf(cx))?;
            let mut output = PartialBuffer::new(output);

            let done = match this.state {
                State::Encoding => this.encoder.flush(&mut output)?,

                State::Finishing | State::Done => {
                    return Poll::Ready(Err(io::Error::other("Flush after close")))
                }
            };

            let produced = output.written().len();
            this.writer.as_mut().produce(produced);

            if done {
                return Poll::Ready(Ok(()));
            }
        }
    }

    fn do_poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let mut this = self.project();

        loop {
            let output = ready!(this.writer.as_mut().poll_partial_flush_buf(cx))?;
            let mut output = PartialBuffer::new(output);

            *this.state = match this.state {
                State::Encoding | State::Finishing => {
                    if this.encoder.finish(&mut output)? {
                        State::Done
                    } else {
                        State::Finishing
                    }
                }

                State::Done => State::Done,
            };

            let produced = output.written().len();
            this.writer.as_mut().produce(produced);

            if let State::Done = this.state {
                return Poll::Ready(Ok(()));
            }
        }
    }
}

impl<W: AsyncWrite, E: Encode> AsyncWrite for Encoder<W, E> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        if buf.is_empty() {
            return Poll::Ready(Ok(0));
        }

        let mut input = PartialBuffer::new(buf);

        match self.do_poll_write(cx, &mut input)? {
            Poll::Pending if input.written().is_empty() => Poll::Pending,
            _ => Poll::Ready(Ok(input.written().len())),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        ready!(self.as_mut().do_poll_flush(cx))?;
        ready!(self.project().writer.as_mut().poll_flush(cx))?;
        Poll::Ready(Ok(()))
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        ready!(self.as_mut().do_poll_close(cx))?;
        ready!(self.project().writer.as_mut().poll_close(cx))?;
        Poll::Ready(Ok(()))
    }
}

impl<W: AsyncRead, E> AsyncRead for Encoder<W, E> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        self.get_pin_mut().poll_read(cx, buf)
    }

    fn poll_read_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &mut [IoSliceMut<'_>],
    ) -> Poll<io::Result<usize>> {
        self.get_pin_mut().poll_read_vectored(cx, bufs)
    }
}

impl<W: AsyncBufRead, E> AsyncBufRead for Encoder<W, E> {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        self.get_pin_mut().poll_fill_buf(cx)
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        self.get_pin_mut().consume(amt)
    }
}
