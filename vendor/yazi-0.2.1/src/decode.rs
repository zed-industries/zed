//! RFC 1590 decompression implementation.

#![allow(
    clippy::needless_range_loop,
    clippy::new_without_default,
    clippy::too_many_arguments
)]

use super::{Error, Format};
use alloc::boxed::Box;
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::io::{self, Write};

/// Stateful context for decompression.
///
/// See the crate level [decompression](index.html#decompression) section
/// for detailed usage.
pub struct Decoder(InflateContext);

impl Decoder {
    /// Creates a new deflate decoder.
    pub fn new() -> Self {
        Self(InflateContext::new())
    }

    /// Creates a new deflate decoder on the heap.
    pub fn boxed() -> Box<Self> {
        Box::new(Self(InflateContext::new()))
    }

    /// Sets the expected format of the input data for the next usage of the
    /// decoder.
    pub fn set_format(&mut self, format: Format) {
        self.0.reset(format == Format::Zlib)
    }

    /// Creates a decoder stream that will write into the specified writer.
    #[cfg(feature = "std")]
    pub fn stream<'a, W: Write>(
        &'a mut self,
        writer: &'a mut W,
    ) -> DecoderStream<'a, impl Sink + 'a> {
        self.0.reset(self.0.zlib);
        DecoderStream {
            ctx: &mut self.0,
            sink: WriterSink {
                writer,
                ring: RingBuffer::new(),
                written: 0,
            },
            finished: false,
        }
    }

    /// Creates a decoder stream that will write into the specified vector.
    /// The resulting stream will not clear the vector but will instead append
    /// the decompressed data.     
    pub fn stream_into_vec<'a>(
        &'a mut self,
        vec: &'a mut Vec<u8>,
    ) -> DecoderStream<'a, impl Sink + 'a> {
        self.0.reset(self.0.zlib);
        DecoderStream {
            ctx: &mut self.0,
            sink: VecSink::new(vec),
            finished: false,
        }
    }

    /// Creates a decoder stream that will write into the specified buffer. The
    /// stream will generate an overflow error if the buffer is not large enough
    /// to contain the decompressed data.
    pub fn stream_into_buf<'a>(
        &'a mut self,
        buf: &'a mut [u8],
    ) -> DecoderStream<'a, impl Sink + 'a> {
        self.0.reset(self.0.zlib);
        DecoderStream {
            ctx: &mut self.0,
            sink: BufSink {
                buffer: buf,
                pos: 0,
            },
            finished: false,
        }
    }
}

/// Decompression stream combining a decoder context with an output.
///
/// See the crate level [decompression](index.html#decompression) section
/// for detailed usage.
pub struct DecoderStream<'a, S: Sink> {
    ctx: &'a mut InflateContext,
    sink: S,
    finished: bool,
}

impl<S: Sink> DecoderStream<'_, S> {
    /// Writes the specified buffer to the stream, producing decompressed data
    /// in the output.
    pub fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        if self.finished {
            return Err(Error::Finished);
        }
        self.ctx.inflate(buf, &mut self.sink, false)
    }

    /// Returns the number of decompressed bytes that have been written to the
    /// output.
    pub fn decompressed_size(&self) -> u64 {
        self.sink.written()
    }

    /// Consumes the stream, flushing any input that may be buffered. Returns
    /// the total number of decompressed bytes written to the output and an
    /// optional checksum if the stream was zlib encoded.
    pub fn finish(mut self) -> Result<(u64, Option<u32>), Error> {
        if self.finished {
            return Err(Error::Finished);
        }
        self.finished = true;
        self.ctx.inflate(&[], &mut self.sink, true)?;
        Ok((self.sink.written(), self.ctx.checksum))
    }
}

impl<S: Sink> Drop for DecoderStream<'_, S> {
    fn drop(&mut self) {
        if !self.finished {
            let _ = self.ctx.inflate(&[], &mut self.sink, true);
            self.finished = true;
        }
    }
}

#[cfg(feature = "std")]
impl<S: Sink> Write for DecoderStream<'_, S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.ctx.inflate(buf, &mut self.sink, false) {
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

/// Decompresses a buffer of the specified format into a vector.
///
/// On success, returns a vector containing the decompressed data and
/// optionally an Adler-32 checksum if the source data was zlib
/// encoded.
pub fn decompress(buf: &[u8], format: Format) -> Result<(Vec<u8>, Option<u32>), Error> {
    let mut decoder = Decoder::new();
    decoder.set_format(format);
    let mut vec = Vec::with_capacity(buf.len() * 2);
    let mut stream = decoder.stream_into_vec(&mut vec);
    stream.write(buf)?;
    let (_, checksum) = stream.finish()?;
    Ok((vec, checksum))
}

struct InflateContext {
    zlib: bool,
    state: State,
    remainder: Remainder,
    pos: usize,
    bit_buffer: u64,
    bits_in: u32,
    trees: Trees,
    checksum: Option<u32>,
    last_block: bool,
    done: bool,
}

impl InflateContext {
    #[inline(always)]
    fn new() -> Self {
        Self {
            zlib: false,
            state: State::Block,
            remainder: Remainder::new(),
            bit_buffer: 0,
            bits_in: 0,
            pos: 0,
            trees: Trees::new(),
            checksum: None,
            last_block: false,
            done: false,
        }
    }

    fn reset(&mut self, zlib: bool) {
        self.zlib = zlib;
        self.state = if zlib { State::Header } else { State::Block };
        self.remainder.pos = 0;
        self.remainder.avail = 0;
        self.pos = 0;
        self.bit_buffer = 0;
        self.bits_in = 0;
        self.checksum = None;
        self.last_block = false;
        self.done = false;
    }

