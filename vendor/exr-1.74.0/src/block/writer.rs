//! Composable structures to handle writing an image.


use std::fmt::Debug;
use std::io::Seek;
use std::iter::Peekable;
use std::ops::Not;

use smallvec::alloc::collections::BTreeMap;
use crate::block::UncompressedBlock;
use crate::block::chunk::Chunk;
use crate::error::{Error, Result, UnitResult, usize_to_u64};
use crate::io::{Data, Tracking, Write};
use crate::meta::{Headers, MetaData, OffsetTables};
use crate::meta::attribute::LineOrder;

/// Write an exr file by writing one chunk after another in a closure.
/// In the closure, you are provided a chunk writer, which should be used to write all the chunks.
/// Assumes the your write destination is buffered.
pub fn write_chunks_with<W: Write + Seek>(
    buffered_write: W, headers: Headers, pedantic: bool,
    write_chunks: impl FnOnce(MetaData, &mut ChunkWriter<W>) -> UnitResult
) -> UnitResult {
    // this closure approach ensures that after writing all chunks, the file is always completed and checked and flushed
    let (meta, mut writer) = ChunkWriter::new_for_buffered(buffered_write, headers, pedantic)?;
    write_chunks(meta, &mut writer)?;
    writer.complete_meta_data()
}

/// Can consume compressed pixel chunks, writing them a file.
/// Use `sequential_blocks_compressor` or `parallel_blocks_compressor` to compress your data,
/// or use `compress_all_blocks_sequential` or `compress_all_blocks_parallel`.
/// Use `on_progress` to obtain a new writer
/// that triggers a callback for each block.
// #[must_use]
#[derive(Debug)]
#[must_use]
pub struct ChunkWriter<W> {
    header_count: usize,
    byte_writer: Tracking<W>,
    chunk_indices_byte_location: std::ops::Range<usize>,
    chunk_indices_increasing_y: OffsetTables,
    chunk_count: usize, // TODO compose?
}

/// A new writer that triggers a callback
/// for each block written to the inner writer.
#[derive(Debug)]
#[must_use]
pub struct OnProgressChunkWriter<'w, W, F> {
    chunk_writer: &'w mut W,
    written_chunks: usize,
    on_progress: F,
}

/// Write chunks to a byte destination.
/// Then write each chunk with `writer.write_chunk(chunk)`.
pub trait ChunksWriter: Sized {

    /// The total number of chunks that the complete file will contain.
    fn total_chunks_count(&self) -> usize;

    /// Any more calls will result in an error and have no effect.
    /// If writing results in an error, the file and the writer
    /// may remain in an invalid state and should not be used further.
    /// Errors when the chunk at this index was already written.
    fn write_chunk(&mut self, index_in_header_increasing_y: usize, chunk: Chunk) -> UnitResult;

