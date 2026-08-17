//! A buffer for reading data from the network.
//!
//! The `ReadBuffer` is a buffer of bytes similar to a first-in, first-out queue.
//! It is filled by reading from a stream supporting `Read` and is then
//! accessible as a cursor for reading bytes.

use std::io::{Cursor, Read, Result as IoResult};

use bytes::Buf;

/// A FIFO buffer for reading packets from the network.
#[derive(Debug)]
pub struct ReadBuffer<const CHUNK_SIZE: usize> {
    storage: Cursor<Vec<u8>>,
    chunk: Box<[u8; CHUNK_SIZE]>,
}

impl<const CHUNK_SIZE: usize> ReadBuffer<CHUNK_SIZE> {
    /// Create a new empty input buffer.
    pub fn new() -> Self {
        Self::with_capacity(CHUNK_SIZE)
    }

    /// Create a new empty input buffer with a given `capacity`.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_partially_read(Vec::with_capacity(capacity))
    }

    /// Create a input buffer filled with previously read data.
    pub fn from_partially_read(part: Vec<u8>) -> Self {
        Self { storage: Cursor::new(part), chunk: Box::new([0; CHUNK_SIZE]) }
    }

    /// Get a cursor to the data storage.
    pub fn as_cursor(&self) -> &Cursor<Vec<u8>> {
        &self.storage
    }

    /// Get a cursor to the mutable data storage.
    pub fn as_cursor_mut(&mut self) -> &mut Cursor<Vec<u8>> {
        &mut self.storage
    }

    /// Consume the `ReadBuffer` and get the internal storage.
    pub fn into_vec(mut self) -> Vec<u8> {
        // Current implementation of `tungstenite-rs` expects that the `into_vec()` drains
        // the data from the container that has already been read by the cursor.
        self.clean_up();

        // Now we can safely return the internal container.
        self.storage.into_inner()
    }

    /// Read next portion of data from the given input stream.
    pub fn read_from<S: Read>(&mut self, stream: &mut S) -> IoResult<usize> {
        self.clean_up();
        let size = stream.read(&mut *self.chunk)?;
        self.storage.get_mut().extend_from_slice(&self.chunk[..size]);
        Ok(size)
    }

    /// Cleans ups the part of the vector that has been already read by the cursor.
    fn clean_up(&mut self) {
        let pos = self.storage.position() as usize;
        self.storage.get_mut().drain(0..pos).count();
        self.storage.set_position(0);
    }
}

impl<const CHUNK_SIZE: usize> Buf for ReadBuffer<CHUNK_SIZE> {
    fn remaining(&self) -> usize {
        Buf::remaining(self.as_cursor())
    }

    fn chunk(&self) -> &[u8] {
        Buf::chunk(self.as_cursor())
    }

    fn advance(&mut self, cnt: usize) {
        Buf::advance(self.as_cursor_mut(), cnt);
    }
}

impl<const CHUNK_SIZE: usize> Default for ReadBuffer<CHUNK_SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_reading() {
        let mut input = Cursor::new(b"Hello World!".to_vec());
        let mut buffer = ReadBuffer::<4096>::new();
        let size = buffer.read_from(&mut input).unwrap();
        assert_eq!(size, 12);
        assert_eq!(buffer.chunk(), b"Hello World!");
    }

    #[test]
    fn reading_in_chunks() {
        let mut inp = Cursor::new(b"Hello World!".to_vec());
        let mut buf = ReadBuffer::<4>::new();

        let size = buf.read_from(&mut inp).unwrap();
        assert_eq!(size, 4);
        assert_eq!(buf.chunk(), b"Hell");

        buf.advance(2);
        assert_eq!(buf.chunk(), b"ll");
        assert_eq!(buf.storage.get_mut(), b"Hell");

        let size = buf.read_from(&mut inp).unwrap();
        assert_eq!(size, 4);
        assert_eq!(buf.chunk(), b"llo Wo");
        assert_eq!(buf.storage.get_mut(), b"llo Wo");

        let size = buf.read_from(&mut inp).unwrap();
        assert_eq!(size, 4);
        assert_eq!(buf.chunk(), b"llo World!");
    }
}
