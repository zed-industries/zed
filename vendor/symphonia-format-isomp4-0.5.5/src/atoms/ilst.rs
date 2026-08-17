// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::{decode_error, Result};
use symphonia_core::io::{BufReader, ReadBytes};
use symphonia_core::meta::{
    MetadataBuilder, MetadataRevision, StandardTagKey, StandardVisualKey, Tag,
};
use symphonia_core::meta::{Value, Visual};
use symphonia_core::util::bits;
use symphonia_metadata::{id3v1, itunes};

use crate::atoms::{Atom, AtomHeader, AtomIterator, AtomType};

use encoding_rs::{SHIFT_JIS, UTF_16BE};
use log::warn;

/// Data type enumeration for metadata value atoms as defined in the QuickTime File Format standard.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum DataType {
    AffineTransformF64,
    Bmp,
    DimensionsF32,
    Float32,
    Float64,
    Jpeg,
    /// The data type is implicit to the atom.
    NoType,
    Png,
    PointF32,
    QuickTimeMetadata,
    RectF32,
    ShiftJis,
    SignedInt16,
    SignedInt32,
    SignedInt64,
    SignedInt8,
    SignedIntVariable,
    UnsignedInt16,
    UnsignedInt32,
    UnsignedInt64,
    UnsignedInt8,
    UnsignedIntVariable,
    Utf16,
    Utf16Sort,
    Utf8,
    Utf8Sort,
    Unknown(u32),
}

impl From<u32> for DataType {
    fn from(value: u32) -> Self {
        match value {
            0 => DataType::NoType,
            1 => DataType::Utf8,
            2 => DataType::Utf16,
            3 => DataType::ShiftJis,
            4 => DataType::Utf8Sort,
            5 => DataType::Utf16Sort,
            13 => DataType::Jpeg,
            14 => DataType::Png,
            21 => DataType::SignedIntVariable,
            22 => DataType::UnsignedIntVariable,
            23 => DataType::Float32,
            24 => DataType::Float64,
            27 => DataType::Bmp,
            28 => DataType::QuickTimeMetadata,
            65 => DataType::SignedInt8,
            66 => DataType::SignedInt16,
            67 => DataType::SignedInt32,
            70 => DataType::PointF32,
            71 => DataType::DimensionsF32,
            72 => DataType::RectF32,
            74 => DataType::SignedInt64,
            75 => DataType::UnsignedInt8,
            76 => DataType::UnsignedInt16,
            77 => DataType::UnsignedInt32,
            78 => DataType::UnsignedInt64,
            79 => DataType::AffineTransformF64,
            _ => DataType::Unknown(value),
        }
    }
}

fn parse_no_type(data: &[u8]) -> Option<Value> {
    // Latin1, potentially null-terminated.
    let end = data.iter().position(|&c| c == b'\0').unwrap_or(data.len());
    let text = String::from_utf8_lossy(&data[..end]);
    Some(Value::from(text))
}

fn parse_utf8(data: &[u8]) -> Option<Value> {
    // UTF8, no null-terminator or count.
    let text = String::from_utf8_lossy(data);
    Some(Value::from(text))
}

fn parse_utf16(data: &[u8]) -> Option<Value> {
    // UTF16 BE
    let text = UTF_16BE.decode(data).0;
    Some(Value::from(text))
}

fn parse_shift_jis(data: &[u8]) -> Option<Value> {
    // Shift-JIS
    let text = SHIFT_JIS.decode(data).0;
    Some(Value::from(text))
}

fn parse_signed_int8(data: &[u8]) -> Option<Value> {
    match data.len() {
        1 => {
            let s = bits::sign_extend_leq8_to_i8(data[0], 8);
            Some(Value::from(s))
        }
        _ => None,
    }
}

fn parse_signed_int16(data: &[u8]) -> Option<Value> {
    match data.len() {
        2 => {
            let u = BufReader::new(data).read_be_u16().ok()?;
            let s = bits::sign_extend_leq16_to_i16(u, 16);
            Some(Value::from(s))
        }
        _ => None,
    }
}

fn parse_signed_int32(data: &[u8]) -> Option<Value> {
    match data.len() {
        4 => {
            let u = BufReader::new(data).read_be_u32().ok()?;
            let s = bits::sign_extend_leq32_to_i32(u, 32);
            Some(Value::from(s))
        }
        _ => None,
    }
}

fn parse_signed_int64(data: &[u8]) -> Option<Value> {
    match data.len() {
        8 => {
            let u = BufReader::new(data).read_be_u64().ok()?;
            let s = bits::sign_extend_leq64_to_i64(u, 64);
            Some(Value::from(s))
        }
        _ => None,
    }
}

