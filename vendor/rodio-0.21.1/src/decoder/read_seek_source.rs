use std::io::{Read, Result, Seek, SeekFrom};

use symphonia::core::io::MediaSource;

use super::Settings;

/// A wrapper around a `Read + Seek` type that implements Symphonia's `MediaSource` trait.
///
/// This type allows standard Rust I/O types to be used with Symphonia's media framework
/// by implementing the required `MediaSource` trait.
pub struct ReadSeekSource<T: Read + Seek + Send + Sync> {
    /// The wrapped reader/seeker
    inner: T,
    /// Optional length of the media source in bytes.
    /// When known, this can help with seeking and duration calculations.
    byte_len: Option<u64>,
    /// Whether this media source reports as seekable.
    is_seekable: bool,
}

impl<T: Read + Seek + Send + Sync> ReadSeekSource<T> {
    /// Creates a new `ReadSeekSource` by wrapping a reader/seeker.
    ///
    /// # Arguments
    /// * `inner` - The reader/seeker to wrap
    /// * `settings` - Decoder settings for configuring the source
    #[inline]
    pub fn new(inner: T, settings: &Settings) -> Self {
        ReadSeekSource {
            inner,
            byte_len: settings.byte_len,
            is_seekable: settings.is_seekable,
        }
    }
}

impl<T: Read + Seek + Send + Sync> MediaSource for ReadSeekSource<T> {
    /// Returns whether this media source reports as seekable.
    #[inline]
    fn is_seekable(&self) -> bool {
        self.is_seekable
    }

    /// Returns the total length of the media source in bytes, if known.
    #[inline]
    fn byte_len(&self) -> Option<u64> {
        self.byte_len
    }
}

impl<T: Read + Seek + Send + Sync> Read for ReadSeekSource<T> {
    #[inline]
    /// Reads bytes from the underlying reader into the provided buffer.
    ///
    /// Delegates to the inner reader's implementation.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.inner.read(buf)
    }
}

impl<T: Read + Seek + Send + Sync> Seek for ReadSeekSource<T> {
    /// Seeks to a position in the underlying reader.
    ///
    /// Delegates to the inner reader's implementation.
    #[inline]
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.inner.seek(pos)
    }
}
