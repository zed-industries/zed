// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The `meta` module defines basic metadata elements, and management structures.

use std::borrow::Cow;
use std::collections::VecDeque;
use std::convert::From;
use std::fmt;
use std::num::NonZeroU32;

use crate::errors::Result;
use crate::io::MediaSourceStream;

/// `Limit` defines an upper-bound on how much of a resource should be allocated when the amount to
/// be allocated is specified by the media stream, which is untrusted. A limit will place an
/// upper-bound on this allocation at the risk of breaking potentially valid streams. Limits are
/// used to prevent denial-of-service attacks.
///
/// All limits can be defaulted to a reasonable value specific to the situation. These defaults will
/// generally not break any normal streams.
#[derive(Copy, Clone, Debug)]
pub enum Limit {
    /// Do not impose any limit.
    None,
    /// Use the a reasonable default specified by the `FormatReader` or `Decoder` implementation.
    Default,
    /// Specify the upper limit of the resource. Units are case specific.
    Maximum(usize),
}

impl Limit {
    /// Gets the numeric limit of the limit, or default value. If there is no limit, None is
    /// returned.
    pub fn limit_or_default(&self, default: usize) -> Option<usize> {
        match self {
            Limit::None => None,
            Limit::Default => Some(default),
            Limit::Maximum(max) => Some(*max),
        }
    }
}

impl Default for Limit {
    fn default() -> Self {
        Limit::Default
    }
}

/// `MetadataOptions` is a common set of options that all metadata readers use.
#[derive(Copy, Clone, Debug, Default)]
pub struct MetadataOptions {
    /// The maximum size limit in bytes that a tag may occupy in memory once decoded. Tags exceeding
    /// this limit will be skipped by the demuxer. Take note that tags in-memory are stored as UTF-8
    /// and therefore may occupy more than one byte per character.
    pub limit_metadata_bytes: Limit,

    /// The maximum size limit in bytes that a visual (picture) may occupy.
    pub limit_visual_bytes: Limit,
}

/// `StandardVisualKey` is an enumeration providing standardized keys for common visual dispositions.
/// A demuxer may assign a `StandardVisualKey` to a `Visual` if the disposition of the attached
/// visual is known and can be mapped to a standard key.
///
/// The visual types listed here are derived from, though do not entirely cover, the ID3v2 APIC
/// frame specification.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum StandardVisualKey {
    FileIcon,
    OtherIcon,
    FrontCover,
    BackCover,
    Leaflet,
    Media,
    LeadArtistPerformerSoloist,
    ArtistPerformer,
    Conductor,
    BandOrchestra,
    Composer,
    Lyricist,
    RecordingLocation,
    RecordingSession,
    Performance,
    ScreenCapture,
    Illustration,
    BandArtistLogo,
    PublisherStudioLogo,
}