    fn inflate<S: Sink>(
        &mut self,
        mut buf: &[u8],
        sink: &mut S,
        is_last: bool,
    ) -> Result<(), Error> {
        while !self.done && (is_last || !buf.is_empty()) {
            let mut bits = Bits::new(self.bit_buffer, self.bits_in);
            let (res, used_remainder) = if self.remainder.avail != 0 {
                let used = self.remainder.push(buf);
                buf = &buf[used..];
                let mut source = Source::from_remainder(&self.remainder);
                let res = inflate(
                    self.zlib,
                    &mut self.state,
                    &mut self.last_block,
                    &mut self.done,
                    &mut source,
                    &mut bits,
                    &mut self.trees,
                    sink,
                    &mut self.checksum,
                    is_last,
                );
                let source_pos = source.pos;
                self.remainder.pos = source_pos;
                self.remainder.avail -= source_pos;
                (res, true)
            } else {
                let mut source = Source::new(buf);
                let res = inflate(
                    self.zlib,
                    &mut self.state,
                    &mut self.last_block,
                    &mut self.done,
                    &mut source,
                    &mut bits,
                    &mut self.trees,
                    sink,
                    &mut self.checksum,
                    is_last,
                );
                buf = &buf[source.pos..];
                (res, false)
            };
            self.bit_buffer = bits.bit_buffer;
            self.bits_in = bits.bits_in;
            let more_input = !buf.is_empty();
            match res {
                Err(Error::Underflow) => {
                    if is_last && !more_input {
                        return res;
                    } else if !more_input {
                        return Ok(());
                    } else if self.remainder.avail != 0 || !used_remainder {
                        let used = self.remainder.push(buf);
                        buf = &buf[used..];
                    }
                }
                Err(_) => {
                    return res;
                }
                _ => {
                    if is_last {
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum State {
    Header,
    Block,
    Copy(usize),
    Inflate,
    Match(u32),
}

fn inflate<S: Sink>(
    zlib: bool,
    state: &mut State,
    last_block: &mut bool,
    done: &mut bool,
    source: &mut Source,
    bits: &mut Bits,
    trees: &mut Trees,
    sink: &mut S,
    checksum: &mut Option<u32>,
    is_last: bool,
) -> Result<(), Error> {
    loop {
        match *state {
            State::Header => {
                if bits.bytes_available(source) < 2 {
                    return Err(Error::Underflow);
                }
                verify_zlib_header(source, bits)?;
                *state = State::Block;
                continue;
            }
            State::Block => {
                if *last_block {
                    if zlib && checksum.is_none() {
                        bits.skip(bits.bits_in & 7);
                        if bits.bytes_available(source) < 4 {
                            return Err(Error::Underflow);
                        }
                        *checksum = Some(read_zlib_checksum(source, bits)?);
                    }
                    *done = true;
                    return Ok(());
                }
                if bits.bytes_available(source) < 286 && !is_last {
                    return Err(Error::Underflow);
                }
                let header = bits.try_pop_source(source, 3)?;
                *last_block = header & 1 != 0;
                match header >> 1 {
                    0 => {
                        bits.try_skip(bits.bits_in & 7)?;
                        let mut parts = [0u32; 4];
                        for part in &mut parts {
                            if bits.bits_in >= 8 {
                                *part = bits.pop(8);
                            } else {
                                *part = *source
                                    .buffer
                                    .get(source.pos)
                                    .ok_or(Error::InvalidBitstream)?
                                    as u32;
                                source.pos += 1;
                                source.avail -= 1;
                            }
                        }
                        let length = parts[0] | (parts[1] << 8);
                        let inv_length = parts[2] | (parts[3] << 8);
                        if length != (!inv_length & 0xFFFF) {
                            return Err(Error::InvalidBitstream);
                        }
                        let mut remaining = length as usize;
                        while bits.bits_in >= 8 && remaining > 0 {
                            sink.push(bits.pop(8) as u8)?;
                            remaining -= 1;
                        }
                        if bits.bits_in == 0 {
                            bits.bit_buffer = 0;
                        }
                        *state = State::Copy(remaining);
                        while remaining > 0 {
                            let bytes = source.try_get(remaining)?;
                            sink.write(bytes)?;
                            remaining -= bytes.len();
                            *state = State::Copy(remaining);
                        }
                        *state = State::Block;
                        continue;
                    }
                    1 => {
                        const DISTANCE_LENGTHS: [u8; 32] = [5; 32];
                        let mut lengths: [u8; 288] = [0; 288];
                        lengths[0..144].iter_mut().for_each(|p| *p = 8);
                        lengths[144..256].iter_mut().for_each(|p| *p = 9);
                        lengths[256..280].iter_mut().for_each(|p| *p = 7);
                        lengths[280..288].iter_mut().for_each(|p| *p = 8);
                        trees.lt.build(&lengths[..288]);
                        trees.dt.build(&DISTANCE_LENGTHS);
                        *state = State::Inflate;
                        continue;
                    }
                    2 => {
                        decode_trees(source, bits, &mut trees.lt, &mut trees.dt, is_last)?;
                        *state = State::Inflate;
                        continue;
                    }
                    _ => {
                        return Err(Error::InvalidBitstream);
                    }
                }
            }
            State::Copy(mut remaining) => {
                while remaining > 0 {
                    let bytes = source.try_get(remaining)?;
                    sink.write(bytes)?;
                    remaining -= bytes.len();
                    *state = State::Copy(remaining);
                }
                *state = State::Block;
                continue;
            }
            State::Inflate => {
                let mut lbits = *bits;
                let mut entry = 0;
                if !is_last {
                    loop {
                        let mut handle_match = false;
                        while lbits.bits_in >= 15 {
                            entry = trees.lt.table[lbits.peek(LITERAL_LENGTH_TABLE_BITS) as usize];
                            if entry & ENTRY_SUBTABLE != 0 {
                                lbits.skip(LITERAL_LENGTH_TABLE_BITS);
                                entry = trees.lt.table[(((entry >> ENTRY_SHIFT) & 0xFFFF)
                                    + lbits.peek(entry & ENTRY_LENGTH_MASK))
                                    as usize];
                            }
                            lbits.skip(entry & ENTRY_LENGTH_MASK);
                            if entry & ENTRY_LITERAL == 0 {
                                handle_match = true;
                                break;
                            }
                            sink.push((entry >> ENTRY_SHIFT) as u8)?;
                        }
                        if !handle_match {
                            if lbits.fill(source) >= 15 {
                                entry =
                                    trees.lt.table[lbits.peek(LITERAL_LENGTH_TABLE_BITS) as usize];
                                if entry & ENTRY_SUBTABLE != 0 {
                                    lbits.skip(LITERAL_LENGTH_TABLE_BITS);
                                    entry = trees.lt.table[(((entry >> ENTRY_SHIFT) & 0xFFFF)
                                        + lbits.peek(entry & ENTRY_LENGTH_MASK))
                                        as usize];
                                }
                                lbits.skip(entry & ENTRY_LENGTH_MASK);
                                if entry & ENTRY_LITERAL != 0 {
                                    sink.push((entry >> ENTRY_SHIFT) as u8)?;
                                    continue;
                                }
                            } else {
                                *bits = lbits;
                                return Err(Error::Underflow);
                            }
                        }
                        entry >>= ENTRY_SHIFT;
                        if lbits.fill(source) >= 33 {
                            let length = ((entry >> LENGTH_BASE_SHIFT)
                                + lbits.pop(entry & EXTRA_LENGTH_BITS_MASK))
                                as usize;
                            if length == 0 {
                                *bits = lbits;
                                *state = State::Block;
                                break;
                            }
                            entry = trees.dt.table[lbits.peek(DISTANCE_TABLE_BITS) as usize];
                            if entry & ENTRY_SUBTABLE != 0 {
                                lbits.skip(DISTANCE_TABLE_BITS);
                                entry = trees.dt.table[(((entry >> ENTRY_SHIFT) & 0xFFFF)
                                    + lbits.peek(entry & ENTRY_LENGTH_MASK))
                                    as usize];
                            }
                            lbits.skip(entry & ENTRY_LENGTH_MASK);
                            entry >>= ENTRY_SHIFT;
                            let distance = ((entry & DISTANCE_BASE_MASK)
                                + lbits.pop(entry >> EXTRA_DISTANCE_BITS_SHIFT))
                                as usize;
                            sink.apply_match(distance, length)?;
                        } else {
                            *bits = lbits;
                            *state = State::Match(entry);
                            return Err(Error::Underflow);
                        }
                    }
                } else {
                    loop {
                        if lbits.bits_in < 15 {
                            lbits.fill(source);
                        }
                        let mut entry =
                            trees.lt.table[lbits.peek(LITERAL_LENGTH_TABLE_BITS) as usize];
                        if entry & ENTRY_SUBTABLE != 0 {
                            lbits.try_skip(LITERAL_LENGTH_TABLE_BITS)?;
                            entry = trees.lt.table[(((entry >> ENTRY_SHIFT) & 0xFFFF)
                                + lbits.peek(entry & ENTRY_LENGTH_MASK))
                                as usize];
                        }
                        lbits.try_skip(entry & ENTRY_LENGTH_MASK)?;
                        if entry & ENTRY_LITERAL != 0 {
                            sink.push((entry >> ENTRY_SHIFT) as u8)?;
                            continue;
                        }
                        entry >>= ENTRY_SHIFT;
                        lbits.fill(source);
                        let length = ((entry >> LENGTH_BASE_SHIFT)
                            + lbits.try_pop(entry & EXTRA_LENGTH_BITS_MASK)?)
                            as usize;
                        if length == 0 {
                            *bits = lbits;
                            *state = State::Block;
                            break;
                        }
                        entry = trees.dt.table[lbits.peek(DISTANCE_TABLE_BITS) as usize];
                        if entry & ENTRY_SUBTABLE != 0 {
                            lbits.try_skip(DISTANCE_TABLE_BITS)?;
                            entry = trees.dt.table[(((entry >> ENTRY_SHIFT) & 0xFFFF)
                                + lbits.peek(entry & ENTRY_LENGTH_MASK))
                                as usize];
                        }
                        lbits.try_skip(entry & ENTRY_LENGTH_MASK)?;
                        entry >>= ENTRY_SHIFT;
                        let distance = ((entry & DISTANCE_BASE_MASK)
                            + lbits.try_pop(entry >> EXTRA_DISTANCE_BITS_SHIFT)?)
                            as usize;
                        sink.apply_match(distance, length)?;
                    }
                }
            }
            State::Match(mut entry) => {
                let mut lbits = *bits;
                if !is_last {
                    if lbits.fill(source) < 33 {
                        *bits = lbits;
                        return Err(Error::Underflow);
                    }
                    let length = ((entry >> LENGTH_BASE_SHIFT)
                        + lbits.pop(entry & EXTRA_LENGTH_BITS_MASK))
                        as usize;
                    if length == 0 {
                        *bits = lbits;
                        *state = State::Block;
                        continue;
                    }
                    entry = trees.dt.table[lbits.peek(DISTANCE_TABLE_BITS) as usize];
                    if entry & ENTRY_SUBTABLE != 0 {
                        lbits.skip(DISTANCE_TABLE_BITS);
                        entry = trees.dt.table[(((entry >> ENTRY_SHIFT) & 0xFFFF)
                            + lbits.peek(entry & ENTRY_LENGTH_MASK))
                            as usize];
                    }
                    lbits.skip(entry & ENTRY_LENGTH_MASK);
                    entry >>= ENTRY_SHIFT;
                    let distance = ((entry & DISTANCE_BASE_MASK)
                        + lbits.pop(entry >> EXTRA_DISTANCE_BITS_SHIFT))
                        as usize;
                    *bits = lbits;
                    *state = State::Inflate;
                    sink.apply_match(distance, length)?;
                } else {
                    let length = ((entry >> LENGTH_BASE_SHIFT)
                        + lbits.try_pop(entry & EXTRA_LENGTH_BITS_MASK)?)
                        as usize;
                    if length == 0 {
                        *bits = lbits;
                        *state = State::Block;
                        continue;
                    }
                    entry = trees.dt.table[lbits.peek(DISTANCE_TABLE_BITS) as usize];
                    if entry & ENTRY_SUBTABLE != 0 {
                        lbits.try_skip(DISTANCE_TABLE_BITS)?;
                        entry = trees.dt.table[(((entry >> ENTRY_SHIFT) & 0xFFFF)
                            + lbits.peek(entry & ENTRY_LENGTH_MASK))
                            as usize];
                    }
                    lbits.try_skip(entry & ENTRY_LENGTH_MASK)?;
                    entry >>= ENTRY_SHIFT;
                    let distance = ((entry & DISTANCE_BASE_MASK)
                        + lbits.try_pop(entry >> EXTRA_DISTANCE_BITS_SHIFT)?)
                        as usize;
                    *bits = lbits;
                    *state = State::Inflate;
                    sink.apply_match(distance, length)?;
                }
            }
        }
    }
}

fn decode_trees(
    source: &mut Source,
    bits: &mut Bits,
    lt: &mut LiteralLengthTree,
    dt: &mut DistanceTree,
    is_last: bool,
) -> Result<(), Error> {
    let mut lengths: [u8; MAX_LENGTHS] = [0; MAX_LENGTHS];
    bits.fill(source);
    let ltlen;
    let dtlen;
    if !is_last {
        ltlen = bits.pop(5) as usize + 257;
        dtlen = bits.pop(5) as usize + 1;
        let ptlen = bits.pop(4) as usize + 4;
        if ltlen > 286 || dtlen > 30 {
            return Err(Error::InvalidBitstream);
        }
        for length in &mut lengths[0..19] {
            *length = 0;
        }
        bits.fill(source);
        for code in &PRECODE_SWIZZLE[..ptlen] {
            let clen = bits.try_pop_source(source, 3)?;
            lengths[*code as usize] = clen as u8;
        }
        if !lt.build_precode(&lengths[..19]) {
            return Err(Error::InvalidBitstream);
        }
        let mut i = 0;
        while i < (ltlen + dtlen) {
            if bits.bits_in < 7 {
                bits.fill(source);
            }
            let entry = lt.table[bits.peek(7) as usize];
            bits.skip(entry & ENTRY_LENGTH_MASK);
            let presym = entry >> ENTRY_SHIFT;
            if presym < 16 {
                lengths[i] = presym as u8;
                i += 1;
                continue;
            }
            if bits.bits_in < 7 {
                bits.fill(source);
            }
            if presym > 18 || (presym == 16 && i == 0) {
                return Err(Error::InvalidBitstream);
            }
            let (extra_bits, extra) =
                [(2, 3), (3, 3), (7, 11), (0, 0)][(presym as usize - 16) & 0x3];
            let count = bits.pop(extra_bits) as usize + extra;
            let l = if presym == 16 { lengths[i - 1] } else { 0 };
            let p = lengths
                .get_mut(i..i + count)
                .ok_or(Error::InvalidBitstream)?;
            p.iter_mut().for_each(|p| *p = l);
            i += count;
        }
    } else {
        ltlen = bits.try_pop(5)? as usize + 257;
        dtlen = bits.try_pop(5)? as usize + 1;
        let ptlen = bits.try_pop(4)? as usize + 4;
        if ltlen > 286 || dtlen > 30 {
            return Err(Error::InvalidBitstream);
        }
        for length in &mut lengths[0..19] {
            *length = 0;
        }
        bits.fill(source);
        for code in &PRECODE_SWIZZLE[..ptlen] {
            let clen = bits.try_pop_source(source, 3)?;
            lengths[*code as usize] = clen as u8;
        }
        if !lt.build_precode(&lengths[..19]) {
            return Err(Error::InvalidBitstream);
        }
        let mut i = 0;
        while i < (ltlen + dtlen) {
            if bits.bits_in < 7 {
                bits.fill(source);
            }
            let entry = lt.table[bits.peek(7) as usize];
            bits.try_skip(entry & ENTRY_LENGTH_MASK)?;
            let presym = entry >> ENTRY_SHIFT;
            if presym < 16 {
                lengths[i] = presym as u8;
                i += 1;
                continue;
            }
            if bits.bits_in < 7 {
                bits.fill(source);
            }
            if presym > 18 || (presym == 16 && i == 0) {
                return Err(Error::InvalidBitstream);
            }
            let (extra_bits, extra) =
                [(2, 3), (3, 3), (7, 11), (0, 0)][(presym as usize - 16) & 0x3];
            let count = bits.try_pop(extra_bits)? as usize + extra;
            let l = if presym == 16 { lengths[i - 1] } else { 0 };
            let p = lengths
                .get_mut(i..i + count)
                .ok_or(Error::InvalidBitstream)?;
            p.iter_mut().for_each(|p| *p = l);
            i += count;
        }
    }
    if lengths[256] == 0
        || !lt.build(&lengths[..ltlen])
        || !dt.build(&lengths[ltlen..ltlen + dtlen])
    {
        return Err(Error::InvalidBitstream);
    }
    Ok(())
}

fn build_tree(
    table: &mut [u32],
    lengths: &[u8],
    entries: &[u32],
    table_bits: usize,
    max_codeword_len: usize,
) -> bool {
    let mut len_counts = [0usize; MAX_CODE_SIZE + 1];
    let mut offsets = [0usize; MAX_CODE_SIZE + 1];
    let mut sorted_entries: [u32; 288] = [0; 288];
    for &len in lengths {
        len_counts[len as usize] += 1;
    }
    offsets[1] = len_counts[0];
    let mut codespace_used = 0;
    for len in 1..max_codeword_len {
        offsets[len + 1] = offsets[len] + len_counts[len];
        codespace_used = (codespace_used << 1) + len_counts[len];
    }
    codespace_used = (codespace_used << 1) + len_counts[max_codeword_len];
    for sym in 0..lengths.len() {
        let len = lengths[sym];
        let idx = &mut offsets[len as usize];
        sorted_entries[*idx] = entries[sym];
        *idx += 1;
    }
    let sorted_entries = &mut sorted_entries[offsets[0]..];
    if codespace_used > (1 << max_codeword_len) {
        return false;
    }
    if codespace_used < (1 << max_codeword_len) {
        let entry = if codespace_used == 0 {
            entries[0] | 1
        } else {
            if codespace_used != (1 << (max_codeword_len - 1)) || len_counts[1] != 1 {
                return false;
            }
            sorted_entries[0] | 1
        };
        for i in 0..(1 << table_bits) {
            table[i] = entry;
        }
        return true;
    }
    let mut len = 1;
    let mut count;
    loop {
        count = len_counts[len & 15];
        if count != 0 {
            break;
        }
        len += 1;
    }
    let mut codeword = 0;
    let mut cur_table_end = 1 << len;
    let mut s = 0;
    while len <= table_bits {
        loop {
            table[codeword] = sorted_entries[s] | len as u32;
            s += 1;
            if codeword == cur_table_end - 1 {
                while len < table_bits {
                    table.copy_within(0..cur_table_end, cur_table_end);
                    cur_table_end <<= 1;
                    len += 1;
                }
                return true;
            }
            let bit = 1 << (31 - ((codeword ^ (cur_table_end - 1)) as u32).leading_zeros());
            codeword &= bit - 1;
            codeword |= bit;
            count -= 1;
            if count == 0 {
                break;
            }
        }
        loop {
            len += 1;
            if len <= table_bits {
                table.copy_within(0..cur_table_end, cur_table_end);
                cur_table_end <<= 1;
            }
            count = len_counts[len & 15];
            if count != 0 {
                break;
            }
        }
    }
    cur_table_end = 1 << table_bits;
    let mut subtable_prefix = !0;
    let mut subtable_start = 0;
    loop {
        if (codeword & ((1 << table_bits) - 1)) != subtable_prefix {
            subtable_prefix = codeword & ((1 << table_bits) - 1);
            subtable_start = cur_table_end;
            let mut subtable_bits = len - table_bits;
            codespace_used = count;
            while codespace_used < (1 << subtable_bits) {
                subtable_bits += 1;
                codespace_used = (codespace_used << 1) + len_counts[table_bits + subtable_bits];
            }
            cur_table_end = subtable_start + (1 << subtable_bits);
            table[subtable_prefix] =
                ENTRY_SUBTABLE | (subtable_start << 8) as u32 | subtable_bits as u32;
        }
        let entry = sorted_entries[s] | (len - table_bits) as u32;
        s += 1;
        let mut i = subtable_start + (codeword >> table_bits);
        let stride = 1 << (len - table_bits);
        loop {
            table[i] = entry;
            i += stride;
            if i >= cur_table_end {
                break;
            }
        }
        if codeword == (1 << len) - 1 {
            return true;
        }
        let bit = 1 << (31 - ((codeword ^ ((1 << len) - 1)) as u32).leading_zeros());
        codeword &= bit - 1;
        codeword |= bit;
        count -= 1;
        while count == 0 {
            len += 1;
            count = len_counts[len & 15];
        }
    }
}

fn verify_zlib_header(source: &mut Source, bits: &mut Bits) -> Result<(), Error> {
    let cmf = bits.try_pop_source(source, 8)?;
    let flg = bits.try_pop_source(source, 8)?;
    if (256 * cmf + flg) % 31 != 0 || cmf & 0x0F != 8 || (cmf >> 4) > 7 || flg & 0x20 != 0 {
        return Err(Error::InvalidBitstream);
    }
    Ok(())
}

fn read_zlib_checksum(source: &mut Source, bits: &mut Bits) -> Result<u32, Error> {
    let mut parts = [0u32; 4];
    for part in &mut parts {
        *part = bits.try_pop_source(source, 8)?;
    }
    Ok((parts[0] << 24) | (parts[1] << 16) | (parts[2] << 8) | parts[3])
}

struct Trees {
    lt: LiteralLengthTree,
    dt: DistanceTree,
}

impl Trees {
    #[inline(always)]
    fn new() -> Self {
        Self {
            lt: LiteralLengthTree::new(),
            dt: DistanceTree::new(),
        }
    }
}

struct Remainder {
    buffer: [u8; 286],
    pos: usize,
    avail: usize,
}

impl Remainder {
    fn new() -> Self {
        Self {
            buffer: [0; 286],
            pos: 0,
            avail: 0,
        }
    }

    fn push(&mut self, buf: &[u8]) -> usize {
        if self.pos != 0 {
            self.buffer.copy_within(self.pos..self.pos + self.avail, 0);
            self.pos = 0;
        }
        let extra = self.buffer.len() - self.avail;
        let copy_len = extra.min(buf.len());
        self.buffer[self.avail..self.avail + copy_len].copy_from_slice(&buf[0..copy_len]);
        self.avail += copy_len;
        copy_len
    }
}

struct Source<'a> {
    buffer: &'a [u8],
    pos: usize,
    avail: usize,
}

impl<'a> Source<'a> {
    fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            pos: 0,
            avail: buffer.len(),
        }
    }

    fn from_remainder(remainder: &'a Remainder) -> Self {
        Self {
            buffer: &remainder.buffer[remainder.pos..remainder.pos + remainder.avail],
            pos: 0,
            avail: remainder.avail,
        }
    }

    fn try_get(&mut self, len: usize) -> Result<&[u8], Error> {
        let bytes = self.get(len);
        if bytes.is_empty() {
            return Err(Error::Underflow);
        }
        Ok(bytes)
    }

    #[inline(always)]
    fn get(&mut self, len: usize) -> &[u8] {
        let len = len.min(self.avail);
        let pos = self.pos;
        let bytes = &self.buffer[pos..pos + len];
        self.pos += len;
        self.avail -= len;
        bytes
    }
}

#[derive(Copy, Clone)]
struct Bits {
    bit_buffer: u64,
    bits_in: u32,
}

impl Bits {
    fn new(bit_buffer: u64, bits_in: u32) -> Self {
        Self {
            bit_buffer,
            bits_in,
        }
    }

    fn bytes_available(&self, source: &Source) -> usize {
        source.avail + (self.bits_in as usize / 8)
    }

    #[inline(always)]
    fn fill(&mut self, source: &mut Source) -> u32 {
        let count = (64 - self.bits_in as usize) >> 3;
        let bytes = source.get(count);
        let len = bytes.len();
        let mut i = 0;
        while (i + 4) <= len {
            use core::convert::TryInto;
            let v = u32::from_le_bytes((&bytes[i..i + 4]).try_into().unwrap()) as u64;
            self.bit_buffer |= v << self.bits_in;
            self.bits_in += 32;
            i += 4;
        }
        while i < len {
            self.bit_buffer |= (bytes[i] as u64) << self.bits_in;
            self.bits_in += 8;
            i += 1;
        }
        self.bits_in
    }

    #[inline(always)]
    fn try_pop_source(&mut self, source: &mut Source, len: u32) -> Result<u32, Error> {
        if self.bits_in < len && self.fill(source) < len {
            return Err(Error::Underflow);
        }
        let bits = self.bit_buffer & ((1 << len) - 1);
        self.bit_buffer >>= len;
        self.bits_in -= len;
        Ok(bits as u32)
    }

    #[inline(always)]
    fn try_pop(&mut self, len: u32) -> Result<u32, Error> {
        if self.bits_in < len {
            return Err(Error::Underflow);
        }
        let bits = self.bit_buffer & ((1 << len) - 1);
        self.bit_buffer >>= len;
        self.bits_in -= len;
        Ok(bits as u32)
    }

    #[inline(always)]
    fn try_skip(&mut self, len: u32) -> Result<(), Error> {
        if self.bits_in < len {
            return Err(Error::Underflow);
        }
        self.bit_buffer >>= len;
        self.bits_in -= len;
        Ok(())
    }

    #[inline(always)]
    fn peek(&mut self, len: u32) -> u32 {
        (self.bit_buffer & ((1 << len) - 1)) as u32
    }

    #[inline(always)]
    fn pop(&mut self, len: u32) -> u32 {
        let bits = self.bit_buffer & ((1 << len) - 1);
        self.bit_buffer >>= len;
        self.bits_in -= len;
        bits as u32
    }

    #[inline(always)]
    fn skip(&mut self, len: u32) {
        self.bit_buffer >>= len;
        self.bits_in -= len;
    }
}

#[inline(always)]
fn copy_match(buf: &mut [u8], pos: usize, len: usize, buf_end: usize) {
    let dist = buf_end - pos;
    if dist > len {
        buf.copy_within(pos..pos + len, buf_end);
    } else {
        for i in 0..len {
            buf[buf_end + i] = buf[pos + i];
        }
    }
}

#[doc(hidden)]
pub trait Sink {
    fn written(&self) -> u64;
    fn push(&mut self, byte: u8) -> Result<(), Error>;
    fn write(&mut self, bytes: &[u8]) -> Result<(), Error>;
    fn apply_match(&mut self, dist: usize, len: usize) -> Result<(), Error>;
}

struct VecSink<'a> {
    buffer: &'a mut Vec<u8>,
    start_pos: usize,
    pos: usize,
}

impl<'a> VecSink<'a> {
    fn new(buffer: &'a mut Vec<u8>) -> Self {
        let start_pos = buffer.len();
        Self {
            buffer,
            start_pos,
            pos: start_pos,
        }
    }
}

impl Drop for VecSink<'_> {
    fn drop(&mut self) {
        self.buffer.truncate(self.pos);
    }
}

impl Sink for VecSink<'_> {
    fn written(&self) -> u64 {
        (self.pos - self.start_pos) as u64
    }