fn parse_var_signed_int(data: &[u8]) -> Option<Value> {
    match data.len() {
        1 => parse_signed_int8(data),
        2 => parse_signed_int16(data),
        4 => parse_signed_int32(data),
        _ => None,
    }
}

fn parse_unsigned_int8(data: &[u8]) -> Option<Value> {
    match data.len() {
        1 => Some(Value::from(data[0])),
        _ => None,
    }
}

fn parse_unsigned_int16(data: &[u8]) -> Option<Value> {
    match data.len() {
        2 => {
            let u = BufReader::new(data).read_be_u16().ok()?;
            Some(Value::from(u))
        }
        _ => None,
    }
}

fn parse_unsigned_int32(data: &[u8]) -> Option<Value> {
    match data.len() {
        4 => {
            let u = BufReader::new(data).read_be_u32().ok()?;
            Some(Value::from(u))
        }
        _ => None,
    }
}

fn parse_unsigned_int64(data: &[u8]) -> Option<Value> {
    match data.len() {
        8 => {
            let u = BufReader::new(data).read_be_u64().ok()?;
            Some(Value::from(u))
        }
        _ => None,
    }
}

fn parse_var_unsigned_int(data: &[u8]) -> Option<Value> {
    match data.len() {
        1 => parse_unsigned_int8(data),
        2 => parse_unsigned_int16(data),
        4 => parse_unsigned_int32(data),
        _ => None,
    }
}

fn parse_float32(data: &[u8]) -> Option<Value> {
    match data.len() {
        4 => {
            let f = BufReader::new(data).read_be_f32().ok()?;
            Some(Value::Float(f64::from(f)))
        }
        _ => None,
    }
}

fn parse_float64(data: &[u8]) -> Option<Value> {
    match data.len() {
        8 => {
            let f = BufReader::new(data).read_be_f64().ok()?;
            Some(Value::Float(f))
        }
        _ => None,
    }
}

fn parse_tag_value(data_type: DataType, data: &[u8]) -> Option<Value> {
    match data_type {
        DataType::NoType => parse_no_type(data),
        DataType::Utf8 | DataType::Utf8Sort => parse_utf8(data),
        DataType::Utf16 | DataType::Utf16Sort => parse_utf16(data),
        DataType::ShiftJis => parse_shift_jis(data),
        DataType::UnsignedInt8 => parse_unsigned_int8(data),
        DataType::UnsignedInt16 => parse_unsigned_int16(data),
        DataType::UnsignedInt32 => parse_unsigned_int32(data),
        DataType::UnsignedInt64 => parse_unsigned_int64(data),
        DataType::UnsignedIntVariable => parse_var_unsigned_int(data),
        DataType::SignedInt8 => parse_signed_int8(data),
        DataType::SignedInt16 => parse_signed_int16(data),
        DataType::SignedInt32 => parse_signed_int32(data),
        DataType::SignedInt64 => parse_signed_int64(data),
        DataType::SignedIntVariable => parse_var_signed_int(data),
        DataType::Float32 => parse_float32(data),
        DataType::Float64 => parse_float64(data),
        _ => None,
    }
}

/// Reads and parses a `MetaTagAtom` from the provided iterator and adds it to the `MetadataBuilder`
/// if there are no errors.
fn add_generic_tag<B: ReadBytes>(
    iter: &mut AtomIterator<B>,
    builder: &mut MetadataBuilder,
    std_key: Option<StandardTagKey>,
) -> Result<()> {
    let tag = iter.read_atom::<MetaTagAtom>()?;

    for value_atom in tag.values.iter() {
        // Parse the value atom data into a string, if possible.
        if let Some(value) = parse_tag_value(value_atom.data_type, &value_atom.data) {
            builder.add_tag(Tag::new(std_key, "", value));
        }
        else {
            warn!("unsupported data type {:?} for {:?} tag", value_atom.data_type, std_key);
        }
    }

    Ok(())
}

fn add_var_unsigned_int_tag<B: ReadBytes>(
    iter: &mut AtomIterator<B>,
    builder: &mut MetadataBuilder,
    std_key: StandardTagKey,
) -> Result<()> {
    let tag = iter.read_atom::<MetaTagAtom>()?;

    if let Some(value_atom) = tag.values.first() {
        if let Some(value) = parse_var_unsigned_int(&value_atom.data) {
            builder.add_tag(Tag::new(Some(std_key), "", value));
        }
        else {
            warn!("got unexpected data for {:?} tag", std_key);
        }
    }

    Ok(())
}