/// `StandardTagKey` is an enumeration providing standardized keys for common tag types.
/// A tag reader may assign a `StandardTagKey` to a `Tag` if the tag's key is generally
/// accepted to map to a specific usage.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum StandardTagKey {
    AcoustidFingerprint,
    AcoustidId,
    Album,
    AlbumArtist,
    Arranger,
    Artist,
    Bpm,
    Comment,
    Compilation,
    Composer,
    Conductor,
    ContentGroup,
    Copyright,
    Date,
    Description,
    DiscNumber,
    DiscSubtitle,
    DiscTotal,
    EncodedBy,
    Encoder,
    EncoderSettings,
    EncodingDate,
    Engineer,
    Ensemble,
    Genre,
    IdentAsin,
    IdentBarcode,
    IdentCatalogNumber,
    IdentEanUpn,
    IdentIsrc,
    IdentPn,
    IdentPodcast,
    IdentUpc,
    Label,
    Language,
    License,
    Lyricist,
    Lyrics,
    MediaFormat,
    MixDj,
    MixEngineer,
    Mood,
    MovementName,
    MovementNumber,
    MusicBrainzAlbumArtistId,
    MusicBrainzAlbumId,
    MusicBrainzArtistId,
    MusicBrainzDiscId,
    MusicBrainzGenreId,
    MusicBrainzLabelId,
    MusicBrainzOriginalAlbumId,
    MusicBrainzOriginalArtistId,
    MusicBrainzRecordingId,
    MusicBrainzReleaseGroupId,
    MusicBrainzReleaseStatus,
    MusicBrainzReleaseTrackId,
    MusicBrainzReleaseType,
    MusicBrainzTrackId,
    MusicBrainzWorkId,
    Opus,
    OriginalAlbum,
    OriginalArtist,
    OriginalDate,
    OriginalFile,
    OriginalWriter,
    Owner,
    Part,
    PartTotal,
    Performer,
    Podcast,
    PodcastCategory,
    PodcastDescription,
    PodcastKeywords,
    Producer,
    PurchaseDate,
    Rating,
    ReleaseCountry,
    ReleaseDate,
    Remixer,
    ReplayGainAlbumGain,
    ReplayGainAlbumPeak,
    ReplayGainTrackGain,
    ReplayGainTrackPeak,
    Script,
    SortAlbum,
    SortAlbumArtist,
    SortArtist,
    SortComposer,
    SortTrackTitle,
    TaggingDate,
    TrackNumber,
    TrackSubtitle,
    TrackTitle,
    TrackTotal,
    TvEpisode,
    TvEpisodeTitle,
    TvNetwork,
    TvSeason,
    TvShowTitle,
    Url,
    UrlArtist,
    UrlCopyright,
    UrlInternetRadio,
    UrlLabel,
    UrlOfficial,
    UrlPayment,
    UrlPodcast,
    UrlPurchase,
    UrlSource,
    Version,
    Writer,
}

/// A `Tag` value.
///
/// Note: The data types in this enumeration are a generalization. Depending on the particular tag
/// format, the actual data type a specific tag may have a lesser width or encoding than the data
/// type in this enumeration.
#[derive(Clone, Debug)]
pub enum Value {
    /// A binary buffer.
    Binary(Box<[u8]>),
    /// A boolean value.
    Boolean(bool),
    /// A flag or indicator. A flag carries no data, but the presence of the tag has an implicit
    /// meaning.
    Flag,
    /// A floating point number.
    Float(f64),
    /// A signed integer.
    SignedInt(i64),
    /// A string. This is also the catch-all type for tags with unconventional data types.
    String(String),
    /// An unsigned integer.
    UnsignedInt(u64),
}

macro_rules! impl_from_for_value {
    ($value:ident, $from:ty, $conv:expr) => {
        impl From<$from> for Value {
            fn from($value: $from) -> Self {
                $conv
            }
        }
    };
}

impl_from_for_value!(v, &[u8], Value::Binary(Box::from(v)));
impl_from_for_value!(v, bool, Value::Boolean(v));
impl_from_for_value!(v, f32, Value::Float(f64::from(v)));
impl_from_for_value!(v, f64, Value::Float(v));
impl_from_for_value!(v, i8, Value::SignedInt(i64::from(v)));
impl_from_for_value!(v, i16, Value::SignedInt(i64::from(v)));
impl_from_for_value!(v, i32, Value::SignedInt(i64::from(v)));
impl_from_for_value!(v, i64, Value::SignedInt(v));
impl_from_for_value!(v, u8, Value::UnsignedInt(u64::from(v)));
impl_from_for_value!(v, u16, Value::UnsignedInt(u64::from(v)));
impl_from_for_value!(v, u32, Value::UnsignedInt(u64::from(v)));
impl_from_for_value!(v, u64, Value::UnsignedInt(v));
impl_from_for_value!(v, &str, Value::String(String::from(v)));
impl_from_for_value!(v, String, Value::String(v));
impl_from_for_value!(v, Cow<'_, str>, Value::String(String::from(v)));

fn buffer_to_hex_string(buf: &[u8]) -> String {
    let mut output = String::with_capacity(5 * buf.len());

    for ch in buf {
        let u = (ch & 0xf0) >> 4;
        let l = ch & 0x0f;
        output.push_str("\\0x");
        output.push(if u < 10 { (b'0' + u) as char } else { (b'a' + u - 10) as char });
        output.push(if l < 10 { (b'0' + l) as char } else { (b'a' + l - 10) as char });
    }

    output
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement default formatters for each type.
        match self {
            Value::Binary(ref buf) => f.write_str(&buffer_to_hex_string(buf)),
            Value::Boolean(boolean) => fmt::Display::fmt(boolean, f),
            Value::Flag => write!(f, "<flag>"),
            Value::Float(float) => fmt::Display::fmt(float, f),
            Value::SignedInt(int) => fmt::Display::fmt(int, f),
            Value::String(ref string) => fmt::Display::fmt(string, f),
            Value::UnsignedInt(uint) => fmt::Display::fmt(uint, f),
        }
    }
}

