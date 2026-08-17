//! RFC 1590 compression implementation.

#![allow(clippy::needless_range_loop, clippy::new_without_default)]

use super::{Adler32, Error, Format};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::convert::TryInto;

#[cfg(feature = "std")]
use std::io::{self, Write};

/// The level of compression-- a compromise between speed and size.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CompressionLevel {
    /// No compression. Outputs raw blocks.
    None,
    /// Fast compression.
    BestSpeed,
    /// Compromise between speed and size.
    Default,
    /// Slower compression for smaller size.
    BestSize,
    /// A specific compression level from 1-10.
    Specific(u8),
}

impl CompressionLevel {
    fn to_raw(self) -> usize {
        use CompressionLevel::*;
        match self {
            None => 0,
            BestSpeed => 1,
            Default => 6,
            BestSize => 9,
            Specific(level) => 10.min(level as usize),
        }
    }
}

/// Selects between various specialized compressor modes.
#[derive(Copy, Clone)]
pub enum CompressionStrategy {
    /// Let it do its thing.
    Default,
    /// Run-length encoding only.
    RLE,
    /// Ignore matches fewer than 5 bytes.
    Filtered,
    /// Static blocks only.
    Static,
    /// Huffman encoding only.
    Huffman,
}

/// Stateful context for compression.
///
/// See the crate level [compression](index.html#compression) section
/// for detailed usage.
pub struct Encoder(DeflateContext);

impl Encoder {
    /// Creates a new deflate encoder. Note that creating an encoder with this
    /// method allocates a large (200-300k) chunk of data on the stack and is
    /// likely to cause an overflow if not carefully managed. See the [`boxed()`]
    /// constructor for a safer method that allocates on the heap.
    ///
    /// [`boxed()`]: Self::boxed
    pub fn new() -> Self {
        let flags = make_flags(
            false,
            CompressionLevel::Default,
            CompressionStrategy::Default,
        );
        Self(DeflateContext {
            flags,
            ready: true,
            zlib: false,
            level: CompressionLevel::Default,
            strategy: CompressionStrategy::Default,
            greedy_parsing: flags & GREEDY_PARSING != 0,
            block_index: 0,
            saved_match_dist: 0,
            saved_match_len: 0,
            saved_lit: 0,
            saved_bit_buffer: 0,
            saved_bits_in: 0,
            adler32: Adler32::new(),
            lt: LiteralLengthTree::new(),
            dt: DistanceTree::new(),
            pt: PrecodeTree::new(),
            cb: CodeBuffer::new(),
            dict: Dictionary::new(flags),
        })
    }

    /// Creates a new deflate encoder on the heap.
    pub fn boxed() -> Box<Self> {
        let flags = make_flags(
            false,
            CompressionLevel::Default,
            CompressionStrategy::Default,
        );
        Box::new(Self(DeflateContext {
            flags,
            ready: true,
            zlib: false,
            level: CompressionLevel::Default,
            strategy: CompressionStrategy::Default,
            greedy_parsing: flags & GREEDY_PARSING != 0,
            block_index: 0,
            saved_match_dist: 0,
            saved_match_len: 0,
            saved_lit: 0,
            saved_bit_buffer: 0,
            saved_bits_in: 0,
            adler32: Adler32::new(),
            lt: LiteralLengthTree::new(),
            dt: DistanceTree::new(),
            pt: PrecodeTree::new(),
            cb: CodeBuffer::new(),
            dict: Dictionary::new(flags),
        }))
    }

    /// Sets the format of the output bitstream for the next usage of the
    /// encoder.
    pub fn set_format(&mut self, format: Format) {
        self.0.reset(format == Format::Zlib);
    }

    /// Sets the compression level for the next usage of the encoder.
    pub fn set_level(&mut self, level: CompressionLevel) {
        let flags = make_flags(self.0.zlib, level, self.0.strategy);
        self.0.flags = flags;
        self.0.level = level;
        self.0.greedy_parsing = flags & GREEDY_PARSING != 0;
        self.0.dict.max_probes = Dictionary::probes_from_flags(flags);
    }

    /// Sets the compression strategy for the next usage of the encoder.
    pub fn set_strategy(&mut self, strategy: CompressionStrategy) {
        let flags = make_flags(self.0.zlib, self.0.level, strategy);
        self.0.flags = flags;
        self.0.strategy = strategy;
        self.0.greedy_parsing = flags & GREEDY_PARSING != 0;
        self.0.dict.max_probes = Dictionary::probes_from_flags(flags);
    }

    /// Creates an encoder stream that will write into the specified writer.
    #[cfg(feature = "std")]
    pub fn stream<'a, W: Write>(
        &'a mut self,
        writer: &'a mut W,
    ) -> EncoderStream<'a, impl Sink + 'a> {
        self.0.reset(self.0.zlib);
        EncoderStream {
            ctx: &mut self.0,
            sink: WriterSink::new(writer),
            finished: false,
        }
    }

    /// Creates an encoder stream that will write into the specified vector.
    /// The resulting stream will not clear the vector but will instead append
    /// the compressed data.   
    pub fn stream_into_vec<'a>(
        &'a mut self,
        vec: &'a mut Vec<u8>,
    ) -> EncoderStream<'a, impl Sink + 'a> {
        self.0.reset(self.0.zlib);
        EncoderStream {
            ctx: &mut self.0,
            sink: VecSink::new(vec),
            finished: false,
        }
    }

    /// Creates an encoder stream that will write into the specified buffer.
    /// The stream will generate an overflow error if the buffer is not large
    /// enough to contain the compressed data.
    pub fn stream_into_buf<'a>(
        &'a mut self,
        buf: &'a mut [u8],
    ) -> EncoderStream<'a, impl Sink + 'a> {
        self.0.reset(self.0.zlib);
        EncoderStream {
            ctx: &mut self.0,
            sink: BufSink::new(buf),
            finished: false,
        }
    }
}

/// Compression stream combining an encoder context with an output.
///
/// See the crate level [compression](index.html#compression) section
/// for detailed usage.
pub struct EncoderStream<'a, S: Sink> {
    ctx: &'a mut DeflateContext,
    sink: S,
    finished: bool,
}

impl<S: Sink> EncoderStream<'_, S> {
    /// Writes the specified buffer to the stream, producing compressed data
    /// in the output.
    pub fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        if self.finished {
            return Err(Error::Finished);
        }
        self.ctx.deflate(buf, &mut self.sink, false)
    }

    /// Returns the number of compressed bytes that have been written to the
    /// output.
    pub fn compressed_size(&self) -> u64 {
        self.sink.written()
    }

    /// Consumes the stream, flushing any input that may be buffered and any
    /// remaining output. Returns the total number of compressed bytes written
    /// to the output.
    pub fn finish(mut self) -> Result<u64, Error> {
        if self.finished {
            return Err(Error::Finished);
        }
        self.finished = true;
        self.ctx.deflate(&[], &mut self.sink, true)?;
        self.ctx.flush_block(&mut self.sink, true)?;
        Ok(self.sink.written())
    }
}

