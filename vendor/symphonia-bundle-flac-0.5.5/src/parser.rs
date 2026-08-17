// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::checksum::Crc16Ansi;
use symphonia_core::errors::Result;
use symphonia_core::formats::Packet;
use symphonia_core::io::{BufReader, Monitor, ReadBytes, SeekBuffered};
use symphonia_core::util::bits;
use symphonia_utils_xiph::flac::metadata::StreamInfo;

use log::warn;

use crate::frame::*;

struct MovingAverage<const N: usize> {
    samples: [usize; N],
    count: usize,
}

impl<const N: usize> MovingAverage<N> {
    /// Push a value onto the moving average filter.
    pub fn push(&mut self, value: usize) {
        self.samples[self.count % N] = value;
        self.count += 1;
    }

    /// Reset the moving average filter to 0.
    pub fn reset(&mut self) {
        self.count = 0;
    }

    /// Calculate the moving average.
    pub fn average(&self) -> usize {
        if self.count >= N {
            // If greater-than N values were pushed, then all samples must be averaged.
            self.samples.iter().sum::<usize>() / N
        }
        else if self.count > 0 {
            // If less-than N values were pushed, then only the first 0..N samples need to be
            // averaged.
            self.samples.iter().take(self.count).sum::<usize>() / self.count
        }
        else {
            // No samples.
            0
        }
    }
}

impl<const N: usize> Default for MovingAverage<N> {
    fn default() -> Self {
        Self { samples: [0; N], count: 0 }
    }
}

/// Frame synchronization information.
pub struct SyncInfo {
    /// The timestamp of the first audio frame in the packet.
    pub ts: u64,
    /// The number of audio frames in the packet.
    pub dur: u64,
}

/// A parsed packet.
struct ParsedPacket {
    /// The packet data.
    buf: Box<[u8]>,
    /// The packet's synchronization information.
    sync: SyncInfo,
}

/// A fragment footer.
struct FragmentFooter {
    crc: u16,
}

impl FragmentFooter {
    fn new(buf: &[u8]) -> Self {
        let mut crc = [0; 2];
        crc[0..2].copy_from_slice(buf);

        FragmentFooter { crc: u16::from_be_bytes(crc) }
    }
}

/// A FLAC packet fragment state tracker.
struct FragmentState {
    /// A running total CRC16 of a packet if it started at the first byte of this fragment.
    crc16: Crc16Ansi,
    /// The running total size of a packet if it started at the first byte of this fragment.
    total_len: usize,
}

/// A FLAC packet fragment.
struct Fragment {
    /// The fragment data.
    data: Box<[u8]>,
    /// The footer. If the fragment contains a footer (the fragment is either a whole packet, or
    /// the last fragment of a packet), then the footer contains a valid CRC16 for the packet.
    footer: FragmentFooter,
    /// True if the CRC16 of this fragment, excluding the footer, matches the CRC16 in the footer.
    crc_match: bool,
    /// The running fragment state.
    state: FragmentState,
}

impl Fragment {
    /// Create a new packet fragment with the given buffer.
    fn new(data: Box<[u8]>) -> Self {
        let total_len = data.len();

        let (top, bottom) = data.split_at(total_len - 2);

        let footer = FragmentFooter::new(bottom);

        let mut crc16 = Crc16Ansi::new(0);
        crc16.process_buf_bytes(top);
        let crc_match = footer.crc == crc16.crc();
        crc16.process_buf_bytes(bottom);

        Self { data, footer, crc_match, state: FragmentState { crc16, total_len } }
    }

    /// Append the buffer to the CRC.
    fn update(&mut self, frag: &Fragment) -> bool {
        let len = frag.data.len();

        let (top, bottom) = frag.data.split_at(len - 2);

        self.state.crc16.process_buf_bytes(top);
        let crc_match = frag.footer.crc == self.state.crc16.crc();
        self.state.crc16.process_buf_bytes(bottom);

        self.state.total_len += len;

        crc_match
    }

