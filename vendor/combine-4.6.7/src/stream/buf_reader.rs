use std::io::{self, BufRead, Read};

#[cfg(any(
    feature = "futures-03",
    feature = "tokio-02",
    feature = "tokio-03",
    feature = "tokio"
))]
use std::pin::Pin;

#[cfg(any(feature = "futures-03", feature = "tokio-02", feature = "tokio-03"))]
use std::mem::MaybeUninit;

#[cfg(feature = "futures-core-03")]
use std::task::{Context, Poll};

#[cfg(feature = "futures-03")]
use std::future::Future;

use bytes::{Buf, BufMut, BytesMut};

#[cfg(feature = "pin-project-lite")]
use pin_project_lite::pin_project;

#[cfg(feature = "tokio-03")]
use tokio_03_dep::io::AsyncBufRead as _;

#[cfg(feature = "tokio")]
use tokio_dep::io::AsyncBufRead as _;

#[cfg(feature = "futures-core-03")]
use futures_core_03::ready;

#[cfg(feature = "pin-project-lite")]
pin_project! {
    /// `BufReader` used by `Decoder` when it is constructed with [`Decoder::new_bufferless`][]
    ///
    /// [`Decoder::new_bufferless`]: ../decoder/struct.Decoder.html#method.new_bufferless
    #[derive(Debug)]
    pub struct BufReader<R> {
        #[pin]
        inner: R,
        buf: BytesMut
    }
}

#[cfg(not(feature = "pin-project-lite"))]
/// `BufReader` used by `Decoder` when it is constructed with [`Decoder::new_bufferless`][]
///
/// [`Decoder::new_bufferless`]: ../decoder/struct.Decoder.html#method.new_bufferless
#[derive(Debug)]
pub struct BufReader<R> {
    inner: R,
    buf: BytesMut,
}

impl<R> BufReader<R> {
    /// Creates a new `BufReader` with a default buffer capacity. The default is currently 8 KB,
    /// but may change in the future.
    pub fn new(inner: R) -> Self {
        Self::with_capacity(8096, inner)
    }

    /// Creates a new `BufReader` with the specified buffer capacity.
    pub fn with_capacity(capacity: usize, inner: R) -> Self {
        let buf = BytesMut::with_capacity(capacity);

        Self { inner, buf }
    }

    /// Gets a reference to the underlying reader.
    ///
    /// It is inadvisable to directly read from the underlying reader.
    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    /// Gets a mutable reference to the underlying reader.
    ///
    /// It is inadvisable to directly read from the underlying reader.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    #[cfg(feature = "pin-project-lite")]
    /// Gets a pinned mutable reference to the underlying reader.
    ///
    /// It is inadvisable to directly read from the underlying reader.
    pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut R> {
        self.project().inner
    }

    /// Consumes this `BufWriter`, returning the underlying reader.
    ///
    /// Note that any leftover data in the internal buffer is lost.
    pub fn into_inner(self) -> R {
        self.inner
    }

    /// Returns a reference to the internally buffered data.
    ///
    /// Unlike `fill_buf`, this will not attempt to fill the buffer if it is empty.
    pub fn buffer(&self) -> &[u8] {
        &self.buf
    }

    /// Invalidates all data in the internal buffer.
    #[inline]
    #[cfg(any(feature = "tokio-02", feature = "tokio-03", feature = "tokio"))]
    fn discard_buffer(self: Pin<&mut Self>) {
        let me = self.project();
        me.buf.clear();
    }
}

mod sealed {
    pub trait Sealed {}
}

#[doc(hidden)]
pub trait CombineBuffer<R>: sealed::Sealed {
    fn buffer<'a>(&'a self, read: &'a R) -> &'a [u8];

    fn advance(&mut self, read: &mut R, len: usize);

    #[cfg(feature = "pin-project-lite")]
    fn advance_pin(&mut self, read: Pin<&mut R>, len: usize);
}

#[doc(hidden)]
pub trait CombineSyncRead<R>: CombineBuffer<R> {
    fn extend_buf_sync(&mut self, read: &mut R) -> io::Result<usize>;
}