impl<S: Sink> Drop for EncoderStream<'_, S> {
    fn drop(&mut self) {
        if !self.finished {
            self.finished = true;
            let _ = self.ctx.deflate(&[], &mut self.sink, true);
            let _ = self.ctx.flush_block(&mut self.sink, true);
        }
    }
}

#[cfg(feature = "std")]
impl<S: Sink> Write for EncoderStream<'_, S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.ctx.deflate(buf, &mut self.sink, false) {
            Ok(_) => Ok(buf.len()),
            Err(err) => match err {
                Error::Io(err) => Err(err),
                Error::Underflow | Error::Overflow => {
                    Err(io::Error::from(io::ErrorKind::InvalidInput))
                }
                _ => Err(io::Error::from(io::ErrorKind::InvalidData)),
            },
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Compresses a buffer into a vector with the specified format and
/// compression level.
pub fn compress(buf: &[u8], format: Format, level: CompressionLevel) -> Result<Vec<u8>, Error> {
    let mut encoder = Encoder::boxed();
    encoder.set_format(format);
    encoder.set_level(level);
    let mut vec = Vec::new();
    let mut stream = encoder.stream_into_vec(&mut vec);
    stream.write(buf)?;
    stream.finish()?;
    Ok(vec)
}

struct DeflateContext {
    flags: u32,
    ready: bool,
    zlib: bool,
    level: CompressionLevel,
    strategy: CompressionStrategy,
    greedy_parsing: bool,
    block_index: u32,
    saved_match_dist: usize,
    saved_match_len: usize,
    saved_lit: u8,
    saved_bit_buffer: u32,
    saved_bits_in: u32,
    adler32: Adler32,
    lt: LiteralLengthTree,
    dt: DistanceTree,
    pt: PrecodeTree,
    cb: CodeBuffer,
    dict: Dictionary,
}

impl DeflateContext {
    fn deflate<S: Sink>(&mut self, buf: &[u8], sink: &mut S, is_last: bool) -> Result<(), Error> {
        if !is_last && buf.is_empty() {
            return Ok(());
        }
        self.deflate_inner(buf, sink, is_last)?;
        if self.flags & WRITE_ZLIB_HEADER != 0 {
            self.adler32.update(buf);
        }
        Ok(())
    }

    fn deflate_inner<S: Sink>(
        &mut self,
        data: &[u8],
        sink: &mut S,
        is_last: bool,
    ) -> Result<(), Error> {
        const DICT_MASK: usize = DICTIONARY_SIZE - 1;
        self.ready = false;
        let mut src_pos = 0;
        let mut lookahead_size = self.dict.lookahead_size;
        let mut lookahead_pos = self.dict.lookahead_pos;
        let mut saved_lit = self.saved_lit;
        let mut saved_match_dist = self.saved_match_dist;
        let mut saved_match_len = self.saved_match_len;
        while src_pos < data.len() || (is_last && lookahead_size != 0) {
            let src_buf_left = data.len() - src_pos;
            let num_bytes_to_process = src_buf_left.min(MAX_MATCH_LEN - lookahead_size);
            if lookahead_size + self.dict.len >= MIN_MATCH_LEN - 1 && num_bytes_to_process > 0 {
                let dict = &mut self.dict;
                let mut dst_pos = (lookahead_pos + lookahead_size) & DICT_MASK;
                let mut ins_pos = lookahead_pos + lookahead_size - 2;
                let mut hash = (u32::from(dict.dict[ins_pos & DICT_MASK]) << HASH_SHIFT)
                    ^ u32::from(dict.dict[(ins_pos + 1) & DICT_MASK]);
                lookahead_size += num_bytes_to_process;
                for &c in &data[src_pos..src_pos + num_bytes_to_process] {
                    dict.dict[dst_pos] = c;
                    if dst_pos < MAX_MATCH_LEN - 1 {
                        dict.dict[DICTIONARY_SIZE + dst_pos] = c;
                    }
                    hash = ((hash << HASH_SHIFT) ^ u32::from(c)) & (HASH_SIZE as u32 - 1);
                    dict.next[ins_pos & DICT_MASK] = dict.hash[hash as usize];
                    dict.hash[hash as usize] = ins_pos as u16;
                    dst_pos = (dst_pos + 1) & DICT_MASK;
                    ins_pos += 1;
                }
                src_pos += num_bytes_to_process;
            } else {
                let dict = &mut self.dict;
                for &c in &data[src_pos..src_pos + num_bytes_to_process] {
                    let dst_pos = (lookahead_pos + lookahead_size) & DICT_MASK;
                    dict.dict[dst_pos] = c;
                    if dst_pos < MAX_MATCH_LEN - 1 {
                        dict.dict[DICTIONARY_SIZE + dst_pos] = c;
                    }
                    lookahead_size += 1;
                    if lookahead_size + dict.len >= MIN_MATCH_LEN {
                        let ins_pos = lookahead_pos + lookahead_size - 3;
                        let hash = ((u32::from(dict.dict[ins_pos & DICT_MASK])
                            << (HASH_SHIFT * 2))
                            ^ ((u32::from(dict.dict[(ins_pos + 1) & DICT_MASK]) << HASH_SHIFT)
                                ^ u32::from(c)))
                            & (HASH_SIZE as u32 - 1);
                        dict.next[ins_pos & DICT_MASK] = dict.hash[hash as usize];
                        dict.hash[hash as usize] = ins_pos as u16;
                    }
                }
                src_pos += num_bytes_to_process;
            }
            self.dict.len = self.dict.len.min(DICTIONARY_SIZE - lookahead_size);
            if lookahead_size < MAX_MATCH_LEN && !is_last {
                break;
            }
            let mut len_to_move = 1;
            let mut cur_match_dist = 0;
            let mut cur_match_len = if saved_match_len != 0 {
                saved_match_len
            } else {
                MIN_MATCH_LEN - 1
            };
            let cur_pos = lookahead_pos & DICT_MASK;
            if self.flags & (RLE_MATCHES | FORCE_RAW) != 0 {
                if self.dict.len != 0 && self.flags & FORCE_RAW == 0 {
                    let c = self.dict.dict[cur_pos.wrapping_sub(1) & DICT_MASK];
                    cur_match_len = self.dict.dict[cur_pos..(cur_pos + lookahead_size)]
                        .iter()
                        .take_while(|&x| *x == c)
                        .count();
                    if cur_match_len < MIN_MATCH_LEN {
                        cur_match_len = 0
                    } else {
                        cur_match_dist = 1
                    }
                }
            } else {
                let dist_len = self.dict.find_match(
                    lookahead_pos,
                    self.dict.len,
                    lookahead_size,
                    cur_match_dist,
                    cur_match_len,
                );
                cur_match_dist = dist_len.0;
                cur_match_len = dist_len.1;
            }
            let far_and_small = cur_match_len == MIN_MATCH_LEN && cur_match_dist >= 8 * 1024;
            let filter_small = self.flags & FILTER_MATCHES != 0 && cur_match_len <= 5;
            if far_and_small || filter_small || cur_pos == cur_match_dist {
                cur_match_dist = 0;
                cur_match_len = 0;
            }
            if saved_match_len != 0 {
                if cur_match_len > saved_match_len {
                    self.cb.push_literal(saved_lit, &mut self.lt);
                    if cur_match_len >= 128 {
                        self.cb.push_match(
                            cur_match_len,
                            cur_match_dist,
                            &mut self.lt,
                            &mut self.dt,
                        );
                        saved_match_len = 0;
                        len_to_move = cur_match_len;
                    } else {
                        saved_lit = self.dict.get(cur_pos);
                        saved_match_dist = cur_match_dist;
                        saved_match_len = cur_match_len;
                    }
                } else {
                    self.cb.push_match(
                        saved_match_len,
                        saved_match_dist,
                        &mut self.lt,
                        &mut self.dt,
                    );
                    len_to_move = saved_match_len - 1;
                    saved_match_len = 0;
                }
            } else if cur_match_dist == 0 {
                self.cb.push_literal(self.dict.get(cur_pos), &mut self.lt);
            } else if self.greedy_parsing || (self.flags & RLE_MATCHES != 0) || cur_match_len >= 128
            {
                self.cb
                    .push_match(cur_match_len, cur_match_dist, &mut self.lt, &mut self.dt);
                len_to_move = cur_match_len;
            } else {
                saved_lit = self.dict.get(cur_pos);
                saved_match_dist = cur_match_dist;
                saved_match_len = cur_match_len;
            }
            lookahead_pos += len_to_move;
            assert!(lookahead_size >= len_to_move);
            lookahead_size -= len_to_move;
            self.dict.len = (self.dict.len + len_to_move).min(DICTIONARY_SIZE);
            let lz_buf_tight = self.cb.pos > CODE_BUFFER_SIZE - 8;
            let raw = self.flags & FORCE_RAW != 0;
            let fat = ((self.cb.pos * 115) >> 7) >= self.cb.total_bytes;
            let fat_or_raw = (self.cb.total_bytes > 31 * 1024) && (fat || raw);
            if lz_buf_tight || fat_or_raw {
                self.dict.lookahead_size = lookahead_size;
                self.dict.lookahead_pos = lookahead_pos;
                self.flush_block(sink, false)?;
            }
        }
        self.dict.lookahead_size = lookahead_size;
        self.dict.lookahead_pos = lookahead_pos;
        self.saved_lit = saved_lit;
        self.saved_match_dist = saved_match_dist;
        self.saved_match_len = saved_match_len;
        Ok(())
    }

