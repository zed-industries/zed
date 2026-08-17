// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::{decode_error, Result};
use symphonia_core::io::ReadBytes;

pub(crate) mod alac;
pub(crate) mod co64;
pub(crate) mod ctts;
pub(crate) mod edts;
pub(crate) mod elst;
pub(crate) mod esds;
pub(crate) mod flac;
pub(crate) mod ftyp;
pub(crate) mod hdlr;
pub(crate) mod ilst;
pub(crate) mod mdhd;
pub(crate) mod mdia;
pub(crate) mod mehd;
pub(crate) mod meta;
pub(crate) mod mfhd;
pub(crate) mod minf;
pub(crate) mod moof;
pub(crate) mod moov;
pub(crate) mod mvex;
pub(crate) mod mvhd;
pub(crate) mod opus;
pub(crate) mod sidx;
pub(crate) mod smhd;
pub(crate) mod stbl;
pub(crate) mod stco;
pub(crate) mod stsc;
pub(crate) mod stsd;
pub(crate) mod stss;
pub(crate) mod stsz;
pub(crate) mod stts;
pub(crate) mod tfhd;
pub(crate) mod tkhd;
pub(crate) mod traf;
pub(crate) mod trak;
pub(crate) mod trex;
pub(crate) mod trun;
pub(crate) mod udta;
pub(crate) mod wave;

pub use self::meta::MetaAtom;
pub use alac::AlacAtom;
pub use co64::Co64Atom;
#[allow(unused_imports)]
pub use ctts::CttsAtom;
pub use edts::EdtsAtom;
pub use elst::ElstAtom;
pub use esds::EsdsAtom;
pub use flac::FlacAtom;
pub use ftyp::FtypAtom;
pub use hdlr::HdlrAtom;
pub use ilst::IlstAtom;
pub use mdhd::MdhdAtom;
pub use mdia::MdiaAtom;
pub use mehd::MehdAtom;
pub use mfhd::MfhdAtom;
pub use minf::MinfAtom;
pub use moof::MoofAtom;
pub use moov::MoovAtom;
pub use mvex::MvexAtom;
pub use mvhd::MvhdAtom;
pub use opus::OpusAtom;
pub use sidx::SidxAtom;
pub use smhd::SmhdAtom;
pub use stbl::StblAtom;
pub use stco::StcoAtom;
pub use stsc::StscAtom;
pub use stsd::StsdAtom;
#[allow(unused_imports)]
pub use stss::StssAtom;
pub use stsz::StszAtom;
pub use stts::SttsAtom;
pub use tfhd::TfhdAtom;
pub use tkhd::TkhdAtom;
pub use traf::TrafAtom;
pub use trak::TrakAtom;
pub use trex::TrexAtom;
pub use trun::TrunAtom;
pub use udta::UdtaAtom;
pub use wave::WaveAtom;