#[cfg(any(feature = "tokio-02", feature = "tokio-03", feature = "tokio"))]
#[doc(hidden)]
pub trait CombineRead<R, T: ?Sized>: CombineBuffer<R> {
    fn poll_extend_buf(
        &mut self,
        cx: &mut Context<'_>,
        read: Pin<&mut R>,
    ) -> Poll<io::Result<usize>>;
}

#[cfg(feature = "futures-03")]
#[doc(hidden)]
pub trait CombineAsyncRead<R>: CombineBuffer<R> {
    fn poll_extend_buf(
        &mut self,
        cx: &mut Context<'_>,
        read: Pin<&mut R>,
    ) -> Poll<io::Result<usize>>;

    fn extend_buf<'a>(&'a mut self, read: Pin<&'a mut R>) -> ExtendBuf<'a, Self, R>
    where
        Self: Sized;
}

#[cfg(feature = "futures-03")]
pin_project_lite::pin_project! {
    #[doc(hidden)]
    pub struct ExtendBuf<'a, C, R> {
        buffer: &'a mut C,
        read: Pin<&'a mut R>
    }
}

#[cfg(feature = "futures-03")]
impl<'a, C, R> Future for ExtendBuf<'a, C, R>
where
    C: CombineAsyncRead<R>,
{
    type Output = io::Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.project();
        me.buffer.poll_extend_buf(cx, me.read.as_mut())
    }
}

/// Marker used by `Decoder` for an internal buffer
#[derive(Default)]
pub struct Buffer(pub(crate) BytesMut);

impl sealed::Sealed for Buffer {}

impl<R> CombineBuffer<R> for Buffer {
    fn buffer<'a>(&'a self, _read: &'a R) -> &'a [u8] {
        &self.0
    }

    fn advance(&mut self, _read: &mut R, len: usize) {
        self.0.advance(len);
    }

    #[cfg(feature = "pin-project-lite")]
    fn advance_pin(&mut self, _read: Pin<&mut R>, len: usize) {
        self.0.advance(len);
    }
}

impl<R> CombineSyncRead<R> for Buffer
where
    R: Read,
{
    fn extend_buf_sync(&mut self, read: &mut R) -> io::Result<usize> {
        extend_buf_sync(&mut self.0, read)
    }
}

#[cfg(feature = "futures-03")]
impl<R> CombineAsyncRead<R> for Buffer
where
    R: futures_io_03::AsyncRead,
{
    fn poll_extend_buf(
        &mut self,
        cx: &mut Context<'_>,
        read: Pin<&mut R>,
    ) -> Poll<io::Result<usize>> {
        poll_extend_buf(&mut self.0, cx, read)
    }

    fn extend_buf<'a>(&'a mut self, read: Pin<&'a mut R>) -> ExtendBuf<'a, Self, R> {
        if !self.0.has_remaining_mut() {
            self.0.reserve(8 * 1024);
        }
        // Copy of tokio's read_buf method (but it has to force initialize the buffer)
        let bs = self.0.chunk_mut();

        for i in 0..bs.len() {
            bs.write_byte(i, 0);
        }
        ExtendBuf { buffer: self, read }
    }
}

#[cfg(feature = "tokio-02")]
impl<R> CombineRead<R, dyn tokio_02_dep::io::AsyncRead> for Buffer
where
    R: tokio_02_dep::io::AsyncRead,
{
    fn poll_extend_buf(
        &mut self,
        cx: &mut Context<'_>,
        read: Pin<&mut R>,
    ) -> Poll<io::Result<usize>> {
        if !self.0.has_remaining_mut() {
            self.0.reserve(8 * 1024);
        }
        read.poll_read_buf(cx, &mut Bytes05(&mut self.0))
    }
}

#[cfg(feature = "tokio-03")]
fn tokio_03_to_read_buf(bs: &mut BytesMut) -> tokio_03_dep::io::ReadBuf<'_> {
    let uninit = bs.chunk_mut();
    unsafe {
        tokio_03_dep::io::ReadBuf::uninit(std::slice::from_raw_parts_mut(
            uninit.as_mut_ptr() as *mut MaybeUninit<u8>,
            uninit.len(),
        ))
    }
}