    /// Obtain a new writer that calls the specified closure for each block that is written to this writer.
    fn on_progress<F>(&mut self, on_progress: F) -> OnProgressChunkWriter<'_, Self, F> where F: FnMut(f64) {
        OnProgressChunkWriter { chunk_writer: self, written_chunks: 0, on_progress }
    }

    /// Obtain a new writer that can compress blocks to chunks, which are then passed to this writer.
    fn sequential_blocks_compressor<'w>(&'w mut self, meta: &'w MetaData) -> SequentialBlocksCompressor<'w, Self> {
        SequentialBlocksCompressor::new(meta, self)
    }

    #[cfg(feature = "rayon")]
    /// Obtain a new writer that can compress blocks to chunks on multiple threads, which are then passed to this writer.
    /// Returns none if the sequential compressor should be used instead (thread pool creation failure or too large performance overhead).
    fn parallel_blocks_compressor<'w>(&'w mut self, meta: &'w MetaData) -> Option<ParallelBlocksCompressor<'w, Self>> {
        ParallelBlocksCompressor::new(meta, self)
    }

    /// Compresses all blocks to the file.
    /// The index of the block must be in increasing line order within the header.
    /// Obtain iterator with `MetaData::collect_ordered_blocks(...)` or similar methods.
    fn compress_all_blocks_sequential(mut self, meta: &MetaData, blocks: impl Iterator<Item=(usize, UncompressedBlock)>) -> UnitResult {
        let mut writer = self.sequential_blocks_compressor(meta);

        // TODO check block order if line order is not unspecified!
        for (index_in_header_increasing_y, block) in blocks {
            writer.compress_block(index_in_header_increasing_y, block)?;
        }

        // TODO debug_assert_eq!(self.is_complete());
        Ok(())
    }

    #[cfg(feature = "rayon")]
    /// Compresses all blocks to the file.
    /// The index of the block must be in increasing line order within the header.
    /// Obtain iterator with `MetaData::collect_ordered_blocks(...)` or similar methods.
    /// Will fallback to sequential processing where threads are not available, or where it would not speed up the process.
    fn compress_all_blocks_parallel(mut self, meta: &MetaData, blocks: impl Iterator<Item=(usize, UncompressedBlock)>) -> UnitResult {
        let mut parallel_writer = match self.parallel_blocks_compressor(meta) {
            None => return self.compress_all_blocks_sequential(meta, blocks),
            Some(writer) => writer,
        };

        // TODO check block order if line order is not unspecified!
        for (index_in_header_increasing_y, block) in blocks {
            parallel_writer.add_block_to_compression_queue(index_in_header_increasing_y, block)?;
        }

        // TODO debug_assert_eq!(self.is_complete());
        Ok(())
    }
}


impl<W> ChunksWriter for ChunkWriter<W> where W: Write + Seek {

    /// The total number of chunks that the complete file will contain.
    fn total_chunks_count(&self) -> usize { self.chunk_count }

    /// Any more calls will result in an error and have no effect.
    /// If writing results in an error, the file and the writer
    /// may remain in an invalid state and should not be used further.
    /// Errors when the chunk at this index was already written.
    fn write_chunk(&mut self, index_in_header_increasing_y: usize, chunk: Chunk) -> UnitResult {
        let header_chunk_indices = &mut self.chunk_indices_increasing_y[chunk.layer_index];

        if index_in_header_increasing_y >= header_chunk_indices.len() {
            return Err(Error::invalid("too large chunk index"));
        }

        let chunk_index_slot = &mut header_chunk_indices[index_in_header_increasing_y];
        if *chunk_index_slot != 0 {
            return Err(Error::invalid(format!("chunk at index {} is already written", index_in_header_increasing_y)));
        }

        *chunk_index_slot = usize_to_u64(self.byte_writer.byte_position(), "seek position")?;
        chunk.write(&mut self.byte_writer, self.header_count)?;
        Ok(())
    }
}

impl<W> ChunkWriter<W> where W: Write + Seek {
    // -- the following functions are private, because they must be called in a strict order --

    /// Writes the meta data and zeroed offset tables as a placeholder.
    fn new_for_buffered(buffered_byte_writer: W, headers: Headers, pedantic: bool) -> Result<(MetaData, Self)> {
        let mut write = Tracking::new(buffered_byte_writer);
        let requirements = MetaData::write_validating_to_buffered(&mut write, headers.as_slice(), pedantic)?;

        // TODO: use increasing line order where possible, but this requires us to know whether we want to be parallel right now
        /*// if non-parallel compression, we always use increasing order anyways
        if !parallel || !has_compression {
            for header in &mut headers {
                if header.line_order == LineOrder::Unspecified {
                    header.line_order = LineOrder::Increasing;
                }
            }
        }*/

        let offset_table_size: usize = headers.iter().map(|header| header.chunk_count).sum();

        let offset_table_start_byte = write.byte_position();
        let offset_table_end_byte = write.byte_position() + offset_table_size * u64::BYTE_SIZE;

        // skip offset tables, filling with 0, will be updated after the last chunk has been written
        write.seek_write_to(offset_table_end_byte)?;

        let header_count = headers.len();
        let chunk_indices_increasing_y = headers.iter()
            .map(|header| vec![0_u64; header.chunk_count]).collect();

        let meta_data = MetaData { requirements, headers };

        Ok((meta_data, ChunkWriter {
            header_count,
            byte_writer: write,
            chunk_count: offset_table_size,
            chunk_indices_byte_location: offset_table_start_byte .. offset_table_end_byte,
            chunk_indices_increasing_y,
        }))
    }

