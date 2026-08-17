use crate::codecs::Encode;
use crate::core::util::PartialBuffer;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use futures_core::ready;
use pin_project_lite::pin_project;
use std::io::{IoSlice, Result};
use tokio::io::{AsyncBufRead, AsyncRead, AsyncWrite, ReadBuf};

#[derive(Debug)]
enum State {
    Encoding,
    Flushing,
    Finishing,
    Done,
}

pin_project! {
    #[derive(Debug)]
    pub struct Encoder<R, E> {
        #[pin]
        reader: R,
        encoder: E,
        state: State,
    }
}

impl<R: AsyncBufRead, E: Encode> Encoder<R, E> {
    pub fn new(reader: R, encoder: E) -> Self {
        Self {
            reader,
            encoder,
            state: State::Encoding,
        }
    }

    pub fn with_capacity(reader: R, encoder: E, _cap: usize) -> Self {
        Self::new(reader, encoder)
    }
}

impl<R, E> Encoder<R, E> {
    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn get_mut(&mut self) -> &mut R {
        &mut self.reader
    }

    pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut R> {
        self.project().reader
    }

    pub(crate) fn get_encoder_ref(&self) -> &E {
        &self.encoder
    }

    pub fn into_inner(self) -> R {
        self.reader
    }
}
impl<R: AsyncBufRead, E: Encode> Encoder<R, E> {
    fn do_poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        output: &mut PartialBuffer<&mut [u8]>,
    ) -> Poll<Result<()>> {
        let mut this = self.project();
        let mut read = 0usize;

        loop {
            *this.state = match this.state {
                State::Encoding => {
                    let res = this.reader.as_mut().poll_fill_buf(cx);

                    match res {
                        Poll::Pending => {
                            if read == 0 {
                                return Poll::Pending;
                            } else {
                                State::Flushing
                            }
                        }
                        Poll::Ready(res) => {
                            let input = res?;

                            if input.is_empty() {
                                State::Finishing
                            } else {
                                let mut input = PartialBuffer::new(input);
                                this.encoder.encode(&mut input, output)?;
                                let len = input.written().len();
                                this.reader.as_mut().consume(len);
                                read += len;

                                State::Encoding
                            }
                        }
                    }
                }

                State::Flushing => {
                    if this.encoder.flush(output)? {
                        read = 0;
                        State::Encoding
                    } else {
                        State::Flushing
                    }
                }

                State::Finishing => {
                    if this.encoder.finish(output)? {
                        State::Done
                    } else {
                        State::Finishing
                    }
                }

                State::Done => State::Done,
            };

            if let State::Done = *this.state {
                return Poll::Ready(Ok(()));
            }
            if output.unwritten().is_empty() {
                return Poll::Ready(Ok(()));
            }
        }
    }
}

impl<R: AsyncBufRead, E: Encode> AsyncRead for Encoder<R, E> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<()>> {
        if buf.remaining() == 0 {
            return Poll::Ready(Ok(()));
        }

        let mut output = PartialBuffer::new(buf.initialize_unfilled());
        match self.do_poll_read(cx, &mut output)? {
            Poll::Pending if output.written().is_empty() => Poll::Pending,
            _ => {
                let len = output.written().len();
                buf.advance(len);
                Poll::Ready(Ok(()))
            }
        }
    }
}

impl<R: AsyncWrite, E> AsyncWrite for Encoder<R, E> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize>> {
        self.get_pin_mut().poll_write(cx, buf)
    }

    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut bufs: &[IoSlice<'_>],
    ) -> Poll<Result<usize>> {
        self.get_pin_mut().poll_write_vectored(cx, bufs)
    }

    fn is_write_vectored(&self) -> bool {
        self.get_ref().is_write_vectored()
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        self.get_pin_mut().poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        self.get_pin_mut().poll_shutdown(cx)
    }
}
