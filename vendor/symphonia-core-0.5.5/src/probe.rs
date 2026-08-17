// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The `probe` module provides methods and traits to support auto-detection of media formats from
//! arbitrary media streams.

use crate::errors::{unsupported_error, Result};
use crate::formats::{FormatOptions, FormatReader};
use crate::io::{MediaSourceStream, ReadBytes, SeekBuffered};
use crate::meta::{Metadata, MetadataLog, MetadataOptions, MetadataReader};

use log::{debug, error};

mod bloom {

    fn fnv1a32(value: &[u8; 2]) -> u32 {
        const INIT: u32 = 0x811c_9dc5;
        const PRIME: u32 = 0x0100_0193;

        let mut state = INIT;

        for byte in value.iter() {
            state = (state ^ u32::from(*byte)).wrapping_mul(PRIME);
        }

        state
    }

    pub struct BloomFilter {
        filter: Box<[u64]>,
    }

    impl Default for BloomFilter {
        fn default() -> Self {
            BloomFilter { filter: vec![0; BloomFilter::M >> 6].into_boxed_slice() }
        }
    }

    impl BloomFilter {
        /// The number of bits, m, used by the bloom filter. Use 16384 bits (2KiB) by default.
        const M: usize = 2 * 1024 * 8;

        pub fn insert(&mut self, key: &[u8; 2]) {
            let hash = fnv1a32(key);

            let h0 = (hash >> 16) as u16;
            let h1 = (hash >> 0) as u16;

            let i0 = h0 as usize & (BloomFilter::M - 1);
            let i1 = h0.wrapping_add(h1.wrapping_mul(1)) as usize & (BloomFilter::M - 1);
            let i2 = h0.wrapping_add(h1.wrapping_mul(2)) as usize & (BloomFilter::M - 1);

            self.filter[i0 >> 6] |= 1 << (i0 & 63);
            self.filter[i1 >> 6] |= 1 << (i1 & 63);
            self.filter[i2 >> 6] |= 1 << (i2 & 63);
        }

        pub fn may_contain(&self, key: &[u8; 2]) -> bool {
            let hash = fnv1a32(key);

            let h0 = (hash >> 16) as u16;
            let h1 = (hash >> 0) as u16;

            let i0 = h0 as usize & (BloomFilter::M - 1);
            let i1 = h0.wrapping_add(h1.wrapping_mul(1)) as usize & (BloomFilter::M - 1);
            let i2 = h0.wrapping_add(h1.wrapping_mul(2)) as usize & (BloomFilter::M - 1);

            if (self.filter[i0 >> 6] & (1 << (i0 & 63))) == 0 {
                return false;
            }
            if (self.filter[i1 >> 6] & (1 << (i1 & 63))) == 0 {
                return false;
            }
            if (self.filter[i2 >> 6] & (1 << (i2 & 63))) == 0 {
                return false;
            }

            true
        }
    }
}

/// `Instantiate` is an enumeration of instantiation functions used by `Descriptor` and `Probe` to
/// instantiate metadata and container format readers.
#[derive(Copy, Clone)]
pub enum Instantiate {
    /// Instantiation function for a `FormatReader`.
    Format(fn(MediaSourceStream, &FormatOptions) -> Result<Box<dyn FormatReader>>),
    /// Instantiation function for a `MetadataReader`.
    Metadata(fn(&MetadataOptions) -> Box<dyn MetadataReader>),
}

/// `Descriptor` provides declarative information about container and metadata formats.
/// `Descriptor`s are used by `Probe` and related machinery to scan a `MediaSourceStream` for media.
#[derive(Copy, Clone)]
pub struct Descriptor {
    /// A short ASCII-only string identifying the codec.
    pub short_name: &'static str,
    /// A longer, more descriptive, string identifying the codec.
    pub long_name: &'static str,
    /// A list of case-insensitive file extensions that are generally used by the format.
    pub extensions: &'static [&'static str],
    /// A list of case-insensitive MIME types that are generally used by the format.
    pub mime_types: &'static [&'static str],
    /// A byte-string start-of-stream marker that will be searched for within the stream.
    pub markers: &'static [&'static [u8]],
    /// A function to score a context buffer.
    pub score: fn(&[u8]) -> u8,
    /// An instantiation function.
    pub inst: Instantiate,
}