    fn flush_block<S: Sink>(&mut self, sink: &mut S, finish: bool) -> Result<(), Error> {
        sink.set_bit_buffer(self.saved_bit_buffer, self.saved_bits_in);
        let mut snapshot;
        let use_raw_block = (self.flags & FORCE_RAW != 0)
            && (self.dict.lookahead_pos - self.dict.code_buffer_offset) <= self.dict.len;
        self.cb.init_flag();
        if self.flags & WRITE_ZLIB_HEADER != 0 && self.block_index == 0 {
            let header = make_zlib_header(self.flags);
            sink.put_bits(header[0].into(), 8)?;
            sink.put_bits(header[1].into(), 8)?;
        }
        sink.put_bits(finish as u32, 1)?;
        snapshot = sink.snapshot();
        let comp_success = if !use_raw_block {
            let use_static = (self.flags & FORCE_STATIC != 0) || (self.cb.total_bytes < 48);
            self.emit_block(sink, use_static)?;
            true
        } else {
            false
        };
        let end_pos = sink.snapshot().pos;
        let expanded = (self.cb.total_bytes > 32)
            && (end_pos - snapshot.pos + 1 >= self.cb.total_bytes)
            && (self.dict.lookahead_pos - self.dict.code_buffer_offset <= self.dict.len);
        if use_raw_block || expanded {
            sink.restore(&snapshot);
            sink.put_bits(0, 2)?;
            sink.pad()?;
            sink.put_bits(self.cb.total_bytes as u32 & 0xFFFF, 16)?;
            sink.put_bits(!self.cb.total_bytes as u32 & 0xFFFF, 16)?;
            for i in 0..self.cb.total_bytes {
                let pos = (self.dict.code_buffer_offset + i) & DICTIONARY_SIZE_MASK;
                sink.put_bits(u32::from(self.dict.dict[pos]), 8)?;
            }
        } else if !comp_success {
            sink.restore(&snapshot);
            self.emit_block(sink, true)?;
        }
        if finish {
            sink.pad()?;
            if self.flags & WRITE_ZLIB_HEADER != 0 {
                let mut adler = self.adler32.finish();
                for _ in 0..4 {
                    sink.put_bits((adler >> 24) & 0xFF, 8)?;
                    adler <<= 8;
                }
            }
        }
        self.lt.reset();
        self.dt.reset();
        self.cb.pos = 1;
        self.cb.flags_offset = 0;
        self.cb.flags_left = 8;
        self.dict.code_buffer_offset += self.cb.total_bytes;
        self.cb.total_bytes = 0;
        self.block_index += 1;
        snapshot = sink.snapshot();
        self.saved_bit_buffer = snapshot.bit_buffer;
        self.saved_bits_in = snapshot.bits_in;
        sink.flush()
    }

    fn reset(&mut self, zlib: bool) {
        if self.ready && zlib == self.zlib {
            return;
        }
        let flags = make_flags(zlib, self.level, self.strategy);
        self.zlib = zlib;
        self.flags = flags;
        self.greedy_parsing = flags & GREEDY_PARSING != 0;
        self.block_index = 0;
        self.saved_lit = 0;
        self.saved_match_dist = 0;
        self.saved_match_len = 0;
        self.saved_bit_buffer = 0;
        self.saved_bits_in = 0;
        self.dict.code_buffer_offset = 0;
        self.dict.len = 0;
        self.dict.lookahead_pos = 0;
        self.dict.lookahead_size = 0;
        self.dict.max_probes = Dictionary::probes_from_flags(flags);
        self.cb.reset();
        if !self.ready {
            self.lt.reset();
            self.dt.reset();
            self.pt.reset();
        }
        self.ready = true;
        self.adler32 = Adler32::new();
    }
}

