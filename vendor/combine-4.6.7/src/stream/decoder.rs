use crate::{
    error::ParseError,
    stream::buf_reader::{Buffer, Bufferless, CombineBuffer},
};

use std::{
    fmt,
    io::{self, Read},
};

#[cfg(feature = "pin-project-lite")]
use std::pin::Pin;

#[derive(Debug)]
pub enum Error<E, P> {
    Parse(E),
    Io { position: P, error: io::Error },
}

impl<'a, P> From<Error<crate::easy::Errors<u8, &'a [u8], P>, P>>
    for crate::easy::Errors<u8, &'a [u8], P>
where
    P: Ord + Clone,
{
    fn from(e: Error<crate::easy::Errors<u8, &'a [u8], P>, P>) -> Self {
        match e {
            Error::Parse(e) => e,
            Error::Io { position, error } => {
                crate::easy::Errors::from_error(position, crate::easy::Error::Other(error.into()))
            }
        }
    }
}

impl<E, P> std::error::Error for Error<E, P>
where
    E: std::error::Error,
    P: fmt::Display + fmt::Debug,
{
}

impl<E: fmt::Display, P: fmt::Display> fmt::Display for Error<E, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parse(e) => e.fmt(f),
            Error::Io { position: _, error } => error.fmt(f),
        }
    }
}

#[derive(Default)]
/// Used together with the `decode!` macro
pub struct Decoder<S, P, C = Buffer> {
    position: P,
    state: S,
    buffer: C,
    end_of_input: bool,
}

impl<S, P> Decoder<S, P, Buffer>
where
    P: Default,
    S: Default,
{
    /// Constructs a new [`Decoder`] with an internal buffer. Allows any `AsyncRead/Read` instance to
    /// be used when decoding but there may be data left in the internal buffer after decoding
    /// (accessible with [`Decoder::buffer`])
    pub fn new() -> Self {
        Decoder::default()
    }

    /// Constructs a new [`Decoder`] with an internal buffer. Allows any `AsyncRead/Read` instance to
    /// be used when decoding but there may be data left in the internal buffer after decoding
    /// (accessible with [`Decoder::buffer`])
    pub fn new_buffer() -> Self {
        Decoder::new()
    }
}

impl<S, P> Decoder<S, P, Bufferless>
where
    P: Default,
    S: Default,
{
    /// Constructs a new `Decoder` without an internal buffer. Requires the read instance to be
    /// wrapped with combine's [`BufReader`] instance to
    ///
    /// [`BufReader`]: super::buf_reader::BufReader
    pub fn new_bufferless() -> Self {
        Decoder::default()
    }
}

impl<S, P> Decoder<S, P> {
    pub fn buffer(&self) -> &[u8] {
        &self.buffer.0
    }
}

impl<S, P, C> Decoder<S, P, C> {
    #[doc(hidden)]
    pub fn advance<R>(&mut self, read: &mut R, removed: usize)
    where
        C: CombineBuffer<R>,
    {
        // Remove the data we have parsed and adjust `removed` to be the amount of data we
        // committed from `self.reader`
        self.buffer.advance(read, removed)
    }

    #[doc(hidden)]
    #[cfg(feature = "pin-project-lite")]
    pub fn advance_pin<R>(&mut self, read: Pin<&mut R>, removed: usize)
    where
        C: CombineBuffer<R>,
    {
        // Remove the data we have parsed and adjust `removed` to be the amount of data we
        // committed from `self.reader`
        self.buffer.advance_pin(read, removed);
    }

    pub fn position(&self) -> &P {
        &self.position
    }

    #[doc(hidden)]
    pub fn __inner(&mut self) -> (&mut S, &mut P, &C, bool) {
        (
            &mut self.state,
            &mut self.position,
            &self.buffer,
            self.end_of_input,
        )
    }
}

impl<S, P, C> Decoder<S, P, C>
where
    C: ,
{
    #[doc(hidden)]
    pub fn __before_parse<R>(&mut self, mut reader: R) -> io::Result<()>
    where
        R: Read,
        C: crate::stream::buf_reader::CombineSyncRead<R>,
    {
        if self.buffer.extend_buf_sync(&mut reader)? == 0 {
            self.end_of_input = true;
        }

        Ok(())
    }
}

#[cfg(feature = "tokio-02")]
impl<S, P, C> Decoder<S, P, C> {
    #[doc(hidden)]
    pub async fn __before_parse_tokio_02<R>(&mut self, mut reader: Pin<&mut R>) -> io::Result<()>
    where
        R: tokio_02_dep::io::AsyncRead,
        C: crate::stream::buf_reader::CombineRead<R, dyn tokio_02_dep::io::AsyncRead>,
    {
        let copied =
            crate::future_ext::poll_fn(|cx| self.buffer.poll_extend_buf(cx, reader.as_mut()))
                .await?;
        if copied == 0 {
            self.end_of_input = true;
        }

        Ok(())
    }
}

#[cfg(feature = "tokio-03")]
impl<S, P, C> Decoder<S, P, C> {
    #[doc(hidden)]
    pub async fn __before_parse_tokio_03<R>(&mut self, mut reader: Pin<&mut R>) -> io::Result<()>
    where
        R: tokio_03_dep::io::AsyncRead,
        C: crate::stream::buf_reader::CombineRead<R, dyn tokio_03_dep::io::AsyncRead>,
    {
        let copied =
            crate::future_ext::poll_fn(|cx| self.buffer.poll_extend_buf(cx, reader.as_mut()))
                .await?;
        if copied == 0 {
            self.end_of_input = true;
        }

        Ok(())
    }
}

#[cfg(feature = "tokio")]
impl<S, P, C> Decoder<S, P, C> {
    #[doc(hidden)]
    pub async fn __before_parse_tokio<R>(&mut self, mut reader: Pin<&mut R>) -> io::Result<()>
    where
        R: tokio_dep::io::AsyncRead,
        C: crate::stream::buf_reader::CombineRead<R, dyn tokio_dep::io::AsyncRead>,
    {
        let copied =
            crate::future_ext::poll_fn(|cx| self.buffer.poll_extend_buf(cx, reader.as_mut()))
                .await?;
        if copied == 0 {
            self.end_of_input = true;
        }

        Ok(())
    }
}

#[cfg(feature = "futures-03")]
impl<S, P, C> Decoder<S, P, C> {
    #[doc(hidden)]
    pub async fn __before_parse_async<R>(&mut self, reader: Pin<&mut R>) -> io::Result<()>
    where
        R: futures_io_03::AsyncRead,
        C: crate::stream::buf_reader::CombineAsyncRead<R>,
    {
        let copied = self.buffer.extend_buf(reader).await?;

        if copied == 0 {
            self.end_of_input = true;
        }
        Ok(())
    }
}