#[cfg(feature = "tokio-03")]
impl<R> CombineRead<R, dyn tokio_03_dep::io::AsyncRead> for Buffer
where
    R: tokio_03_dep::io::AsyncRead,
{
    fn poll_extend_buf(
        &mut self,
        cx: &mut Context<'_>,
        read: Pin<&mut R>,
    ) -> Poll<io::Result<usize>> {
        tokio_03_read_buf(cx, read, &mut self.0)
    }
}

#[cfg(feature = "tokio-03")]
fn tokio_03_read_buf(
    cx: &mut Context<'_>,
    read: Pin<&mut impl tokio_03_dep::io::AsyncRead>,
    bs: &mut bytes::BytesMut,
) -> Poll<io::Result<usize>> {
    if !bs.has_remaining_mut() {
        bs.reserve(8 * 1024);
    }

    let mut buf = tokio_03_to_read_buf(bs);
    ready!(read.poll_read(cx, &mut buf))?;
    unsafe {
        let n = buf.filled().len();
        bs.advance_mut(n);
        Poll::Ready(Ok(n))
    }
}

#[cfg(feature = "tokio")]
impl<R> CombineRead<R, dyn tokio_dep::io::AsyncRead> for Buffer
where
    R: tokio_dep::io::AsyncRead,
{
    fn poll_extend_buf(
        &mut self,
        cx: &mut Context<'_>,
        read: Pin<&mut R>,
    ) -> Poll<io::Result<usize>> {
        tokio_read_buf(read, cx, &mut self.0)
    }
}

#[cfg(feature = "tokio")]
fn tokio_read_buf(
    read: Pin<&mut impl tokio_dep::io::AsyncRead>,
    cx: &mut Context<'_>,
    bs: &mut bytes::BytesMut,
) -> Poll<io::Result<usize>> {
    if !bs.has_remaining_mut() {
        bs.reserve(8 * 1024);
    }

    tokio_util::io::poll_read_buf(read, cx, bs)
}

/// Marker used by `Decoder` for an external buffer
#[derive(Default)]
pub struct Bufferless;

impl sealed::Sealed for Bufferless {}

impl<R> CombineBuffer<BufReader<R>> for Bufferless {
    fn buffer<'a>(&'a self, read: &'a BufReader<R>) -> &'a [u8] {
        &read.buf
    }

    fn advance(&mut self, read: &mut BufReader<R>, len: usize) {
        read.buf.advance(len);
    }

    #[cfg(feature = "pin-project-lite")]
    fn advance_pin(&mut self, read: Pin<&mut BufReader<R>>, len: usize) {
        read.project().buf.advance(len);
    }
}

impl<R> CombineSyncRead<BufReader<R>> for Bufferless
where
    R: Read,
{
    fn extend_buf_sync(&mut self, read: &mut BufReader<R>) -> io::Result<usize> {
        extend_buf_sync(&mut read.buf, &mut read.inner)
    }
}

fn extend_buf_sync<R>(buf: &mut BytesMut, read: &mut R) -> io::Result<usize>
where
    R: Read,
{
    let size = 8 * 1024;
    if !buf.has_remaining_mut() {
        buf.reserve(size);
    }

    // Copy of tokio's poll_read_buf method (but it has to force initialize the buffer)
    let n = {
        let bs = buf.chunk_mut();

        let initial_size = bs.len().min(size);
        let bs = &mut bs[..initial_size];
        for i in 0..bs.len() {
            bs.write_byte(i, 0);
        }

        // Convert to `&mut [u8]`
        // SAFETY: the entire buffer is preinitialized above
        let bs = unsafe { &mut *(bs as *mut _ as *mut [u8]) };

        let n = read.read(bs)?;
        assert!(
            n <= bs.len(),
            "AsyncRead reported that it initialized more than the number of bytes in the buffer"
        );
        n
    };

    // SAFETY: the entire buffer has been preinitialized
    unsafe { buf.advance_mut(n) };

    Ok(n)
}

#[cfg(feature = "tokio-02")]
struct Bytes05<'a>(&'a mut BytesMut);

#[cfg(feature = "tokio-02")]
impl bytes_05::BufMut for Bytes05<'_> {
    fn remaining_mut(&self) -> usize {
        self.0.remaining_mut()
    }
    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.0.advance_mut(cnt)
    }
    fn bytes_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        unsafe { &mut *(self.0.chunk_mut() as *mut _ as *mut [MaybeUninit<u8>]) }
    }
}

