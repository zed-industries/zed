//! Virtual pipes.
//!
//! These types provide easy implementations of `WasiFile` that mimic much of the behavior of Unix
//! pipes. These are particularly helpful for redirecting WASI stdio handles to destinations other
//! than OS files.
//!
//! Some convenience constructors are included for common backing types like `Vec<u8>` and `String`,
//! but the virtual pipes can be instantiated with any `Read` or `Write` type.
//!
use anyhow::anyhow;
use bytes::Bytes;
use std::pin::{Pin, pin};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use tokio::io::{self, AsyncRead, AsyncWrite};
use tokio::sync::mpsc;
use wasmtime_wasi_io::{
    poll::Pollable,
    streams::{InputStream, OutputStream, StreamError},
};

pub use crate::p2::write_stream::AsyncWriteStream;

#[derive(Debug, Clone)]
pub struct MemoryInputPipe {
    buffer: Arc<Mutex<Bytes>>,
}

impl MemoryInputPipe {
    pub fn new(bytes: impl Into<Bytes>) -> Self {
        Self {
            buffer: Arc::new(Mutex::new(bytes.into())),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.lock().unwrap().is_empty()
    }
}

#[async_trait::async_trait]
impl InputStream for MemoryInputPipe {
    fn read(&mut self, size: usize) -> Result<Bytes, StreamError> {
        let mut buffer = self.buffer.lock().unwrap();
        if buffer.is_empty() {
            return Err(StreamError::Closed);
        }

        let size = size.min(buffer.len());
        let read = buffer.split_to(size);
        Ok(read)
    }
}

#[async_trait::async_trait]
impl Pollable for MemoryInputPipe {
    async fn ready(&mut self) {}
}

impl AsyncRead for MemoryInputPipe {
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &mut io::ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let mut buffer = self.buffer.lock().unwrap();
        let size = buf.remaining().min(buffer.len());
        let read = buffer.split_to(size);
        buf.put_slice(&read);
        Poll::Ready(Ok(()))
    }
}

#[derive(Debug, Clone)]
pub struct MemoryOutputPipe {
    capacity: usize,
    buffer: Arc<Mutex<bytes::BytesMut>>,
}

impl MemoryOutputPipe {
    pub fn new(capacity: usize) -> Self {
        MemoryOutputPipe {
            capacity,
            buffer: std::sync::Arc::new(std::sync::Mutex::new(bytes::BytesMut::new())),
        }
    }

    pub fn contents(&self) -> bytes::Bytes {
        self.buffer.lock().unwrap().clone().freeze()
    }

    pub fn try_into_inner(self) -> Option<bytes::BytesMut> {
        std::sync::Arc::into_inner(self.buffer).map(|m| m.into_inner().unwrap())
    }
}

#[async_trait::async_trait]
impl OutputStream for MemoryOutputPipe {
    fn write(&mut self, bytes: Bytes) -> Result<(), StreamError> {
        let mut buf = self.buffer.lock().unwrap();
        if bytes.len() > self.capacity - buf.len() {
            return Err(StreamError::Trap(anyhow!(
                "write beyond capacity of MemoryOutputPipe"
            )));
        }
        buf.extend_from_slice(bytes.as_ref());
        // Always ready for writing
        Ok(())
    }
    fn flush(&mut self) -> Result<(), StreamError> {
        // This stream is always flushed
        Ok(())
    }
    fn check_write(&mut self) -> Result<usize, StreamError> {
        let consumed = self.buffer.lock().unwrap().len();
        if consumed < self.capacity {
            Ok(self.capacity - consumed)
        } else {
            // Since the buffer is full, no more bytes will ever be written
            Err(StreamError::Closed)
        }
    }
}

#[async_trait::async_trait]
impl Pollable for MemoryOutputPipe {
    async fn ready(&mut self) {}
}

impl AsyncWrite for MemoryOutputPipe {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let mut buffer = self.buffer.lock().unwrap();
        let amt = buf.len().min(self.capacity - buffer.len());
        buffer.extend_from_slice(&buf[..amt]);
        Poll::Ready(Ok(amt))
    }
    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