    /// Parse the frame header from the fragment.
    fn parse_header(&self) -> FrameHeader {
        let mut reader = BufReader::new(&self.data);
        let sync = reader.read_be_u16().unwrap();
        read_frame_header(&mut reader, sync).unwrap()
    }
}

#[derive(Default)]
struct PacketBuilder {
    /// Queue of fragments to merged to form a packet.
    frags: Vec<Fragment>,
    /// The maximum allowed size for a packet. This is the maximum frame size stated in the stream
    /// info header, if provided.
    max_size: Option<usize>,
    /// The average size of a packet.
    avg_size: Option<usize>,
    /// The last valid header,
    last_header: Option<FrameHeader>,
}

impl PacketBuilder {
    fn set_max_frame_size(&mut self, max_size: Option<usize>) {
        self.max_size = max_size;
    }

    fn get_max_frame_size(&self) -> usize {
        self.max_size.unwrap_or(FLAC_MAX_FRAME_SIZE)
    }

    fn set_avg_frame_size(&mut self, avg_size: Option<usize>) {
        self.avg_size = avg_size;
    }

    fn get_max_avg_frame_size(&self) -> usize {
        self.avg_size.map(|s| 4 * s).unwrap_or(FLAC_MAX_FRAME_SIZE)
    }

    fn last_header(&self) -> Option<&FrameHeader> {
        self.last_header.as_ref()
    }

    fn push_fragment(&mut self, frag: Fragment) {
        // Prune the fragment queue to guard against unbounded growth.
        if let Some(first) = self.frags.first() {
            // Pruning heuristics:
            //
            // 1) If the stream information block defines a maximum frame size, do not exceed that
            //    frame size.
            //
            // 2) If a maximum frame size is not provided, a frame may never exceed 16MB as per
            //    the FLAC specification.
            //
            // 3) If a maximum frame size is not established, but an average frame size size has
            //    been determined, do not exceed it by a factor of 4.
            //
            // 4) If the fragment would have a depth > 4 after the new fragment is pushed.
            let prune = if first.state.total_len > self.get_max_frame_size() {
                warn!(
                    "dropping fragment: packet would exceed maximum size of {} bytes",
                    self.get_max_avg_frame_size()
                );
                true
            }
            else if first.state.total_len > self.get_max_avg_frame_size() {
                warn!(
                    "dropping fragment: packet would exeed 4x average historical size of {} bytes",
                    self.get_max_avg_frame_size()
                );
                true
            }
            else if self.frags.len() >= 4 {
                warn!("dropping fragment: packet would exceed fragment count limit");
                true
            }
            else {
                false
            };

            if prune {
                self.frags.remove(0);
            }
        }

        // debug!("saving fragment {}: len={}", self.frags.len(), frag.state.total_len);

        self.frags.push(frag);
    }

    fn try_build(&mut self, stream_info: &StreamInfo, frag: Fragment) -> Option<ParsedPacket> {
        let (header, data) = if frag.crc_match {
            // The fragment has a CRC that matches the expected CRC.
            (frag.parse_header(), frag.data)
        }
        else {
            // The fragment does not have a CRC that matches the expected CRC.
            //
            // For each existing fragment, update its running CRC with the payload of the new
            // fragment. If an exisiting fragment, denoted as F, after having it's running CRC
            // updated matches the expected CRC of the new fragment. Then all fragments preceeding F
            // are discarded, and all fragments from F up-to and including the new fragment are
            // merged to form a packet.
            let start = self.frags.iter_mut().position(|f| f.update(&frag));

            if let Some(i) = start {
                // A range of fragments has been found that forms a packet.
                let total_len = self.frags[i].state.total_len;

                // debug!("merging {} fragments: total_len={}", self.frags.len() - i + 1, total_len);

                // Merge fragment data buffers.
                let mut data = Vec::with_capacity(total_len);

                for f in self.frags[i..].iter() {
                    data.extend_from_slice(&f.data);
                }

                data.extend_from_slice(&frag.data);

                (self.frags[i].parse_header(), data.into_boxed_slice())
            }
            else {
                // A range of fragments has not been found that forms a packet.
                self.push_fragment(frag);

                return None;
            }
        };

        // Drop all existing fragments.
        self.frags.clear();

        let sync = calc_sync_info(stream_info, &header);

        self.last_header = Some(header);

        Some(ParsedPacket { buf: data, sync })
    }