impl DeflateContext {
    fn start_dynamic_block<S: Sink>(&mut self, sink: &mut S) -> Result<(), Error> {
        const CODE_SIZES_LEN: usize = LITERAL_LENGTH_TREE_SIZE + DISTANCE_TREE_SIZE;
        let mut code_sizes_to_pack = [0u8; CODE_SIZES_LEN];
        let mut packed = [0u8; CODE_SIZES_LEN];
        self.lt.counts[256] = 1;
        self.lt.optimize(false);
        self.dt.optimize(false);
        let mut num_lit_codes = 286;
        while num_lit_codes > 257 {
            if self.lt.code_sizes[num_lit_codes - 1] != 0 {
                break;
            }
            num_lit_codes -= 1;
        }
        let mut num_dist_codes = 30;
        while num_dist_codes > 1 {
            if self.dt.code_sizes[num_dist_codes - 1] != 0 {
                break;
            }
            num_dist_codes -= 1;
        }
        code_sizes_to_pack[0..num_lit_codes].copy_from_slice(&self.lt.code_sizes[0..num_lit_codes]);
        code_sizes_to_pack[num_lit_codes..num_lit_codes + num_dist_codes]
            .copy_from_slice(&self.dt.code_sizes[0..num_dist_codes]);
        let total_code_sizes_to_pack = num_lit_codes + num_dist_codes;
        let mut num_packed = 0;
        for i in 0..PRECODE_TREE_SIZE {
            self.pt.counts[i] = 0;
        }
        let mut rle = Rle::new();
        for i in 0..total_code_sizes_to_pack {
            let code_size = code_sizes_to_pack[i] as usize;
            if code_size == 0 {
                rle.prev(&mut packed, &mut num_packed, &mut self.pt);
                rle.z_count += 1;
                if rle.z_count == 138 {
                    rle.zero(&mut packed, &mut num_packed, &mut self.pt);
                }
            } else {
                rle.zero(&mut packed, &mut num_packed, &mut self.pt);
                if code_size != rle.prev {
                    rle.prev(&mut packed, &mut num_packed, &mut self.pt);
                    self.pt.counts[code_size] += 1;
                    packed[num_packed] = code_size as u8;
                    num_packed += 1;
                } else {
                    rle.repeat_count += 1;
                    if rle.repeat_count == 6 {
                        rle.prev(&mut packed, &mut num_packed, &mut self.pt);
                    }
                }
            }
            rle.prev = code_size;
        }
        if rle.repeat_count != 0 {
            rle.prev(&mut packed, &mut num_packed, &mut self.pt);
        } else {
            rle.zero(&mut packed, &mut num_packed, &mut self.pt);
        }
        self.pt.optimize();
        sink.put_bits(2, 2)?;
        sink.put_bits(num_lit_codes as u32 - 257, 5)?;
        sink.put_bits(num_dist_codes as u32 - 1, 5)?;
        let mut num_bit_lengths = 0;
        for i in (0..=18).rev() {
            if self.pt.code_sizes[PRECODE_SWIZZLE[i] as usize] != 0 {
                num_bit_lengths = i;
                break;
            }
        }
        num_bit_lengths = 4.max(num_bit_lengths + 1);
        sink.put_bits(num_bit_lengths as u32 - 4, 4)?;
        for swizzle in &PRECODE_SWIZZLE[..num_bit_lengths] {
            sink.put_bits(self.pt.code_sizes[*swizzle as usize] as u32, 3)?;
        }
        let mut i = 0;
        while i < num_packed {
            let code = packed[i] as usize;
            i += 1;
            sink.put_bits(self.pt.codes[code] as u32, self.pt.code_sizes[code] as u32)?;
            if code >= 16 {
                sink.put_bits(packed[i] as u32, [2, 3, 7][code - 16])?;
                i += 1;
            }
        }
        Ok(())
    }

    fn start_static_block<S: Sink>(&mut self, sink: &mut S) -> Result<(), Error> {
        let lengths = &mut self.lt.code_sizes;
        lengths[0..144].iter_mut().for_each(|p| *p = 8);
        lengths[144..256].iter_mut().for_each(|p| *p = 9);
        lengths[256..280].iter_mut().for_each(|p| *p = 7);
        lengths[280..288].iter_mut().for_each(|p| *p = 8);
        self.dt.code_sizes = [5; 32];
        self.lt.optimize(true);
        self.dt.optimize(true);
        sink.put_bits(1, 2)
    }

    fn emit_block<S: Sink>(&mut self, sink: &mut S, is_static: bool) -> Result<(), Error> {
        if is_static {
            self.start_static_block(sink)?;
        } else {
            self.start_dynamic_block(sink)?;
        }
        self.cb.emit(sink, &self.lt, &self.dt)
    }
}

struct Rle {
    prev: usize,
    repeat_count: usize,
    z_count: usize,
}

impl Rle {
    fn new() -> Self {
        Self {
            prev: 0xFF,
            repeat_count: 0,
            z_count: 0,
        }
    }

    #[inline(always)]
    fn prev(&mut self, code_sizes: &mut [u8], count: &mut usize, pt: &mut PrecodeTree) {
        if self.repeat_count == 0 {
            return;
        }
        if self.repeat_count < 3 {
            pt.counts[self.prev] += self.repeat_count as u16;
            while self.repeat_count != 0 {
                code_sizes[*count] = self.prev as u8;
                *count += 1;
                self.repeat_count -= 1;
            }
        } else {
            pt.counts[16] += 1;
            code_sizes[*count] = 16;
            *count += 1;
            code_sizes[*count] = (self.repeat_count - 3) as u8;
            *count += 1;
        }
        self.repeat_count = 0;
    }

    #[inline(always)]
    fn zero(&mut self, code_sizes: &mut [u8], count: &mut usize, pt: &mut PrecodeTree) {
        if self.z_count == 0 {
            return;
        }
        if self.z_count < 3 {
            pt.counts[0] += self.z_count as u16;
            while self.z_count != 0 {
                code_sizes[*count] = 0;
                *count += 1;
                self.z_count -= 1;
            }
        } else if self.z_count <= 10 {
            pt.counts[17] += 1;
            code_sizes[*count] = 17;
            *count += 1;
            code_sizes[*count] = (self.z_count - 3) as u8;
            *count += 1;
        } else {
            pt.counts[18] += 1;
            code_sizes[*count] = 18;
            *count += 1;
            code_sizes[*count] = (self.z_count - 11) as u8;
            *count += 1;
        }
        self.z_count = 0;
    }
}

struct LiteralLengthTree {
    pub counts: [u16; LITERAL_LENGTH_TREE_SIZE],
    pub codes: [u16; LITERAL_LENGTH_TREE_SIZE],
    pub code_sizes: [u8; LITERAL_LENGTH_TREE_SIZE],
}

impl LiteralLengthTree {
    #[inline(always)]
    fn new() -> Self {
        Self {
            counts: [0; LITERAL_LENGTH_TREE_SIZE],
            codes: [0; LITERAL_LENGTH_TREE_SIZE],
            code_sizes: [0; LITERAL_LENGTH_TREE_SIZE],
        }
    }

    fn reset(&mut self) {
        self.counts.iter_mut().for_each(|p| *p = 0);
    }

    fn optimize(&mut self, is_static: bool) {
        huffman::optimize(
            &mut self.counts,
            &mut self.codes,
            &mut self.code_sizes,
            15,
            is_static,
        );
    }
}

struct DistanceTree {
    pub counts: [u16; DISTANCE_TREE_SIZE],
    pub codes: [u16; DISTANCE_TREE_SIZE],
    pub code_sizes: [u8; DISTANCE_TREE_SIZE],
}

impl DistanceTree {
    #[inline(always)]
    fn new() -> Self {
        Self {
            counts: [0; DISTANCE_TREE_SIZE],
            codes: [0; DISTANCE_TREE_SIZE],
            code_sizes: [0; DISTANCE_TREE_SIZE],
        }
    }

    fn reset(&mut self) {
        self.counts.iter_mut().for_each(|p| *p = 0);
    }

    fn optimize(&mut self, is_static: bool) {
        huffman::optimize(
            &mut self.counts,
            &mut self.codes,
            &mut self.code_sizes,
            15,
            is_static,
        );
    }
}

struct PrecodeTree {
    pub counts: [u16; PRECODE_TREE_SIZE],
    pub codes: [u16; PRECODE_TREE_SIZE],
    pub code_sizes: [u8; PRECODE_TREE_SIZE],
}

