use crate::poll::Pollable;
use alloc::boxed::Box;
use anyhow::Result;
use bytes::Bytes;

/// `Pollable::ready()` for `InputStream` and `OutputStream` may return
/// prematurely due to `io::ErrorKind::WouldBlock`.
///
/// To ensure that `blocking_` functions return a valid non-empty result,
/// we use a loop with a maximum iteration limit.
///
/// This constant defines the maximum number of loop attempts allowed.
const MAX_BLOCKING_ATTEMPTS: u8 = 10;

/// Host trait for implementing the `wasi:io/streams.input-stream` resource: A
/// bytestream which can be read from.
#[async_trait::async_trait]
pub trait InputStream: Pollable {
    /// Reads up to `size` bytes, returning a buffer holding these bytes on
    /// success.
    ///
    /// This function does not block the current thread and is the equivalent of
    /// a non-blocking read. On success all bytes read are returned through
    /// `Bytes`, which is no larger than the `size` provided. If the returned
    /// list of `Bytes` is empty then no data is ready to be read at this time.
    ///
    /// # Errors
    ///
    /// The [`StreamError`] return value communicates when this stream is
    /// closed, when a read fails, or when a trap should be generated.
    fn read(&mut self, size: usize) -> StreamResult<Bytes>;

    /// Similar to `read`, except that it blocks until at least one byte can be
    /// read.
    async fn blocking_read(&mut self, size: usize) -> StreamResult<Bytes> {
        if size == 0 {
            self.ready().await;
            return self.read(size);
        }

        let mut i = 0;
        loop {
            // This `ready` call may return prematurely due to `io::ErrorKind::WouldBlock`.
            self.ready().await;
            let data = self.read(size)?;
            if !data.is_empty() {
                return Ok(data);
            }
            if i >= MAX_BLOCKING_ATTEMPTS {
                return Err(StreamError::trap("max blocking attempts exceeded"));
            }
            i += 1;
        }
    }

    /// Same as the `read` method except that bytes are skipped.
    ///
    /// Note that this method is non-blocking like `read` and returns the same
    /// errors.
    fn skip(&mut self, nelem: usize) -> StreamResult<usize> {
        let bs = self.read(nelem)?;
        Ok(bs.len())
    }

    /// Similar to `skip`, except that it blocks until at least one byte can be
    /// skipped.
    async fn blocking_skip(&mut self, nelem: usize) -> StreamResult<usize> {
        let bs = self.blocking_read(nelem).await?;
        Ok(bs.len())
    }

    /// Cancel any asynchronous work and wait for it to wrap up.
    async fn cancel(&mut self) {}
}

/// Representation of the `error` resource type in the `wasi:io/error`
/// interface.
///
/// This is currently `anyhow::Error` to retain full type information for
/// errors.
pub type Error = anyhow::Error;

pub type StreamResult<T> = Result<T, StreamError>;

#[derive(Debug)]
pub enum StreamError {
    Closed,
    LastOperationFailed(anyhow::Error),
    Trap(anyhow::Error),
}

impl StreamError {
    pub fn trap(msg: &str) -> StreamError {
        StreamError::Trap(anyhow::anyhow!("{msg}"))
    }
}

impl alloc::fmt::Display for StreamError {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        match self {
            StreamError::Closed => write!(f, "closed"),
            StreamError::LastOperationFailed(e) => write!(f, "last operation failed: {e}"),
            StreamError::Trap(e) => write!(f, "trap: {e}"),
        }
    }
}

impl core::error::Error for StreamError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            StreamError::Closed => None,
            StreamError::LastOperationFailed(e) | StreamError::Trap(e) => e.source(),
        }
    }
}

impl From<wasmtime::component::ResourceTableError> for StreamError {
    fn from(error: wasmtime::component::ResourceTableError) -> Self {
        Self::Trap(error.into())
    }
}

/// Host trait for implementing the `wasi:io/streams.output-stream` resource:
/// A bytestream which can be written to.
#[async_trait::async_trait]
pub trait OutputStream: Pollable {
    /// Write bytes after obtaining a permit to write those bytes
    ///
    /// Prior to calling [`write`](Self::write) the caller must call
    /// [`check_write`](Self::check_write), which resolves to a non-zero permit
    ///
    /// This method must never block.  The [`check_write`](Self::check_write)
    /// permit indicates the maximum amount of bytes that are permitted to be
    /// written in a single [`write`](Self::write) following the
    /// [`check_write`](Self::check_write) resolution.
    ///
    /// # Errors
    ///
    /// Returns a [`StreamError`] if:
    /// - stream is closed
    /// - prior operation ([`write`](Self::write) or [`flush`](Self::flush)) failed
    /// - caller performed an illegal operation (e.g. wrote more bytes than were permitted)
    fn write(&mut self, bytes: Bytes) -> StreamResult<()>;