    /// Seek back to the meta data, write offset tables, and flush the byte writer.
    /// Leaves the writer seeked to the middle of the file, therefore we drop it.
    fn complete_meta_data(mut self) -> UnitResult {
        if self.chunk_indices_increasing_y.iter().flatten().any(|&index| index == 0) {
            return Err(Error::invalid("some chunks are not written yet"))
        }

        // write all offset tables
        debug_assert_ne!(self.byte_writer.byte_position(), self.chunk_indices_byte_location.end, "offset table has already been updated");
        self.byte_writer.seek_write_to(self.chunk_indices_byte_location.start)?;

        for table in self.chunk_indices_increasing_y {
            u64::write_slice_le(&mut self.byte_writer, table.as_slice())?;
        }

        self.byte_writer.flush()?; // make sure we catch all (possibly delayed) io errors before returning
        Ok(())
    }

}


impl<'w, W, F> ChunksWriter for OnProgressChunkWriter<'w, W, F> where W: 'w + ChunksWriter, F: FnMut(f64) {
    fn total_chunks_count(&self) -> usize {
        self.chunk_writer.total_chunks_count()
    }

    fn write_chunk(&mut self, index_in_header_increasing_y: usize, chunk: Chunk) -> UnitResult {
        let total_chunks = self.total_chunks_count();
        let on_progress = &mut self.on_progress;

        // guarantee on_progress being called with 0 once
        if self.written_chunks == 0 { on_progress(0.0); }

        self.chunk_writer.write_chunk(index_in_header_increasing_y, chunk)?;

        self.written_chunks += 1;

        on_progress({
            // guarantee finishing with progress 1.0 for last block at least once, float division might slightly differ from 1.0
            if self.written_chunks == total_chunks { 1.0 }
            else { self.written_chunks as f64 / total_chunks as f64 }
        });

        Ok(())
    }
}


/// Write blocks that appear in any order and reorder them before writing.
#[derive(Debug)]
#[must_use]
pub struct SortedBlocksWriter<'w, W> {
    chunk_writer: &'w mut W,
    pending_chunks: BTreeMap<usize, (usize, Chunk)>,
    unwritten_chunk_indices: Peekable<std::ops::Range<usize>>,
    requires_sorting: bool, // using this instead of Option, because of borrowing
}


impl<'w, W> SortedBlocksWriter<'w, W> where W: ChunksWriter {