    fn reset(&mut self) {
        self.frags.clear();
        self.last_header = None;
    }
}

#[derive(Default)]
pub struct PacketParser {
    /// Stream information.
    info: StreamInfo,
    /// Frame size moving average.
    fsma: MovingAverage<4>,
    /// Packet builder.
    builder: PacketBuilder,
}

impl PacketParser {
    /// Perform a soft reset of the parser. Call this after a discontinuity in the stream.
    fn soft_reset(&mut self) {
        self.builder.reset();
        self.fsma.reset();
    }

    /// Tries to read a fragment upto the maximum size of a FLAC frame using the reader and returns
    /// it. If a fragment cannot be read, then the reader has lost synchronization and must be
    /// resynchronized.
    fn try_read_fragment<B>(
        &self,
        reader: &mut B,
        avg_frame_size: usize,
    ) -> Result<Option<Fragment>>
    where
        B: ReadBytes + SeekBuffered,
    {
        // The initial number of bytes to read.
        //
        // For this we use the average frame size clamped to a reasonable size with an added bias of
        // the maximum frame header size so that if the frame size is constant we don't do a second
        // read to synchronize to the next frame header.
        let init_read_size = avg_frame_size.clamp(1024, 32768) + FLAC_MAX_FRAME_HEADER_SIZE;

        // Buffer in which the fragment will be read.
        let mut buf: Vec<u8> = vec![0; init_read_size];

        // Do the initial read.
        //
        // Note: This will always read atleast a single byte, or return an error (i.e., EOF).
        let mut end = reader.read_buf(&mut buf)?;

        // Invariant: The packet parser was synchronized before starting to read_fragment.
        //
        // If the previous call left the reader on a fragment boundary then we need to skip past one
        // byte so we don't get stuck synchronizing to the same frame header it synchronized to.
        //
        // If the last call could not fully read to the next frame header (due to IO errors, etc.),
        // then by definition there is more data to read belonging the previous call's fragment.
        // Therefore, the next byte should not be a new frame header so it is safe to also skip the
        // first byte.
        let mut pos = 1;

        // Read until the next frame header is found, or an IO error such as EOF.
        let size = 'found: loop {
            // Find the next frame header. Start by searching for the sync preamble.
            while let Some((offset, sync)) = scan_for_sync_preamble(&buf[pos..end]) {
                let size = pos + offset;

                let frame = &buf[size..];

                // If the frame buffer passes a quick sanity check, then attempt to parse the
                // frame header in its entirety.
                if is_likely_frame_header(frame) {
                    // Parse the frame header from the frame buffer.
                    if let Ok(header) = read_frame_header(&mut BufReader::new(&frame[2..]), sync) {
                        // Get the last header to check monotonicity in the strict header check.
                        let last_header = self.builder.last_header();

                        // Perform a strict header check.
                        if strict_frame_header_check(&self.info, &header, last_header) {
                            // Rewind the reader such that the next read will start on the sync
                            // word of the frame header.
                            reader.seek_buffered_rev(end - size);

                            break 'found size;
                        }
                    }
                }

                // Continue scanning one byte after the false-positive preamble.
                pos += offset + 1;
            }

            // If enough data has been read such even a FLAC frame of the maximum size should've
            // been fully read, and the header for the next frame found, then synchronization has
            // been lost.
            if end >= FLAC_MAX_FRAME_SIZE + FLAC_MAX_FRAME_HEADER_SIZE {
                return Ok(None);
            }

            // Calculate the required buffer size after reading a new 1kB chunk of data, and grow
            // the buffer if necessary.
            let next_read_end = end + 1024;

            if next_read_end > buf.len() {
                buf.resize(next_read_end, 0);
            }

            // After reading a new chunk of data, reconsider up-to 16 bytes (the maximum FLAC frame
            // header size) of old data such that if a frame header was partially read in the last
            // iteration it will be considered again in the next iteration.
            pos = end.saturating_sub(FLAC_MAX_FRAME_HEADER_SIZE);

            // Read the new chunk.
            end += match reader.read_buf(&mut buf[end..next_read_end]) {
                Ok(read) => read,
                Err(_) => break 'found end,
            }
        };

