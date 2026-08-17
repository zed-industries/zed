//! Composable structures to handle reading an image.


use std::convert::TryFrom;
use std::fmt::Debug;
use std::io::{Read, Seek};
use crate::block::{BlockIndex, UncompressedBlock};
use crate::block::chunk::{Chunk, TileCoordinates};
use crate::error::{Error, Result, u64_to_usize, UnitResult};
use crate::io::{PeekRead, Tracking};
use crate::meta::{MetaData, OffsetTables};
use crate::meta::header::Header;

/// Decode the meta data from a byte source, keeping the source ready for further reading.
/// Continue decoding the remaining bytes by calling `filtered_chunks` or `all_chunks`.
#[derive(Debug)]
pub struct Reader<R> {
    meta_data: MetaData,
    remaining_reader: PeekRead<Tracking<R>>, // TODO does R need to be Seek or is Tracking enough?
}

impl<R: Read + Seek> Reader<R> {

    /// Start the reading process.
    /// Immediately decodes the meta data into an internal field.
    /// Access it via`meta_data()`.
    pub fn read_from_buffered(read: R, pedantic: bool) -> Result<Self> {
        let mut remaining_reader = PeekRead::new(Tracking::new(read));
        let meta_data = MetaData::read_validated_from_buffered_peekable(&mut remaining_reader, pedantic)?;
        Ok(Self { meta_data, remaining_reader })
    }

    // must not be mutable, as reading the file later on relies on the meta data
    /// The decoded exr meta data from the file.
    pub fn meta_data(&self) -> &MetaData { &self.meta_data }

    /// The decoded exr meta data from the file.
    pub fn headers(&self) -> &[Header] { &self.meta_data.headers }

    /// Obtain the meta data ownership.
    pub fn into_meta_data(self) -> MetaData { self.meta_data }

    /// Prepare to read all the chunks from the file.
    /// Does not decode the chunks now, but returns a decoder.
    /// Reading all chunks reduces seeking the file, but some chunks might be read without being used.
    pub fn all_chunks(mut self, pedantic: bool) -> Result<AllChunksReader<R>> {
        let total_chunk_count = {
            if pedantic {
                let offset_tables = MetaData::read_offset_tables(&mut self.remaining_reader, &self.meta_data.headers)?;
                validate_offset_tables(self.meta_data.headers.as_slice(), &offset_tables, self.remaining_reader.byte_position())?;
                offset_tables.iter().map(|table| table.len()).sum()
            }
            else {
                MetaData::skip_offset_tables(&mut self.remaining_reader, &self.meta_data.headers)?
            }
        };

        Ok(AllChunksReader {
            meta_data: self.meta_data,
            remaining_chunks: 0 .. total_chunk_count,
            remaining_bytes: self.remaining_reader,
            pedantic
        })
    }

    /// Prepare to read some the chunks from the file.
    /// Does not decode the chunks now, but returns a decoder.
    /// Reading only some chunks may seeking the file, potentially skipping many bytes.
    // TODO tile indices add no new information to block index??
    pub fn filter_chunks(mut self, pedantic: bool, mut filter: impl FnMut(&MetaData, TileCoordinates, BlockIndex) -> bool) -> Result<FilteredChunksReader<R>> {
        let offset_tables = MetaData::read_offset_tables(&mut self.remaining_reader, &self.meta_data.headers)?;

        // TODO regardless of pedantic, if invalid, read all chunks instead, and filter after reading each chunk?
        if pedantic {
            validate_offset_tables(
                self.meta_data.headers.as_slice(), &offset_tables,
                self.remaining_reader.byte_position()
            )?;
        }

        let mut filtered_offsets = Vec::with_capacity(
            (self.meta_data.headers.len() * 32).min(2*2048)
        );

        // TODO detect whether the filter actually would skip chunks, and aviod sorting etc when not filtering is applied

        for (header_index, header) in self.meta_data.headers.iter().enumerate() { // offset tables are stored same order as headers
            for (block_index, tile) in header.blocks_increasing_y_order().enumerate() { // in increasing_y order
                let data_indices = header.get_absolute_block_pixel_coordinates(tile.location)?;

                let block = BlockIndex {
                    layer: header_index,
                    level: tile.location.level_index,
                    pixel_position: data_indices.position.to_usize("data indices start")?,
                    pixel_size: data_indices.size,
                };

                if filter(&self.meta_data, tile.location, block) {
                    filtered_offsets.push(offset_tables[header_index][block_index]) // safe indexing from `enumerate()`
                }
            };
        }

        filtered_offsets.sort_unstable(); // enables reading continuously if possible (already sorted where line order increasing)

        if pedantic {
            // table is sorted. if any two neighbours are equal, we have duplicates. this is invalid.
            if filtered_offsets.windows(2).any(|pair| pair[0] == pair[1]) {
                return Err(Error::invalid("chunk offset table"))
            }
        }

        Ok(FilteredChunksReader {
            meta_data: self.meta_data,
            expected_filtered_chunk_count: filtered_offsets.len(),
            remaining_filtered_chunk_indices: filtered_offsets.into_iter(),
            remaining_bytes: self.remaining_reader
        })
    }
}