#[cfg(feature = "tokio-02")]
impl<R> CombineRead<BufReader<R>, dyn tokio_02_dep::io::AsyncRead> for Bufferless
where
    R: tokio_02_dep::io::AsyncRead,
{
    fn poll_extend_buf(
        &mut self,
        cx: &mut Context<'_>,
        read: Pin<&mut BufReader<R>>,
    ) -> Poll<io::Result<usize>> {
        let me = read.project();

        if !me.buf.has_remaining_mut() {
            me.buf.reserve(8 * 1024);
        }
        tokio_02_dep::io::AsyncRead::poll_read_buf(me.inner, cx, &mut Bytes05(me.buf))
    }
}

#[cfg(feature = "tokio-03")]
impl<R> CombineRead<BufReader<R>, dyn tokio_03_dep::io::AsyncRead> for Bufferless
where
    R: tokio_03_dep::io::AsyncRead,
{
    fn poll_extend_buf(
        &mut self,
        cx: &mut Context<'_>,
        read: Pin<&mut BufReader<R>>,
    ) -> Poll<io::Result<usize>> {
        let me = read.project();

        tokio_03_read_buf(cx, me.inner, me.buf)
    }
}

#[cfg(feature = "tokio")]
impl<R> CombineRead<BufReader<R>, dyn tokio_dep::io::AsyncRead> for Bufferless
where
    R: tokio_dep::io::AsyncRead,
{
    fn poll_extend_buf(
        &mut self,
        cx: &mut Context<'_>,
        read: Pin<&mut BufReader<R>>,
    ) -> Poll<io::Result<usize>> {
        let me = read.project();

        tokio_read_buf(me.inner, cx, me.buf)
    }
}

#[cfg(feature = "futures-03")]
impl<R> CombineAsyncRead<BufReader<R>> for Bufferless
where
    R: futures_io_03::AsyncRead,
{
    fn poll_extend_buf(
        &mut self,
        cx: &mut Context<'_>,
        read: Pin<&mut BufReader<R>>,
    ) -> Poll<io::Result<usize>> {
        let me = read.project();

        poll_extend_buf(me.buf, cx, me.inner)
    }

    fn extend_buf<'a>(
        &'a mut self,
        mut read: Pin<&'a mut BufReader<R>>,
    ) -> ExtendBuf<'a, Self, BufReader<R>> {
        let me = read.as_mut().project();

        if !me.buf.has_remaining_mut() {
            me.buf.reserve(8 * 1024);
        }
        // Copy of tokio's read_buf method (but it has to force initialize the buffer)
        let bs = me.buf.chunk_mut();

        for i in 0..bs.len() {
            bs.write_byte(i, 0);
        }
        ExtendBuf { buffer: self, read }
    }
}

#[cfg(feature = "futures-03")]
fn poll_extend_buf<R>(
    buf: &mut BytesMut,
    cx: &mut Context<'_>,
    read: Pin<&mut R>,
) -> Poll<io::Result<usize>>
where
    R: futures_io_03::AsyncRead,
{
    // Copy of tokio's read_buf method (but it has to force initialize the buffer)
    let n = {
        let bs = buf.chunk_mut();
        // preinit the buffer
        for i in 0..bs.len() {
            bs.write_byte(i, 0);
        }

        // Convert to `&mut [u8]`
        // SAFETY: preinitialize the buffer
        let bs = unsafe { &mut *(bs as *mut _ as *mut [u8]) };

        let n = ready!(read.poll_read(cx, bs))?;
        assert!(
            n <= bs.len(),
            "AsyncRead reported that it initialized more than the number of bytes in the buffer"
        );
        n
    };
    // SAFETY: the buffer was preinitialized
    unsafe { buf.advance_mut(n) };
    Poll::Ready(Ok(n))
}