    #[inline(always)]
    fn push(&mut self, byte: u8) -> Result<(), Error> {
        self.buffer.push(byte);
        self.pos += 1;
        Ok(())
    }

    fn write(&mut self, bytes: &[u8]) -> Result<(), Error> {
        let len = bytes.len();
        self.buffer.extend_from_slice(bytes);
        self.pos += len;
        Ok(())
    }

    #[inline(always)]
    fn apply_match(&mut self, dist: usize, len: usize) -> Result<(), Error> {
        let buf_len = self.pos - self.start_pos;
        if dist > buf_len {
            return Err(Error::InvalidBitstream);
        }
        let pos = self.pos - dist;
        self.buffer.resize(self.pos + len, 0);
        copy_match(self.buffer, pos, len, self.pos);
        self.pos += len;
        Ok(())
    }
}

struct BufSink<'a> {
    buffer: &'a mut [u8],
    pos: usize,
}

impl Sink for BufSink<'_> {
    fn written(&self) -> u64 {
        self.pos as u64
    }

    #[inline(always)]
    fn push(&mut self, byte: u8) -> Result<(), Error> {
        if self.pos < self.buffer.len() {
            self.buffer[self.pos] = byte;
            self.pos += 1;
            Ok(())
        } else {
            Err(Error::Overflow)
        }
    }

    fn write(&mut self, bytes: &[u8]) -> Result<(), Error> {
        let len = bytes.len();
        if self.pos + len <= self.buffer.len() {
            self.buffer[self.pos..self.pos + len].copy_from_slice(bytes);
            self.pos += len;
            Ok(())
        } else {
            Err(Error::Overflow)
        }
    }

    #[inline(always)]
    fn apply_match(&mut self, dist: usize, len: usize) -> Result<(), Error> {
        if dist > self.pos {
            return Err(Error::InvalidBitstream);
        }
        if self.pos + len > self.buffer.len() {
            return Err(Error::Overflow);
        }
        let pos = self.pos - dist;
        copy_match(self.buffer, pos, len, self.pos);
        self.pos += len;
        Ok(())
    }
}