fn validate_offset_tables(headers: &[Header], offset_tables: &OffsetTables, chunks_start_byte: usize) -> UnitResult {
    let max_pixel_bytes: usize = headers.iter() // when compressed, chunks are smaller, but never larger than max
        .map(|header| header.max_pixel_file_bytes())
        .sum();

    // check that each offset is within the bounds
    let end_byte = chunks_start_byte + max_pixel_bytes;
    let is_invalid = offset_tables.iter().flatten().map(|&u64| u64_to_usize(u64, "chunk start"))
        .any(|maybe_chunk_start| match maybe_chunk_start {
            Ok(chunk_start) => chunk_start < chunks_start_byte || chunk_start > end_byte,
            Err(_) => true
        });

    if is_invalid { Err(Error::invalid("offset table")) }
    else { Ok(()) }
}




/// Decode the desired chunks and skip the unimportant chunks in the file.
/// The decoded chunks can be decompressed by calling
/// `decompress_parallel`, `decompress_sequential`, or `sequential_decompressor` or `parallel_decompressor`.
/// Call `on_progress` to have a callback with each block.
/// Also contains the image meta data.
#[derive(Debug)]
pub struct FilteredChunksReader<R> {
    meta_data: MetaData,
    expected_filtered_chunk_count: usize,
    remaining_filtered_chunk_indices: std::vec::IntoIter<u64>,
    remaining_bytes: PeekRead<Tracking<R>>,
}

/// Decode all chunks in the file without seeking.
/// The decoded chunks can be decompressed by calling
/// `decompress_parallel`, `decompress_sequential`, or `sequential_decompressor` or `parallel_decompressor`.
/// Call `on_progress` to have a callback with each block.
/// Also contains the image meta data.
#[derive(Debug)]
pub struct AllChunksReader<R> {
    meta_data: MetaData,
    remaining_chunks: std::ops::Range<usize>,
    remaining_bytes: PeekRead<Tracking<R>>,
    pedantic: bool,
}

/// Decode chunks in the file without seeking.
/// Calls the supplied closure for each chunk.
/// The decoded chunks can be decompressed by calling
/// `decompress_parallel`, `decompress_sequential`, or `sequential_decompressor`.
/// Also contains the image meta data.
#[derive(Debug)]
pub struct OnProgressChunksReader<R, F> {
    chunks_reader: R,
    decoded_chunks: usize,
    callback: F,
}

/// Decode chunks in the file.
/// The decoded chunks can be decompressed by calling
/// `decompress_parallel`, `decompress_sequential`, or `sequential_decompressor`.
/// Call `on_progress` to have a callback with each block.
/// Also contains the image meta data.
pub trait ChunksReader: Sized + Iterator<Item=Result<Chunk>> + ExactSizeIterator {

    /// The decoded exr meta data from the file.
    fn meta_data(&self) -> &MetaData;

    /// The decoded exr headers from the file.
    fn headers(&self) -> &[Header] { &self.meta_data().headers }

    /// The number of chunks that this reader will return in total.
    /// Can be less than the total number of chunks in the file, if some chunks are skipped.
    fn expected_chunk_count(&self) -> usize;

    /// Read the next compressed chunk from the file.
    /// Equivalent to `.next()`, as this also is an iterator.
    /// Returns `None` if all chunks have been read.
    fn read_next_chunk(&mut self) -> Option<Result<Chunk>> { self.next() }