fn add_var_signed_int_tag<B: ReadBytes>(
    iter: &mut AtomIterator<B>,
    builder: &mut MetadataBuilder,
    std_key: StandardTagKey,
) -> Result<()> {
    let tag = iter.read_atom::<MetaTagAtom>()?;

    if let Some(value_atom) = tag.values.first() {
        if let Some(value) = parse_var_signed_int(&value_atom.data) {
            builder.add_tag(Tag::new(Some(std_key), "", value));
        }
        else {
            warn!("got unexpected data for {:?} tag", std_key);
        }
    }

    Ok(())
}

fn add_boolean_tag<B: ReadBytes>(
    iter: &mut AtomIterator<B>,
    builder: &mut MetadataBuilder,
    std_key: StandardTagKey,
) -> Result<()> {
    let tag = iter.read_atom::<MetaTagAtom>()?;

    // There should only be 1 value.
    if let Some(value) = tag.values.first() {
        // Boolean tags are just "flags", only add a tag if the boolean is true (1).
        if let Some(bool_value) = value.data.first() {
            if *bool_value == 1 {
                builder.add_tag(Tag::new(Some(std_key), "", Value::Flag));
            }
        }
    }

    Ok(())
}

fn add_m_of_n_tag<B: ReadBytes>(
    iter: &mut AtomIterator<B>,
    builder: &mut MetadataBuilder,
    m_key: StandardTagKey,
    n_key: StandardTagKey,
) -> Result<()> {
    let tag = iter.read_atom::<MetaTagAtom>()?;

    // There should only be 1 value.
    if let Some(value) = tag.values.first() {
        // The trkn and disk atoms contains an 8 byte value buffer, where the 4th and 6th bytes
        // indicate the track/disk number and total number of tracks/disks, respectively. Odd.
        if value.data.len() == 8 {
            let m = value.data[3];
            let n = value.data[5];

            builder.add_tag(Tag::new(Some(m_key), "", Value::from(m)));
            builder.add_tag(Tag::new(Some(n_key), "", Value::from(n)));
        }
    }

    Ok(())
}

fn add_visual_tag<B: ReadBytes>(
    iter: &mut AtomIterator<B>,
    builder: &mut MetadataBuilder,
) -> Result<()> {
    let tag = iter.read_atom::<MetaTagAtom>()?;

    // There could be more than one attached image.
    for value in tag.values {
        let media_type = match value.data_type {
            DataType::Bmp => "image/bmp",
            DataType::Jpeg => "image/jpeg",
            DataType::Png => "image/png",
            _ => "",
        };

        builder.add_visual(Visual {
            media_type: media_type.into(),
            dimensions: None,
            bits_per_pixel: None,
            color_mode: None,
            usage: Some(StandardVisualKey::FrontCover),
            tags: Default::default(),
            data: value.data,
        });
    }

    Ok(())
}

fn add_advisory_tag<B: ReadBytes>(
    _iter: &mut AtomIterator<B>,
    _builder: &mut MetadataBuilder,
) -> Result<()> {
    Ok(())
}

fn add_media_type_tag<B: ReadBytes>(
    iter: &mut AtomIterator<B>,
    builder: &mut MetadataBuilder,
) -> Result<()> {
    let tag = iter.read_atom::<MetaTagAtom>()?;

    // There should only be 1 value.
    if let Some(value) = tag.values.first() {
        if let Some(media_type_value) = value.data.first() {
            let media_type = match media_type_value {
                0 => "Movie",
                1 => "Normal",
                2 => "Audio Book",
                5 => "Whacked Bookmark",
                6 => "Music Video",
                9 => "Short Film",
                10 => "TV Show",
                11 => "Booklet",
                _ => "Unknown",
            };

            builder.add_tag(Tag::new(
                Some(StandardTagKey::MediaFormat),
                "",
                Value::from(media_type),
            ));
        }
    }

    Ok(())
}

fn add_id3v1_genre_tag<B: ReadBytes>(
    iter: &mut AtomIterator<B>,
    builder: &mut MetadataBuilder,
) -> Result<()> {
    let tag = iter.read_atom::<MetaTagAtom>()?;

    // There should only be 1 value.
    if let Some(value) = tag.values.first() {
        // The ID3v1 genre is stored as a unsigned 16-bit big-endian integer.
        let index = BufReader::new(&value.data).read_be_u16()?;

        // The stored index uses 1-based indexing, but the ID3v1 genre list is 0-based.
        if index > 0 {
            if let Some(genre) = id3v1::util::genre_name((index - 1) as u8) {
                builder.add_tag(Tag::new(Some(StandardTagKey::Genre), "", Value::from(*genre)));
            }
        }
    }

    Ok(())
}