        // trace!(
        //     "read fragment: len={: >5}, avg_frame_size={: >5}, init_read_size={: >5}, discard={: >5}",
        //     size,
        //     avg_frame_size,
        //     init_read_size,
        //     end - size
        // );

        // Truncate the buffer at the start of the new frame header.
        buf.truncate(size);

        Ok(Some(Fragment::new(buf.into_boxed_slice())))
    }

    /// Reads a fragment using the reader and performs resynchronization when necessary.
    fn read_fragment<B>(&mut self, reader: &mut B, avg_frame_size: usize) -> Result<Fragment>
    where
        B: ReadBytes + SeekBuffered,
    {
        loop {
            if let Some(fragment) = self.try_read_fragment(reader, avg_frame_size)? {
                return Ok(fragment);
            }

            // If a fragment could not be read, synchronization was lost. Try to resync.
            warn!("synchronization lost");
            let _ = self.resync(reader)?;
        }
    }

    /// Parse the next packet from the stream.
    pub fn parse<B>(&mut self, reader: &mut B) -> Result<Packet>
    where
        B: ReadBytes + SeekBuffered,
    {
        let avg_frame_size = self.fsma.average();

        // Update the packet builder with the latest average frame size.
        self.builder.set_avg_frame_size(Some(avg_frame_size));

        // Build a packet.
        let parsed = loop {
            let fragment = self.read_fragment(reader, avg_frame_size)?;

            if let Some(packet) = self.builder.try_build(&self.info, fragment) {
                break packet;
            }
        };

        // Update the frame size moving average.
        self.fsma.push(parsed.buf.len());

        Ok(Packet::new_from_boxed_slice(0, parsed.sync.ts, parsed.sync.dur, parsed.buf))
    }

    /// Resync the reader to the start of the next frame.
    pub fn resync<B>(&mut self, reader: &mut B) -> Result<SyncInfo>
    where
        B: ReadBytes + SeekBuffered,
    {
        let init_pos = reader.pos();

        let mut frame_pos;

        let header = loop {
            let sync = sync_frame(reader)?;

            frame_pos = reader.pos() - 2;

            if let Ok(header) = read_frame_header(reader, sync) {
                // Do a strict frame header check with no previous header.
                if strict_frame_header_check(&self.info, &header, None) {
                    break header;
                }
            }

            // If the header check failed, then seek to one byte past the start of the false frame
            // and continue trying to resynchronize.
            reader.seek_buffered(frame_pos + 1);
        };

        let sync = calc_sync_info(&self.info, &header);

        // Rewind reader back to the start of the frame.
        reader.seek_buffered(frame_pos);

        // If the reader was moved, soft reset the parser.
        if init_pos != reader.pos() {
            self.soft_reset();
        }

        Ok(sync)
    }

    /// Reset the packet parser for a new stream.
    pub fn reset(&mut self, info: StreamInfo) {
        let max_frame_size =
            if info.frame_byte_len_max > 0 { Some(info.frame_byte_len_max as usize) } else { None };

        self.info = info;
        self.builder.set_max_frame_size(max_frame_size);
        self.soft_reset();
    }
}