    /// Create a new reader that calls the provided progress
    /// callback for each chunk that is read from the file.
    /// If the file can be successfully decoded,
    /// the progress will always at least once include 0.0 at the start and 1.0 at the end.
    fn on_progress<F>(self, on_progress: F) -> OnProgressChunksReader<Self, F> where F: FnMut(f64) {
        OnProgressChunksReader { chunks_reader: self, callback: on_progress, decoded_chunks: 0 }
    }

    #[cfg(feature = "rayon")]
    /// Decompress all blocks in the file, using multiple cpu cores, and call the supplied closure for each block.
    /// The order of the blocks is not deterministic.
    /// You can also use `parallel_decompressor` to obtain an iterator instead.
    /// Will fallback to sequential processing where threads are not available, or where it would not speed up the process.
    // FIXME try async + futures instead of rayon! Maybe even allows for external async decoding? (-> impl Stream<UncompressedBlock>)
    fn decompress_parallel(
        self, pedantic: bool,
        mut insert_block: impl FnMut(&MetaData, UncompressedBlock) -> UnitResult
    ) -> UnitResult
    {
        let mut decompressor = match self.parallel_decompressor(pedantic) {
            Err(old_self) => return old_self.decompress_sequential(pedantic, insert_block),
            Ok(decompressor) => decompressor,
        };

        while let Some(block) = decompressor.next() {
            insert_block(decompressor.meta_data(), block?)?;
        }

        debug_assert_eq!(decompressor.len(), 0, "compressed blocks left after decompressing all blocks");
        Ok(())
    }

    #[cfg(feature = "rayon")]
    /// Return an iterator that decompresses the chunks with multiple threads.
    /// The order of the blocks is not deterministic.
    /// Use `ParallelBlockDecompressor::new` if you want to use your own thread pool.
    /// By default, this uses as many threads as there are CPUs.
    /// Returns the `self` if there is no need for parallel decompression.
    fn parallel_decompressor(self, pedantic: bool) -> std::result::Result<ParallelBlockDecompressor<Self>, Self> {
        ParallelBlockDecompressor::new(self, pedantic)
    }

    /// Return an iterator that decompresses the chunks in this thread.
    /// You can alternatively use `sequential_decompressor` if you prefer an external iterator.
    fn decompress_sequential(
        self, pedantic: bool,
        mut insert_block: impl FnMut(&MetaData, UncompressedBlock) -> UnitResult
    ) -> UnitResult
    {
        let mut decompressor = self.sequential_decompressor(pedantic);
        while let Some(block) = decompressor.next() {
            insert_block(decompressor.meta_data(), block?)?;
        }

        debug_assert_eq!(decompressor.len(), 0, "compressed blocks left after decompressing all blocks");
        Ok(())
    }

    /// Prepare reading the chunks sequentially, only a single thread, but with less memory overhead.
    fn sequential_decompressor(self, pedantic: bool) -> SequentialBlockDecompressor<Self> {
        SequentialBlockDecompressor { remaining_chunks_reader: self, pedantic }
    }
}

impl<R, F> ChunksReader for OnProgressChunksReader<R, F> where R: ChunksReader, F: FnMut(f64) {
    fn meta_data(&self) -> &MetaData { self.chunks_reader.meta_data() }
    fn expected_chunk_count(&self) -> usize { self.chunks_reader.expected_chunk_count() }
}

impl<R, F> ExactSizeIterator for OnProgressChunksReader<R, F> where R: ChunksReader, F: FnMut(f64) {}
impl<R, F> Iterator for OnProgressChunksReader<R, F> where R: ChunksReader, F: FnMut(f64) {
    type Item = Result<Chunk>;

    fn next(&mut self) -> Option<Self::Item> {
        self.chunks_reader.next().map(|item|{
            {
                let total_chunks = self.expected_chunk_count() as f64;
                let callback = &mut self.callback;
                callback(self.decoded_chunks as f64 / total_chunks);
            }

            self.decoded_chunks += 1;
            item
        })
            .or_else(||{
                debug_assert_eq!(
                    self.decoded_chunks, self.expected_chunk_count(),
                    "chunks reader finished but not all chunks are decompressed"
                );

                let callback = &mut self.callback;
                callback(1.0);
                None
            })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.chunks_reader.size_hint()
    }
}