    /// New sorting writer. Returns `None` if sorting is not required.
    pub fn new(meta_data: &MetaData, chunk_writer: &'w mut W) -> SortedBlocksWriter<'w, W> {
        let requires_sorting = meta_data.headers.iter()
            .any(|header| header.line_order != LineOrder::Unspecified);

        let total_chunk_count = chunk_writer.total_chunks_count();

        SortedBlocksWriter {
            pending_chunks: BTreeMap::new(),
            unwritten_chunk_indices: (0 .. total_chunk_count).peekable(),
            requires_sorting,
            chunk_writer
        }
    }

    /// Write the chunk or stash it. In the closure, write all chunks that can be written now.
    pub fn write_or_stash_chunk(&mut self, chunk_index_in_file: usize, chunk_y_index: usize, chunk: Chunk) -> UnitResult {
        if self.requires_sorting.not() {
            return self.chunk_writer.write_chunk(chunk_y_index, chunk);
        }

        // write this chunk now if possible
        if self.unwritten_chunk_indices.peek() == Some(&chunk_index_in_file){
            self.chunk_writer.write_chunk(chunk_y_index, chunk)?;
            self.unwritten_chunk_indices.next().expect("peeked chunk index is missing");

            // write all pending blocks that are immediate successors of this block
            while let Some((next_chunk_y_index, next_chunk)) = self
                .unwritten_chunk_indices.peek().cloned()
                .and_then(|id| self.pending_chunks.remove(&id))
            {
                self.chunk_writer.write_chunk(next_chunk_y_index, next_chunk)?;
                self.unwritten_chunk_indices.next().expect("peeked chunk index is missing");
            }
        }

        else {
            // the argument block is not to be written now,
            // and all the pending blocks are not next up either,
            // so just stash this block
            self.pending_chunks.insert(chunk_index_in_file, (chunk_y_index, chunk));
        }

        Ok(())
    }

    /// Where the chunks will be written to.
    pub fn inner_chunks_writer(&self) -> &W {
        &self.chunk_writer
    }
}



/// Compress blocks to a chunk writer in this thread.
#[derive(Debug)]
#[must_use]
pub struct SequentialBlocksCompressor<'w, W> {
    meta: &'w MetaData,
    chunks_writer: &'w mut W,
}

impl<'w, W> SequentialBlocksCompressor<'w, W> where W: 'w + ChunksWriter {

    /// New blocks writer.
    pub fn new(meta: &'w MetaData, chunks_writer: &'w mut W) -> Self { Self { meta, chunks_writer, } }

    /// This is where the compressed blocks are written to.
    pub fn inner_chunks_writer(&'w self) -> &'w W { self.chunks_writer }

    /// Compress a single block immediately. The index of the block must be in increasing line order.
    pub fn compress_block(&mut self, index_in_header_increasing_y: usize, block: UncompressedBlock) -> UnitResult {
        self.chunks_writer.write_chunk(
            index_in_header_increasing_y,
            block.compress_to_chunk(&self.meta.headers)?
        )
    }
}



/// Compress blocks to a chunk writer with multiple threads.
#[cfg(feature = "rayon")]
#[derive(Debug)]
#[must_use]
pub struct ParallelBlocksCompressor<'w, W> {
    meta: &'w MetaData,
    sorted_writer: SortedBlocksWriter<'w, W>,

    sender: std::sync::mpsc::Sender<Result<(usize, usize, Chunk)>>,
    receiver: std::sync::mpsc::Receiver<Result<(usize, usize, Chunk)>>,
    pool: rayon_core::ThreadPool,

    currently_compressing_count: usize,
    written_chunk_count: usize, // used to check for last chunk
    max_threads: usize,
    next_incoming_chunk_index: usize, // used to remember original chunk order
}