/// Atom types.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AtomType {
    Ac3,
    AdvisoryTag,
    Alac,
    ALaw,
    AlbumArtistTag,
    AlbumTag,
    ArtistLowerTag,
    ArtistTag,
    CategoryTag,
    ChunkOffset,
    ChunkOffset64,
    CommentTag,
    CompilationTag,
    ComposerTag,
    CompositionTimeToSample,
    CopyrightTag,
    CoverTag,
    CustomGenreTag,
    DateTag,
    DescriptionTag,
    DiskNumberTag,
    Edit,
    EditList,
    EncodedByTag,
    EncoderTag,
    Esds,
    F32SampleEntry,
    F64SampleEntry,
    FileType,
    Flac,
    FlacDsConfig,
    Free,
    FreeFormTag,
    GaplessPlaybackTag,
    GenreTag,
    GroupingTag,
    Handler,
    HdVideoTag,
    IdentPodcastTag,
    KeywordTag,
    LongDescriptionTag,
    Lpcm,
    LyricsTag,
    Media,
    MediaData,
    MediaHeader,
    MediaInfo,
    MediaTypeTag,
    Meta,
    MetaList,
    MetaTagData,
    MetaTagMeaning,
    MetaTagName,
    Movie,
    MovieExtends,
    MovieExtendsHeader,
    MovieFragment,
    MovieFragmentHeader,
    MovieHeader,
    Mp3,
    Mp4a,
    MuLaw,
    Opus,
    OpusDsConfig,
    OwnerTag,
    PodcastTag,
    PurchaseDateTag,
    QtWave,
    RatingTag,
    S16BeSampleEntry,
    S16LeSampleEntry,
    S24SampleEntry,
    S32SampleEntry,
    SampleDescription,
    SampleSize,
    SampleTable,
    SampleToChunk,
    SegmentIndex,
    Skip,
    SortAlbumArtistTag,
    SortAlbumTag,
    SortArtistTag,
    SortComposerTag,
    SortNameTag,
    SoundMediaHeader,
    SyncSample,
    TempoTag,
    TimeToSample,
    Track,
    TrackExtends,
    TrackFragment,
    TrackFragmentHeader,
    TrackFragmentRun,
    TrackHeader,
    TrackNumberTag,
    TrackTitleTag,
    TvEpisodeNameTag,
    TvEpisodeNumberTag,
    TvNetworkNameTag,
    TvSeasonNumberTag,
    TvShowNameTag,
    U8SampleEntry,
    UrlPodcastTag,
    UserData,
    Other([u8; 4]),
}