#[cfg(feature = "tokio-02")]
impl<R: tokio_02_dep::io::AsyncRead> tokio_02_dep::io::AsyncRead for BufReader<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        use tokio_02_dep::io::AsyncBufRead;

        // If we don't have any buffered data and we're doing a massive read
        // (larger than our internal buffer), bypass our internal buffer
        // entirely.
        if !self.buf.has_remaining_mut() && buf.len() >= self.buf.len() {
            let res = ready!(self.as_mut().get_pin_mut().poll_read(cx, buf));
            self.discard_buffer();
            return Poll::Ready(res);
        }
        let mut rem = ready!(self.as_mut().poll_fill_buf(cx))?;
        let nread = rem.read(buf)?;
        self.consume(nread);
        Poll::Ready(Ok(nread))
    }

    // we can't skip unconditionally because of the large buffer case in read.
    unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [MaybeUninit<u8>]) -> bool {
        self.inner.prepare_uninitialized_buffer(buf)
    }
}

#[cfg(feature = "tokio-02")]
impl<R: tokio_02_dep::io::AsyncRead> tokio_02_dep::io::AsyncBufRead for BufReader<R> {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        let me = self.project();

        // If we've reached the end of our internal buffer then we need to fetch
        // some more data from the underlying reader.
        // Branch using `>=` instead of the more correct `==`
        // to tell the compiler that the pos..cap slice is always valid.

        if me.buf.is_empty() {
            ready!(me.inner.poll_read_buf(cx, &mut Bytes05(me.buf)))?;
        }
        Poll::Ready(Ok(&me.buf[..]))
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        let me = self.project();
        me.buf.advance(amt);
    }
}

#[cfg(feature = "tokio-02")]
impl<R: tokio_02_dep::io::AsyncRead + tokio_02_dep::io::AsyncWrite> tokio_02_dep::io::AsyncWrite
    for BufReader<R>
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.get_pin_mut().poll_write(cx, buf)
    }

    fn poll_write_buf<B: bytes_05::Buf>(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut B,
    ) -> Poll<io::Result<usize>> {
        self.get_pin_mut().poll_write_buf(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_pin_mut().poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_pin_mut().poll_shutdown(cx)
    }
}

#[cfg(feature = "tokio-03")]
impl<R: tokio_03_dep::io::AsyncRead> tokio_03_dep::io::AsyncRead for BufReader<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio_03_dep::io::ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        // If we don't have any buffered data and we're doing a massive read
        // (larger than our internal buffer), bypass our internal buffer
        // entirely.
        if !self.buf.has_remaining_mut() && buf.remaining() >= self.buf.len() {
            let res = ready!(self.as_mut().get_pin_mut().poll_read(cx, buf));
            self.discard_buffer();
            return Poll::Ready(res);
        }
        let rem = ready!(self.as_mut().poll_fill_buf(cx))?;
        let amt = std::cmp::min(rem.len(), buf.remaining());
        buf.put_slice(&rem[..amt]);
        self.consume(amt);
        Poll::Ready(Ok(()))
    }
}

#[cfg(feature = "tokio-03")]
impl<R: tokio_03_dep::io::AsyncRead> tokio_03_dep::io::AsyncBufRead for BufReader<R> {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        let me = self.project();

        // If we've reached the end of our internal buffer then we need to fetch
        // some more data from the underlying reader.
        if me.buf.is_empty() {
            ready!(tokio_03_read_buf(cx, me.inner, me.buf))?;
        }
        Poll::Ready(Ok(&me.buf[..]))
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        let me = self.project();
        me.buf.advance(amt);
    }
}

#[cfg(feature = "tokio-03")]
impl<R: tokio_03_dep::io::AsyncRead + tokio_03_dep::io::AsyncWrite> tokio_03_dep::io::AsyncWrite
    for BufReader<R>
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.get_pin_mut().poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_pin_mut().poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_pin_mut().poll_shutdown(cx)
    }
}

#[cfg(feature = "tokio")]
impl<R: tokio_dep::io::AsyncRead> tokio_dep::io::AsyncRead for BufReader<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio_dep::io::ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        // If we don't have any buffered data and we're doing a massive read
        // (larger than our internal buffer), bypass our internal buffer
        // entirely.
        if !self.buf.has_remaining_mut() && buf.remaining() >= self.buf.len() {
            let res = ready!(self.as_mut().get_pin_mut().poll_read(cx, buf));
            self.discard_buffer();
            return Poll::Ready(res);
        }
        let rem = ready!(self.as_mut().poll_fill_buf(cx))?;
        let amt = std::cmp::min(rem.len(), buf.remaining());
        buf.put_slice(&rem[..amt]);
        self.consume(amt);
        Poll::Ready(Ok(()))
    }
}