impl<R: Read + Seek> ChunksReader for AllChunksReader<R> {
    fn meta_data(&self) -> &MetaData { &self.meta_data }
    fn expected_chunk_count(&self) -> usize { self.remaining_chunks.end }
}

impl<R: Read + Seek> ExactSizeIterator for AllChunksReader<R> {}
impl<R: Read + Seek> Iterator for AllChunksReader<R> {
    type Item = Result<Chunk>;

    fn next(&mut self) -> Option<Self::Item> {
        // read as many chunks as the file should contain (inferred from meta data)
        let next_chunk = self.remaining_chunks.next()
            .map(|_| Chunk::read(&mut self.remaining_bytes, &self.meta_data));

        // if no chunks are left, but some bytes remain, return error
        if self.pedantic && next_chunk.is_none() && self.remaining_bytes.peek_u8().is_ok() {
            return Some(Err(Error::invalid("end of file expected")));
        }

        next_chunk
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining_chunks.len(), Some(self.remaining_chunks.len()))
    }
}

impl<R: Read + Seek> ChunksReader for FilteredChunksReader<R> {
    fn meta_data(&self) -> &MetaData { &self.meta_data }
    fn expected_chunk_count(&self) -> usize { self.expected_filtered_chunk_count }
}

impl<R: Read + Seek> ExactSizeIterator for FilteredChunksReader<R> {}
impl<R: Read + Seek> Iterator for FilteredChunksReader<R> {
    type Item = Result<Chunk>;

    fn next(&mut self) -> Option<Self::Item> {
        // read as many chunks as we have desired chunk offsets
        self.remaining_filtered_chunk_indices.next().map(|next_chunk_location|{
            self.remaining_bytes.skip_to( // no-op for seek at current position, uses skip_bytes for small amounts
                  usize::try_from(next_chunk_location)?
            )?;

            let meta_data = &self.meta_data;
            Chunk::read(&mut self.remaining_bytes, meta_data)
        })

        // TODO remember last chunk index and then seek to index+size and check whether bytes are left?
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining_filtered_chunk_indices.len(), Some(self.remaining_filtered_chunk_indices.len()))
    }
}

/// Read all chunks from the file, decompressing each chunk immediately.
/// Implements iterator.
#[derive(Debug)]
pub struct SequentialBlockDecompressor<R: ChunksReader> {
    remaining_chunks_reader: R,
    pedantic: bool,
}

impl<R: ChunksReader> SequentialBlockDecompressor<R> {

    /// The extracted meta data from the image file.
    pub fn meta_data(&self) -> &MetaData { self.remaining_chunks_reader.meta_data() }

    /// Read and then decompress a single block of pixels from the byte source.
    pub fn decompress_next_block(&mut self) -> Option<Result<UncompressedBlock>> {
        self.remaining_chunks_reader.read_next_chunk().map(|compressed_chunk|{
            UncompressedBlock::decompress_chunk(compressed_chunk?, &self.remaining_chunks_reader.meta_data(), self.pedantic)
        })
    }
}

#[cfg(feature = "rayon")]
/// Decompress the chunks in a file in parallel.
/// The first call to `next` will fill the thread pool with jobs,
/// starting to decompress the next few blocks.
/// These jobs will finish, even if you stop reading more blocks.
/// Implements iterator.
#[derive(Debug)]
pub struct ParallelBlockDecompressor<R: ChunksReader> {
    remaining_chunks: R,
    sender: std::sync::mpsc::Sender<Result<UncompressedBlock>>,
    receiver: std::sync::mpsc::Receiver<Result<UncompressedBlock>>,
    currently_decompressing_count: usize,
    max_threads: usize,

    shared_meta_data_ref: std::sync::Arc<MetaData>,
    pedantic: bool,

    pool: rayon_core::ThreadPool,
}

#[cfg(feature = "rayon")]
impl<R: ChunksReader> ParallelBlockDecompressor<R> {

    /// Create a new decompressor. Does not immediately spawn any tasks.
    /// Decompression starts after the first call to `next`.
    /// Returns the chunks if parallel decompression should not be used.
    /// Use `new_with_thread_pool` to customize the threadpool.
    pub fn new(chunks: R, pedantic: bool) -> std::result::Result<Self, R> {
        Self::new_with_thread_pool(chunks, pedantic, ||{
            rayon_core::ThreadPoolBuilder::new()
                .thread_name(|index| format!("OpenEXR Block Decompressor Thread #{}", index))
                .build()
        })
    }

