// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The `format` module provides the traits and support structures necessary to implement media
//! demuxers.

use crate::codecs::CodecParameters;
use crate::errors::Result;
use crate::io::{BufReader, MediaSourceStream};
use crate::meta::{Metadata, Tag};
use crate::units::{Time, TimeStamp};

pub mod prelude {
    //! The `formats` module prelude.

    pub use crate::units::{Duration, TimeBase, TimeStamp};

    pub use super::{Cue, FormatOptions, FormatReader, Packet, SeekMode, SeekTo, SeekedTo, Track};
}

/// `SeekTo` specifies a position to seek to.
pub enum SeekTo {
    /// Seek to a `Time` in regular time units.
    Time {
        /// The `Time` to seek to.
        time: Time,
        /// If `Some`, specifies which track's timestamp should be returned after the seek. If
        /// `None`, then the default track's timestamp is returned. If the container does not have
        /// a default track, then the first track's timestamp is returned.
        track_id: Option<u32>,
    },
    /// Seek to a track's `TimeStamp` in that track's timebase units.
    TimeStamp {
        /// The `TimeStamp` to seek to.
        ts: TimeStamp,
        /// Specifies which track `ts` is relative to.
        track_id: u32,
    },
}

/// `SeekedTo` is the result of a seek.
#[derive(Copy, Clone, Debug)]
pub struct SeekedTo {
    /// The track the seek was relative to.
    pub track_id: u32,
    /// The `TimeStamp` required for the requested seek.
    pub required_ts: TimeStamp,
    /// The `TimeStamp` that was seeked to.
    pub actual_ts: TimeStamp,
}

/// `SeekMode` selects the precision of a seek.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SeekMode {
    /// Coarse seek mode is a best-effort attempt to seek to the requested position. The actual
    /// position seeked to may be before or after the requested position. Coarse seeking is an
    /// optional performance enhancement. If a `FormatReader` does not support this mode an
    /// accurate seek will be performed instead.
    Coarse,
    /// Accurate (aka sample-accurate) seek mode will be always seek to a position before the
    /// requested position.
    Accurate,
}

/// `FormatOptions` is a common set of options that all demuxers use.
#[derive(Copy, Clone, Debug)]
pub struct FormatOptions {
    /// If a `FormatReader` requires a seek index, but the container does not provide one, build the
    /// seek index during instantiation instead of building it progressively. Default: `false`.
    pub prebuild_seek_index: bool,
    /// If a seek index needs to be built, this value determines how often in seconds of decoded
    /// content an entry is added to the index. Default: `20`.
    ///
    /// Note: This is a CPU vs. memory trade-off. A high value will increase the amount of IO
    /// required during a seek, whereas a low value will require more memory. The default chosen is
    /// a good compromise for casual playback of music, podcasts, movies, etc. However, for
    /// highly-interactive applications, this value should be decreased.
    pub seek_index_fill_rate: u16,
    /// Enable support for gapless playback. Default: `false`.
    ///
    /// When enabled, the reader will provide trim information in packets that may be used by
    /// decoders to trim any encoder delay or padding.
    ///
    /// When enabled, this option will also alter the value and interpretation of timestamps and
    /// durations such that they are relative to the non-trimmed region.
    pub enable_gapless: bool,
}

impl Default for FormatOptions {
    fn default() -> Self {
        FormatOptions {
            prebuild_seek_index: false,
            seek_index_fill_rate: 20,
            enable_gapless: false,
        }
    }
}

/// A `Cue` is a designated point of time within a media stream.
///
/// A `Cue` may be a mapping from either a source track, a chapter, cuesheet, or a timestamp
/// depending on the source media. A `Cue`'s duration is the difference between the `Cue`'s
/// timestamp and the next. Each `Cue` may contain an optional index of points relative to the `Cue`
/// that never exceed the timestamp of the next `Cue`. A `Cue` may also have associated `Tag`s.
#[derive(Clone, Debug)]
pub struct Cue {
    /// A unique index for the `Cue`.
    pub index: u32,
    /// The starting timestamp in number of frames from the start of the stream.
    pub start_ts: u64,
    /// A list of `Tag`s associated with the `Cue`.
    pub tags: Vec<Tag>,
    /// A list of `CuePoints`s that are contained within this `Cue`. These points are children of
    /// the `Cue` since the `Cue` itself is an implicit `CuePoint`.
    pub points: Vec<CuePoint>,
}