/// A `Tag` encapsulates a key-value pair of metadata.
#[derive(Clone, Debug)]
pub struct Tag {
    /// If the `Tag`'s key string is commonly associated with a typical type, meaning, or purpose,
    /// then if recognized a `StandardTagKey` will be assigned to this `Tag`.
    ///
    /// This is a best effort guess since not all metadata formats have a well defined or specified
    /// tag mapping. However, it is recommended that consumers prefer `std_key` over `key`, if
    /// provided.
    pub std_key: Option<StandardTagKey>,
    /// A key string indicating the type, meaning, or purpose of the `Tag`s value.
    ///
    /// Note: The meaning of `key` is dependant on the underlying metadata format.
    pub key: String,
    /// The value of the `Tag`.
    pub value: Value,
}

impl Tag {
    /// Create a new `Tag`.
    pub fn new(std_key: Option<StandardTagKey>, key: &str, value: Value) -> Tag {
        Tag { std_key, key: key.to_string(), value }
    }

    /// Returns true if the `Tag`'s key string was recognized and a `StandardTagKey` was assigned,
    /// otherwise false is returned.
    pub fn is_known(&self) -> bool {
        self.std_key.is_some()
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.std_key {
            Some(ref std_key) => {
                write!(f, "{{ std_key={:?}, key=\"{}\", value={} }}", std_key, self.key, self.value)
            }
            None => write!(f, "{{ key=\"{}\", value={} }}", self.key, self.value),
        }
    }
}

/// A 2 dimensional (width and height) size type.
#[derive(Copy, Clone, Debug, Default)]
pub struct Size {
    /// The width in pixels.
    pub width: u32,
    /// The height in pixels.
    pub height: u32,
}

/// `ColorMode` indicates how the color of a pixel is encoded in a `Visual`.
#[derive(Copy, Clone, Debug)]
pub enum ColorMode {
    /// Each pixel in the `Visual` stores its own color information.
    Discrete,
    /// Each pixel in the `Visual` stores an index into a color palette containing the color
    /// information. The value stored by this variant indicates the number of colors in the color
    /// palette.
    Indexed(NonZeroU32),
}

/// A `Visual` is any 2 dimensional graphic.
#[derive(Clone, Debug)]
pub struct Visual {
    /// The Media Type (MIME Type) used to encode the `Visual`.
    pub media_type: String,
    /// The dimensions of the `Visual`.
    ///
    /// Note: This value may not be accurate as it comes from metadata, not the embedded graphic
    /// itself. Consider it only a hint.
    pub dimensions: Option<Size>,
    /// The number of bits-per-pixel (aka bit-depth) of the unencoded image.
    ///
    /// Note: This value may not be accurate as it comes from metadata, not the embedded graphic
    /// itself. Consider it only a hint.
    pub bits_per_pixel: Option<NonZeroU32>,
    /// The color mode of the `Visual`.
    ///
    /// Note: This value may not be accurate as it comes from metadata, not the embedded graphic
    /// itself. Consider it only a hint.
    pub color_mode: Option<ColorMode>,
    /// The usage and/or content of the `Visual`.
    pub usage: Option<StandardVisualKey>,
    /// Any tags associated with the `Visual`.
    pub tags: Vec<Tag>,
    /// The data of the `Visual`, encoded as per `media_type`.
    pub data: Box<[u8]>,
}

/// `VendorData` is any binary metadata that is proprietary to a certain application or vendor.
#[derive(Clone, Debug)]
pub struct VendorData {
    /// A text representation of the vendor's application identifier.
    pub ident: String,
    /// The vendor data.
    pub data: Box<[u8]>,
}

