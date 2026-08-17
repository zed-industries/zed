// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::BTreeMap;
use std::io::{Seek, SeekFrom};

use symphonia_core::errors::{reset_error, seek_error, unsupported_error};
use symphonia_core::errors::{Error, Result, SeekErrorKind};
use symphonia_core::formats::prelude::*;
use symphonia_core::io::{MediaSource, MediaSourceStream, ReadBytes, SeekBuffered};
use symphonia_core::meta::{Metadata, MetadataLog};
use symphonia_core::probe::{Descriptor, Instantiate, QueryDescriptor};
use symphonia_core::support_format;

use log::{debug, info, warn};

use super::common::SideData;
use super::logical::LogicalStream;
use super::mappings;
use super::page::*;
use super::physical;

/// OGG demultiplexer.
///
/// `OggReader` implements a demuxer for Xiph's OGG container format.
pub struct OggReader {
    reader: MediaSourceStream,
    tracks: Vec<Track>,
    cues: Vec<Cue>,
    metadata: MetadataLog,
    options: FormatOptions,
    /// The page reader.
    pages: PageReader,
    /// `LogicalStream` for each serial.
    streams: BTreeMap<u32, LogicalStream>,
    /// The position of the first byte of the current physical stream.
    phys_byte_range_start: u64,
    /// The position of the first byte of the next physical stream, if available.
    phys_byte_range_end: Option<u64>,
}

impl OggReader {
    fn read_page(&mut self) -> Result<()> {
        // Try reading pages until a page is successfully read, or an IO error.
        loop {
            match self.pages.try_next_page(&mut self.reader) {
                Ok(_) => break,
                Err(Error::IoError(e)) => return Err(Error::from(e)),
                Err(e) => {
                    warn!("{}", e);
                }
            }
        }

        let page = self.pages.page();

        // If the page is marked as a first page, then try to start a new physical stream.
        if page.header.is_first_page {
            self.start_new_physical_stream()?;
            return reset_error();
        }

        if let Some(stream) = self.streams.get_mut(&page.header.serial) {
            // TODO: Process side data.
            let _side_data = stream.read_page(&page)?;
        }
        else {
            // If there is no associated logical stream with this page, then this is a
            // completely random page within the physical stream. Discard it.
        }

        Ok(())
    }

    fn peek_logical_packet(&self) -> Option<&Packet> {
        let page = self.pages.page();

        if let Some(stream) = self.streams.get(&page.header.serial) {
            stream.peek_packet()
        }
        else {
            None
        }
    }

    fn discard_logical_packet(&mut self) {
        let page = self.pages.page();

        // Consume a packet from the logical stream belonging to the current page.
        if let Some(stream) = self.streams.get_mut(&page.header.serial) {
            stream.consume_packet();
        }
    }

    fn next_logical_packet(&mut self) -> Result<Packet> {
        loop {
            let page = self.pages.page();

            // Read the next packet. Packets are only ever buffered in the logical stream of the
            // current page.
            if let Some(stream) = self.streams.get_mut(&page.header.serial) {
                if let Some(packet) = stream.next_packet() {
                    return Ok(packet);
                }
            }

            self.read_page()?;
        }
    }