/// A `CuePoint` is a point, represented as a frame offset, within a `Cue`.
///
/// A `CuePoint` provides more precise indexing within a parent `Cue`. Additional `Tag`s may be
/// associated with a `CuePoint`.
#[derive(Clone, Debug)]
pub struct CuePoint {
    /// The offset of the first frame in the `CuePoint` relative to the start of the parent `Cue`.
    pub start_offset_ts: u64,
    /// A list of `Tag`s associated with the `CuePoint`.
    pub tags: Vec<Tag>,
}

/// A `Track` is an independently coded media bitstream. A media format may contain multiple tracks
/// in one container. Each of those tracks are represented by one `Track`.
#[derive(Clone, Debug)]
pub struct Track {
    /// A unique identifier for the track.
    pub id: u32,
    /// The codec parameters for the track.
    pub codec_params: CodecParameters,
    /// The language of the track. May be unknown.
    pub language: Option<String>,
}

impl Track {
    pub fn new(id: u32, codec_params: CodecParameters) -> Self {
        Track { id, codec_params, language: None }
    }
}

/// A `FormatReader` is a container demuxer. It provides methods to probe a media container for
/// information and access the tracks encapsulated in the container.
///
/// Most, if not all, media containers contain metadata, then a number of packetized, and
/// interleaved codec bitstreams. These bitstreams are usually referred to as tracks. Generally,
/// the encapsulated bitstreams are independently encoded using some codec. The allowed codecs for a
/// container are defined in the specification of the container format.
///
/// While demuxing, packets are read one-by-one and may be discarded or decoded at the choice of
/// the caller. The contents of a packet is undefined: it may be a frame of video, a millisecond
/// of audio, or a subtitle, but a packet will never contain data from two different bitstreams.
/// Therefore the caller can be selective in what tracks(s) should be decoded and consumed.
///
/// `FormatReader` provides an Iterator-like interface over packets for easy consumption and
/// filtering. Seeking will invalidate the state of any `Decoder` processing packets from the
/// `FormatReader` and should be reset after a successful seek operation.
pub trait FormatReader: Send + Sync {
    /// Attempt to instantiate a `FormatReader` using the provided `FormatOptions` and
    /// `MediaSourceStream`. The reader will probe the container to verify format support, determine
    /// the number of tracks, and read any initial metadata.
    fn try_new(source: MediaSourceStream, options: &FormatOptions) -> Result<Self>
    where
        Self: Sized;

    /// Gets a list of all `Cue`s.
    fn cues(&self) -> &[Cue];

    /// Gets the metadata revision log.
    fn metadata(&mut self) -> Metadata<'_>;

    /// Seek, as precisely as possible depending on the mode, to the `Time` or track `TimeStamp`
    /// requested. Returns the requested and actual `TimeStamps` seeked to, as well as the `Track`.
    ///
    /// After a seek, all `Decoder`s consuming packets from this reader should be reset.
    ///
    /// Note: The `FormatReader` by itself cannot seek to an exact audio frame, it is only capable
    /// of seeking to the nearest `Packet`. Therefore, to seek to an exact frame, a `Decoder` must
    /// decode packets until the requested position is reached. When using the accurate `SeekMode`,
    /// the seeked position will always be before the requested position. If the coarse `SeekMode`
    /// is used, then the seek position may be after the requested position. Coarse seeking is an
    /// optional performance enhancement, therefore, a coarse seek may sometimes be an accurate
    /// seek.
    fn seek(&mut self, mode: SeekMode, to: SeekTo) -> Result<SeekedTo>;

    /// Gets a list of tracks in the container.
    fn tracks(&self) -> &[Track];