impl From<[u8; 4]> for AtomType {
    fn from(val: [u8; 4]) -> Self {
        match &val {
            b".mp3" => AtomType::Mp3,
            b"ac-3" => AtomType::Ac3,
            b"alac" => AtomType::Alac,
            b"alaw" => AtomType::ALaw,
            b"co64" => AtomType::ChunkOffset64,
            b"ctts" => AtomType::CompositionTimeToSample,
            b"data" => AtomType::MetaTagData,
            b"dfLa" => AtomType::FlacDsConfig,
            b"dOps" => AtomType::OpusDsConfig,
            b"edts" => AtomType::Edit,
            b"elst" => AtomType::EditList,
            b"esds" => AtomType::Esds,
            b"fl32" => AtomType::F32SampleEntry,
            b"fl64" => AtomType::F64SampleEntry,
            b"fLaC" => AtomType::Flac,
            b"free" => AtomType::Free,
            b"ftyp" => AtomType::FileType,
            b"hdlr" => AtomType::Handler,
            b"ilst" => AtomType::MetaList,
            b"in24" => AtomType::S24SampleEntry,
            b"in32" => AtomType::S32SampleEntry,
            b"lpcm" => AtomType::Lpcm,
            b"mdat" => AtomType::MediaData,
            b"mdhd" => AtomType::MediaHeader,
            b"mdia" => AtomType::Media,
            b"mean" => AtomType::MetaTagMeaning,
            b"mehd" => AtomType::MovieExtendsHeader,
            b"meta" => AtomType::Meta,
            b"mfhd" => AtomType::MovieFragmentHeader,
            b"minf" => AtomType::MediaInfo,
            b"moof" => AtomType::MovieFragment,
            b"moov" => AtomType::Movie,
            b"mp4a" => AtomType::Mp4a,
            b"mvex" => AtomType::MovieExtends,
            b"mvhd" => AtomType::MovieHeader,
            b"name" => AtomType::MetaTagName,
            b"Opus" => AtomType::Opus,
            b"raw " => AtomType::U8SampleEntry,
            b"sidx" => AtomType::SegmentIndex,
            b"skip" => AtomType::Skip,
            b"smhd" => AtomType::SoundMediaHeader,
            b"sowt" => AtomType::S16LeSampleEntry,
            b"stbl" => AtomType::SampleTable,
            b"stco" => AtomType::ChunkOffset,
            b"stsc" => AtomType::SampleToChunk,
            b"stsd" => AtomType::SampleDescription,
            b"stss" => AtomType::SyncSample,
            b"stsz" => AtomType::SampleSize,
            b"stts" => AtomType::TimeToSample,
            b"tfhd" => AtomType::TrackFragmentHeader,
            b"tkhd" => AtomType::TrackHeader,
            b"traf" => AtomType::TrackFragment,
            b"trak" => AtomType::Track,
            b"trex" => AtomType::TrackExtends,
            b"trun" => AtomType::TrackFragmentRun,
            b"twos" => AtomType::S16BeSampleEntry,
            b"udta" => AtomType::UserData,
            b"ulaw" => AtomType::MuLaw,
            b"wave" => AtomType::QtWave,
            // Metadata Boxes
            b"----" => AtomType::FreeFormTag,
            b"aART" => AtomType::AlbumArtistTag,
            b"catg" => AtomType::CategoryTag,
            b"covr" => AtomType::CoverTag,
            b"cpil" => AtomType::CompilationTag,
            b"cprt" => AtomType::CopyrightTag,
            b"desc" => AtomType::DescriptionTag,
            b"disk" => AtomType::DiskNumberTag,
            b"egid" => AtomType::IdentPodcastTag,
            b"gnre" => AtomType::GenreTag,
            b"hdvd" => AtomType::HdVideoTag,
            b"keyw" => AtomType::KeywordTag,
            b"ldes" => AtomType::LongDescriptionTag,
            b"ownr" => AtomType::OwnerTag,
            b"pcst" => AtomType::PodcastTag,
            b"pgap" => AtomType::GaplessPlaybackTag,
            b"purd" => AtomType::PurchaseDateTag,
            b"purl" => AtomType::UrlPodcastTag,
            b"rate" => AtomType::RatingTag,
            b"rtng" => AtomType::AdvisoryTag,
            b"soaa" => AtomType::SortAlbumArtistTag,
            b"soal" => AtomType::SortAlbumTag,
            b"soar" => AtomType::SortArtistTag,
            b"soco" => AtomType::SortComposerTag,
            b"sonm" => AtomType::SortNameTag,
            b"stik" => AtomType::MediaTypeTag,
            b"tmpo" => AtomType::TempoTag,
            b"trkn" => AtomType::TrackNumberTag,
            b"tven" => AtomType::TvEpisodeNameTag,
            b"tves" => AtomType::TvEpisodeNumberTag,
            b"tvnn" => AtomType::TvNetworkNameTag,
            b"tvsh" => AtomType::TvShowNameTag,
            b"tvsn" => AtomType::TvSeasonNumberTag,
            b"\xa9alb" => AtomType::AlbumTag,
            b"\xa9art" => AtomType::ArtistLowerTag,
            b"\xa9ART" => AtomType::ArtistTag,
            b"\xa9cmt" => AtomType::CommentTag,
            b"\xa9day" => AtomType::DateTag,
            b"\xa9enc" => AtomType::EncodedByTag,
            b"\xa9gen" => AtomType::CustomGenreTag,
            b"\xa9grp" => AtomType::GroupingTag,
            b"\xa9lyr" => AtomType::LyricsTag,
            b"\xa9nam" => AtomType::TrackTitleTag,
            b"\xa9too" => AtomType::EncoderTag,
            b"\xa9wrt" => AtomType::ComposerTag,
            _ => AtomType::Other(val),
        }
    }
}

/// Common atom header.
#[derive(Copy, Clone, Debug)]
pub struct AtomHeader {
    /// The atom type.
    pub atype: AtomType,
    /// The total size of the atom including the header.
    pub atom_len: u64,
    /// The size of the payload data.
    pub data_len: u64,
}

impl AtomHeader {
    const HEADER_SIZE: u64 = 8;
    const EXTENDED_HEADER_SIZE: u64 = AtomHeader::HEADER_SIZE + 8;
    const EXTRA_DATA_SIZE: u64 = 4;

    /// Reads an atom header from the provided `ByteStream`.
    pub fn read<B: ReadBytes>(reader: &mut B) -> Result<AtomHeader> {
        let mut atom_len = u64::from(reader.read_be_u32()?);
        let atype = AtomType::from(reader.read_quad_bytes()?);

        let data_len = match atom_len {
            0 => 0,
            1 => {
                atom_len = reader.read_be_u64()?;

                // The atom size should be atleast the length of the header.
                if atom_len < AtomHeader::EXTENDED_HEADER_SIZE {
                    return decode_error("isomp4: atom size is invalid");
                }

                atom_len - AtomHeader::EXTENDED_HEADER_SIZE
            }
            _ => {
                // The atom size should be atleast the length of the header.
                if atom_len < AtomHeader::HEADER_SIZE {
                    return decode_error("isomp4: atom size is invalid");
                }

                atom_len - AtomHeader::HEADER_SIZE
            }
        };

        Ok(AtomHeader { atype, atom_len, data_len })
    }

