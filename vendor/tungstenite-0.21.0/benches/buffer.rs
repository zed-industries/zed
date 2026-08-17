use std::io::{Cursor, Read, Result as IoResult};

use bytes::Buf;
use criterion::*;
use input_buffer::InputBuffer;

use tungstenite::buffer::ReadBuffer;

const CHUNK_SIZE: usize = 4096;

/// A FIFO buffer for reading packets from the network.
#[derive(Debug)]
pub struct StackReadBuffer<const CHUNK_SIZE: usize> {
    storage: Cursor<Vec<u8>>,
    chunk: [u8; CHUNK_SIZE],
}

impl<const CHUNK_SIZE: usize> StackReadBuffer<CHUNK_SIZE> {
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
        Self { storage: Cursor::new(part), chunk: [0; CHUNK_SIZE] }
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
        let size = stream.read(&mut self.chunk)?;
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

impl<const CHUNK_SIZE: usize> Buf for StackReadBuffer<CHUNK_SIZE> {
    fn remaining(&self) -> usize {
        Buf::remaining(self.as_cursor())
    }

    fn chunk(&self) -> &[u8] {
        Buf::chunk(self.as_cursor())
    }

    fn advance(&mut self, cnt: usize) {
        Buf::advance(self.as_cursor_mut(), cnt)
    }
}

impl<const CHUNK_SIZE: usize> Default for StackReadBuffer<CHUNK_SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

#[inline]
fn input_buffer(mut stream: impl Read) {
    let mut buffer = InputBuffer::with_capacity(CHUNK_SIZE);
    while buffer.read_from(&mut stream).unwrap() != 0 {}
}

#[inline]
fn stack_read_buffer(mut stream: impl Read) {
    let mut buffer = StackReadBuffer::<CHUNK_SIZE>::new();
    while buffer.read_from(&mut stream).unwrap() != 0 {}
}

#[inline]
fn heap_read_buffer(mut stream: impl Read) {
    let mut buffer = ReadBuffer::<CHUNK_SIZE>::new();
    while buffer.read_from(&mut stream).unwrap() != 0 {}
}

fn benchmark(c: &mut Criterion) {
    const STREAM_SIZE: usize = 1024 * 1024 * 4;
    let data: Vec<u8> = (0..STREAM_SIZE).map(|_| rand::random()).collect();
    let stream = Cursor::new(data);

    let mut group = c.benchmark_group("buffers");
    group.throughput(Throughput::Bytes(STREAM_SIZE as u64));
    group.bench_function("InputBuffer", |b| b.iter(|| input_buffer(black_box(stream.clone()))));
    group.bench_function("ReadBuffer (stack)", |b| {
        b.iter(|| stack_read_buffer(black_box(stream.clone())))
    });
    group.bench_function("ReadBuffer (heap)", |b| {
        b.iter(|| heap_read_buffer(black_box(stream.clone())))
    });
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