#[cfg(feature = "std")]
struct WriterSink<W> {
    writer: W,
    ring: RingBuffer,
    written: u64,
}

#[cfg(feature = "std")]
impl<W: Write> Sink for WriterSink<W> {
    fn written(&self) -> u64 {
        self.written
    }

    #[inline]
    fn push(&mut self, byte: u8) -> Result<(), Error> {
        self.ring.push(byte);
        self.written += 1;
        match self.writer.write_all(&[byte]) {
            Err(err) => Err(Error::Io(err)),
            Ok(_) => Ok(()),
        }
    }

    fn write(&mut self, bytes: &[u8]) -> Result<(), Error> {
        for &b in bytes {
            self.ring.push(b);
        }
        self.written += bytes.len() as u64;
        match self.writer.write_all(bytes) {
            Err(err) => Err(Error::Io(err)),
            Ok(_) => Ok(()),
        }
    }

    #[inline]
    fn apply_match(&mut self, dist: usize, len: usize) -> Result<(), Error> {
        if dist > self.ring.len {
            return Err(Error::InvalidBitstream);
        }
        let pos = self.ring.len - dist;
        for i in 0..len {
            self.push(self.ring.get(pos + i))?;
        }
        Ok(())
    }
}

#[cfg(feature = "std")]
struct RingBuffer {
    buffer: [u8; RING_BUFFER_SIZE],
    len: usize,
}