    #[allow(dead_code)]
    pub fn base_header_len(&self) -> u64 {
        match self.atom_len {
            0 => AtomHeader::HEADER_SIZE,
            _ => self.atom_len - self.data_len,
        }
    }

    /// For applicable atoms, reads the atom header extra data: a tuple composed of a u8 version
    /// number, and a u24 bitset of flags.
    pub fn read_extra<B: ReadBytes>(reader: &mut B) -> Result<(u8, u32)> {
        Ok((reader.read_u8()?, reader.read_be_u24()?))
    }
}

pub trait Atom: Sized {
    #[allow(dead_code)]
    fn header(&self) -> AtomHeader;

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self>;
}

pub struct AtomIterator<B: ReadBytes> {
    reader: B,
    len: Option<u64>,
    cur_atom: Option<AtomHeader>,
    base_pos: u64,
    next_atom_pos: u64,
}

impl<B: ReadBytes> AtomIterator<B> {
    pub fn new_root(reader: B, len: Option<u64>) -> Self {
        let base_pos = reader.pos();

        AtomIterator { reader, len, cur_atom: None, base_pos, next_atom_pos: base_pos }
    }

    pub fn new(reader: B, container: AtomHeader) -> Self {
        let base_pos = reader.pos();

        AtomIterator {
            reader,
            len: Some(container.data_len),
            cur_atom: None,
            base_pos,
            next_atom_pos: base_pos,
        }
    }

    pub fn into_inner(self) -> B {
        self.reader
    }

    pub fn inner_mut(&mut self) -> &mut B {
        &mut self.reader
    }

    pub fn next(&mut self) -> Result<Option<AtomHeader>> {
        // Ignore any remaining data in the current atom that was not read.
        let cur_pos = self.reader.pos();

        if cur_pos < self.next_atom_pos {
            self.reader.ignore_bytes(self.next_atom_pos - cur_pos)?;
        }
        else if cur_pos > self.next_atom_pos {
            // This is very bad, either the atom's length was incorrect or the demuxer erroroneously
            // overread an atom.
            return decode_error("isomp4: overread atom");
        }

        // If len is specified, then do not read more than len bytes.
        if let Some(len) = self.len {
            if self.next_atom_pos - self.base_pos >= len {
                return Ok(None);
            }
        }

        // Read the next atom header.
        let atom = AtomHeader::read(&mut self.reader)?;

        // Calculate the start position for the next atom (the exclusive end of the current atom).
        self.next_atom_pos = match atom.atom_len {
            0 => {
                // An atom with a length of zero is defined to span to the end of the stream. If
                // len is available, use it for the next atom start position, otherwise, use u64 max
                // which will trip an end of stream error on the next iteration.
                self.len.map(|l| self.base_pos + l).unwrap_or(u64::MAX)
            }

            len => self.next_atom_pos + len,
        };

        self.cur_atom = Some(atom);

        Ok(self.cur_atom)
    }

    pub fn next_no_consume(&mut self) -> Result<Option<AtomHeader>> {
        if self.cur_atom.is_some() {
            Ok(self.cur_atom)
        }
        else {
            self.next()
        }
    }

    pub fn read_atom<A: Atom>(&mut self) -> Result<A> {
        // It is not possible to read the current atom more than once because ByteStream is not
        // seekable. Therefore, raise an assert if read_atom is called more than once between calls
        // to next, or after next returns None.
        assert!(self.cur_atom.is_some());
        A::read(&mut self.reader, self.cur_atom.take().unwrap())
    }

    pub fn consume_atom(&mut self) {
        assert!(self.cur_atom.take().is_some());
    }
}