impl PrecodeTree {
    fn new() -> Self {
        Self {
            counts: [0; PRECODE_TREE_SIZE],
            codes: [0; PRECODE_TREE_SIZE],
            code_sizes: [0; PRECODE_TREE_SIZE],
        }
    }

    fn reset(&mut self) {
        self.counts.iter_mut().for_each(|p| *p = 0);
    }

    fn optimize(&mut self) {
        huffman::optimize(
            &mut self.counts,
            &mut self.codes,
            &mut self.code_sizes,
            7,
            false,
        );
    }
}

mod huffman {
    const MAX_HUFF_SYMBOLS: usize = 288;
    const MAX_SUPPORTED_HUFF_CODE_SIZE: usize = 32;

    #[derive(Copy, Clone, Default)]
    struct SymbolFrequency {
        key: u16,
        index: u16,
    }

    pub fn optimize(
        counts: &mut [u16],
        codes: &mut [u16],
        code_sizes: &mut [u8],
        size_limit: usize,
        is_static: bool,
    ) {
        let mut num_codes = [0i32; 1 + MAX_SUPPORTED_HUFF_CODE_SIZE];
        let mut next_code = [0u32; 1 + MAX_SUPPORTED_HUFF_CODE_SIZE];
        let len = counts.len();
        if is_static {
            for i in 0..len {
                num_codes[code_sizes[i] as usize] += 1;
            }
        } else {
            let mut syms0 = [SymbolFrequency::default(); MAX_HUFF_SYMBOLS];
            let mut syms1 = [SymbolFrequency::default(); MAX_HUFF_SYMBOLS];
            let mut used = 0;
            for i in 0..len {
                let count = counts[i];
                if count != 0 {
                    let sym = &mut syms0[used];
                    used += 1;
                    sym.key = count;
                    sym.index = i as u16;
                }
            }
            let syms = sort_symbols(&mut syms0[..used], &mut syms1[..used]);
            minimum_redundancy(syms);
            for s in syms.iter() {
                num_codes[s.key as usize] += 1;
            }
            enforce_size_limit(&mut num_codes, used, size_limit);
            codes.iter_mut().for_each(|p| *p = 0);
            code_sizes.iter_mut().for_each(|p| *p = 0);
            let mut last = used;
            for i in 1..=size_limit {
                let first = last - num_codes[i] as usize;
                for sym in &syms[first..last] {
                    code_sizes[sym.index as usize] = i as u8;
                }
                last = first;
            }
        }
        next_code[1] = 0;
        let mut j = 0;
        for i in 2..=size_limit {
            j = (j + num_codes[i - 1]) << 1;
            next_code[i] = j as u32;
        }
        for i in 0..len {
            let code_size = code_sizes[i] as usize;
            if code_size == 0 {
                continue;
            }
            let mut code = next_code[code_size];
            let mut rev_code = 0;
            next_code[code_size] += 1;
            for _ in 0..code_size {
                rev_code = (rev_code << 1) | (code & 1);
                code >>= 1;
            }
            codes[i] = rev_code as u16;
        }
    }

    fn sort_symbols<'a>(
        syms0: &'a mut [SymbolFrequency],
        syms1: &'a mut [SymbolFrequency],
    ) -> &'a mut [SymbolFrequency] {
        let mut hist = [[0u32; 256]; 2];
        for freq in syms0.iter() {
            let key = freq.key as usize;
            hist[0][key & 0xFF] += 1;
            hist[1][(key >> 8) & 0xFF] += 1;
        }
        let mut passes = 2;
        if syms0.len() == hist[1][0] as usize {
            passes -= 1;
        }
        let mut offsets = [0u32; 256];
        let mut cur_syms = syms0;
        let mut new_syms = syms1;
        for pass in 0..passes {
            let mut offset = 0;
            for i in 0..256 {
                offsets[i] = offset;
                offset += hist[pass][i];
            }
            for sym in cur_syms.iter() {
                let j = ((sym.key >> (pass * 8)) & 0xFF) as usize;
                new_syms[offsets[j] as usize] = *sym;
                offsets[j] += 1;
            }
            core::mem::swap(&mut cur_syms, &mut new_syms);
        }
        cur_syms
    }

    fn minimum_redundancy(a: &mut [SymbolFrequency]) {
        let n = a.len();
        if n == 0 {
            return;
        } else if n == 1 {
            a[0].key = 1;
            return;
        }
        a[0].key += a[1].key;
        let mut root = 0;
        let mut leaf = 2;
        for next in 1..n - 1 {
            if leaf >= n || a[root].key < a[leaf].key {
                a[next].key = a[root].key;
                a[root].key = next as u16;
                root += 1;
            } else {
                a[next].key = a[leaf].key;
                leaf += 1;
            }
            if leaf >= n || (root < next && a[root].key < a[leaf].key) {
                a[next].key += a[root].key;
                a[root].key = next as u16;
                root += 1;
            } else {
                a[next].key += a[leaf].key;
                leaf += 1;
            }
        }
        a[n - 2].key = 0;
        for next in (0..n - 2).rev() {
            a[next].key = a[a[next].key as usize].key + 1;
        }
        let mut avail = 1isize;
        let mut used = 0isize;
        let mut depth = 0;
        let mut root = n as isize - 2;
        let mut next = n as isize - 1;
        while avail > 0 {
            while root >= 0 && a[root as usize].key == depth {
                used += 1;
                root -= 1;
            }
            while avail > used {
                a[next as usize].key = depth;
                next -= 1;
                avail -= 1;
            }
            avail = 2 * used;
            depth += 1;
            used = 0;
        }
    }

    fn enforce_size_limit(num_codes: &mut [i32], len: usize, size_limit: usize) {
        if len <= 1 {
            return;
        }
        for i in size_limit + 1..=MAX_SUPPORTED_HUFF_CODE_SIZE {
            num_codes[size_limit] += num_codes[i];
        }
        let mut total = 0;
        for i in (1..=size_limit).rev() {
            total += (num_codes[i] as u32) << (size_limit - i);
        }
        while total != (1 << size_limit) {
            num_codes[size_limit] -= 1;
            for i in (1..size_limit).rev() {
                if num_codes[i] != 0 {
                    num_codes[i] -= 1;
                    num_codes[i + 1] += 2;
                    break;
                }
            }
            total -= 1;
        }
    }
}

struct CodeBuffer {
    pub buffer: [u8; CODE_BUFFER_SIZE],
    pub pos: usize,
    pub flags_offset: usize,
    pub flags_left: usize,
    pub total_bytes: usize,
}

impl CodeBuffer {
    #[inline(always)]
    fn new() -> Self {
        Self {
            buffer: [0u8; CODE_BUFFER_SIZE],
            pos: 1,
            flags_offset: 0,
            flags_left: 8,
            total_bytes: 0,
        }
    }

    fn reset(&mut self) {
        self.pos = 1;
        self.flags_offset = 0;
        self.flags_left = 8;
        self.total_bytes = 0;
    }

    fn init_flag(&mut self) {
        if self.flags_left == 8 {
            self.buffer[self.flags_offset] = 0;
            self.pos -= 1;
        } else {
            self.buffer[self.flags_offset] >>= self.flags_left;
        }
    }