/// `Metadata` is a container for a single discrete revision of metadata information.
#[derive(Clone, Debug, Default)]
pub struct MetadataRevision {
    tags: Vec<Tag>,
    visuals: Vec<Visual>,
    vendor_data: Vec<VendorData>,
}

impl MetadataRevision {
    /// Gets an immutable slice to the `Tag`s in this revision.
    ///
    /// If a tag read from the source contained multiple values, then there will be one `Tag` item
    /// per value, with each item having the same key and standard key.
    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }

    /// Gets an immutable slice to the `Visual`s in this revision.
    pub fn visuals(&self) -> &[Visual] {
        &self.visuals
    }

    /// Gets an immutable slice to the `VendorData` in this revision.
    pub fn vendor_data(&self) -> &[VendorData] {
        &self.vendor_data
    }
}

/// `MetadataBuilder` is the builder for `Metadata` revisions.
#[derive(Clone, Debug, Default)]
pub struct MetadataBuilder {
    metadata: MetadataRevision,
}

impl MetadataBuilder {
    /// Instantiate a new `MetadataBuilder`.
    pub fn new() -> Self {
        MetadataBuilder { metadata: Default::default() }
    }

    /// Add a `Tag` to the metadata.
    pub fn add_tag(&mut self, tag: Tag) -> &mut Self {
        self.metadata.tags.push(tag);
        self
    }

    /// Add a `Visual` to the metadata.
    pub fn add_visual(&mut self, visual: Visual) -> &mut Self {
        self.metadata.visuals.push(visual);
        self
    }

    /// Add `VendorData` to the metadata.
    pub fn add_vendor_data(&mut self, vendor_data: VendorData) -> &mut Self {
        self.metadata.vendor_data.push(vendor_data);
        self
    }

    /// Yield the constructed `Metadata` revision.
    pub fn metadata(self) -> MetadataRevision {
        self.metadata
    }
}

/// A reference to the metadata inside of a [MetadataLog].
#[derive(Debug)]
pub struct Metadata<'a> {
    revisions: &'a mut VecDeque<MetadataRevision>,
}

impl Metadata<'_> {
    /// Returns `true` if the current metadata revision is the newest, `false` otherwise.
    pub fn is_latest(&self) -> bool {
        self.revisions.len() <= 1
    }

    /// Gets an immutable reference to the current, and therefore oldest, revision of the metadata.
    pub fn current(&self) -> Option<&MetadataRevision> {
        self.revisions.front()
    }

    /// Skips to, and gets an immutable reference to the latest, and therefore newest, revision of
    /// the metadata.
    pub fn skip_to_latest(&mut self) -> Option<&MetadataRevision> {
        loop {
            if self.pop().is_none() {
                break;
            }
        }
        self.current()
    }

    /// If there are newer `Metadata` revisions, advances the `MetadataLog` by discarding the
    /// current revision and replacing it with the next revision, returning the discarded
    /// `Metadata`. When there are no newer revisions, `None` is returned. As such, `pop` will never
    /// completely empty the log.
    pub fn pop(&mut self) -> Option<MetadataRevision> {
        if self.revisions.len() > 1 {
            self.revisions.pop_front()
        }
        else {
            None
        }
    }
}

/// `MetadataLog` is a container for time-ordered `Metadata` revisions.
#[derive(Clone, Debug, Default)]
pub struct MetadataLog {
    revisions: VecDeque<MetadataRevision>,
}

impl MetadataLog {
    /// Returns a reducable reference to the metadata inside the log.
    pub fn metadata(&mut self) -> Metadata<'_> {
        Metadata { revisions: &mut self.revisions }
    }

    /// Pushes a new `Metadata` revision onto the log.
    pub fn push(&mut self, rev: MetadataRevision) {
        self.revisions.push_back(rev);
    }
}

pub trait MetadataReader: Send + Sync {
    /// Instantiates the `MetadataReader` with the provided `MetadataOptions`.
    fn new(options: &MetadataOptions) -> Self
    where
        Self: Sized;

    /// Read all metadata and return it if successful.
    fn read_all(&mut self, reader: &mut MediaSourceStream) -> Result<MetadataRevision>;
}