    fn do_seek(&mut self, serial: u32, required_ts: u64) -> Result<SeekedTo> {
        // If the reader is seekable, then use the bisection method to coarsely seek to the nearest
        // page that ends before the required timestamp.
        if self.reader.is_seekable() {
            let stream = self.streams.get_mut(&serial).unwrap();

            // Bisection method byte ranges. When these two values are equal, the bisection has
            // converged on the position of the correct page.
            let mut start_byte_pos = self.phys_byte_range_start;
            let mut end_byte_pos = self.phys_byte_range_end.unwrap();

            // Bisect the stream while the byte range is large. For smaller ranges, a linear scan is
            // faster than having the the binary search converge.
            while end_byte_pos - start_byte_pos > 2 * OGG_PAGE_MAX_SIZE as u64 {
                // Find the middle of the upper and lower byte search range.
                let mid_byte_pos = (start_byte_pos + end_byte_pos) / 2;

                // Seek to the middle of the byte range.
                self.reader.seek(SeekFrom::Start(mid_byte_pos))?;

                // Read the next page.
                match self.pages.next_page_for_serial(&mut self.reader, serial) {
                    Ok(_) => (),
                    _ => {
                        // No more pages for the stream from the mid-point onwards.
                        debug!(
                            "seek: bisect step: byte_range=[{}, {}, {}]",
                            start_byte_pos, mid_byte_pos, end_byte_pos,
                        );

                        end_byte_pos = mid_byte_pos;
                        continue;
                    }
                }

                // Probe the page to get the start and end timestamp.
                let (start_ts, end_ts) = stream.inspect_page(&self.pages.page());

                debug!(
                    "seek: bisect step: page={{ start_ts={}, end_ts={} }} byte_range=[{}, {}, {}]",
                    start_ts, end_ts, start_byte_pos, mid_byte_pos, end_byte_pos,
                );

                if required_ts < start_ts {
                    // The required timestamp is less-than the timestamp of the first sample in the
                    // page. Update the upper bound and bisect again.
                    end_byte_pos = mid_byte_pos;
                }
                else if required_ts > end_ts {
                    // The required timestamp is greater-than the timestamp of the final sample in
                    // the in the page. Update the lower bound and bisect again.
                    start_byte_pos = mid_byte_pos;
                }
                else {
                    // The sample with the required timestamp is contained in the page. The
                    // bisection has converged on the correct page so stop the bisection.
                    start_byte_pos = mid_byte_pos;
                    end_byte_pos = mid_byte_pos;
                    break;
                }
            }

            // If the bisection did not converge, then the linear search must continue from the
            // lower-bound (start) position of what would've been the next iteration of bisection.
            if start_byte_pos != end_byte_pos {
                self.reader.seek(SeekFrom::Start(start_byte_pos))?;

                match self.pages.next_page_for_serial(&mut self.reader, serial) {
                    Ok(_) => (),
                    _ => return seek_error(SeekErrorKind::OutOfRange),
                }
            }

            // Reset all logical bitstreams since the physical stream will be reading from a new
            // location now.
            for (&s, stream) in self.streams.iter_mut() {
                stream.reset();

                // Read in the current page since it contains our timestamp.
                if s == serial {
                    stream.read_page(&self.pages.page())?;
                }
            }
        }

        // Consume packets until reaching the desired timestamp.
        let actual_ts = loop {
            match self.peek_logical_packet() {
                Some(packet) => {
                    if packet.track_id() == serial && packet.ts + packet.dur >= required_ts {
                        break packet.ts;
                    }

                    self.discard_logical_packet();
                }
                _ => self.read_page()?,
            }
        };

        debug!(
            "seeked track={:#x} to packet_ts={} (delta={})",
            serial,
            actual_ts,
            actual_ts as i64 - required_ts as i64
        );

        Ok(SeekedTo { track_id: serial, actual_ts, required_ts })
    }

    fn start_new_physical_stream(&mut self) -> Result<()> {
        // The new mapper set.
        let mut streams = BTreeMap::<u32, LogicalStream>::new();

        // The start of page position.
        let mut byte_range_start = self.reader.pos();

        // Pre-condition: This function is only called when the current page is marked as a
        // first page.
        assert!(self.pages.header().is_first_page);

        info!("starting new physical stream");

        // The first page of each logical stream, marked with the first page flag, must contain the
        // identification packet for the encapsulated codec bitstream. The first page for each
        // logical stream from the current logical stream group must appear before any other pages.
        // That is to say, if there are N logical streams, then the first N pages must contain the
        // identification packets for each respective logical stream.
        loop {
            let header = self.pages.header();

            if !header.is_first_page {
                break;
            }

            byte_range_start = self.reader.pos();

            // There should only be a single packet, the identification packet, in the first page.
            if let Some(pkt) = self.pages.first_packet() {
                // If a stream mapper has been detected, create a logical stream with it.
                if let Some(mapper) = mappings::detect(pkt)? {
                    info!(
                        "selected {} mapper for stream with serial={:#x}",
                        mapper.name(),
                        header.serial
                    );

                    let stream = LogicalStream::new(mapper, self.options.enable_gapless);
                    streams.insert(header.serial, stream);
                }
            }

            // Read the next page.
            self.pages.try_next_page(&mut self.reader)?;
        }

        // Each logical stream may contain additional header packets after the identification packet
        // that contains format-relevant information such as setup and metadata. These packets,
        // for all logical streams, should be grouped together after the identification packets.
        // Reading pages consumes these headers and returns any relevant data as side data. Read
        // pages until all headers are consumed and the first bitstream packets are buffered.
        loop {
            let page = self.pages.page();

            if let Some(stream) = streams.get_mut(&page.header.serial) {
                let side_data = stream.read_page(&page)?;

                // Consume each piece of side data.
                for data in side_data {
                    match data {
                        SideData::Metadata(rev) => self.metadata.push(rev),
                    }
                }

                if stream.has_packets() {
                    break;
                }
            }

            // The current page has been consumed and we're committed to reading a new one. Record
            // the end of the current page.
            byte_range_start = self.reader.pos();

            self.pages.try_next_page(&mut self.reader)?;
        }

        // Probe the logical streams for their start and end pages.
        physical::probe_stream_start(&mut self.reader, &mut self.pages, &mut streams);

        let mut byte_range_end = Default::default();

        // If the media source stream is seekable, then try to determine the duration of each
        // logical stream, and the length in bytes of the physical stream.
        if self.reader.is_seekable() {
            if let Some(total_len) = self.reader.byte_len() {
                byte_range_end = physical::probe_stream_end(
                    &mut self.reader,
                    &mut self.pages,
                    &mut streams,
                    byte_range_start,
                    total_len,
                )?;
            }
        }

        // At this point it can safely be assumed that a new physical stream is starting.

        // First, clear the existing track listing.
        self.tracks.clear();

        // Second, add a track for all streams.
        for (&serial, stream) in streams.iter() {
            // Warn if the track is not ready. This should not happen if the physical stream was
            // muxed properly.
            if !stream.is_ready() {
                warn!("track for serial={:#x} may not be ready", serial);
            }

            self.tracks.push(Track::new(serial, stream.codec_params().clone()));
        }

        // Third, replace all logical streams with the new set.
        self.streams = streams;

        // Last, store the lower and upper byte boundaries of the physical stream for seeking.
        self.phys_byte_range_start = byte_range_start;
        self.phys_byte_range_end = byte_range_end;

        Ok(())
    }
}