    /// Create a new decompressor. Does not immediately spawn any tasks.
    /// Decompression starts after the first call to `next`.
    /// Returns the chunks if parallel decompression should not be used.
    pub fn new_with_thread_pool<CreatePool>(chunks: R, pedantic: bool, try_create_thread_pool: CreatePool)
        -> std::result::Result<Self, R>
        where CreatePool: FnOnce() -> std::result::Result<rayon_core::ThreadPool, rayon_core::ThreadPoolBuildError>
    {
        use crate::compression::Compression;

        let is_entirely_uncompressed = chunks.meta_data().headers.iter()
            .all(|head|head.compression == Compression::Uncompressed);

        // if no compression is used in the file, don't use a threadpool
        if is_entirely_uncompressed {
            return Err(chunks);
        }

        // in case thread pool creation fails (for example on WASM currently),
        // we revert to sequential decompression
        let pool = match try_create_thread_pool() {
            Ok(pool) => pool,

            // TODO print warning?
            Err(_) => return Err(chunks),
        };

        let max_threads = pool.current_num_threads().max(1).min(chunks.len()) + 2; // ca one block for each thread at all times

        let (send, recv) = std::sync::mpsc::channel(); // TODO bounded channel simplifies logic?

        Ok(Self {
            shared_meta_data_ref: std::sync::Arc::new(chunks.meta_data().clone()),
            currently_decompressing_count: 0,
            remaining_chunks: chunks,
            sender: send,
            receiver: recv,
            pedantic,
            max_threads,

            pool,
        })
    }

    /// Fill the pool with decompression jobs. Returns the first job that finishes.
    pub fn decompress_next_block(&mut self) -> Option<Result<UncompressedBlock>> {

        while self.currently_decompressing_count < self.max_threads {
            let block = self.remaining_chunks.next();
            if let Some(block) = block {
                let block = match block {
                    Ok(block) => block,
                    Err(error) => return Some(Err(error))
                };

                let sender = self.sender.clone();
                let meta = self.shared_meta_data_ref.clone();
                let pedantic = self.pedantic;

                self.currently_decompressing_count += 1;

                self.pool.spawn(move || {
                    let decompressed_or_err = UncompressedBlock::decompress_chunk(
                        block, &meta, pedantic
                    );

                    // by now, decompressing could have failed in another thread.
                    // the error is then already handled, so we simply
                    // don't send the decompressed block and do nothing
                    let _ = sender.send(decompressed_or_err);
                });
            }
            else {
                // there are no chunks left to decompress
                break;
            }
        }

        if self.currently_decompressing_count > 0 {
            let next = self.receiver.recv()
                .expect("all decompressing senders hung up but more messages were expected");

            self.currently_decompressing_count -= 1;
            Some(next)
        }
        else {
            debug_assert!(self.receiver.try_recv().is_err(), "uncompressed chunks left in channel after decompressing all chunks"); // TODO not reliable
            debug_assert_eq!(self.len(), 0, "compressed chunks left after decompressing all chunks");
            None
        }
    }

    /// The extracted meta data of the image file.
    pub fn meta_data(&self) -> &MetaData { self.remaining_chunks.meta_data() }
}

impl<R: ChunksReader> ExactSizeIterator for SequentialBlockDecompressor<R> {}
impl<R: ChunksReader> Iterator for SequentialBlockDecompressor<R> {
    type Item = Result<UncompressedBlock>;
    fn next(&mut self) -> Option<Self::Item> { self.decompress_next_block() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.remaining_chunks_reader.size_hint() }
}

#[cfg(feature = "rayon")]
impl<R: ChunksReader> ExactSizeIterator for ParallelBlockDecompressor<R> {}
#[cfg(feature = "rayon")]
impl<R: ChunksReader> Iterator for ParallelBlockDecompressor<R> {
    type Item = Result<UncompressedBlock>;
    fn next(&mut self) -> Option<Self::Item> { self.decompress_next_block() }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining_chunks.len() + self.currently_decompressing_count;
        (remaining, Some(remaining))
    }
}