#[cfg(feature = "std")]
impl RingBuffer {
    #[inline(always)]
    fn new() -> Self {
        Self {
            buffer: [0; RING_BUFFER_SIZE],
            len: 0,
        }
    }

    #[inline(always)]
    fn push(&mut self, value: u8) {
        self.buffer[self.len & (RING_BUFFER_SIZE - 1)] = value;
        self.len += 1;
    }

    #[inline(always)]
    fn get(&self, index: usize) -> u8 {
        self.buffer[index & (RING_BUFFER_SIZE - 1)]
    }
}

struct LiteralLengthTree {
    table: [u32; LITERAL_LENGTH_TREE_SIZE],
}

impl LiteralLengthTree {
    #[inline(always)]
    fn new() -> Self {
        Self {
            table: [0; LITERAL_LENGTH_TREE_SIZE],
        }
    }

    fn build(&mut self, lengths: &[u8]) -> bool {
        build_tree(&mut self.table, lengths, &LITERAL_LENGTH_ENTRIES, 10, 15)
    }

    fn build_precode(&mut self, lengths: &[u8]) -> bool {
        build_tree(&mut self.table, &lengths[..19], &PRECODE_ENTRIES, 7, 7)
    }
}

struct DistanceTree {
    table: [u32; DISTANCE_TREE_SIZE],
}