    /// Gets the default track. If the `FormatReader` has a method of determining the default track,
    /// this function should return it. Otherwise, the first track is returned. If no tracks are
    /// present then `None` is returned.
    fn default_track(&self) -> Option<&Track> {
        self.tracks().first()
    }

    /// Get the next packet from the container.
    ///
    /// If `ResetRequired` is returned, then the track list must be re-examined and all `Decoder`s
    /// re-created. All other errors are unrecoverable.
    fn next_packet(&mut self) -> Result<Packet>;

    /// Destroys the `FormatReader` and returns the underlying media source stream
    fn into_inner(self: Box<Self>) -> MediaSourceStream;
}

/// A `Packet` contains a discrete amount of encoded data for a single codec bitstream. The exact
/// amount of data is bounded, but not defined, and is dependant on the container and/or the
/// encapsulated codec.
#[derive(Clone)]
pub struct Packet {
    /// The track id.
    track_id: u32,
    /// The timestamp of the packet. When gapless support is enabled, this timestamp is relative to
    /// the end of the encoder delay.
    ///
    /// This timestamp is in `TimeBase` units.
    pub ts: u64,
    /// The duration of the packet. When gapless support is enabled, the duration does not include
    /// the encoder delay or padding.
    ///
    /// The duration is in `TimeBase` units.
    pub dur: u64,
    /// When gapless support is enabled, this is the number of decoded frames that should be trimmed
    /// from the start of the packet to remove the encoder delay. Must be 0 in all other cases.
    pub trim_start: u32,
    /// When gapless support is enabled, this is the number of decoded frames that should be trimmed
    /// from the end of the packet to remove the encoder padding. Must be 0 in all other cases.
    pub trim_end: u32,
    /// The packet buffer.
    pub data: Box<[u8]>,
}

impl Packet {
    /// Create a new `Packet` from a slice.
    pub fn new_from_slice(track_id: u32, ts: u64, dur: u64, buf: &[u8]) -> Self {
        Packet { track_id, ts, dur, trim_start: 0, trim_end: 0, data: Box::from(buf) }
    }

    /// Create a new `Packet` from a boxed slice.
    pub fn new_from_boxed_slice(track_id: u32, ts: u64, dur: u64, data: Box<[u8]>) -> Self {
        Packet { track_id, ts, dur, trim_start: 0, trim_end: 0, data }
    }

    /// Create a new `Packet` with trimming information from a slice.
    pub fn new_trimmed_from_slice(
        track_id: u32,
        ts: u64,
        dur: u64,
        trim_start: u32,
        trim_end: u32,
        buf: &[u8],
    ) -> Self {
        Packet { track_id, ts, dur, trim_start, trim_end, data: Box::from(buf) }
    }

    /// Create a new `Packet` with trimming information from a boxed slice.
    pub fn new_trimmed_from_boxed_slice(
        track_id: u32,
        ts: u64,
        dur: u64,
        trim_start: u32,
        trim_end: u32,
        data: Box<[u8]>,
    ) -> Self {
        Packet { track_id, ts, dur, trim_start, trim_end, data }
    }

    /// The track identifier of the track this packet belongs to.
    pub fn track_id(&self) -> u32 {
        self.track_id
    }

    /// Get the timestamp of the packet in `TimeBase` units.
    ///
    /// If gapless support is enabled, then this timestamp is relative to the end of the encoder
    /// delay.
    pub fn ts(&self) -> u64 {
        self.ts
    }

    /// Get the duration of the packet in `TimeBase` units.
    ///
    /// If gapless support is enabled, then this is the duration after the encoder delay and padding
    /// is trimmed.
    pub fn dur(&self) -> u64 {
        self.dur
    }

    /// Get the duration of the packet in `TimeBase` units if no decoded frames are trimmed.
    ///
    /// If gapless support is disabled, then this is the same as the duration.
    pub fn block_dur(&self) -> u64 {
        self.dur + u64::from(self.trim_start) + u64::from(self.trim_end)
    }

    /// Get the number of frames to trim from the start of the decoded packet.
    pub fn trim_start(&self) -> u32 {
        self.trim_start
    }

