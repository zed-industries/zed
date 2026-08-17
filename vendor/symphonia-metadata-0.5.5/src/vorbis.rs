// Symphonia
// Copyright (c) 2019-2024 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! A Vorbic COMMENT metadata reader for FLAC or OGG formats.

use std::collections::HashMap;

use lazy_static::lazy_static;
use log::warn;

use symphonia_core::errors::Result;
use symphonia_core::io::{BufReader, ReadBytes};
use symphonia_core::meta::{MetadataBuilder, StandardTagKey, Tag, Value};

use crate::flac;

lazy_static! {
    static ref VORBIS_COMMENT_MAP: HashMap<&'static str, StandardTagKey> = {
        let mut m = HashMap::new();
        m.insert("album artist"                , StandardTagKey::AlbumArtist);
        m.insert("album"                       , StandardTagKey::Album);
        m.insert("albumartist"                 , StandardTagKey::AlbumArtist);
        m.insert("albumartistsort"             , StandardTagKey::SortAlbumArtist);
        m.insert("albumsort"                   , StandardTagKey::SortAlbum);
        m.insert("arranger"                    , StandardTagKey::Arranger);
        m.insert("artist"                      , StandardTagKey::Artist);
        m.insert("artistsort"                  , StandardTagKey::SortArtist);
        // TODO: Is Author a synonym for Writer?
        m.insert("author"                      , StandardTagKey::Writer);
        m.insert("barcode"                     , StandardTagKey::IdentBarcode);
        m.insert("bpm"                         , StandardTagKey::Bpm);
        m.insert("catalog #"                   , StandardTagKey::IdentCatalogNumber);
        m.insert("catalog"                     , StandardTagKey::IdentCatalogNumber);
        m.insert("catalognumber"               , StandardTagKey::IdentCatalogNumber);
        m.insert("catalogue #"                 , StandardTagKey::IdentCatalogNumber);
        m.insert("comment"                     , StandardTagKey::Comment);
        m.insert("compileation"                , StandardTagKey::Compilation);
        m.insert("composer"                    , StandardTagKey::Composer);
        m.insert("conductor"                   , StandardTagKey::Conductor);
        m.insert("copyright"                   , StandardTagKey::Copyright);
        m.insert("date"                        , StandardTagKey::Date);
        m.insert("description"                 , StandardTagKey::Description);
        m.insert("disc"                        , StandardTagKey::DiscNumber);
        m.insert("discnumber"                  , StandardTagKey::DiscNumber);
        m.insert("discsubtitle"                , StandardTagKey::DiscSubtitle);
        m.insert("disctotal"                   , StandardTagKey::DiscTotal);
        m.insert("disk"                        , StandardTagKey::DiscNumber);
        m.insert("disknumber"                  , StandardTagKey::DiscNumber);
        m.insert("disksubtitle"                , StandardTagKey::DiscSubtitle);
        m.insert("disktotal"                   , StandardTagKey::DiscTotal);
        m.insert("djmixer"                     , StandardTagKey::MixDj);
        m.insert("ean/upn"                     , StandardTagKey::IdentEanUpn);
        m.insert("encoded-by"                  , StandardTagKey::EncodedBy);
        m.insert("encoder settings"            , StandardTagKey::EncoderSettings);
        m.insert("encoder"                     , StandardTagKey::Encoder);
        m.insert("encoding"                    , StandardTagKey::EncoderSettings);
        m.insert("engineer"                    , StandardTagKey::Engineer);
        m.insert("ensemble"                    , StandardTagKey::Ensemble);
        m.insert("genre"                       , StandardTagKey::Genre);
        m.insert("isrc"                        , StandardTagKey::IdentIsrc);
        m.insert("language"                    , StandardTagKey::Language);
        m.insert("label"                       , StandardTagKey::Label);
        m.insert("license"                     , StandardTagKey::License);
        m.insert("lyricist"                    , StandardTagKey::Lyricist);
        m.insert("lyrics"                      , StandardTagKey::Lyrics);
        m.insert("media"                       , StandardTagKey::MediaFormat);
        m.insert("mixer"                       , StandardTagKey::MixEngineer);
        m.insert("mood"                        , StandardTagKey::Mood);
        m.insert("musicbrainz_albumartistid"   , StandardTagKey::MusicBrainzAlbumArtistId);
        m.insert("musicbrainz_albumid"         , StandardTagKey::MusicBrainzAlbumId);
        m.insert("musicbrainz_artistid"        , StandardTagKey::MusicBrainzArtistId);
        m.insert("musicbrainz_discid"          , StandardTagKey::MusicBrainzDiscId);
        m.insert("musicbrainz_originalalbumid" , StandardTagKey::MusicBrainzOriginalAlbumId);
        m.insert("musicbrainz_originalartistid", StandardTagKey::MusicBrainzOriginalArtistId);
        m.insert("musicbrainz_recordingid"     , StandardTagKey::MusicBrainzRecordingId);
        m.insert("musicbrainz_releasegroupid"  , StandardTagKey::MusicBrainzReleaseGroupId);
        m.insert("musicbrainz_releasetrackid"  , StandardTagKey::MusicBrainzReleaseTrackId);
        m.insert("musicbrainz_trackid"         , StandardTagKey::MusicBrainzTrackId);
        m.insert("musicbrainz_workid"          , StandardTagKey::MusicBrainzWorkId);
        m.insert("opus"                        , StandardTagKey::Opus);
        m.insert("organization"                , StandardTagKey::Label);
        m.insert("originaldate"                , StandardTagKey::OriginalDate);
        m.insert("part"                        , StandardTagKey::Part);
        m.insert("performer"                   , StandardTagKey::Performer);
        m.insert("producer"                    , StandardTagKey::Producer);
        m.insert("productnumber"               , StandardTagKey::IdentPn);
        // TODO: Is Publisher a synonym for Label?
        m.insert("publisher"                   , StandardTagKey::Label);
        m.insert("rating"                      , StandardTagKey::Rating);
        m.insert("releasecountry"              , StandardTagKey::ReleaseCountry);
        m.insert("remixer"                     , StandardTagKey::Remixer);
        m.insert("replaygain_album_gain"       , StandardTagKey::ReplayGainAlbumGain);
        m.insert("replaygain_album_peak"       , StandardTagKey::ReplayGainAlbumPeak);
        m.insert("replaygain_track_gain"       , StandardTagKey::ReplayGainTrackGain);
        m.insert("replaygain_track_peak"       , StandardTagKey::ReplayGainTrackPeak);
        m.insert("script"                      , StandardTagKey::Script);
        m.insert("subtitle"                    , StandardTagKey::TrackSubtitle);
        m.insert("title"                       , StandardTagKey::TrackTitle);
        m.insert("titlesort"                   , StandardTagKey::SortTrackTitle);
        m.insert("totaldiscs"                  , StandardTagKey::DiscTotal);
        m.insert("totaltracks"                 , StandardTagKey::TrackTotal);
        m.insert("tracknumber"                 , StandardTagKey::TrackNumber);
        m.insert("tracktotal"                  , StandardTagKey::TrackTotal);
        m.insert("unsyncedlyrics"              , StandardTagKey::Lyrics);
        m.insert("upc"                         , StandardTagKey::IdentUpc);
        m.insert("version"                     , StandardTagKey::Remixer);
        m.insert("version"                     , StandardTagKey::Version);
        m.insert("writer"                      , StandardTagKey::Writer);
        m.insert("year"                        , StandardTagKey::Date);
        m
    };
}