    /// Trigger a flush of any bytes buffered in this stream implementation.
    ///
    /// This method may be called at any time and must never block.
    ///
    /// After this method is called, [`check_write`](Self::check_write) must
    /// pend until flush is complete.
    ///
    /// When [`check_write`](Self::check_write) becomes ready after a flush,
    /// that guarantees that all prior writes have been flushed from the
    /// implementation successfully, or that any error associated with those
    /// writes is reported in the return value of [`flush`](Self::flush) or
    /// [`check_write`](Self::check_write)
    ///
    /// # Errors
    ///
    /// Returns a [`StreamError`] if:
    /// - stream is closed
    /// - prior operation ([`write`](Self::write) or [`flush`](Self::flush)) failed
    /// - caller performed an illegal operation (e.g. wrote more bytes than were permitted)
    fn flush(&mut self) -> StreamResult<()>;

    /// Returns the number of bytes that are ready to be written to this stream.
    ///
    /// Zero bytes indicates that this stream is not currently ready for writing
    /// and `ready()` must be awaited first.
    ///
    /// Note that this method does not block.
    ///
    /// # Errors
    ///
    /// Returns an [`StreamError`] if:
    /// - stream is closed
    /// - prior operation ([`write`](Self::write) or [`flush`](Self::flush)) failed
    fn check_write(&mut self) -> StreamResult<usize>;

    /// Perform a write of up to 4096 bytes, and then flush the stream. Block
    /// until all of these operations are complete, or an error occurs.
    ///
    /// This is a convenience wrapper around the use of `check-write`,
    /// `subscribe`, `write`, and `flush`, and is implemented with the
    /// following pseudo-code:
    ///
    /// ```text
    /// let pollable = this.subscribe();
    /// while !contents.is_empty() {
    ///     // Wait for the stream to become writable
    ///     pollable.block();
    ///     let Ok(n) = this.check-write(); // eliding error handling
    ///     let len = min(n, contents.len());
    ///     let (chunk, rest) = contents.split_at(len);
    ///     this.write(chunk  );            // eliding error handling
    ///     contents = rest;
    /// }
    /// this.flush();
    /// // Wait for completion of `flush`
    /// pollable.block();
    /// // Check for any errors that arose during `flush`
    /// let _ = this.check-write();         // eliding error handling
    /// ```
    async fn blocking_write_and_flush(&mut self, mut bytes: Bytes) -> StreamResult<()> {
        loop {
            let permit = self.write_ready().await?;
            let len = bytes.len().min(permit);
            let chunk = bytes.split_to(len);
            self.write(chunk)?;
            if bytes.is_empty() {
                break;
            }
        }

        // If the stream encounters an error, return it, but if the stream
        // has become closed, do not.
        match self.flush() {
            Ok(_) => {}
            Err(StreamError::Closed) => {}
            Err(e) => Err(e)?,
        };
        match self.write_ready().await {
            Ok(_) => {}
            Err(StreamError::Closed) => {}
            Err(e) => Err(e)?,
        };

        Ok(())
    }

    /// Repeatedly write a byte to a stream.
    /// Important: this write must be non-blocking!
    /// Returning an Err which downcasts to a [`StreamError`] will be
    /// reported to Wasm as the empty error result. Otherwise, errors will trap.
    fn write_zeroes(&mut self, nelem: usize) -> StreamResult<()> {
        let n = self.check_write()?;
        if nelem > n {
            return Err(StreamError::trap(
                "cannot write more zeroes than `check_write` allows",
            ));
        };
        // TODO: We could optimize this to not allocate one big zeroed buffer, and instead write
        // repeatedly from a 'static buffer of zeros.
        let bs = Bytes::from_iter(core::iter::repeat(0).take(nelem));
        self.write(bs)?;
        Ok(())
    }

    /// Simultaneously waits for this stream to be writable and then returns how
    /// much may be written or the last error that happened.
    async fn write_ready(&mut self) -> StreamResult<usize> {
        let mut i = 0;
        loop {
            // This `ready` call may return prematurely due to `io::ErrorKind::WouldBlock`.
            self.ready().await;
            let n = self.check_write()?;
            if n > 0 {
                return Ok(n);
            }
            if i >= MAX_BLOCKING_ATTEMPTS {
                return Err(StreamError::trap("max blocking attempts exceeded"));
            }
            i += 1;
        }
    }

    /// Cancel any asynchronous work and wait for it to wrap up.
    async fn cancel(&mut self) {}
}

#[async_trait::async_trait]
impl Pollable for Box<dyn OutputStream> {
    async fn ready(&mut self) {
        (**self).ready().await
    }
}

#[async_trait::async_trait]
impl Pollable for Box<dyn InputStream> {
    async fn ready(&mut self) {
        (**self).ready().await
    }
}

pub type DynInputStream = Box<dyn InputStream>;

pub type DynOutputStream = Box<dyn OutputStream>;
