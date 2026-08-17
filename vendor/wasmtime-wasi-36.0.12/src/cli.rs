use crate::p2;
use std::pin::Pin;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite, empty};
use wasmtime_wasi_io::streams::{InputStream, OutputStream};

mod empty;
mod file;
mod locked_async;
mod mem;
mod stdout;
mod worker_thread_stdin;

pub use self::file::{InputFile, OutputFile};
pub use self::locked_async::{AsyncStdinStream, AsyncStdoutStream};

// Convenience reexport for stdio types so tokio doesn't have to be imported
// itself.
#[doc(no_inline)]
pub use tokio::io::{Stderr, Stdin, Stdout, stderr, stdin, stdout};

pub struct WasiCliCtx {
    pub environment: Vec<(String, String)>,
    pub arguments: Vec<String>,
    pub initial_cwd: Option<String>,
    pub stdin: Box<dyn StdinStream>,
    pub stdout: Box<dyn StdoutStream>,
    pub stderr: Box<dyn StdoutStream>,
}

impl Default for WasiCliCtx {
    fn default() -> WasiCliCtx {
        WasiCliCtx {
            environment: Vec::new(),
            arguments: Vec::new(),
            initial_cwd: None,
            stdin: Box::new(empty()),
            stdout: Box::new(empty()),
            stderr: Box::new(empty()),
        }
    }
}

pub trait IsTerminal {
    /// Returns whether this stream is backed by a TTY.
    fn is_terminal(&self) -> bool;
}

/// A trait used to represent the standard input to a guest program.
///
/// Note that there are many built-in implementations of this trait for various
/// types such as [`tokio::io::Stdin`], [`tokio::io::Empty`], and
/// [`p2::pipe::MemoryInputPipe`].
pub trait StdinStream: IsTerminal + Send {
    /// Creates a fresh stream which is reading stdin.
    ///
    /// Note that the returned stream must share state with all other streams
    /// previously created. Guests may create multiple handles to the same stdin
    /// and they should all be synchronized in their progress through the
    /// program's input.
    ///
    /// Note that this means that if one handle becomes ready for reading they
    /// all become ready for reading. Subsequently if one is read from it may
    /// mean that all the others are no longer ready for reading. This is
    /// basically a consequence of the way the WIT APIs are designed today.
    fn async_stream(&self) -> Box<dyn AsyncRead + Send + Sync>;

    /// Same as [`Self::async_stream`] except that a WASIp2 [`InputStream`] is
    /// returned.
    ///
    /// Note that this has a default implementation which uses
    /// [`p2::pipe::AsyncReadStream`] as an adapter, but this can be overridden
    /// if there's a more specialized implementation available.
    fn p2_stream(&self) -> Box<dyn InputStream> {
        Box::new(p2::pipe::AsyncReadStream::new(Pin::from(
            self.async_stream(),
        )))
    }
}

/// Similar to [`StdinStream`], except for output.
///
/// This is used both for a guest stdin and a guest stdout.
///
/// Note that there are many built-in implementations of this trait for various
/// types such as [`tokio::io::Stdout`], [`tokio::io::Empty`], and
/// [`p2::pipe::MemoryOutputPipe`].
pub trait StdoutStream: IsTerminal + Send {
    /// Returns a fresh new stream which can write to this output stream.
    ///
    /// Note that all output streams should output to the same logical source.
    /// This means that it's possible for each independent stream to acquire a
    /// separate "permit" to write and then act on that permit. Note that
    /// additionally at this time once a permit is "acquired" there's no way to
    /// release it, for example you can wait for readiness and then never
    /// actually write in WASI. This means that acquisition of a permit for one
    /// stream cannot discount the size of a permit another stream could
    /// obtain.
    ///
    /// Implementations must be able to handle this
    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync>;

    /// Same as [`Self::async_stream`] except that a WASIp2 [`OutputStream`] is
    /// returned.
    ///
    /// Note that this has a default implementation which uses
    /// [`p2::pipe::AsyncWriteStream`] as an adapter, but this can be overridden
    /// if there's a more specialized implementation available.
    fn p2_stream(&self) -> Box<dyn OutputStream> {
        Box::new(p2::pipe::AsyncWriteStream::new(
            8192, // FIXME: extract this to a constant.
            Pin::from(self.async_stream()),
        ))
    }
}

// Forward `&T => T`
impl<T: ?Sized + IsTerminal> IsTerminal for &T {
    fn is_terminal(&self) -> bool {
        T::is_terminal(self)
    }
}
impl<T: ?Sized + StdinStream + Sync> StdinStream for &T {
    fn p2_stream(&self) -> Box<dyn InputStream> {
        T::p2_stream(self)
    }
    fn async_stream(&self) -> Box<dyn AsyncRead + Send + Sync> {
        T::async_stream(self)
    }
}
impl<T: ?Sized + StdoutStream + Sync> StdoutStream for &T {
    fn p2_stream(&self) -> Box<dyn OutputStream> {
        T::p2_stream(self)
    }
    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync> {
        T::async_stream(self)
    }
}

// Forward `&mut T => T`
impl<T: ?Sized + IsTerminal> IsTerminal for &mut T {
    fn is_terminal(&self) -> bool {
        T::is_terminal(self)
    }
}
impl<T: ?Sized + StdinStream + Sync> StdinStream for &mut T {
    fn p2_stream(&self) -> Box<dyn InputStream> {
        T::p2_stream(self)
    }
    fn async_stream(&self) -> Box<dyn AsyncRead + Send + Sync> {
        T::async_stream(self)
    }
}
impl<T: ?Sized + StdoutStream + Sync> StdoutStream for &mut T {
    fn p2_stream(&self) -> Box<dyn OutputStream> {
        T::p2_stream(self)
    }
    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync> {
        T::async_stream(self)
    }
}