/// Provides a [`InputStream`] impl from a [`tokio::io::AsyncRead`] impl
pub struct AsyncReadStream {
    closed: bool,
    buffer: Option<Result<Bytes, StreamError>>,
    receiver: mpsc::Receiver<Result<Bytes, StreamError>>,
    join_handle: Option<crate::runtime::AbortOnDropJoinHandle<()>>,
}

impl AsyncReadStream {
    /// Create a [`AsyncReadStream`]. In order to use the [`InputStream`] impl
    /// provided by this struct, the argument must impl [`tokio::io::AsyncRead`].
    pub fn new<T: AsyncRead + Send + 'static>(reader: T) -> Self {
        let (sender, receiver) = mpsc::channel(1);
        let join_handle = crate::runtime::spawn(async move {
            let mut reader = pin!(reader);
            loop {
                use tokio::io::AsyncReadExt;
                let mut buf = bytes::BytesMut::with_capacity(crate::MAX_READ_SIZE_ALLOC);
                let sent = match reader.read_buf(&mut buf).await {
                    Ok(nbytes) if nbytes == 0 => sender.send(Err(StreamError::Closed)).await,
                    Ok(_) => sender.send(Ok(buf.freeze())).await,
                    Err(e) => {
                        sender
                            .send(Err(StreamError::LastOperationFailed(e.into())))
                            .await
                    }
                };
                if sent.is_err() {
                    // no more receiver - stop trying to read
                    break;
                }
            }
        });
        AsyncReadStream {
            closed: false,
            buffer: None,
            receiver,
            join_handle: Some(join_handle),
        }
    }
    pub(crate) fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        if self.buffer.is_some() || self.closed {
            return Poll::Ready(());
        }
        match self.receiver.poll_recv(cx) {
            Poll::Ready(Some(res)) => {
                self.buffer = Some(res);
                Poll::Ready(())
            }
            Poll::Ready(None) => {
                panic!("no more sender for an open AsyncReadStream - should be impossible")
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

#[async_trait::async_trait]
impl InputStream for AsyncReadStream {
    fn read(&mut self, size: usize) -> Result<Bytes, StreamError> {
        use mpsc::error::TryRecvError;

        match self.buffer.take() {
            Some(Ok(mut bytes)) => {
                // TODO: de-duplicate the buffer management with the case below
                let len = bytes.len().min(size);
                let rest = bytes.split_off(len);
                if !rest.is_empty() {
                    self.buffer = Some(Ok(rest));
                }
                return Ok(bytes);
            }
            Some(Err(e)) => {
                self.closed = true;
                return Err(e);
            }
            None => {}
        }

        match self.receiver.try_recv() {
            Ok(Ok(mut bytes)) => {
                let len = bytes.len().min(size);
                let rest = bytes.split_off(len);
                if !rest.is_empty() {
                    self.buffer = Some(Ok(rest));
                }

                Ok(bytes)
            }
            Ok(Err(e)) => {
                self.closed = true;
                Err(e)
            }
            Err(TryRecvError::Empty) => Ok(Bytes::new()),
            Err(TryRecvError::Disconnected) => Err(StreamError::Trap(anyhow!(
                "AsyncReadStream sender died - should be impossible"
            ))),
        }
    }

    async fn cancel(&mut self) {
        match self.join_handle.take() {
            Some(task) => _ = task.cancel().await,
            None => {}
        }
    }
}

#[async_trait::async_trait]
impl Pollable for AsyncReadStream {
    async fn ready(&mut self) {
        std::future::poll_fn(|cx| self.poll_ready(cx)).await
    }
}

/// An output stream that consumes all input written to it, and is always ready.
#[derive(Copy, Clone)]
pub struct SinkOutputStream;

#[async_trait::async_trait]
impl OutputStream for SinkOutputStream {
    fn write(&mut self, _buf: Bytes) -> Result<(), StreamError> {
        Ok(())
    }
    fn flush(&mut self) -> Result<(), StreamError> {
        // This stream is always flushed
        Ok(())
    }

    fn check_write(&mut self) -> Result<usize, StreamError> {
        // This stream is always ready for writing.
        Ok(usize::MAX)
    }
}

#[async_trait::async_trait]
impl Pollable for SinkOutputStream {
    async fn ready(&mut self) {}
}

/// A stream that is ready immediately, but will always report that it's closed.
#[derive(Copy, Clone)]
pub struct ClosedInputStream;

#[async_trait::async_trait]
impl InputStream for ClosedInputStream {
    fn read(&mut self, _size: usize) -> Result<Bytes, StreamError> {
        Err(StreamError::Closed)
    }
}

#[async_trait::async_trait]
impl Pollable for ClosedInputStream {
    async fn ready(&mut self) {}
}

/// An output stream that is always closed.
#[derive(Copy, Clone)]
pub struct ClosedOutputStream;

#[async_trait::async_trait]
impl OutputStream for ClosedOutputStream {
    fn write(&mut self, _: Bytes) -> Result<(), StreamError> {
        Err(StreamError::Closed)
    }
    fn flush(&mut self) -> Result<(), StreamError> {
        Err(StreamError::Closed)
    }

    fn check_write(&mut self) -> Result<usize, StreamError> {
        Err(StreamError::Closed)
    }
}

#[async_trait::async_trait]
impl Pollable for ClosedOutputStream {
    async fn ready(&mut self) {}
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;
    use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

    // This is a gross way to handle CI running under qemu for non-x86 architectures.
    #[cfg(not(target_arch = "x86_64"))]
    const TEST_ITERATIONS: usize = 10;

    #[cfg(target_arch = "x86_64")]
    const TEST_ITERATIONS: usize = 100;

    async fn resolves_immediately<F, O>(fut: F) -> O
    where
        F: futures::Future<Output = O>,
    {
        // The input `fut` should resolve immediately, but in case it
        // accidentally doesn't don't hang the test indefinitely. Provide a
        // generous timeout to account for CI sensitivity and various systems.
        tokio::time::timeout(Duration::from_secs(2), fut)
            .await
            .expect("operation timed out")
    }

    async fn never_resolves<F: futures::Future>(fut: F) {
        // The input `fut` should never resolve, so only give it a small window
        // of budget before we time out. If `fut` is actually resolved this
        // should show up as a flaky test.
        tokio::time::timeout(Duration::from_millis(10), fut)
            .await
            .err()
            .expect("operation should time out");
    }

    pub fn simplex(size: usize) -> (impl AsyncRead, impl AsyncWrite) {
        let (a, b) = tokio::io::duplex(size);
        let (_read_half, write_half) = tokio::io::split(a);
        let (read_half, _write_half) = tokio::io::split(b);
        (read_half, write_half)
    }

    #[test_log::test(tokio::test(flavor = "multi_thread"))]
    async fn empty_read_stream() {
        let mut reader = AsyncReadStream::new(tokio::io::empty());

        // In a multi-threaded context, the value of state is not deterministic -- the spawned
        // reader task may run on a different thread.
        match reader.read(10) {
            // The reader task ran before we tried to read, and noticed that the input was empty.
            Err(StreamError::Closed) => {}

            // The reader task hasn't run yet. Call `ready` to await and fill the buffer.
            Ok(bs) => {
                assert!(bs.is_empty());
                resolves_immediately(reader.ready()).await;
                assert!(matches!(reader.read(0), Err(StreamError::Closed)));
            }
            res => panic!("unexpected: {res:?}"),
        }
    }

    #[test_log::test(tokio::test(flavor = "multi_thread"))]
    async fn infinite_read_stream() {
        let mut reader = AsyncReadStream::new(tokio::io::repeat(0));

        let bs = reader.read(10).unwrap();
        if bs.is_empty() {
            // Reader task hasn't run yet. Call `ready` to await and fill the buffer.
            resolves_immediately(reader.ready()).await;
            // Now a read should succeed
            let bs = reader.read(10).unwrap();
            assert_eq!(bs.len(), 10);
        } else {
            assert_eq!(bs.len(), 10);
        }

        // Subsequent reads should succeed
        let bs = reader.read(10).unwrap();
        assert_eq!(bs.len(), 10);

        // Even 0-length reads should succeed and show its open
        let bs = reader.read(0).unwrap();
        assert_eq!(bs.len(), 0);
    }

    async fn finite_async_reader(contents: &[u8]) -> impl AsyncRead + Send + 'static + use<> {
        let (r, mut w) = simplex(contents.len());
        w.write_all(contents).await.unwrap();
        r
    }

    #[test_log::test(tokio::test(flavor = "multi_thread"))]
    async fn finite_read_stream() {
        let mut reader = AsyncReadStream::new(finite_async_reader(&[1; 123]).await);

        let bs = reader.read(123).unwrap();
        if bs.is_empty() {
            // Reader task hasn't run yet. Call `ready` to await and fill the buffer.
            resolves_immediately(reader.ready()).await;
            // Now a read should succeed
            let bs = reader.read(123).unwrap();
            assert_eq!(bs.len(), 123);
        } else {
            assert_eq!(bs.len(), 123);
        }

        // The AsyncRead's should be empty now, but we have a race where the reader task hasn't
        // yet send that to the AsyncReadStream.
        match reader.read(0) {
            Err(StreamError::Closed) => {} // Correct!
            Ok(bs) => {
                assert!(bs.is_empty());
                // Need to await to give this side time to catch up
                resolves_immediately(reader.ready()).await;
                // Now a read should show closed
                assert!(matches!(reader.read(0), Err(StreamError::Closed)));
            }
            res => panic!("unexpected: {res:?}"),
        }
    }

    #[test_log::test(tokio::test(flavor = "multi_thread"))]
    // Test that you can write items into the stream, and they get read out in the order they were
    // written, with the proper indications of readiness for reading:
    async fn multiple_chunks_read_stream() {
        let (r, mut w) = simplex(1024);
        let mut reader = AsyncReadStream::new(r);

        w.write_all(&[123]).await.unwrap();

        let bs = reader.read(1).unwrap();
        if bs.is_empty() {
            // Reader task hasn't run yet. Call `ready` to await and fill the buffer.
            resolves_immediately(reader.ready()).await;
            // Now a read should succeed
            let bs = reader.read(1).unwrap();
            assert_eq!(*bs, [123u8]);
        } else {
            assert_eq!(*bs, [123u8]);
        }

        // The stream should be empty and open now:
        let bs = reader.read(1).unwrap();
        assert!(bs.is_empty());

        // We can wait on readiness and it will time out:
        never_resolves(reader.ready()).await;

        // Still open and empty:
        let bs = reader.read(1).unwrap();
        assert!(bs.is_empty());

        // Put something else in the stream:
        w.write_all(&[45]).await.unwrap();

        // Wait readiness (yes we could possibly win the race and read it out faster, leaving that
        // out of the test for simplicity)
        resolves_immediately(reader.ready()).await;

        // read the something else back out:
        let bs = reader.read(1).unwrap();
        assert_eq!(*bs, [45u8]);

        // nothing else in there:
        let bs = reader.read(1).unwrap();
        assert!(bs.is_empty());

        // We can wait on readiness and it will time out:
        never_resolves(reader.ready()).await;

        // nothing else in there:
        let bs = reader.read(1).unwrap();
        assert!(bs.is_empty());

        // Now close the pipe:
        drop(w);

        // Wait readiness (yes we could possibly win the race and read it out faster, leaving that
        // out of the test for simplicity)
        resolves_immediately(reader.ready()).await;

        // empty and now closed:
        assert!(matches!(reader.read(1), Err(StreamError::Closed)));
    }

    #[test_log::test(tokio::test(flavor = "multi_thread"))]
    // At the moment we are restricting AsyncReadStream from buffering more than 4k. This isn't a
    // suitable design for all applications, and we will probably make a knob or change the
    // behavior at some point, but this test shows the behavior as it is implemented:
    async fn backpressure_read_stream() {
        let (r, mut w) = simplex(4 * crate::MAX_READ_SIZE_ALLOC); // Make sure this buffer isn't a bottleneck
        let mut reader = AsyncReadStream::new(r);

        let writer_task = tokio::task::spawn(async move {
            // Write twice as much as we can buffer up in an AsyncReadStream:
            w.write_all(&[123; 2 * crate::MAX_READ_SIZE_ALLOC])
                .await
                .unwrap();
            w
        });

        resolves_immediately(reader.ready()).await;

        // Now we expect the reader task has sent 4k from the stream to the reader.
        // Try to read out one bigger than the buffer available:
        let bs = reader.read(crate::MAX_READ_SIZE_ALLOC + 1).unwrap();
        assert_eq!(bs.len(), crate::MAX_READ_SIZE_ALLOC);

        // Allow the crank to turn more:
        resolves_immediately(reader.ready()).await;

        // Again we expect the reader task has sent 4k from the stream to the reader.
        // Try to read out one bigger than the buffer available:
        let bs = reader.read(crate::MAX_READ_SIZE_ALLOC + 1).unwrap();
        assert_eq!(bs.len(), crate::MAX_READ_SIZE_ALLOC);

        // The writer task is now finished - join with it:
        let w = resolves_immediately(writer_task).await;

        // And close the pipe:
        drop(w);

        // Allow the crank to turn more:
        resolves_immediately(reader.ready()).await;

        // Now we expect the reader to be empty, and the stream.dropd:
        assert!(matches!(
            reader.read(crate::MAX_READ_SIZE_ALLOC + 1),
            Err(StreamError::Closed)
        ));
    }

    #[test_log::test(test_log::test(tokio::test(flavor = "multi_thread")))]
    async fn sink_write_stream() {
        let mut writer = AsyncWriteStream::new(2048, tokio::io::sink());
        let chunk = Bytes::from_static(&[0; 1024]);

        let readiness = resolves_immediately(writer.write_ready())
            .await
            .expect("write_ready does not trap");
        assert_eq!(readiness, 2048);
        // I can write whatever:
        writer.write(chunk.clone()).expect("write does not error");

        // This may consume 1k of the buffer:
        let readiness = resolves_immediately(writer.write_ready())
            .await
            .expect("write_ready does not trap");
        assert!(
            readiness == 1024 || readiness == 2048,
            "readiness should be 1024 or 2048, got {readiness}"
        );

        if readiness == 1024 {
            writer.write(chunk.clone()).expect("write does not error");

            let readiness = resolves_immediately(writer.write_ready())
                .await
                .expect("write_ready does not trap");
            assert!(
                readiness == 1024 || readiness == 2048,
                "readiness should be 1024 or 2048, got {readiness}"
            );
        }
    }

    #[test_log::test(tokio::test(flavor = "multi_thread"))]
    async fn closed_write_stream() {
        // Run many times because the test is nondeterministic:
        for n in 0..TEST_ITERATIONS {
            closed_write_stream_(n).await
        }
    }
    #[tracing::instrument]
    async fn closed_write_stream_(n: usize) {
        let (reader, writer) = simplex(1);
        let mut writer = AsyncWriteStream::new(1024, writer);

        // Drop the reader to allow the worker to transition to the closed state eventually.
        drop(reader);

        // First the api is going to report the last operation failed, then subsequently
        // it will be reported as closed. We set this flag once we see LastOperationFailed.
        let mut should_be_closed = false;

        // Write some data to the stream to ensure we have data that cannot be flushed.
        let chunk = Bytes::from_static(&[0; 1]);
        writer
            .write(chunk.clone())
            .expect("first write should succeed");

        // The rest of this test should be valid whether or not we check write readiness:
        let mut write_ready_res = None;
        if n % 2 == 0 {
            let r = resolves_immediately(writer.write_ready()).await;
            // Check write readiness:
            match r {
                // worker hasn't processed write yet:
                Ok(1023) => {}
                // worker reports failure:
                Err(StreamError::LastOperationFailed(_)) => {
                    tracing::debug!("discovered stream failure in first write_ready");
                    should_be_closed = true;
                }
                r => panic!("unexpected write_ready: {r:?}"),
            }
            write_ready_res = Some(r);
        }

        // When we drop the simplex reader, that causes the simplex writer to return BrokenPipe on
        // its write. Now that the buffering crank has turned, our next write will give BrokenPipe.
        let flush_res = writer.flush();
        match flush_res {
            // worker reports failure:
            Err(StreamError::LastOperationFailed(_)) => {
                tracing::debug!("discovered stream failure trying to flush");
                assert!(!should_be_closed);
                should_be_closed = true;
            }
            // Already reported failure, now closed
            Err(StreamError::Closed) => {
                assert!(
                    should_be_closed,
                    "expected a LastOperationFailed before we see Closed. {write_ready_res:?}"
                );
            }
            // Also possible the worker hasn't processed write yet:
            Ok(()) => {}
            Err(e) => panic!("unexpected flush error: {e:?} {write_ready_res:?}"),
        }

        // Waiting for the flush to complete should always indicate that the channel has been
        // closed.
        match resolves_immediately(writer.write_ready()).await {
            // worker reports failure:
            Err(StreamError::LastOperationFailed(_)) => {
                tracing::debug!("discovered stream failure trying to flush");
                assert!(!should_be_closed);
            }
            // Already reported failure, now closed
            Err(StreamError::Closed) => {
                assert!(should_be_closed);
            }
            r => {
                panic!(
                    "stream should be reported closed by the end of write_ready after flush, got {r:?}. {write_ready_res:?} {flush_res:?}"
                )
            }
        }
    }

    #[test_log::test(tokio::test(flavor = "multi_thread"))]
    async fn multiple_chunks_write_stream() {
        // Run many times because the test is nondeterministic:
        for n in 0..TEST_ITERATIONS {
            multiple_chunks_write_stream_aux(n).await
        }
    }
    #[tracing::instrument]
    async fn multiple_chunks_write_stream_aux(_: usize) {
        use std::ops::Deref;

        let (mut reader, writer) = simplex(1024);
        let mut writer = AsyncWriteStream::new(1024, writer);

        // Write a chunk:
        let chunk = Bytes::from_static(&[123; 1]);

        let permit = resolves_immediately(writer.write_ready())
            .await
            .expect("write should be ready");
        assert_eq!(permit, 1024);

        writer.write(chunk.clone()).expect("write does not trap");

        // At this point the message will either be waiting for the worker to process the write, or
        // it will be buffered in the simplex channel.
        let permit = resolves_immediately(writer.write_ready())
            .await
            .expect("write should be ready");
        assert!(matches!(permit, 1023 | 1024));

        let mut read_buf = vec![0; chunk.len()];
        let read_len = reader.read_exact(&mut read_buf).await.unwrap();
        assert_eq!(read_len, chunk.len());
        assert_eq!(read_buf.as_slice(), chunk.deref());

        // Write a second, different chunk:
        let chunk2 = Bytes::from_static(&[45; 1]);

        // We're only guaranteed to see a consistent write budget if we flush.
        writer.flush().expect("channel is still alive");

        let permit = resolves_immediately(writer.write_ready())
            .await
            .expect("write should be ready");
        assert_eq!(permit, 1024);

        writer.write(chunk2.clone()).expect("write does not trap");

        // At this point the message will either be waiting for the worker to process the write, or
        // it will be buffered in the simplex channel.
        let permit = resolves_immediately(writer.write_ready())
            .await
            .expect("write should be ready");
        assert!(matches!(permit, 1023 | 1024));

        let mut read2_buf = vec![0; chunk2.len()];
        let read2_len = reader.read_exact(&mut read2_buf).await.unwrap();
        assert_eq!(read2_len, chunk2.len());
        assert_eq!(read2_buf.as_slice(), chunk2.deref());

        // We're only guaranteed to see a consistent write budget if we flush.
        writer.flush().expect("channel is still alive");

        let permit = resolves_immediately(writer.write_ready())
            .await
            .expect("write should be ready");
        assert_eq!(permit, 1024);
    }

    #[test_log::test(tokio::test(flavor = "multi_thread"))]
    async fn backpressure_write_stream() {
        // Run many times because the test is nondeterministic:
        for n in 0..TEST_ITERATIONS {
            backpressure_write_stream_aux(n).await
        }
    }
    #[tracing::instrument]
    async fn backpressure_write_stream_aux(_: usize) {
        use futures::future::poll_immediate;

        // The channel can buffer up to 1k, plus another 1k in the stream, before not
        // accepting more input:
        let (mut reader, writer) = simplex(1024);
        let mut writer = AsyncWriteStream::new(1024, writer);

        let chunk = Bytes::from_static(&[0; 1024]);

        let permit = resolves_immediately(writer.write_ready())
            .await
            .expect("write should be ready");
        assert_eq!(permit, 1024);

        writer.write(chunk.clone()).expect("write succeeds");

        // We might still be waiting for the worker to process the message, or the worker may have
        // processed it and released all the budget back to us.
        let permit = poll_immediate(writer.write_ready()).await;
        assert!(matches!(permit, None | Some(Ok(1024))));

        // Given a little time, the worker will process the message and release all the budget
        // back.
        let permit = resolves_immediately(writer.write_ready())
            .await
            .expect("write should be ready");
        assert_eq!(permit, 1024);

        // Now fill the buffer between here and the writer task. This should always indicate
        // back-pressure because now both buffers (simplex and worker) are full.
        writer.write(chunk.clone()).expect("write does not trap");

        // Try shoving even more down there, and it shouldn't accept more input:
        writer
            .write(chunk.clone())
            .err()
            .expect("unpermitted write does trap");

        // No amount of waiting will resolve the situation, as nothing is emptying the simplex
        // buffer.
        never_resolves(writer.write_ready()).await;

        // There is 2k buffered between the simplex and worker buffers. I should be able to read
        // all of it out:
        let mut buf = [0; 2048];
        reader.read_exact(&mut buf).await.unwrap();

        // and no more:
        never_resolves(reader.read(&mut buf)).await;

        // Now the backpressure should be cleared, and an additional write should be accepted.
        let permit = resolves_immediately(writer.write_ready())
            .await
            .expect("ready is ok");
        assert_eq!(permit, 1024);

        // and the write succeeds:
        writer.write(chunk.clone()).expect("write does not trap");
    }

    #[test_log::test(tokio::test(flavor = "multi_thread"))]
    async fn backpressure_write_stream_with_flush() {
        for n in 0..TEST_ITERATIONS {
            backpressure_write_stream_with_flush_aux(n).await;
        }
    }

    async fn backpressure_write_stream_with_flush_aux(_: usize) {
        // The channel can buffer up to 1k, plus another 1k in the stream, before not
        // accepting more input:
        let (mut reader, writer) = simplex(1024);
        let mut writer = AsyncWriteStream::new(1024, writer);

        let chunk = Bytes::from_static(&[0; 1024]);

        let permit = resolves_immediately(writer.write_ready())
            .await
            .expect("write should be ready");
        assert_eq!(permit, 1024);

        writer.write(chunk.clone()).expect("write succeeds");

        writer.flush().expect("flush succeeds");

        // Waiting for write_ready to resolve after a flush should always show that we have the
        // full budget available, as the message will have flushed to the simplex channel.
        let permit = resolves_immediately(writer.write_ready())
            .await
            .expect("write_ready succeeds");
        assert_eq!(permit, 1024);

        // Write enough to fill the simplex buffer:
        writer.write(chunk.clone()).expect("write does not trap");

        // Writes should be refused until this flush succeeds.
        writer.flush().expect("flush succeeds");

        // Try shoving even more down there, and it shouldn't accept more input:
        writer
            .write(chunk.clone())
            .err()
            .expect("unpermitted write does trap");

        // No amount of waiting will resolve the situation, as nothing is emptying the simplex
        // buffer.
        never_resolves(writer.write_ready()).await;

        // There is 2k buffered between the simplex and worker buffers. I should be able to read
        // all of it out:
        let mut buf = [0; 2048];
        reader.read_exact(&mut buf).await.unwrap();

        // and no more:
        never_resolves(reader.read(&mut buf)).await;

        // Now the backpressure should be cleared, and an additional write should be accepted.
        let permit = resolves_immediately(writer.write_ready())
            .await
            .expect("ready is ok");
        assert_eq!(permit, 1024);

        // and the write succeeds:
        writer.write(chunk.clone()).expect("write does not trap");

        writer.flush().expect("flush succeeds");

        let permit = resolves_immediately(writer.write_ready())
            .await
            .expect("ready is ok");
        assert_eq!(permit, 1024);
    }
}