#[cfg(feature = "rayon")]
impl<'w, W> ParallelBlocksCompressor<'w, W> where W: 'w + ChunksWriter {

    /// New blocks writer. Returns none if sequential compression should be used.
    /// Use `new_with_thread_pool` to customize the threadpool.
    pub fn new(meta: &'w MetaData, chunks_writer: &'w mut W) -> Option<Self> {
        Self::new_with_thread_pool(meta, chunks_writer, ||{
            rayon_core::ThreadPoolBuilder::new()
                .thread_name(|index| format!("OpenEXR Block Compressor Thread #{}", index))
                .build()
        })
    }

    /// New blocks writer. Returns none if sequential compression should be used.
    pub fn new_with_thread_pool<CreatePool>(
        meta: &'w MetaData, chunks_writer: &'w mut W, try_create_thread_pool: CreatePool)
        -> Option<Self>
        where CreatePool: FnOnce() -> std::result::Result<rayon_core::ThreadPool, rayon_core::ThreadPoolBuildError>
    {
        use crate::compression::Compression;

        let is_entirely_uncompressed = meta.headers.iter()
            .all(|head|head.compression == Compression::Uncompressed);

        if is_entirely_uncompressed {
            return None;
        }

        // in case thread pool creation fails (for example on WASM currently),
        // we revert to sequential compression
        let pool = match try_create_thread_pool() {
            Ok(pool) => pool,

            // TODO print warning?
            Err(_) => return None,
        };

        let max_threads = pool.current_num_threads().max(1).min(chunks_writer.total_chunks_count()) + 2; // ca one block for each thread at all times
        let (send, recv) = std::sync::mpsc::channel(); // TODO bounded channel simplifies logic?

        Some(Self {
            sorted_writer: SortedBlocksWriter::new(meta, chunks_writer),
            next_incoming_chunk_index: 0,
            currently_compressing_count: 0,
            written_chunk_count: 0,
            sender: send,
            receiver: recv,
            max_threads,
            pool,
            meta,
        })
    }

    /// This is where the compressed blocks are written to.
    pub fn inner_chunks_writer(&'w self) -> &'w W { self.sorted_writer.inner_chunks_writer() }

    // private, as may underflow counter in release mode
    fn write_next_queued_chunk(&mut self) -> UnitResult {
        debug_assert!(self.currently_compressing_count > 0, "cannot wait for chunks as there are none left");

        let some_compressed_chunk = self.receiver.recv()
            .expect("cannot receive compressed block");

        self.currently_compressing_count -= 1;
        let (chunk_file_index, chunk_y_index, chunk) = some_compressed_chunk?;
        self.sorted_writer.write_or_stash_chunk(chunk_file_index, chunk_y_index, chunk)?;

        self.written_chunk_count += 1;
        Ok(())
    }

    /// Wait until all currently compressing chunks in the compressor have been written.
    pub fn write_all_queued_chunks(&mut self) -> UnitResult {
        while self.currently_compressing_count > 0 {
            self.write_next_queued_chunk()?;
        }

        debug_assert_eq!(self.currently_compressing_count, 0, "counter does not match block count");
        Ok(())
    }

    /// Add a single block to the compressor queue. The index of the block must be in increasing line order.
    /// When calling this function for the last block, this method waits until all the blocks have been written.
    /// This only works when you write as many blocks as the image expects, otherwise you can use `wait_for_all_remaining_chunks`.
    /// Waits for a block from the queue to be written, if the queue already has enough items.
    pub fn add_block_to_compression_queue(&mut self, index_in_header_increasing_y: usize, block: UncompressedBlock) -> UnitResult {

        // if pipe is full, block to wait for a slot to free up
        if self.currently_compressing_count >= self.max_threads {
            self.write_next_queued_chunk()?;
        }

        // add the argument chunk to the compression queueue
        let index_in_file = self.next_incoming_chunk_index;
        let sender = self.sender.clone();
        let meta = self.meta.clone();

        self.pool.spawn(move ||{
            let compressed_or_err = block.compress_to_chunk(&meta.headers);

            // by now, decompressing could have failed in another thread.
            // the error is then already handled, so we simply
            // don't send the decompressed block and do nothing
            let _ = sender.send(compressed_or_err.map(move |compressed| (index_in_file, index_in_header_increasing_y, compressed)));
        });

        self.currently_compressing_count += 1;
        self.next_incoming_chunk_index += 1;

        // if this is the last chunk, wait for all chunks to complete before returning
        if self.written_chunk_count + self.currently_compressing_count == self.inner_chunks_writer().total_chunks_count() {
            self.write_all_queued_chunks()?;
            debug_assert_eq!(
                self.written_chunk_count, self.inner_chunks_writer().total_chunks_count(),
                "written chunk count mismatch"
            );
        }


        Ok(())
    }
}