impl DistanceTree {
    #[inline(always)]
    fn new() -> Self {
        Self {
            table: [0; DISTANCE_TREE_SIZE],
        }
    }

    fn build(&mut self, lengths: &[u8]) -> bool {
        build_tree(&mut self.table, lengths, &DISTANCE_ENTRIES, 8, 15)
    }
}

#[cfg(feature = "std")]
const RING_BUFFER_SIZE: usize = 32768;
const LITERAL_LENGTH_TREE_SIZE: usize = 1334;
const DISTANCE_TREE_SIZE: usize = 402;
const MAX_CODE_SIZE: usize = 15;
const MAX_LENGTHS: usize = 288 + 32;
const ENTRY_LITERAL: u32 = 0x40000000;
const ENTRY_SUBTABLE: u32 = 0x80000000;
const ENTRY_LENGTH_MASK: u32 = 0xFF;
const ENTRY_SHIFT: u32 = 8;
const LITERAL_LENGTH_TABLE_BITS: u32 = 10;
const DISTANCE_TABLE_BITS: u32 = 8;
const EXTRA_LENGTH_BITS_MASK: u32 = 0xFF;
const LENGTH_BASE_SHIFT: u32 = 8;
const EXTRA_DISTANCE_BITS_SHIFT: u32 = 16;
const DISTANCE_BASE_MASK: u32 = (1 << EXTRA_DISTANCE_BITS_SHIFT) - 1;