/// Parse a string containing a base64 encoded FLAC picture block into a visual.
fn parse_base64_picture_block(encoded: &str, metadata: &mut MetadataBuilder) {
    if let Some(data) = base64_decode(encoded) {
        if flac::read_picture_block(&mut BufReader::new(&data), metadata).is_err() {
            warn!("invalid picture block data");
        }
    }
    else {
        warn!("the base64 encoding of a picture block is invalid");
    }
}

/// Parse the given Vorbis Comment string into a `Tag`.
fn parse_comment(tag: &str, metadata: &mut MetadataBuilder) {
    // Vorbis Comments (aka tags) are stored as <key>=<value> where <key> is
    // a reduced ASCII-only identifier and <value> is a UTF8 value.
    //
    // <Key> must only contain ASCII 0x20 through 0x7D, with 0x3D ('=') excluded.
    // ASCII 0x41 through 0x5A inclusive (A-Z) is to be considered equivalent to
    // ASCII 0x61 through 0x7A inclusive (a-z) for tag matching.

    if let Some((key, value)) = tag.split_once('=') {
        let key_lower = key.to_lowercase();

        // A comment with a key "METADATA_BLOCK_PICTURE" is a FLAC picture block encoded in base64.
        // Attempt to decode it as such. If this fails in any way, treat the comment as a regular
        // tag.
        if key_lower == "metadata_block_picture" {
            parse_base64_picture_block(value, metadata);
        }
        else {
            // Attempt to assign a standardized tag key.
            let std_tag = VORBIS_COMMENT_MAP.get(key_lower.as_str()).copied();

            metadata.add_tag(Tag::new(std_tag, key, Value::from(value)));
        }
    }
}

pub fn read_comment_no_framing<B: ReadBytes>(
    reader: &mut B,
    metadata: &mut MetadataBuilder,
) -> Result<()> {
    // Read the vendor string length in bytes.
    let vendor_length = reader.read_u32()?;

    // Ignore the vendor string.
    reader.ignore_bytes(u64::from(vendor_length))?;

    // Read the number of comments.
    let n_comments = reader.read_u32()? as usize;

    for _ in 0..n_comments {
        // Read the comment string length in bytes.
        let comment_length = reader.read_u32()?;

        // Read the comment string.
        let mut comment_bytes = vec![0; comment_length as usize];
        reader.read_buf_exact(&mut comment_bytes)?;

        // Parse the comment string into a Tag and insert it into the parsed tag list.
        parse_comment(&String::from_utf8_lossy(&comment_bytes), metadata);
    }

    Ok(())
}