    #[inline(always)]
    fn push_literal(&mut self, lit: u8, lt: &mut LiteralLengthTree) {
        self.buffer[self.pos] = lit;
        self.pos += 1;
        self.total_bytes += 1;
        self.buffer[self.flags_offset] >>= 1;
        self.flags_left -= 1;
        if self.flags_left == 0 {
            self.flags_left = 8;
            self.flags_offset = self.pos;
            self.pos += 1;
        }
        lt.counts[lit as usize] += 1;
    }

    #[inline(always)]
    fn push_match(
        &mut self,
        len: usize,
        mut dist: usize,
        lt: &mut LiteralLengthTree,
        dt: &mut DistanceTree,
    ) {
        self.total_bytes += len;
        self.buffer[self.pos] = (len - MIN_MATCH_LEN) as u8;
        dist -= 1;
        self.buffer[self.pos + 1] = (dist & 0xFF) as u8;
        self.buffer[self.pos + 2] = (dist >> 8) as u8;
        self.pos += 3;
        self.buffer[self.flags_offset] = (self.buffer[self.flags_offset] >> 1) | 0x80;
        self.flags_left -= 1;
        if self.flags_left == 0 {
            self.flags_left = 8;
            self.flags_offset = self.pos;
            self.pos += 1;
        }
        let s = if dist < 512 {
            SMALL_DIST_SYM[dist & 511] as usize
        } else {
            LARGE_DIST_SYM[(dist >> 8) & 127] as usize
        };
        dt.counts[s] += 1;
        if len >= MIN_MATCH_LEN {
            lt.counts[LEN_SYM[len - MIN_MATCH_LEN] as usize] += 1;
        }
    }

    fn emit<S: Sink>(
        &self,
        sink: &mut S,
        lt: &LiteralLengthTree,
        dt: &DistanceTree,
    ) -> Result<(), Error> {
        let mut flags = 1;
        let snap = sink.snapshot();
        let mut bits = FastBits::new(snap.bit_buffer, snap.bits_in);
        let mut i = 0;
        while i < self.pos {
            if flags == 1 {
                flags = self.buffer[i] as u32 | 0x100;
                i += 1;
            }
            if flags & 1 != 0 {
                if bits.bits_in > 16 {
                    bits.flush(sink)?;
                }
                let match_len = self.buffer[i] as usize;
                let match_dist = self.buffer[i + 1] as usize | ((self.buffer[i + 2] as usize) << 8);
                i += 3;
                let i0 = LEN_SYM[match_len & 0xFF] as usize;
                bits.put(lt.codes[i0] as u32, lt.code_sizes[i0] as u32);
                let extra = LEN_EXTRA[match_len & 0xFF] as usize;
                bits.put(match_len as u32 & BITMASKS[extra], extra as u32);
                let (sym, extra_bits) = if match_dist < 512 {
                    (
                        SMALL_DIST_SYM[match_dist & 511] as usize,
                        SMALL_DIST_EXTRA[match_dist & 511] as usize,
                    )
                } else {
                    (
                        LARGE_DIST_SYM[(match_dist >> 8) & 127] as usize,
                        LARGE_DIST_EXTRA[(match_dist >> 8) & 127] as usize,
                    )
                };
                bits.put(dt.codes[sym] as u32, dt.code_sizes[sym] as u32);
                bits.put(match_dist as u32 & BITMASKS[extra_bits], extra_bits as u32);
            } else {
                let lit = self.buffer[i] as usize;
                i += 1;
                if bits.bits_in > 48 {
                    bits.flush(sink)?;
                }
                bits.put(
                    lt.codes[lit & 0xFF] as u32,
                    lt.code_sizes[lit & 0xFF] as u32,
                );
            }
            flags >>= 1;
        }
        bits.flush(sink)?;
        sink.set_bit_buffer(bits.bit_buffer as u32, bits.bits_in);
        sink.put_bits(lt.codes[256] as u32, lt.code_sizes[256] as u32)
    }
}

struct FastBits {
    bit_buffer: u64,
    bits_in: u32,
    buf: [u8; 8],
}

impl FastBits {
    pub fn new(bit_buffer: u32, bits_in: u32) -> Self {
        Self {
            bit_buffer: bit_buffer as u64,
            bits_in,
            buf: [0; 8],
        }
    }

    #[inline(always)]
    pub fn put(&mut self, bits: u32, len: u32) {
        self.bit_buffer |= (bits as u64) << self.bits_in;
        self.bits_in += len;
    }

    #[inline(always)]
    pub fn flush<S: Sink>(&mut self, sink: &mut S) -> Result<(), Error> {
        let mut i = 0;
        while self.bits_in >= 8 {
            self.buf[i] = self.bit_buffer as u8;
            self.bit_buffer >>= 8;
            self.bits_in -= 8;
            i += 1;
        }
        sink.write(&self.buf[0..i])
    }
}

struct Dictionary {
    pub dict: [u8; DICTIONARY_FULL_SIZE],
    pub next: [u16; DICTIONARY_SIZE],
    pub hash: [u16; HASH_SIZE],
    pub code_buffer_offset: usize,
    pub max_probes: [u32; 2],
    pub lookahead_size: usize,
    pub lookahead_pos: usize,
    pub len: usize,
}

impl Dictionary {
    #[inline(always)]
    fn new(flags: u32) -> Self {
        Self {
            dict: [0; DICTIONARY_FULL_SIZE],
            next: [0; DICTIONARY_SIZE],
            hash: [0; HASH_SIZE],
            code_buffer_offset: 0,
            max_probes: Self::probes_from_flags(flags),
            lookahead_size: 0,
            lookahead_pos: 0,
            len: 0,
        }
    }

    fn probes_from_flags(flags: u32) -> [u32; 2] {
        [
            1 + ((flags & 0xFFF) + 2) / 3,
            1 + (((flags & 0xFFF) >> 2) + 2) / 3,
        ]
    }

    fn read_u64(&self, pos: usize) -> u64 {
        let bytes: [u8; 8] = self.dict[pos..pos + 8].try_into().unwrap();
        u64::from_le_bytes(bytes)
    }

    fn read_u16(&self, pos: usize) -> u16 {
        self.dict[pos] as u16 | ((self.dict[pos + 1] as u16) << 8)
    }

    fn get(&self, pos: usize) -> u8 {
        self.dict[pos.min(self.dict.len() - 1)]
    }