    /// Get the number of frames to trim from the end of the decoded packet.
    pub fn trim_end(&self) -> u32 {
        self.trim_end
    }

    /// Get an immutable slice to the packet buffer.
    pub fn buf(&self) -> &[u8] {
        &self.data
    }

    /// Get a `BufStream` to read the packet data buffer sequentially.
    pub fn as_buf_reader(&self) -> BufReader<'_> {
        BufReader::new(&self.data)
    }
}

pub mod util {
    //! Helper utilities for implementing `FormatReader`s.

    use super::Packet;

    /// A `SeekPoint` is a mapping between a sample or frame number to byte offset within a media
    /// stream.
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct SeekPoint {
        /// The frame or sample timestamp of the `SeekPoint`.
        pub frame_ts: u64,
        /// The byte offset of the `SeekPoint`s timestamp relative to a format-specific location.
        pub byte_offset: u64,
        /// The number of frames the `SeekPoint` covers.
        pub n_frames: u32,
    }

    impl SeekPoint {
        fn new(frame_ts: u64, byte_offset: u64, n_frames: u32) -> Self {
            SeekPoint { frame_ts, byte_offset, n_frames }
        }
    }

    /// A `SeekIndex` stores `SeekPoint`s (generally a sample or frame number to byte offset) within
    /// a media stream and provides methods to efficiently search for the nearest `SeekPoint`(s)
    /// given a timestamp.
    ///
    /// A `SeekIndex` does not require complete coverage of the entire media stream. However, the
    /// better the coverage, the smaller the manual search range the `SeekIndex` will return.
    #[derive(Default)]
    pub struct SeekIndex {
        points: Vec<SeekPoint>,
    }

    /// `SeekSearchResult` is the return value for a search on a `SeekIndex`. It returns a range of
    /// `SeekPoint`s a `FormatReader` should search to find the desired timestamp. Ranges are
    /// lower-bound inclusive, and upper-bound exclusive.
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum SeekSearchResult {
        /// The `SeekIndex` is empty so the desired timestamp could not be found. The entire stream
        /// should be searched for the desired timestamp.
        Stream,
        /// The desired timestamp can be found before, the `SeekPoint`. The stream should be
        /// searched for the desired timestamp from the start of the stream up-to, but not
        /// including, the `SeekPoint`.
        Upper(SeekPoint),
        /// The desired timestamp can be found at, or after, the `SeekPoint`. The stream should be
        /// searched for the desired timestamp starting at the provided `SeekPoint` up-to the end of
        /// the stream.
        Lower(SeekPoint),
        /// The desired timestamp can be found within the range. The stream should be searched for
        /// the desired starting at the first `SeekPoint` up-to, but not-including, the second
        /// `SeekPoint`.
        Range(SeekPoint, SeekPoint),
    }

    impl SeekIndex {
        /// Create an empty `SeekIndex`
        pub fn new() -> SeekIndex {
            SeekIndex { points: Vec::new() }
        }

        /// Insert a `SeekPoint` into the index.
        pub fn insert(&mut self, ts: u64, byte_offset: u64, n_frames: u32) {
            // Create the seek point.
            let seek_point = SeekPoint::new(ts, byte_offset, n_frames);

            // Get the timestamp of the last entry in the index.
            let (last_ts, last_offset) =
                self.points.last().map_or((u64::MAX, u64::MAX), |p| (p.frame_ts, p.byte_offset));

            // If the seek point has a timestamp greater-than and byte offset greater-than or equal to
            // the last entry in the index, then simply append it to the index.
            if ts > last_ts && byte_offset >= last_offset {
                self.points.push(seek_point)
            }
            else if ts < last_ts {
                // If the seek point has a timestamp less-than the last entry in the index, then the
                // insertion point must be found. This case should rarely occur.
                let i = self
                    .points
                    .partition_point(|p| ts > p.frame_ts && byte_offset >= p.byte_offset);

                // Insert if the point found or if the points are empty
                if i < self.points.len() || i == 0 {
                    self.points.insert(i, seek_point);
                }
            }
        }