#[cfg(feature = "tokio")]
impl<R: tokio_dep::io::AsyncRead> tokio_dep::io::AsyncBufRead for BufReader<R> {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        let me = self.project();

        // If we've reached the end of our internal buffer then we need to fetch
        // some more data from the underlying reader.
        if me.buf.is_empty() {
            ready!(tokio_read_buf(me.inner, cx, me.buf))?;
        }
        Poll::Ready(Ok(&me.buf[..]))
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        let me = self.project();
        me.buf.advance(amt);
    }
}

#[cfg(feature = "tokio")]
impl<R: tokio_dep::io::AsyncRead + tokio_dep::io::AsyncWrite> tokio_dep::io::AsyncWrite
    for BufReader<R>
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.get_pin_mut().poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_pin_mut().poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_pin_mut().poll_shutdown(cx)
    }
}

impl<R: Read> Read for BufReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // If we don't have any buffered data and we're doing a massive read
        // (larger than our internal buffer), bypass our internal buffer
        // entirely.
        if !self.buf.has_remaining_mut() && buf.len() >= self.buf.len() {
            let res = self.read(buf);
            self.buf.clear();
            return res;
        }
        let nread = {
            let mut rem = self.fill_buf()?;
            rem.read(buf)?
        };
        self.consume(nread);
        Ok(nread)
    }
}

impl<R: Read> BufRead for BufReader<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        // If we've reached the end of our internal buffer then we need to fetch
        // some more data from the underlying reader.
        // Branch using `>=` instead of the more correct `==`
        // to tell the compiler that the pos..cap slice is always valid.

        if self.buf.is_empty() {
            Bufferless.extend_buf_sync(self)?;
        }
        Ok(&self.buf[..])
    }

    fn consume(&mut self, amt: usize) {
        self.buf.advance(amt);
    }
}

#[cfg(test)]
#[cfg(feature = "tokio-02")]
mod tests {
    use super::{BufReader, Bufferless, CombineRead};

    use std::{io, pin::Pin};

    use {
        bytes_05::BytesMut,
        tokio_02_dep::{
            self as tokio,
            io::{AsyncRead, AsyncReadExt},
        },
    };

    impl<R: AsyncRead> BufReader<R> {
        async fn extend_buf_tokio_02(mut self: Pin<&mut Self>) -> io::Result<usize> {
            crate::future_ext::poll_fn(|cx| Bufferless.poll_extend_buf(cx, self.as_mut())).await
        }
    }

    #[tokio::test]
    async fn buf_reader() {
        let mut read = BufReader::with_capacity(3, &[1u8, 2, 3, 4, 5, 6, 7, 8, 9, 0][..]);

        let mut buf = [0u8; 3];
        read.read(&mut buf).await.unwrap();
        assert_eq!(buf, [1, 2, 3]);

        let mut buf = [0u8; 3];
        read.read(&mut buf).await.unwrap();
        assert_eq!(buf, [4, 5, 6]);

        let mut buf = [0u8; 3];
        read.read(&mut buf).await.unwrap();
        assert_eq!(buf, [7, 8, 9]);

        let mut buf = [1u8; 3];
        read.read(&mut buf).await.unwrap();
        assert_eq!(buf, [0, 1, 1]);
    }