    fn find_match(
        &self,
        lookahead_pos: usize,
        max_dist: usize,
        max_match_len: usize,
        mut match_dist: usize,
        mut match_len: usize,
    ) -> (usize, usize) {
        let max_match_len = max_match_len.min(MAX_MATCH_LEN);
        match_len = match_len.max(1);
        let pos = lookahead_pos & DICTIONARY_SIZE_MASK;
        let mut probe_pos = pos;
        let mut num_probes_left = self.max_probes[(match_len >= 32) as usize];
        if max_match_len <= match_len {
            return (match_dist, match_len);
        }
        let mut c01 = self.read_u16(pos + match_len - 1);
        let s01 = self.read_u16(pos);
        'outer: loop {
            let mut dist;
            'found: loop {
                num_probes_left -= 1;
                if num_probes_left == 0 {
                    return (match_dist, match_len);
                }
                for _ in 0..3 {
                    let next_probe_pos = self.next[probe_pos] as usize;
                    dist = (lookahead_pos - next_probe_pos) & 0xFFFF;
                    if next_probe_pos == 0 || dist > max_dist {
                        return (match_dist, match_len);
                    }
                    probe_pos = next_probe_pos & DICTIONARY_SIZE_MASK;
                    if self.read_u16(probe_pos + match_len - 1) == c01 {
                        break 'found;
                    }
                }
            }
            if dist == 0 {
                return (match_dist, match_len);
            }
            if self.read_u16(probe_pos) != s01 {
                continue;
            }
            let mut p = pos + 2;
            let mut q = probe_pos + 2;
            for _ in 0..32 {
                let p_data: u64 = self.read_u64(p);
                let q_data: u64 = self.read_u64(q);
                let xor_data = p_data ^ q_data;
                if xor_data == 0 {
                    p += 8;
                    q += 8;
                } else {
                    let trailing = xor_data.trailing_zeros() as usize;
                    let probe_len = p - pos + (trailing >> 3);
                    if probe_len > match_len {
                        match_dist = dist;
                        match_len = max_match_len.min(probe_len);
                        if match_len == max_match_len {
                            return (match_dist, match_len);
                        }
                        c01 = self.read_u16(pos + match_len - 1)
                    }
                    continue 'outer;
                }
            }
            return (dist, max_match_len.min(MAX_MATCH_LEN));
        }
    }
}

#[doc(hidden)]
pub struct Snapshot {
    pos: usize,
    bit_buffer: u32,
    bits_in: u32,
}

#[doc(hidden)]
pub trait Sink {
    fn put_bits(&mut self, bits: u32, len: u32) -> Result<(), Error>;
    fn write(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn pad(&mut self) -> Result<(), Error>;
    fn snapshot(&self) -> Snapshot;
    fn restore(&mut self, snapshot: &Snapshot);
    fn set_bit_buffer(&mut self, bit_buffer: u32, bits_in: u32);
    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
    fn written(&self) -> u64;
}

struct BufSink<'a> {
    buffer: &'a mut [u8],
    pos: usize,
    bit_buffer: u32,
    bits_in: u32,
}

impl<'a> BufSink<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self {
            buffer,
            pos: 0,
            bit_buffer: 0,
            bits_in: 0,
        }
    }
}

impl Sink for BufSink<'_> {
    #[inline(always)]
    fn put_bits(&mut self, bits: u32, len: u32) -> Result<(), Error> {
        self.bit_buffer |= bits << self.bits_in;
        self.bits_in += len;
        let limit = self.buffer.len();
        while self.bits_in >= 8 {
            if self.pos == limit {
                return Err(Error::Overflow);
            }
            self.buffer[self.pos] = self.bit_buffer as u8;
            self.pos += 1;
            self.bit_buffer >>= 8;
            self.bits_in -= 8;
        }
        Ok(())
    }

    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        let len = buf.len();
        if self.pos + len > self.buffer.len() {
            return Err(Error::Overflow);
        }
        self.buffer[self.pos..self.pos + len].copy_from_slice(buf);
        self.pos += len;
        Ok(())
    }

    fn pad(&mut self) -> Result<(), Error> {
        if self.bits_in != 0 {
            let len = 8 - self.bits_in;
            self.put_bits(0, len)
        } else {
            Ok(())
        }
    }

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            pos: self.pos,
            bit_buffer: self.bit_buffer,
            bits_in: self.bits_in,
        }
    }

    fn restore(&mut self, snapshot: &Snapshot) {
        self.pos = snapshot.pos;
        self.bit_buffer = snapshot.bit_buffer;
        self.bits_in = snapshot.bits_in;
    }

    fn set_bit_buffer(&mut self, bit_buffer: u32, bits_in: u32) {
        self.bit_buffer = bit_buffer;
        self.bits_in = bits_in;
    }

    fn written(&self) -> u64 {
        self.pos as u64
    }
}

struct VecSink<'a> {
    buffer: &'a mut Vec<u8>,
    start_pos: usize,
    bit_buffer: u32,
    bits_in: u32,
}

impl<'a> VecSink<'a> {
    pub fn new(buffer: &'a mut Vec<u8>) -> Self {
        let start_pos = buffer.len();
        Self {
            buffer,
            start_pos,
            bit_buffer: 0,
            bits_in: 0,
        }
    }
}

impl Sink for VecSink<'_> {
    #[inline(always)]
    fn put_bits(&mut self, bits: u32, len: u32) -> Result<(), Error> {
        self.bit_buffer |= bits << self.bits_in;
        self.bits_in += len;
        while self.bits_in >= 8 {
            self.buffer.push(self.bit_buffer as u8);
            self.bit_buffer >>= 8;
            self.bits_in -= 8;
        }
        Ok(())
    }

    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        self.buffer.extend_from_slice(buf);
        Ok(())
    }

    fn pad(&mut self) -> Result<(), Error> {
        if self.bits_in != 0 {
            let len = 8 - self.bits_in;
            self.put_bits(0, len)
        } else {
            Ok(())
        }
    }

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            pos: self.buffer.len(),
            bit_buffer: self.bit_buffer,
            bits_in: self.bits_in,
        }
    }

    fn restore(&mut self, snapshot: &Snapshot) {
        self.buffer.truncate(snapshot.pos);
        self.bit_buffer = snapshot.bit_buffer;
        self.bits_in = snapshot.bits_in;
    }

    fn set_bit_buffer(&mut self, bit_buffer: u32, bits_in: u32) {
        self.bit_buffer = bit_buffer;
        self.bits_in = bits_in;
    }

    fn written(&self) -> u64 {
        (self.buffer.len() - self.start_pos) as u64
    }
}

#[cfg(feature = "std")]
struct WriterSink<W> {
    writer: W,
    buffer: [u8; OUT_BUFFER_SIZE],
    pos: usize,
    bit_buffer: u32,
    bits_in: u32,
    written: u64,
}

#[cfg(feature = "std")]
impl<W> WriterSink<W> {
    fn new(writer: W) -> Self {
        Self {
            writer,
            buffer: [0; OUT_BUFFER_SIZE],
            pos: 0,
            bit_buffer: 0,
            bits_in: 0,
            written: 0,
        }
    }
}

