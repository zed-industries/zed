use crate::codecs::Decode;
use crate::core::util::PartialBuffer;
use crate::futures::write::{AsyncBufWrite, BufWriter};
use futures_core::ready;
use futures_io::{AsyncBufRead, AsyncRead, AsyncWrite, IoSliceMut};
use pin_project_lite::pin_project;
use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};

#[derive(Debug)]
enum State {
    Decoding,
    Finishing,
    Done,
}

pin_project! {
    #[derive(Debug)]
    pub struct Decoder<W, D> {
        #[pin]
        writer: BufWriter<W>,
        decoder: D,
        state: State,
    }
}

impl<W: AsyncWrite, D: Decode> Decoder<W, D> {
    pub fn new(writer: W, decoder: D) -> Self {
        Self {
            writer: BufWriter::new(writer),
            decoder,
            state: State::Decoding,
        }
    }
}

impl<W, D> Decoder<W, D> {
    pub fn get_ref(&self) -> &W {
        self.writer.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut W {
        self.writer.get_mut()
    }

    pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut W> {
        self.project().writer.get_pin_mut()
    }

    pub fn into_inner(self) -> W {
        self.writer.into_inner()
    }
}

impl<W: AsyncWrite, D: Decode> Decoder<W, D> {
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
                State::Decoding => {
                    if this.decoder.decode(input, &mut output)? {
                        State::Finishing
                    } else {
                        State::Decoding
                    }
                }

                State::Finishing => {
                    if this.decoder.finish(&mut output)? {
                        State::Done
                    } else {
                        State::Finishing
                    }
                }

                State::Done => {
                    return Poll::Ready(Err(io::Error::other("Write after end of stream")))
                }
            };

            let produced = output.written().len();
            this.writer.as_mut().produce(produced);

            if let State::Done = this.state {
                return Poll::Ready(Ok(()));
            }

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

            let (state, done) = match this.state {
                State::Decoding => {
                    let done = this.decoder.flush(&mut output)?;
                    (State::Decoding, done)
                }

                State::Finishing => {
                    if this.decoder.finish(&mut output)? {
                        (State::Done, false)
                    } else {
                        (State::Finishing, false)
                    }
                }

                State::Done => (State::Done, true),
            };

            *this.state = state;

            let produced = output.written().len();
            this.writer.as_mut().produce(produced);

            if done {
                return Poll::Ready(Ok(()));
            }
        }
    }
}

impl<W: AsyncWrite, D: Decode> AsyncWrite for Decoder<W, D> {
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
        if let State::Decoding = self.as_mut().project().state {
            *self.as_mut().project().state = State::Finishing;
        }

        ready!(self.as_mut().do_poll_flush(cx))?;

        if let State::Done = self.as_mut().project().state {
            ready!(self.as_mut().project().writer.as_mut().poll_close(cx))?;
            Poll::Ready(Ok(()))
        } else {
            Poll::Ready(Err(io::Error::other(
                "Attempt to close before finishing input",
            )))
        }
    }
}

impl<W: AsyncRead, D> AsyncRead for Decoder<W, D> {
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

impl<W: AsyncBufRead, D> AsyncBufRead for Decoder<W, D> {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        self.get_pin_mut().poll_fill_buf(cx)
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        self.get_pin_mut().consume(amt)
    }
}