    #[tokio::test]
    async fn buf_reader_buf() {
        let mut read = BufReader::with_capacity(3, &[1u8, 2, 3, 4, 5, 6, 7, 8, 9, 0][..]);

        let mut buf = BytesMut::with_capacity(3);
        read.read_buf(&mut buf).await.unwrap();
        assert_eq!(&buf[..], [1, 2, 3]);

        read.read_buf(&mut buf).await.unwrap();
        assert_eq!(&buf[..], [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    }

    #[tokio::test]
    async fn buf_reader_extend_buf() {
        let read = BufReader::with_capacity(3, &[1u8, 2, 3, 4, 5, 6, 7, 8, 9, 0][..]);
        futures_03_dep::pin_mut!(read);

        assert_eq!(read.as_mut().extend_buf_tokio_02().await.unwrap(), 3);
        assert_eq!(read.buffer(), [1, 2, 3]);

        assert_eq!(read.as_mut().extend_buf_tokio_02().await.unwrap(), 7);
        assert_eq!(read.buffer(), [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    }
}

#[cfg(test)]
#[cfg(feature = "tokio")]
mod tests_tokio_1 {
    use super::{BufReader, Bufferless, CombineRead};

    use std::{io, pin::Pin};

    use {
        bytes::BytesMut,
        tokio_dep::{
            self as tokio,
            io::{AsyncRead, AsyncReadExt},
        },
    };

    impl<R: AsyncRead> BufReader<R> {
        async fn extend_buf_tokio(mut self: Pin<&mut Self>) -> io::Result<usize> {
            crate::future_ext::poll_fn(|cx| Bufferless.poll_extend_buf(cx, self.as_mut())).await
        }
    }

    #[tokio::test]
    async fn buf_reader() {
        let mut read = BufReader::with_capacity(3, &[1u8, 2, 3, 4, 5, 6, 7, 8, 9, 0][..]);

        let mut buf = [0u8; 3];
        read.read(&mut buf).await.unwrap();
        assert_eq!(buf, [1, 2, 3]);

        let mut buf = [0u8; 3];
        read.read(&mut buf).await.unwrap();
        assert_eq!(buf, [4, 5, 6]);

        let mut buf = [0u8; 3];
        read.read(&mut buf).await.unwrap();
        assert_eq!(buf, [7, 8, 9]);

        let mut buf = [1u8; 3];
        read.read(&mut buf).await.unwrap();
        assert_eq!(buf, [0, 1, 1]);
    }

    #[tokio::test]
    async fn buf_reader_buf() {
        let mut read = BufReader::with_capacity(3, &[1u8, 2, 3, 4, 5, 6, 7, 8, 9, 0][..]);

        let mut buf = BytesMut::with_capacity(3);
        read.read_buf(&mut buf).await.unwrap();
        assert_eq!(&buf[..], [1, 2, 3]);

        read.read_buf(&mut buf).await.unwrap();
        assert_eq!(&buf[..], [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    }

    #[tokio::test]
    async fn buf_reader_extend_buf() {
        let read = BufReader::with_capacity(3, &[1u8, 2, 3, 4, 5, 6, 7, 8, 9, 0][..]);
        futures_03_dep::pin_mut!(read);

        assert_eq!(read.as_mut().extend_buf_tokio().await.unwrap(), 3);
        assert_eq!(read.buffer(), [1, 2, 3]);

        assert_eq!(read.as_mut().extend_buf_tokio().await.unwrap(), 7);
        assert_eq!(read.buffer(), [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    }
}

#[cfg(test)]
mod tests_sync {
    use super::{BufReader, Bufferless, CombineSyncRead};

    use std::io::Read;

    #[test]
    #[allow(clippy::unused_io_amount)]
    fn buf_reader() {
        let mut read = BufReader::with_capacity(3, &[1u8, 2, 3, 4, 5, 6, 7, 8, 9, 0][..]);

        let mut buf = [0u8; 3];
        read.read(&mut buf).unwrap();
        assert_eq!(buf, [1, 2, 3]);

        let mut buf = [0u8; 3];
        read.read(&mut buf).unwrap();
        assert_eq!(buf, [4, 5, 6]);

        let mut buf = [0u8; 3];
        read.read(&mut buf).unwrap();
        assert_eq!(buf, [7, 8, 9]);

        let mut buf = [1u8; 3];
        read.read(&mut buf).unwrap();
        assert_eq!(buf, [0, 1, 1]);
    }

    #[test]
    fn buf_reader_extend_buf() {
        let mut read = BufReader::with_capacity(3, &[1u8, 2, 3, 4, 5, 6, 7, 8, 9, 0][..]);

        assert_eq!(Bufferless.extend_buf_sync(&mut read).unwrap(), 3);
        assert_eq!(read.buffer(), [1, 2, 3]);

        assert_eq!(Bufferless.extend_buf_sync(&mut read).unwrap(), 7);
        assert_eq!(read.buffer(), [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    }
}
