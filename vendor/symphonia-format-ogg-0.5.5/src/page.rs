// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::checksum::Crc32;
use symphonia_core::errors::{decode_error, Error, Result};
use symphonia_core::io::{BufReader, Monitor, MonitorStream, ReadBytes, SeekBuffered};

use log::{debug, warn};

const OGG_PAGE_MARKER: [u8; 4] = *b"OggS";
const OGG_PAGE_HEADER_SIZE: usize = 27;

pub const OGG_PAGE_MAX_SIZE: usize = OGG_PAGE_HEADER_SIZE + 255 + 255 * 255;

#[derive(Copy, Clone, Default)]
pub struct PageHeader {
    #[allow(dead_code)]
    pub version: u8,
    pub absgp: u64,
    pub serial: u32,
    pub sequence: u32,
    pub crc: u32,
    pub n_segments: u8,
    pub is_continuation: bool,
    pub is_first_page: bool,
    pub is_last_page: bool,
}

/// Reads a `PageHeader` from the the provided reader.
fn read_page_header<B: ReadBytes>(reader: &mut B) -> Result<PageHeader> {
    // The OggS marker should be present.
    let marker = reader.read_quad_bytes()?;

    if marker != OGG_PAGE_MARKER {
        return decode_error("ogg: missing ogg stream marker");
    }

    let version = reader.read_byte()?;

    // There is only one OGG version, and that is version 0.
    if version != 0 {
        return decode_error("ogg: invalid ogg version");
    }

    let flags = reader.read_byte()?;

    // Only the first 3 least-significant bits are used for flags.
    if flags & 0xf8 != 0 {
        return decode_error("ogg: invalid flag bits set");
    }

    let ts = reader.read_u64()?;
    let serial = reader.read_u32()?;
    let sequence = reader.read_u32()?;
    let crc = reader.read_u32()?;
    let n_segments = reader.read_byte()?;

    Ok(PageHeader {
        version,
        absgp: ts,
        serial,
        sequence,
        crc,
        n_segments,
        is_continuation: (flags & 0x01) != 0,
        is_first_page: (flags & 0x02) != 0,
        is_last_page: (flags & 0x04) != 0,
    })
}

/// Quickly synchronizes the provided reader to the next OGG page capture pattern, but does not
/// perform any further verification.
fn sync_page<B: ReadBytes>(reader: &mut B) -> Result<()> {
    let mut marker = u32::from_be_bytes(reader.read_quad_bytes()?);

    while marker.to_be_bytes() != OGG_PAGE_MARKER {
        marker <<= 8;
        marker |= u32::from(reader.read_u8()?);
    }

    Ok(())
}

/// An iterator over packets within a `Page`.
pub struct PagePackets<'a> {
    lens: core::slice::Iter<'a, u16>,
    data: &'a [u8],
}

impl<'a> PagePackets<'a> {
    /// If this page ends with an incomplete (partial) packet, get a slice to the data associated
    /// with the partial packet.
    pub fn partial_packet(self) -> Option<&'a [u8]> {
        // Consume the rest of the packets.
        let discard = usize::from(self.lens.sum::<u16>());

        if self.data.len() > discard {
            Some(&self.data[discard..])
        }
        else {
            None
        }
    }
}

impl<'a> Iterator for PagePackets<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        match self.lens.next() {
            Some(len) => {
                let (packet, rem) = self.data.split_at(usize::from(*len));
                self.data = rem;
                Some(packet)
            }
            _ => None,
        }
    }
}

/// An OGG page.
pub struct Page<'a> {
    /// The page header.
    pub header: PageHeader,
    packet_lens: &'a [u16],
    page_buf: &'a [u8],
}

impl Page<'_> {
    /// Returns an iterator over all complete packets within the page.
    ///
    /// If this page contains a partial packet, then the partial packet data may be retrieved using
    /// the `partial_packet` function of the iterator.
    pub fn packets(&self) -> PagePackets<'_> {
        PagePackets { lens: self.packet_lens.iter(), data: self.page_buf }
    }

    /// Gets the number of packets completed on this page.
    pub fn num_packets(&self) -> usize {
        self.packet_lens.len()
    }
}

/// A reader of OGG pages.
pub struct PageReader {
    header: PageHeader,
    packet_lens: Vec<u16>,
    page_buf: Vec<u8>,
    page_buf_len: usize,
}

impl PageReader {
    pub fn try_new<B>(reader: &mut B) -> Result<Self>
    where
        B: ReadBytes + SeekBuffered,
    {
        let mut page_reader = PageReader {
            header: Default::default(),
            packet_lens: Vec::new(),
            page_buf: Vec::new(),
            page_buf_len: 0,
        };

        page_reader.try_next_page(reader)?;

        Ok(page_reader)
    }