/// Decode a RFC4648 Base64 encoded string.
fn base64_decode(encoded: &str) -> Option<Box<[u8]>> {
    // A sentinel value indicating that an invalid symbol was encountered.
    const BAD_SYM: u8 = 0xff;

    /// Generates a lookup table mapping RFC4648 base64 symbols to their 6-bit decoded values at
    /// compile time.
    const fn rfc4648_base64_symbols() -> [u8; 256] {
        const SYMBOLS: &[u8; 64] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

        let mut table = [BAD_SYM; 256];
        let mut i = 0;

        while i < SYMBOLS.len() {
            table[SYMBOLS[i] as usize] = i as u8;
            i += 1
        }

        table
    }

    const SYM_VALUE: [u8; 256] = rfc4648_base64_symbols();

    // Trim padding, since it's not required for decoding.
    let encoded = encoded.trim_end_matches('=');

    // Each valid base64 symbol decodes to 6 bits. Therefore, the decoded byte length is 3 / 4 the
    // number of symbols in the base64 encoded string.
    let mut decoded = Vec::with_capacity((encoded.len() * 3) / 4);

    // Decode in chunks of 4 symbols, yielding 3 bytes per chunk. Since base64 symbols are ASCII
    // characters (1 byte per character), iterate over the bytes of the base64 string instead of
    // chars (4 bytes per character). This allows the use of a lookup table to determine the symbol
    // value.
    let mut iter = encoded.as_bytes().chunks_exact(4);

    for enc in &mut iter {
        let v0 = SYM_VALUE[usize::from(enc[0])];
        let v1 = SYM_VALUE[usize::from(enc[1])];
        let v2 = SYM_VALUE[usize::from(enc[2])];
        let v3 = SYM_VALUE[usize::from(enc[3])];

        // Check for invalid symbols.
        if v0 == BAD_SYM || v1 == BAD_SYM || v2 == BAD_SYM || v3 == BAD_SYM {
            return None;
        }

        // 6 bits from v0, 2 bits from v1 (4 remaining).
        decoded.push(((v0 & 0x3f) << 2) | (v1 >> 4));
        // 4 bits from v1, 4 bits from v2 (2 remaining).
        decoded.push(((v1 & 0x0f) << 4) | (v2 >> 2));
        // 2 bits from v2, 6 bits from v3 (0 remaining).
        decoded.push(((v2 & 0x03) << 6) | (v3 >> 0));
    }

    // Decode the remaining 2 to 3 symbols.
    let rem = iter.remainder();

    // If there are atleast 2 symbols remaining, then a minimum of one extra byte may be decoded.
    if rem.len() >= 2 {
        let v0 = SYM_VALUE[usize::from(rem[0])];
        let v1 = SYM_VALUE[usize::from(rem[1])];

        if v0 == BAD_SYM || v1 == BAD_SYM {
            return None;
        }

        decoded.push(((v0 & 0x3f) << 2) | (v1 >> 4));

        // If there were 3 symbols remaining, then one additional byte may be decoded.
        if rem.len() >= 3 {
            let v2 = SYM_VALUE[usize::from(rem[2])];

            if v2 == BAD_SYM {
                return None;
            }

            decoded.push(((v1 & 0x0f) << 4) | (v2 >> 2));
        }
    }
    else if rem.len() == 1 {
        // Atleast 2 symbols are required to decode a single byte. Therefore, this is an error.
        return None;
    }

    Some(decoded.into_boxed_slice())
}

#[cfg(test)]
mod tests {
    use super::base64_decode;

    #[test]
    fn verify_base64_decode() {
        // Valid, with padding.
        assert_eq!(Some(b"".as_slice()), base64_decode("").as_deref());
        assert_eq!(Some(b"f".as_slice()), base64_decode("Zg==").as_deref());
        assert_eq!(Some(b"fo".as_slice()), base64_decode("Zm8=").as_deref());
        assert_eq!(Some(b"foo".as_slice()), base64_decode("Zm9v").as_deref());
        assert_eq!(Some(b"foob".as_slice()), base64_decode("Zm9vYg==").as_deref());
        assert_eq!(Some(b"fooba".as_slice()), base64_decode("Zm9vYmE=").as_deref());
        assert_eq!(Some(b"foobar".as_slice()), base64_decode("Zm9vYmFy").as_deref());
        // Valid, without padding.
        assert_eq!(Some(b"".as_slice()), base64_decode("").as_deref());
        assert_eq!(Some(b"f".as_slice()), base64_decode("Zg").as_deref());
        assert_eq!(Some(b"fo".as_slice()), base64_decode("Zm8").as_deref());
        assert_eq!(Some(b"foo".as_slice()), base64_decode("Zm9v").as_deref());
        assert_eq!(Some(b"foob".as_slice()), base64_decode("Zm9vYg").as_deref());
        assert_eq!(Some(b"fooba".as_slice()), base64_decode("Zm9vYmE").as_deref());
        assert_eq!(Some(b"foobar".as_slice()), base64_decode("Zm9vYmFy").as_deref());
        // Invalid.
        assert_eq!(None, base64_decode("a").as_deref());
        assert_eq!(None, base64_decode("ab!c").as_deref());
        assert_eq!(None, base64_decode("ab=c").as_deref());
    }
}