impl QueryDescriptor for OggReader {
    fn query() -> &'static [Descriptor] {
        &[support_format!(
            "ogg",
            "OGG",
            &["ogg", "ogv", "oga", "ogx", "ogm", "spx", "opus"],
            &["video/ogg", "audio/ogg", "application/ogg"],
            &[b"OggS"]
        )]
    }

    fn score(_context: &[u8]) -> u8 {
        255
    }
}

impl FormatReader for OggReader {
    fn try_new(mut source: MediaSourceStream, options: &FormatOptions) -> Result<Self> {
        // A seekback buffer equal to the maximum OGG page size is required for this reader.
        source.ensure_seekback_buffer(OGG_PAGE_MAX_SIZE);

        let pages = PageReader::try_new(&mut source)?;

        if !pages.header().is_first_page {
            return unsupported_error("ogg: page is not marked as first");
        }

        let mut ogg = OggReader {
            reader: source,
            tracks: Default::default(),
            cues: Default::default(),
            metadata: Default::default(),
            streams: Default::default(),
            options: *options,
            pages,
            phys_byte_range_start: 0,
            phys_byte_range_end: None,
        };

        ogg.start_new_physical_stream()?;

        Ok(ogg)
    }

    fn next_packet(&mut self) -> Result<Packet> {
        self.next_logical_packet()
    }

    fn metadata(&mut self) -> Metadata<'_> {
        self.metadata.metadata()
    }

    fn cues(&self) -> &[Cue] {
        &self.cues
    }

    fn tracks(&self) -> &[Track] {
        &self.tracks
    }

    fn seek(&mut self, _mode: SeekMode, to: SeekTo) -> Result<SeekedTo> {
        // Get the timestamp of the desired audio frame.
        let (required_ts, serial) = match to {
            // Frame timestamp given.
            SeekTo::TimeStamp { ts, track_id } => {
                // Check if the user provided an invalid track ID.
                if let Some(stream) = self.streams.get(&track_id) {
                    let params = stream.codec_params();

                    // Timestamp lower-bound out-of-range.
                    if ts < params.start_ts {
                        return seek_error(SeekErrorKind::OutOfRange);
                    }

                    // Timestamp upper-bound out-of-range.
                    if let Some(dur) = params.n_frames {
                        if ts > dur + params.start_ts {
                            return seek_error(SeekErrorKind::OutOfRange);
                        }
                    }
                }
                else {
                    return seek_error(SeekErrorKind::InvalidTrack);
                }

                (ts, track_id)
            }
            // Time value given, calculate frame timestamp from sample rate.
            SeekTo::Time { time, track_id } => {
                // Get the track serial.
                let serial = if let Some(serial) = track_id {
                    serial
                }
                else if let Some(default_track) = self.default_track() {
                    default_track.id
                }
                else {
                    // No tracks.
                    return seek_error(SeekErrorKind::Unseekable);
                };

                // Convert the time to a timestamp.
                let ts = if let Some(stream) = self.streams.get(&serial) {
                    let params = stream.codec_params();

                    let ts = if let Some(sample_rate) = params.sample_rate {
                        TimeBase::new(1, sample_rate).calc_timestamp(time)
                    }
                    else {
                        // No sample rate. This should never happen.
                        return seek_error(SeekErrorKind::Unseekable);
                    };

                    // Timestamp lower-bound out-of-range.
                    if ts < params.start_ts {
                        return seek_error(SeekErrorKind::OutOfRange);
                    }

                    // Timestamp upper-bound out-of-range.
                    if let Some(dur) = params.n_frames {
                        if ts > dur + params.start_ts {
                            return seek_error(SeekErrorKind::OutOfRange);
                        }
                    }

                    ts
                }
                else {
                    // No mapper for track. The user provided a bad track ID.
                    return seek_error(SeekErrorKind::InvalidTrack);
                };

                (ts, serial)
            }
        };

        debug!("seeking track={:#x} to frame_ts={}", serial, required_ts);

        // Do the actual seek.
        self.do_seek(serial, required_ts)
    }

    fn into_inner(self: Box<Self>) -> MediaSourceStream {
        self.reader
    }
}