// Forward `Box<T> => T`
impl<T: ?Sized + IsTerminal> IsTerminal for Box<T> {
    fn is_terminal(&self) -> bool {
        T::is_terminal(self)
    }
}
impl<T: ?Sized + StdinStream + Sync> StdinStream for Box<T> {
    fn p2_stream(&self) -> Box<dyn InputStream> {
        T::p2_stream(self)
    }
    fn async_stream(&self) -> Box<dyn AsyncRead + Send + Sync> {
        T::async_stream(self)
    }
}
impl<T: ?Sized + StdoutStream + Sync> StdoutStream for Box<T> {
    fn p2_stream(&self) -> Box<dyn OutputStream> {
        T::p2_stream(self)
    }
    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync> {
        T::async_stream(self)
    }
}

// Forward `Arc<T> => T`
impl<T: ?Sized + IsTerminal> IsTerminal for Arc<T> {
    fn is_terminal(&self) -> bool {
        T::is_terminal(self)
    }
}
impl<T: ?Sized + StdinStream + Sync> StdinStream for Arc<T> {
    fn p2_stream(&self) -> Box<dyn InputStream> {
        T::p2_stream(self)
    }
    fn async_stream(&self) -> Box<dyn AsyncRead + Send + Sync> {
        T::async_stream(self)
    }
}
impl<T: ?Sized + StdoutStream + Sync> StdoutStream for Arc<T> {
    fn p2_stream(&self) -> Box<dyn OutputStream> {
        T::p2_stream(self)
    }
    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync> {
        T::async_stream(self)
    }
}

#[cfg(test)]
mod test {
    use crate::cli::{AsyncStdoutStream, StdinStream, StdoutStream};
    use crate::p2::{self, OutputStream};
    use anyhow::Result;
    use bytes::Bytes;
    use tokio::io::AsyncReadExt;

    #[test]
    fn memory_stdin_stream() {
        // A StdinStream has the property that there are multiple
        // InputStreams created, using the stream() method which are each
        // views on the same shared state underneath. Consuming input on one
        // stream results in consuming that input on all streams.
        //
        // The simplest way to measure this is to check if the MemoryInputPipe
        // impl of StdinStream follows this property.

        let pipe =
            p2::pipe::MemoryInputPipe::new("the quick brown fox jumped over the three lazy dogs");

        let mut view1 = pipe.p2_stream();
        let mut view2 = pipe.p2_stream();

        let read1 = view1.read(10).expect("read first 10 bytes");
        assert_eq!(read1, "the quick ".as_bytes(), "first 10 bytes");
        let read2 = view2.read(10).expect("read second 10 bytes");
        assert_eq!(read2, "brown fox ".as_bytes(), "second 10 bytes");
        let read3 = view1.read(10).expect("read third 10 bytes");
        assert_eq!(read3, "jumped ove".as_bytes(), "third 10 bytes");
        let read4 = view2.read(10).expect("read fourth 10 bytes");
        assert_eq!(read4, "r the thre".as_bytes(), "fourth 10 bytes");
    }

    #[tokio::test]
    async fn async_stdin_stream() {
        // A StdinStream has the property that there are multiple
        // InputStreams created, using the stream() method which are each
        // views on the same shared state underneath. Consuming input on one
        // stream results in consuming that input on all streams.
        //
        // AsyncStdinStream is a slightly more complex impl of StdinStream
        // than the MemoryInputPipe above. We can create an AsyncReadStream
        // from a file on the disk, and an AsyncStdinStream from that common
        // stream, then check that the same property holds as above.

        let dir = tempfile::tempdir().unwrap();
        let mut path = std::path::PathBuf::from(dir.path());
        path.push("file");
        std::fs::write(&path, "the quick brown fox jumped over the three lazy dogs").unwrap();

        let file = tokio::fs::File::open(&path)
            .await
            .expect("open created file");
        let stdin_stream = super::AsyncStdinStream::new(file);

        use super::StdinStream;

        let mut view1 = stdin_stream.p2_stream();
        let mut view2 = stdin_stream.p2_stream();

        view1.ready().await;

        let read1 = view1.read(10).expect("read first 10 bytes");
        assert_eq!(read1, "the quick ".as_bytes(), "first 10 bytes");
        let read2 = view2.read(10).expect("read second 10 bytes");
        assert_eq!(read2, "brown fox ".as_bytes(), "second 10 bytes");
        let read3 = view1.read(10).expect("read third 10 bytes");
        assert_eq!(read3, "jumped ove".as_bytes(), "third 10 bytes");
        let read4 = view2.read(10).expect("read fourth 10 bytes");
        assert_eq!(read4, "r the thre".as_bytes(), "fourth 10 bytes");
    }

    #[tokio::test]
    async fn async_stdout_stream_unblocks() {
        let (mut read, write) = tokio::io::duplex(32);
        let stdout = AsyncStdoutStream::new(32, write);

        let task = tokio::task::spawn(async move {
            let mut stream = stdout.p2_stream();
            blocking_write_and_flush(&mut *stream, "x".into())
                .await
                .unwrap();
        });

        let mut buf = [0; 100];
        let n = read.read(&mut buf).await.unwrap();
        assert_eq!(&buf[..n], b"x");

        task.await.unwrap();
    }

    async fn blocking_write_and_flush(s: &mut dyn OutputStream, mut bytes: Bytes) -> Result<()> {
        while !bytes.is_empty() {
            let permit = s.write_ready().await?;
            let len = bytes.len().min(permit);
            let chunk = bytes.split_to(len);
            s.write(chunk)?;
        }

        s.flush()?;
        s.write_ready().await?;
        Ok(())
    }
}