/// The `QueryDescriptor` trait indicates that the implementer may be registered and capable of
/// probing.
pub trait QueryDescriptor {
    /// Returns a list of descriptors.
    fn query() -> &'static [Descriptor];

    /// Using the provided context buffer, score calculate and returns a value between 0 and 255
    /// indicating the confidence of the reader in decoding or parsing the source stream.
    fn score(context: &[u8]) -> u8;
}

/// A `Hint` provides additional information and context when probing a media source stream.
///
/// For example, the `Probe` cannot examine the extension or mime-type of the media because
/// `MediaSourceStream` abstracts away such details. However, the embedder may have this information
/// from a file path, HTTP header, email  attachment metadata, etc. `Hint`s are optional, and won't
/// lead the probe astray if they're wrong, but they may provide an informed initial guess and
/// optimize the guessing process siginificantly especially as more formats are registered.
#[derive(Clone, Debug, Default)]
pub struct Hint {
    extension: Option<String>,
    mime_type: Option<String>,
}

impl Hint {
    /// Instantiate an empty `Hint`.
    pub fn new() -> Self {
        Hint { extension: None, mime_type: None }
    }

    /// Add a file extension `Hint`.
    pub fn with_extension(&mut self, extension: &str) -> &mut Self {
        self.extension = Some(extension.to_owned());
        self
    }

    /// Add a MIME/Media-type `Hint`.
    pub fn mime_type(&mut self, mime_type: &str) -> &mut Self {
        self.mime_type = Some(mime_type.to_owned());
        self
    }
}

/// Metadata that came from the `metadata` field of [`ProbeResult`].
pub struct ProbedMetadata {
    metadata: Option<MetadataLog>,
}

impl ProbedMetadata {
    /// Returns the metadata that was found during probing.
    ///
    /// If any additional metadata was present outside of the container, this is
    /// `Some` and the log will have at least one item in it.
    pub fn get(&mut self) -> Option<Metadata<'_>> {
        self.metadata.as_mut().map(|m| m.metadata())
    }

    /// Returns the inner metadata log, if it was present.
    pub fn into_inner(self) -> Option<MetadataLog> {
        self.metadata
    }
}

/// `ProbeResult` contains the result of a format probe operation.
pub struct ProbeResult {
    /// An instance of a `FormatReader` for the probed format
    pub format: Box<dyn FormatReader>,
    /// A log of `Metadata` revisions read during the probe operation before the instantiation of
    /// the `FormatReader`.
    ///
    /// Metadata that was part of the container format itself can be read by calling `.metadata()`
    /// on `format`.
    pub metadata: ProbedMetadata,
}

/// `Probe` scans a `MediaSourceStream` for metadata and container formats, and provides an
/// iterator-like interface to instantiate readers for the formats encountered.
#[derive(Default)]
pub struct Probe {
    filter: bloom::BloomFilter,
    registered: Vec<Descriptor>,
}

impl Probe {
    const PROBE_SEARCH_LIMIT: u64 = 1 * 1024 * 1024;

    /// Register all `Descriptor`s supported by the parameterized type.
    pub fn register_all<Q: QueryDescriptor>(&mut self) {
        for descriptor in Q::query() {
            self.register(descriptor);
        }
    }

    /// Register a single `Descriptor`.
    pub fn register(&mut self, descriptor: &Descriptor) {
        // Insert 2-byte prefixes for each marker into the bloom filter.
        for marker in descriptor.markers {
            let mut prefix = [0u8; 2];

            match marker.len() {
                2..=16 => prefix.copy_from_slice(&marker[0..2]),
                _ => panic!("invalid marker length (only 2-16 bytes supported)."),
            }

            self.filter.insert(&prefix);
        }

        self.registered.push(*descriptor);
    }