const PRECODE_SWIZZLE: [u8; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

const PRECODE_ENTRIES: [u32; 19] = [
    0x00000000, 0x00000100, 0x00000200, 0x00000300, 0x00000400, 0x00000500, 0x00000600, 0x00000700,
    0x00000800, 0x00000900, 0x00000A00, 0x00000B00, 0x00000C00, 0x00000D00, 0x00000E00, 0x00000F00,
    0x00001000, 0x00001100, 0x00001200,
];

const LITERAL_LENGTH_ENTRIES: [u32; 288] = [
    0x40000000, 0x40000100, 0x40000200, 0x40000300, 0x40000400, 0x40000500, 0x40000600, 0x40000700,
    0x40000800, 0x40000900, 0x40000A00, 0x40000B00, 0x40000C00, 0x40000D00, 0x40000E00, 0x40000F00,
    0x40001000, 0x40001100, 0x40001200, 0x40001300, 0x40001400, 0x40001500, 0x40001600, 0x40001700,
    0x40001800, 0x40001900, 0x40001A00, 0x40001B00, 0x40001C00, 0x40001D00, 0x40001E00, 0x40001F00,
    0x40002000, 0x40002100, 0x40002200, 0x40002300, 0x40002400, 0x40002500, 0x40002600, 0x40002700,
    0x40002800, 0x40002900, 0x40002A00, 0x40002B00, 0x40002C00, 0x40002D00, 0x40002E00, 0x40002F00,
    0x40003000, 0x40003100, 0x40003200, 0x40003300, 0x40003400, 0x40003500, 0x40003600, 0x40003700,
    0x40003800, 0x40003900, 0x40003A00, 0x40003B00, 0x40003C00, 0x40003D00, 0x40003E00, 0x40003F00,
    0x40004000, 0x40004100, 0x40004200, 0x40004300, 0x40004400, 0x40004500, 0x40004600, 0x40004700,
    0x40004800, 0x40004900, 0x40004A00, 0x40004B00, 0x40004C00, 0x40004D00, 0x40004E00, 0x40004F00,
    0x40005000, 0x40005100, 0x40005200, 0x40005300, 0x40005400, 0x40005500, 0x40005600, 0x40005700,
    0x40005800, 0x40005900, 0x40005A00, 0x40005B00, 0x40005C00, 0x40005D00, 0x40005E00, 0x40005F00,
    0x40006000, 0x40006100, 0x40006200, 0x40006300, 0x40006400, 0x40006500, 0x40006600, 0x40006700,
    0x40006800, 0x40006900, 0x40006A00, 0x40006B00, 0x40006C00, 0x40006D00, 0x40006E00, 0x40006F00,
    0x40007000, 0x40007100, 0x40007200, 0x40007300, 0x40007400, 0x40007500, 0x40007600, 0x40007700,
    0x40007800, 0x40007900, 0x40007A00, 0x40007B00, 0x40007C00, 0x40007D00, 0x40007E00, 0x40007F00,
    0x40008000, 0x40008100, 0x40008200, 0x40008300, 0x40008400, 0x40008500, 0x40008600, 0x40008700,
    0x40008800, 0x40008900, 0x40008A00, 0x40008B00, 0x40008C00, 0x40008D00, 0x40008E00, 0x40008F00,
    0x40009000, 0x40009100, 0x40009200, 0x40009300, 0x40009400, 0x40009500, 0x40009600, 0x40009700,
    0x40009800, 0x40009900, 0x40009A00, 0x40009B00, 0x40009C00, 0x40009D00, 0x40009E00, 0x40009F00,
    0x4000A000, 0x4000A100, 0x4000A200, 0x4000A300, 0x4000A400, 0x4000A500, 0x4000A600, 0x4000A700,
    0x4000A800, 0x4000A900, 0x4000AA00, 0x4000AB00, 0x4000AC00, 0x4000AD00, 0x4000AE00, 0x4000AF00,
    0x4000B000, 0x4000B100, 0x4000B200, 0x4000B300, 0x4000B400, 0x4000B500, 0x4000B600, 0x4000B700,
    0x4000B800, 0x4000B900, 0x4000BA00, 0x4000BB00, 0x4000BC00, 0x4000BD00, 0x4000BE00, 0x4000BF00,
    0x4000C000, 0x4000C100, 0x4000C200, 0x4000C300, 0x4000C400, 0x4000C500, 0x4000C600, 0x4000C700,
    0x4000C800, 0x4000C900, 0x4000CA00, 0x4000CB00, 0x4000CC00, 0x4000CD00, 0x4000CE00, 0x4000CF00,
    0x4000D000, 0x4000D100, 0x4000D200, 0x4000D300, 0x4000D400, 0x4000D500, 0x4000D600, 0x4000D700,
    0x4000D800, 0x4000D900, 0x4000DA00, 0x4000DB00, 0x4000DC00, 0x4000DD00, 0x4000DE00, 0x4000DF00,
    0x4000E000, 0x4000E100, 0x4000E200, 0x4000E300, 0x4000E400, 0x4000E500, 0x4000E600, 0x4000E700,
    0x4000E800, 0x4000E900, 0x4000EA00, 0x4000EB00, 0x4000EC00, 0x4000ED00, 0x4000EE00, 0x4000EF00,
    0x4000F000, 0x4000F100, 0x4000F200, 0x4000F300, 0x4000F400, 0x4000F500, 0x4000F600, 0x4000F700,
    0x4000F800, 0x4000F900, 0x4000FA00, 0x4000FB00, 0x4000FC00, 0x4000FD00, 0x4000FE00, 0x4000FF00,
    0x00000000, 0x00030000, 0x00040000, 0x00050000, 0x00060000, 0x00070000, 0x00080000, 0x00090000,
    0x000A0000, 0x000B0100, 0x000D0100, 0x000F0100, 0x00110100, 0x00130200, 0x00170200, 0x001B0200,
    0x001F0200, 0x00230300, 0x002B0300, 0x00330300, 0x003B0300, 0x00430400, 0x00530400, 0x00630400,
    0x00730400, 0x00830500, 0x00A30500, 0x00C30500, 0x00E30500, 0x01020000, 0x01020000, 0x01020000,
];

const DISTANCE_ENTRIES: [u32; 32] = [
    0x00000100, 0x00000200, 0x00000300, 0x00000400, 0x01000500, 0x01000700, 0x02000900, 0x02000D00,
    0x03001100, 0x03001900, 0x04002100, 0x04003100, 0x05004100, 0x05006100, 0x06008100, 0x0600C100,
    0x07010100, 0x07018100, 0x08020100, 0x08030100, 0x09040100, 0x09060100, 0x0A080100, 0x0A0C0100,
    0x0B100100, 0x0B180100, 0x0C200100, 0x0C300100, 0x0D400100, 0x0D600100, 0x0E800100, 0x0EC00100,
];