fn add_freeform_tag<B: ReadBytes>(
    iter: &mut AtomIterator<B>,
    builder: &mut MetadataBuilder,
) -> Result<()> {
    let tag = iter.read_atom::<MetaTagAtom>()?;

    // A user-defined tag should only have 1 value.
    for value_atom in tag.values.iter() {
        // Parse the value atom data into a string, if possible.
        if let Some(value) = parse_tag_value(value_atom.data_type, &value_atom.data) {
            // Gets the fully qualified tag name.
            let full_name = tag.full_name();

            // Try to map iTunes freeform tags to standard tag keys.
            let std_key = itunes::std_key_from_tag(&full_name);

            builder.add_tag(Tag::new(std_key, &full_name, value));
        }
        else {
            warn!("unsupported data type {:?} for free-form tag", value_atom.data_type);
        }
    }

    Ok(())
}

/// Metadata tag data atom.
#[allow(dead_code)]
pub struct MetaTagDataAtom {
    /// Atom header.
    header: AtomHeader,
    /// Tag data.
    pub data: Box<[u8]>,
    /// The data type contained in buf.
    pub data_type: DataType,
}

impl Atom for MetaTagDataAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let (version, flags) = AtomHeader::read_extra(reader)?;

        // For the mov brand, this a data type indicator and must always be 0 (well-known type). It
        // specifies the table in which the next 24-bit integer specifying the actual data type
        // indexes. For iso/mp4, this is a version, and there is only one version, 0. Therefore,
        // flags are interpreted as the actual data type index.
        if version != 0 {
            return decode_error("isomp4: invalid data atom version");
        }

        let data_type = DataType::from(flags);

        // For the mov brand, the next four bytes are country and languages code. However, for
        // iso/mp4 these codes should be ignored.
        let _country = reader.read_be_u16()?;
        let _language = reader.read_be_u16()?;

        // The data payload is the remainder of the atom.
        // TODO: Apply a limit.
        let data = reader
            .read_boxed_slice_exact((header.data_len - AtomHeader::EXTRA_DATA_SIZE - 4) as usize)?;

        Ok(MetaTagDataAtom { header, data, data_type })
    }
}

/// Metadata tag name and mean atom.
#[allow(dead_code)]
pub struct MetaTagNamespaceAtom {
    /// Atom header.
    header: AtomHeader,
    /// For 'mean' atoms, this is the key namespace. For 'name' atom, this is the key name.
    pub value: String,
}

impl Atom for MetaTagNamespaceAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let (_, _) = AtomHeader::read_extra(reader)?;

        let buf = reader
            .read_boxed_slice_exact((header.data_len - AtomHeader::EXTRA_DATA_SIZE) as usize)?;

        // Do a lossy conversion because metadata should not prevent the demuxer from working.
        let value = String::from_utf8_lossy(&buf).to_string();

        Ok(MetaTagNamespaceAtom { header, value })
    }
}

/// A generic metadata tag atom.
#[allow(dead_code)]
pub struct MetaTagAtom {
    /// Atom header.
    header: AtomHeader,
    /// Tag value(s).
    pub values: Vec<MetaTagDataAtom>,
    /// Optional, tag key namespace.
    pub mean: Option<MetaTagNamespaceAtom>,
    /// Optional, tag key name.
    pub name: Option<MetaTagNamespaceAtom>,
}

impl MetaTagAtom {
    pub fn full_name(&self) -> String {
        let mut full_name = String::new();

        if self.mean.is_some() || self.name.is_some() {
            // full_name.push_str("----:");

            if let Some(mean) = &self.mean {
                full_name.push_str(&mean.value);
            }

            full_name.push(':');

            if let Some(name) = &self.name {
                full_name.push_str(&name.value);
            }
        }

        full_name
    }
}

impl Atom for MetaTagAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let mut iter = AtomIterator::new(reader, header);

        let mut mean = None;
        let mut name = None;
        let mut values = Vec::new();

        while let Some(header) = iter.next()? {
            match header.atype {
                AtomType::MetaTagData => {
                    values.push(iter.read_atom::<MetaTagDataAtom>()?);
                }
                AtomType::MetaTagName => {
                    name = Some(iter.read_atom::<MetaTagNamespaceAtom>()?);
                }
                AtomType::MetaTagMeaning => {
                    mean = Some(iter.read_atom::<MetaTagNamespaceAtom>()?);
                }
                _ => (),
            }
        }

        Ok(MetaTagAtom { header, values, mean, name })
    }
}