    /// Searches the provided `MediaSourceStream` for metadata or a container format.
    pub fn next(&self, mss: &mut MediaSourceStream) -> Result<Instantiate> {
        let mut win = 0u16;

        let init_pos = mss.pos();
        let mut count = 0;

        // Scan the stream byte-by-byte. Shifting each byte through a 2-byte window.
        while let Ok(byte) = mss.read_byte() {
            win = (win << 8) | u16::from(byte);

            count += 1;

            if count > Probe::PROBE_SEARCH_LIMIT {
                break;
            }

            if count % 4096 == 0 {
                debug!(
                    "searching for format marker... {}+{} / {} bytes.",
                    init_pos,
                    count,
                    Probe::PROBE_SEARCH_LIMIT
                );
            }

            // Use the bloom filter to check if the the window may be a prefix of a registered
            // marker.
            if self.filter.may_contain(&win.to_be_bytes()) {
                // Using the 2-byte window, and a further 14 bytes, create a larger 16-byte window.
                let mut context = [0u8; 16];

                context[0..2].copy_from_slice(&win.to_be_bytes()[0..2]);
                mss.read_buf_exact(&mut context[2..])?;

                debug!(
                    "found a possible format marker within {:x?} @ {}+{} bytes.",
                    context, init_pos, count,
                );

                // Search for registered markers in the 16-byte window.
                for registered in &self.registered {
                    for marker in registered.markers {
                        let len = marker.len();

                        // If a match is found, return the instantiate.
                        if context[0..len] == **marker {
                            // Re-align the stream to the start of the marker.
                            mss.seek_buffered_rev(16);

                            // TODO: Implement scoring.

                            debug!(
                                "found the format marker {:x?} @ {}+{} bytes.",
                                &context[0..len],
                                init_pos,
                                count,
                            );

                            return Ok(registered.inst);
                        }
                    }
                }

                // If no registered markers were matched, then the bloom filter returned a false
                // positive. Re-align the stream to the end of the 2-byte window and continue the
                // search.
                mss.seek_buffered_rev(16 - 2);
            }
        }

        if count < Probe::PROBE_SEARCH_LIMIT {
            error!("probe reach EOF at {} bytes.", count);
        }
        else {
            // Could not find any marker within the probe limit.
            error!("reached probe limit of {} bytes.", Probe::PROBE_SEARCH_LIMIT);
        }

        unsupported_error("core (probe): no suitable format reader found")
    }

    /// Searches the provided `MediaSourceStream` for a container format. Any metadata that is read
    /// during the search will be queued and attached to the `FormatReader` instance once a
    /// container format is found.
    pub fn format(
        &self,
        _hint: &Hint,
        mut mss: MediaSourceStream,
        format_opts: &FormatOptions,
        metadata_opts: &MetadataOptions,
    ) -> Result<ProbeResult> {
        let mut metadata: MetadataLog = Default::default();

        // Loop over all elements in the stream until a container format is found.
        loop {
            match self.next(&mut mss)? {
                // If a container format is found, return an instance to it's reader.
                Instantiate::Format(fmt) => {
                    let format = fmt(mss, format_opts)?;

                    let metadata =
                        if metadata.metadata().current().is_some() { Some(metadata) } else { None };

                    return Ok(ProbeResult { format, metadata: ProbedMetadata { metadata } });
                }
                // If metadata was found, instantiate the metadata reader, read the metadata, and
                // push it onto the metadata log.
                Instantiate::Metadata(meta) => {
                    let mut reader = meta(metadata_opts);
                    metadata.push(reader.read_all(&mut mss)?);

                    debug!("chaining a metadata element.");
                }
            }
        }

        // This function returns when either the end-of-stream is reached, an error occurs, or a
        // container format is found.
    }
}

/// Convenience macro for declaring a probe `Descriptor` for a `FormatReader`.
#[macro_export]
macro_rules! support_format {
    ($short_name:expr, $long_name:expr, $exts:expr, $mimes:expr, $markers:expr) => {
        Descriptor {
            short_name: $short_name,
            long_name: $long_name,
            extensions: $exts,
            mime_types: $mimes,
            markers: $markers,
            score: Self::score,
            inst: Instantiate::Format(|source, opt| Ok(Box::new(Self::try_new(source, &opt)?))),
        }
    };
}

/// Convenience macro for declaring a probe `Descriptor` for a `MetadataReader`.
#[macro_export]
macro_rules! support_metadata {
    ($short_name:expr, $long_name:expr, $exts:expr, $mimes:expr, $markers:expr) => {
        Descriptor {
            short_name: $short_name,
            long_name: $long_name,
            extensions: $exts,
            mime_types: $mimes,
            markers: $markers,
            score: Self::score,
            inst: Instantiate::Metadata(|opt| Box::new(Self::new(&opt))),
        }
    };
}