    /// Attempts to read the next page. If the page is corrupted or invalid, returns an error.
    pub fn try_next_page<B>(&mut self, reader: &mut B) -> Result<()>
    where
        B: ReadBytes + SeekBuffered,
    {
        let mut header_buf = [0u8; OGG_PAGE_HEADER_SIZE];
        header_buf[..4].copy_from_slice(&OGG_PAGE_MARKER);

        // Synchronize to an OGG page capture pattern.
        sync_page(reader)?;

        // Record the position immediately after synchronization. If the page is found corrupt the
        // reader will need to seek back here to try to regain synchronization.
        let sync_pos = reader.pos();

        // Read the part of the page header after the capture pattern into a buffer.
        reader.read_buf_exact(&mut header_buf[4..])?;

        // Parse the page header buffer.
        let header = read_page_header(&mut BufReader::new(&header_buf))?;

        // debug!(
        //     "page {{ version={}, absgp={}, serial={}, sequence={}, crc={:#x}, n_segments={}, \
        //         is_first={}, is_last={}, is_continuation={} }}",
        //     header.version,
        //     header.absgp,
        //     header.serial,
        //     header.sequence,
        //     header.crc,
        //     header.n_segments,
        //     header.is_first_page,
        //     header.is_last_page,
        //     header.is_continuation,
        // );

        // The CRC of the OGG page requires the page checksum bytes to be zeroed.
        header_buf[22..26].copy_from_slice(&[0u8; 4]);

        // Instantiate a Crc32, initialize it with 0, and feed it the page header buffer.
        let mut crc32 = Crc32::new(0);

        crc32.process_buf_bytes(&header_buf);

        // The remainder of the page will be checksummed as it is read.
        let mut crc32_reader = MonitorStream::new(reader, crc32);

        // Read segment table.
        let mut page_body_len = 0;
        let mut packet_len = 0;

        // TODO: Can this be transactional? A corrupt page causes the PageReader's state not
        // to change.
        self.packet_lens.clear();

        for _ in 0..header.n_segments {
            let seg_len = crc32_reader.read_byte()?;

            page_body_len += usize::from(seg_len);
            packet_len += u16::from(seg_len);

            // A segment with a length < 255 indicates that the segment is the end of a packet.
            // Push the packet length into the packet queue for the stream.
            if seg_len < 255 {
                self.packet_lens.push(packet_len);
                packet_len = 0;
            }
        }

        self.read_page_body(&mut crc32_reader, page_body_len)?;

        let calculated_crc = crc32_reader.monitor().crc();

        // If the CRC for the page is incorrect, then the page is corrupt.
        if header.crc != calculated_crc {
            warn!("crc mismatch: expected {:#x}, got {:#x}", header.crc, calculated_crc);

            // Clear packet buffer.
            self.packet_lens.clear();
            self.page_buf_len = 0;

            // Seek back to the immediately after the previous sync position.
            crc32_reader.into_inner().seek_buffered(sync_pos);

            return decode_error("ogg: crc mismatch");
        }

        self.header = header;

        Ok(())
    }

    /// Reads the next page. If the next page is corrupted or invalid, the page is discarded and
    /// the reader tries again until a valid page is read or end-of-stream.
    pub fn next_page<B>(&mut self, reader: &mut B) -> Result<()>
    where
        B: ReadBytes + SeekBuffered,
    {
        loop {
            match self.try_next_page(reader) {
                Ok(_) => break,
                Err(Error::IoError(e)) => return Err(Error::from(e)),
                _ => (),
            }
        }
        Ok(())
    }

    /// Reads the next page with a specific serial. If the next page is corrupted or invalid, the
    /// page is discarded and the reader tries again until a valid page is read or end-of-stream.
    pub fn next_page_for_serial<B>(&mut self, reader: &mut B, serial: u32) -> Result<()>
    where
        B: ReadBytes + SeekBuffered,
    {
        loop {
            match self.try_next_page(reader) {
                Ok(_) => {
                    // Exit if a page with the specific serial is found.
                    if self.header.serial == serial && !self.header.is_continuation {
                        break;
                    }
                }
                Err(Error::IoError(e)) => return Err(Error::from(e)),
                _ => (),
            }
        }
        Ok(())
    }

    /// Gets a buffer to the first packet, if it exists.
    pub fn first_packet(&self) -> Option<&[u8]> {
        self.packet_lens.first().map(|&len| &self.page_buf[..usize::from(len)])
    }

    /// Gets the current page header.
    pub fn header(&self) -> PageHeader {
        self.header
    }

    /// Gets a reference to the current page.
    pub fn page(&self) -> Page<'_> {
        assert!(self.page_buf_len <= 255 * 255, "ogg pages are <= 65025 bytes");

        Page {
            header: self.header,
            packet_lens: &self.packet_lens,
            page_buf: &self.page_buf[..self.page_buf_len],
        }
    }

    fn read_page_body<B: ReadBytes>(&mut self, reader: &mut B, len: usize) -> Result<()> {
        // This is precondition.
        assert!(len <= 255 * 255);

        if len > self.page_buf.len() {
            // New page buffer size, rounded up to the nearest 8K block.
            let new_buf_len = (len + (8 * 1024 - 1)) & !(8 * 1024 - 1);
            debug!("grow page buffer to {} bytes", new_buf_len);

            self.page_buf.resize(new_buf_len, Default::default());
        }

        self.page_buf_len = len;

        reader.read_buf_exact(&mut self.page_buf[..len])?;

        Ok(())
    }
}