/// User data atom.
#[allow(dead_code)]
pub struct IlstAtom {
    /// Atom header.
    header: AtomHeader,
    /// Metadata revision.
    pub metadata: MetadataRevision,
}

impl Atom for IlstAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let mut iter = AtomIterator::new(reader, header);

        let mut mb = MetadataBuilder::new();

        while let Some(header) = iter.next()? {
            // Ignore standard atoms, check if other is a metadata atom.
            match &header.atype {
                AtomType::AdvisoryTag => add_advisory_tag(&mut iter, &mut mb)?,
                AtomType::AlbumArtistTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::AlbumArtist))?
                }
                AtomType::AlbumTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::Album))?
                }
                AtomType::ArtistLowerTag => (),
                AtomType::ArtistTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::Artist))?
                }
                AtomType::CategoryTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::PodcastCategory))?
                }
                AtomType::CommentTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::Comment))?
                }
                AtomType::CompilationTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::Compilation))?
                }
                AtomType::ComposerTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::Composer))?
                }
                AtomType::CopyrightTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::Copyright))?
                }
                AtomType::CoverTag => add_visual_tag(&mut iter, &mut mb)?,
                AtomType::CustomGenreTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::Genre))?
                }
                AtomType::DateTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::Date))?
                }
                AtomType::DescriptionTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::Description))?
                }
                AtomType::DiskNumberTag => add_m_of_n_tag(
                    &mut iter,
                    &mut mb,
                    StandardTagKey::DiscNumber,
                    StandardTagKey::DiscTotal,
                )?,
                AtomType::EncodedByTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::EncodedBy))?
                }
                AtomType::EncoderTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::Encoder))?
                }
                AtomType::GaplessPlaybackTag => {
                    // TODO: Need standard tag key for gapless playback.
                    // add_boolean_tag(&mut iter, &mut mb, )?
                }
                AtomType::GenreTag => add_id3v1_genre_tag(&mut iter, &mut mb)?,
                AtomType::GroupingTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::ContentGroup))?
                }
                AtomType::HdVideoTag => (),
                AtomType::IdentPodcastTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::IdentPodcast))?
                }
                AtomType::KeywordTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::PodcastKeywords))?
                }
                AtomType::LongDescriptionTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::Description))?
                }
                AtomType::LyricsTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::Lyrics))?
                }
                AtomType::MediaTypeTag => add_media_type_tag(&mut iter, &mut mb)?,
                AtomType::OwnerTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::Owner))?
                }
                AtomType::PodcastTag => {
                    add_boolean_tag(&mut iter, &mut mb, StandardTagKey::Podcast)?
                }
                AtomType::PurchaseDateTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::PurchaseDate))?
                }
                AtomType::RatingTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::Rating))?
                }
                AtomType::SortAlbumArtistTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::SortAlbumArtist))?
                }
                AtomType::SortAlbumTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::SortAlbum))?
                }
                AtomType::SortArtistTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::SortArtist))?
                }
                AtomType::SortComposerTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::SortComposer))?
                }
                AtomType::SortNameTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::SortTrackTitle))?
                }
                AtomType::TempoTag => {
                    add_var_signed_int_tag(&mut iter, &mut mb, StandardTagKey::Bpm)?
                }
                AtomType::TrackNumberTag => add_m_of_n_tag(
                    &mut iter,
                    &mut mb,
                    StandardTagKey::TrackNumber,
                    StandardTagKey::TrackTotal,
                )?,
                AtomType::TrackTitleTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::TrackTitle))?
                }
                AtomType::TvEpisodeNameTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::TvEpisodeTitle))?
                }
                AtomType::TvEpisodeNumberTag => {
                    add_var_unsigned_int_tag(&mut iter, &mut mb, StandardTagKey::TvEpisode)?
                }
                AtomType::TvNetworkNameTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::TvNetwork))?
                }
                AtomType::TvSeasonNumberTag => {
                    add_var_unsigned_int_tag(&mut iter, &mut mb, StandardTagKey::TvSeason)?
                }
                AtomType::TvShowNameTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::TvShowTitle))?
                }
                AtomType::UrlPodcastTag => {
                    add_generic_tag(&mut iter, &mut mb, Some(StandardTagKey::UrlPodcast))?
                }
                AtomType::FreeFormTag => add_freeform_tag(&mut iter, &mut mb)?,
                _ => (),
            }
        }

        Ok(IlstAtom { header, metadata: mb.metadata() })
    }
}