fn calc_sync_info(stream_info: &StreamInfo, header: &FrameHeader) -> SyncInfo {
    let is_fixed = stream_info.block_len_max == stream_info.block_len_min;

    let dur = u64::from(header.block_num_samples);

    let ts = match header.block_sequence {
        BlockSequence::BySample(sample) => sample,
        BlockSequence::ByFrame(frame) if is_fixed => {
            u64::from(frame) * u64::from(stream_info.block_len_min)
        }
        BlockSequence::ByFrame(frame) => {
            // This should not happen in practice.
            warn!("got a fixed size frame for a variable stream, the timestamp may be off");
            u64::from(frame) * dur
        }
    };

    SyncInfo { ts, dur }
}

fn strict_frame_header_check(
    stream_info: &StreamInfo,
    header: &FrameHeader,
    last_header: Option<&FrameHeader>,
) -> bool {
    // Sample rate is fixed for the stream.
    if let Some(sample_rate) = header.sample_rate {
        if sample_rate != stream_info.sample_rate {
            return false;
        }
    }

    // Bits per sample is fixed for the stream.
    if let Some(bps) = header.bits_per_sample {
        if bps != stream_info.bits_per_sample {
            return false;
        }
    }

    let is_fixed = stream_info.block_len_min == stream_info.block_len_max;

    // All blocks should have a block length within the stated bounds. However, the last block may
    // be shorter than the minimum.
    if header.block_num_samples > stream_info.block_len_max {
        return false;
    }

    // Get the last sequence number (frame or sample number).
    let last_seq = match last_header {
        Some(header) => match header.block_sequence {
            BlockSequence::BySample(sample) => sample,
            BlockSequence::ByFrame(frame) => u64::from(frame),
        },
        _ => 0,
    };

    // Sequence scoring: The fragment's blocking strategy is consistent with the stream
    // information block, and the sequence number (frame number or sample number) is
    // monotonic given the current state.
    let is_monotonic = match header.block_sequence {
        BlockSequence::BySample(sample) => !is_fixed && (sample > last_seq || sample == 0),
        BlockSequence::ByFrame(frame) => is_fixed && (u64::from(frame) > last_seq || frame == 0),
    };

    if !is_monotonic {
        return false;
    }

    // Channel assignments.
    let num_frame_channels = match header.channel_assignment {
        ChannelAssignment::Independant(num) => num,
        ChannelAssignment::LeftSide => 2,
        ChannelAssignment::MidSide => 2,
        ChannelAssignment::RightSide => 2,
    };

    if num_frame_channels != stream_info.channels.count() as u32 {
        return false;
    }

    true
}

#[inline(always)]
fn may_contain_sync_preamble(buf: &[u8]) -> bool {
    let mut win = [0u8; 8];
    win.copy_from_slice(&buf[..8]);

    // If, within the current 8 byte window, no single byte is 0xff then there cannot be a
    // frame synchronization preamble present.
    bits::contains_ones_byte_u64(u64::from_ne_bytes(win))
}

/// Scan for frame synchronization premable, and if one is found, return the position of it in
/// buf.
fn scan_for_sync_preamble(buf: &[u8]) -> Option<(usize, u16)> {
    for (p, chunk) in buf.chunks_exact(8).enumerate() {
        // If there is no possibility of a preamble in this chunk (no 0xff byte), then skip to the
        // next chunk.
        if !may_contain_sync_preamble(chunk) {
            continue;
        }

        // Otherwise, there *may* be a frame synchronization preamble in this chunk, or partially in
        // this chunk.
        let mut sync = 0u16;

        // Starting from the beginning of this chunk, read up-to 9 bytes to find the synchronization
        // preamble. 9 bytes must be read to ensure that if the premable started on the last byte
        // of a chunk, then it will be found.
        for (i, byte) in buf[8 * p..].iter().take(8 + 1).enumerate() {
            sync = (sync << 8) | u16::from(*byte);

            if (sync & 0xfffc) == 0xfff8 {
                let offset = (8 * p) + i - 1;
                return Some((offset, sync));
            }
        }
    }

    // No preamble found.
    None
}