#[cfg(feature = "std")]
impl<W: Write> Sink for WriterSink<W> {
    #[inline(always)]
    fn put_bits(&mut self, bits: u32, len: u32) -> Result<(), Error> {
        self.bit_buffer |= bits << self.bits_in;
        self.bits_in += len;
        let limit = self.buffer.len();
        while self.bits_in >= 8 {
            if self.pos == limit {
                return Err(Error::Overflow);
            }
            self.buffer[self.pos] = self.bit_buffer as u8;
            self.pos += 1;
            self.bit_buffer >>= 8;
            self.bits_in -= 8;
        }
        Ok(())
    }

    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        let len = buf.len();
        if self.pos + len > self.buffer.len() {
            return Err(Error::Overflow);
        }
        self.buffer[self.pos..self.pos + len].copy_from_slice(buf);
        self.pos += len;
        Ok(())
    }

    fn pad(&mut self) -> Result<(), Error> {
        if self.bits_in != 0 {
            let len = 8 - self.bits_in;
            self.put_bits(0, len)
        } else {
            Ok(())
        }
    }

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            pos: self.pos,
            bit_buffer: self.bit_buffer,
            bits_in: self.bits_in,
        }
    }

    fn restore(&mut self, snapshot: &Snapshot) {
        self.pos = snapshot.pos;
        self.bit_buffer = snapshot.bit_buffer;
        self.bits_in = snapshot.bits_in;
    }

    fn set_bit_buffer(&mut self, bit_buffer: u32, bits_in: u32) {
        self.bit_buffer = bit_buffer;
        self.bits_in = bits_in;
    }

    fn flush(&mut self) -> Result<(), Error> {
        let res = match self.writer.write_all(&self.buffer[0..self.pos]) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Io(err)),
        };
        self.written += self.pos as u64;
        self.pos = 0;
        res
    }

    fn written(&self) -> u64 {
        self.written
    }
}

fn make_flags(zlib: bool, level: CompressionLevel, strategy: CompressionStrategy) -> u32 {
    let level = level.to_raw();
    let greedy = if level <= 3 { GREEDY_PARSING } else { 0 };
    let mut flags = NUM_PROBES[level] | greedy;
    if zlib {
        flags |= WRITE_ZLIB_HEADER;
    }
    if level == 0 {
        flags |= FORCE_RAW;
    } else {
        use CompressionStrategy::*;
        match strategy {
            Filtered => flags |= FILTER_MATCHES,
            Huffman => flags &= !MAX_PROBES_MASK as u32,
            Static => flags |= FORCE_STATIC,
            RLE => flags |= RLE_MATCHES,
            _ => {}
        }
    }
    flags
}

fn make_zlib_header(flags: u32) -> [u8; 2] {
    const FCHECK_DIVISOR: u32 = 31;
    let num_probes = flags & (MAX_PROBES_MASK as u32);
    let level = if flags & GREEDY_PARSING != 0 {
        if num_probes <= 1 {
            0
        } else {
            1
        }
    } else if num_probes >= NUM_PROBES[9] {
        3
    } else {
        2
    };
    let cmf = 8 | (7 << 4);
    let flg = (level as u8) << 6;
    let rem = ((cmf as u32 * 256) + flg as u32) % FCHECK_DIVISOR;
    let check = (flg & 0b11100000) + (FCHECK_DIVISOR - rem) as u8;
    [cmf, check]
}

const LITERAL_LENGTH_TREE_SIZE: usize = 288;
const DISTANCE_TREE_SIZE: usize = 32;
const PRECODE_TREE_SIZE: usize = 19;
// const CODE_BUFFER_SIZE: usize = 24 * 1024;
// const HASH_BITS: usize = 12;
const CODE_BUFFER_SIZE: usize = 64 * 1024;
const HASH_BITS: usize = 15;
const HASH_SHIFT: usize = (HASH_BITS + 2) / 3;
const HASH_SIZE: usize = 1 << HASH_BITS;
#[cfg(feature = "std")]
const OUT_BUFFER_SIZE: usize = (CODE_BUFFER_SIZE * 13) / 10;
const MIN_MATCH_LEN: usize = 3;
const MAX_MATCH_LEN: usize = 258;
const DICTIONARY_SIZE: usize = 32768;
const DICTIONARY_SIZE_MASK: usize = DICTIONARY_SIZE - 1;
const DICTIONARY_FULL_SIZE: usize = DICTIONARY_SIZE + MAX_MATCH_LEN;

const WRITE_ZLIB_HEADER: u32 = 0x0000_1000;
const GREEDY_PARSING: u32 = 0x0000_4000;
const RLE_MATCHES: u32 = 0x0001_0000;
const FILTER_MATCHES: u32 = 0x0002_0000;
const FORCE_STATIC: u32 = 0x0004_0000;
const FORCE_RAW: u32 = 0x0008_0000;

const MAX_PROBES_MASK: i32 = 0xFFF;
const NUM_PROBES: [u32; 11] = [0, 1, 6, 32, 16, 32, 128, 256, 512, 768, 1500];

const LEN_SYM: [u16; 256] = [
    257, 258, 259, 260, 261, 262, 263, 264, 265, 265, 266, 266, 267, 267, 268, 268, 269, 269, 269,
    269, 270, 270, 270, 270, 271, 271, 271, 271, 272, 272, 272, 272, 273, 273, 273, 273, 273, 273,
    273, 273, 274, 274, 274, 274, 274, 274, 274, 274, 275, 275, 275, 275, 275, 275, 275, 275, 276,
    276, 276, 276, 276, 276, 276, 276, 277, 277, 277, 277, 277, 277, 277, 277, 277, 277, 277, 277,
    277, 277, 277, 277, 278, 278, 278, 278, 278, 278, 278, 278, 278, 278, 278, 278, 278, 278, 278,
    278, 279, 279, 279, 279, 279, 279, 279, 279, 279, 279, 279, 279, 279, 279, 279, 279, 280, 280,
    280, 280, 280, 280, 280, 280, 280, 280, 280, 280, 280, 280, 280, 280, 281, 281, 281, 281, 281,
    281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281,
    281, 281, 281, 281, 281, 281, 281, 281, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282,
    282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282,
    282, 282, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283,
    283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 284, 284, 284, 284,
    284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284,
    284, 284, 284, 284, 284, 284, 284, 284, 285,
];

const LEN_EXTRA: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0,
];

const SMALL_DIST_SYM: [u8; 512] = [
    0, 1, 2, 3, 4, 4, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9,
    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
    12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 13, 13, 13, 13, 13, 13, 13, 13,
    13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 17, 17, 17, 17, 17, 17, 17, 17,
    17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
    17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
    17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
    17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
    17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
];

const SMALL_DIST_EXTRA: [u8; 512] = [
    0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
];

const LARGE_DIST_SYM: [u8; 128] = [
    0, 0, 18, 19, 20, 20, 21, 21, 22, 22, 22, 22, 23, 23, 23, 23, 24, 24, 24, 24, 24, 24, 24, 24,
    25, 25, 25, 25, 25, 25, 25, 25, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
    27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 28, 28, 28, 28, 28, 28, 28, 28,
    28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
    29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
    29, 29, 29, 29, 29, 29, 29, 29,
];

const LARGE_DIST_EXTRA: [u8; 128] = [
    0, 0, 8, 8, 9, 9, 9, 9, 10, 10, 10, 10, 10, 10, 10, 10, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
    12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
    13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
    13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
    13, 13, 13, 13, 13, 13,
];

const PRECODE_SWIZZLE: [u8; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

const BITMASKS: [u32; 17] = [
    0x0000, 0x0001, 0x0003, 0x0007, 0x000F, 0x001F, 0x003F, 0x007F, 0x00FF, 0x01FF, 0x03FF, 0x07FF,
    0x0FFF, 0x1FFF, 0x3FFF, 0x7FFF, 0xFFFF,
];