        /// Search the index to find a bounded range of bytes wherein the specified frame timestamp
        /// will be contained. If the index is empty, this function simply returns a result
        /// indicating the entire stream should be searched manually.
        pub fn search(&self, frame_ts: u64) -> SeekSearchResult {
            // The index must contain atleast one SeekPoint to return a useful result.
            if !self.points.is_empty() {
                let mut lower = 0;
                let mut upper = self.points.len() - 1;

                // If the desired timestamp is less than the first SeekPoint within the index,
                // indicate that the stream should be searched from the beginning.
                if frame_ts < self.points[lower].frame_ts {
                    return SeekSearchResult::Upper(self.points[lower]);
                }
                // If the desired timestamp is greater than or equal to the last SeekPoint within
                // the index, indicate that the stream should be searched from the last SeekPoint.
                else if frame_ts >= self.points[upper].frame_ts {
                    return SeekSearchResult::Lower(self.points[upper]);
                }

                // Desired timestamp is between the lower and upper indicies. Perform a binary
                // search to find a range of SeekPoints containing the desired timestamp. The binary
                // search exits when either two adjacent SeekPoints or a single SeekPoint is found.
                while upper - lower > 1 {
                    let mid = (lower + upper) / 2;
                    let mid_ts = self.points[mid].frame_ts;

                    if frame_ts < mid_ts {
                        upper = mid;
                    }
                    else {
                        lower = mid;
                    }
                }

                return SeekSearchResult::Range(self.points[lower], self.points[upper]);
            }

            // The index is empty, the stream must be searched manually.
            SeekSearchResult::Stream
        }
    }

    /// Given a `Packet`, the encoder delay in frames, and the number of non-delay or padding
    /// frames, adjust the packet's timestamp and duration, and populate the trim information.
    pub fn trim_packet(packet: &mut Packet, delay: u32, num_frames: Option<u64>) {
        packet.trim_start = if packet.ts < u64::from(delay) {
            let trim = (u64::from(delay) - packet.ts).min(packet.dur);
            packet.ts = 0;
            packet.dur -= trim;
            trim as u32
        }
        else {
            packet.ts -= u64::from(delay);
            0
        };

        if let Some(num_frames) = num_frames {
            packet.trim_end = if packet.ts + packet.dur > num_frames {
                let trim = (packet.ts + packet.dur - num_frames).min(packet.dur);
                packet.dur -= trim;
                trim as u32
            }
            else {
                0
            };
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{SeekIndex, SeekPoint, SeekSearchResult};

        #[test]
        fn verify_seek_index_search() {
            let mut index = SeekIndex::new();
            // Normal index insert
            index.insert(479232, 706812, 1152);
            index.insert(959616, 1421536, 1152);
            index.insert(1919232, 2833241, 1152);
            index.insert(2399616, 3546987, 1152);
            index.insert(2880000, 4259455, 1152);

            // Search for point lower than the first entry
            assert_eq!(
                index.search(0),
                SeekSearchResult::Upper(SeekPoint::new(479232, 706812, 1152))
            );

            // Search for point higher than last entry
            assert_eq!(
                index.search(3000000),
                SeekSearchResult::Lower(SeekPoint::new(2880000, 4259455, 1152))
            );

            // Search for point that has equal timestamp with some index
            assert_eq!(
                index.search(959616),
                SeekSearchResult::Range(
                    SeekPoint::new(959616, 1421536, 1152),
                    SeekPoint::new(1919232, 2833241, 1152)
                )
            );

            // Index insert out of order
            index.insert(1440000, 2132419, 1152);

            // Search for point that have out of order index when inserting
            assert_eq!(
                index.search(1000000),
                SeekSearchResult::Range(
                    SeekPoint::new(959616, 1421536, 1152),
                    SeekPoint::new(1440000, 2132419, 1152)
                )
            );

            // Index insert with byte_offset less than last entry
            index.insert(3359232, 0, 0);

            // Search for ignored point because byte_offset less than last entry
            assert_eq!(
                index.search(3359232),
                SeekSearchResult::Lower(SeekPoint::new(2880000, 4259455, 1152))
            );
        }
    }
}
