// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::audio::Channels;
use symphonia_core::codecs::{CodecParameters, CodecType, CODEC_TYPE_MP3, CODEC_TYPE_NULL};
use symphonia_core::codecs::{CODEC_TYPE_PCM_F32BE, CODEC_TYPE_PCM_F32LE};
use symphonia_core::codecs::{CODEC_TYPE_PCM_F64BE, CODEC_TYPE_PCM_F64LE};
use symphonia_core::codecs::{CODEC_TYPE_PCM_S16BE, CODEC_TYPE_PCM_S16LE};
use symphonia_core::codecs::{CODEC_TYPE_PCM_S24BE, CODEC_TYPE_PCM_S24LE};
use symphonia_core::codecs::{CODEC_TYPE_PCM_S32BE, CODEC_TYPE_PCM_S32LE};
use symphonia_core::codecs::{CODEC_TYPE_PCM_S8, CODEC_TYPE_PCM_U8};
use symphonia_core::codecs::{CODEC_TYPE_PCM_U16BE, CODEC_TYPE_PCM_U16LE};
use symphonia_core::codecs::{CODEC_TYPE_PCM_U24BE, CODEC_TYPE_PCM_U24LE};
use symphonia_core::codecs::{CODEC_TYPE_PCM_U32BE, CODEC_TYPE_PCM_U32LE};
use symphonia_core::errors::{decode_error, unsupported_error, Result};
use symphonia_core::io::ReadBytes;

use crate::atoms::{AlacAtom, Atom, AtomHeader, AtomType, EsdsAtom, FlacAtom, OpusAtom, WaveAtom};
use crate::fp::FpU16;

use super::AtomIterator;

/// Sample description atom.
#[allow(dead_code)]
#[derive(Debug)]
pub struct StsdAtom {
    /// Atom header.
    header: AtomHeader,
    /// Sample entry.
    sample_entry: SampleEntry,
}

impl Atom for StsdAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let (_, _) = AtomHeader::read_extra(reader)?;

        let n_entries = reader.read_be_u32()?;

        if n_entries == 0 {
            return decode_error("isomp4: missing sample entry");
        }

        if n_entries > 1 {
            return unsupported_error("isomp4: more than 1 sample entry");
        }

        let sample_entry_header = AtomHeader::read(reader)?;

        let sample_entry = match sample_entry_header.atype {
            AtomType::Mp4a
            | AtomType::Alac
            | AtomType::Flac
            | AtomType::Opus
            | AtomType::Mp3
            | AtomType::Lpcm
            | AtomType::QtWave
            | AtomType::ALaw
            | AtomType::MuLaw
            | AtomType::U8SampleEntry
            | AtomType::S16LeSampleEntry
            | AtomType::S16BeSampleEntry
            | AtomType::S24SampleEntry
            | AtomType::S32SampleEntry
            | AtomType::F32SampleEntry
            | AtomType::F64SampleEntry => read_audio_sample_entry(reader, sample_entry_header)?,
            _ => {
                // Potentially video, subtitles, etc.
                SampleEntry::Other
            }
        };

        Ok(StsdAtom { header, sample_entry })
    }
}

impl StsdAtom {
    /// Fill the provided `CodecParameters` using the sample entry.
    pub fn fill_codec_params(&self, codec_params: &mut CodecParameters) {
        // Audio sample entry.
        if let SampleEntry::Audio(ref entry) = self.sample_entry {
            // General audio parameters.
            codec_params.with_sample_rate(entry.sample_rate as u32);

            // Codec-specific parameters.
            match entry.codec_specific {
                Some(AudioCodecSpecific::Esds(ref esds)) => {
                    esds.fill_codec_params(codec_params);
                }
                Some(AudioCodecSpecific::Alac(ref alac)) => {
                    alac.fill_codec_params(codec_params);
                }
                Some(AudioCodecSpecific::Flac(ref flac)) => {
                    flac.fill_codec_params(codec_params);
                }
                Some(AudioCodecSpecific::Opus(ref opus)) => {
                    opus.fill_codec_params(codec_params);
                }
                Some(AudioCodecSpecific::Mp3) => {
                    codec_params.for_codec(CODEC_TYPE_MP3);
                }
                Some(AudioCodecSpecific::Pcm(ref pcm)) => {
                    // PCM codecs.
                    codec_params
                        .for_codec(pcm.codec_type)
                        .with_bits_per_coded_sample(pcm.bits_per_coded_sample)
                        .with_bits_per_sample(pcm.bits_per_sample)
                        .with_max_frames_per_packet(pcm.frames_per_packet)
                        .with_channels(pcm.channels);
                }
                _ => (),
            }
        }
    }
}

#[derive(Debug)]
pub struct Pcm {
    pub codec_type: CodecType,
    pub bits_per_sample: u32,
    pub bits_per_coded_sample: u32,
    pub frames_per_packet: u64,
    pub channels: Channels,
}

#[derive(Debug)]
pub enum AudioCodecSpecific {
    /// MPEG Elementary Stream descriptor.
    Esds(EsdsAtom),
    /// Apple Lossless Audio Codec (ALAC).
    Alac(AlacAtom),
    /// Free Lossless Audio Codec (FLAC).
    Flac(FlacAtom),
    /// Opus.
    Opus(OpusAtom),
    /// MP3.
    Mp3,
    /// PCM codecs.
    Pcm(Pcm),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct AudioSampleEntry {
    pub num_channels: u32,
    pub sample_size: u16,
    pub sample_rate: f64,
    pub codec_specific: Option<AudioCodecSpecific>,
}

#[derive(Debug)]
pub enum SampleEntry {
    Audio(AudioSampleEntry),
    // Video,
    // Metadata,
    Other,
}

/// Gets if the sample entry atom is for a PCM codec.
fn is_pcm_codec(atype: AtomType) -> bool {
    // PCM data in version 0 and 1 is signalled by the sample entry atom type. In version 2, the
    // atom type for PCM data is always LPCM.
    atype == AtomType::Lpcm || pcm_codec_type(atype) != CODEC_TYPE_NULL
}

/// Gets the PCM codec from the sample entry atom type for version 0 and 1 sample entries.
fn pcm_codec_type(atype: AtomType) -> CodecType {
    match atype {
        AtomType::U8SampleEntry => CODEC_TYPE_PCM_U8,
        AtomType::S16LeSampleEntry => CODEC_TYPE_PCM_S16LE,
        AtomType::S16BeSampleEntry => CODEC_TYPE_PCM_S16BE,
        AtomType::S24SampleEntry => CODEC_TYPE_PCM_S24LE,
        AtomType::S32SampleEntry => CODEC_TYPE_PCM_S32LE,
        AtomType::F32SampleEntry => CODEC_TYPE_PCM_F32LE,
        AtomType::F64SampleEntry => CODEC_TYPE_PCM_F64LE,
        _ => CODEC_TYPE_NULL,
    }
}

/// Determines the number of bytes per PCM sample for a PCM codec type.
fn bytes_per_pcm_sample(pcm_codec_type: CodecType) -> u32 {
    match pcm_codec_type {
        CODEC_TYPE_PCM_S8 | CODEC_TYPE_PCM_U8 => 1,
        CODEC_TYPE_PCM_S16BE | CODEC_TYPE_PCM_S16LE => 2,
        CODEC_TYPE_PCM_U16BE | CODEC_TYPE_PCM_U16LE => 2,
        CODEC_TYPE_PCM_S24BE | CODEC_TYPE_PCM_S24LE => 3,
        CODEC_TYPE_PCM_U24BE | CODEC_TYPE_PCM_U24LE => 3,
        CODEC_TYPE_PCM_S32BE | CODEC_TYPE_PCM_S32LE => 4,
        CODEC_TYPE_PCM_U32BE | CODEC_TYPE_PCM_U32LE => 4,
        CODEC_TYPE_PCM_F32BE | CODEC_TYPE_PCM_F32LE => 4,
        CODEC_TYPE_PCM_F64BE | CODEC_TYPE_PCM_F64LE => 8,
        _ => unreachable!(),
    }
}

/// Gets the PCM codec from the LPCM parameters in the version 2 sample entry atom.
fn lpcm_codec_type(bits_per_sample: u32, lpcm_flags: u32) -> CodecType {
    let is_floating_point = lpcm_flags & 0x1 != 0;
    let is_big_endian = lpcm_flags & 0x2 != 0;
    let is_signed = lpcm_flags & 0x4 != 0;

    if is_floating_point {
        // Floating-point sample format.
        match bits_per_sample {
            32 => {
                if is_big_endian {
                    CODEC_TYPE_PCM_F32BE
                }
                else {
                    CODEC_TYPE_PCM_F32LE
                }
            }
            64 => {
                if is_big_endian {
                    CODEC_TYPE_PCM_F64BE
                }
                else {
                    CODEC_TYPE_PCM_F64LE
                }
            }
            _ => CODEC_TYPE_NULL,
        }
    }
    else {
        // Integer sample format.
        if is_signed {
            // Signed-integer sample format.
            match bits_per_sample {
                8 => CODEC_TYPE_PCM_S8,
                16 => {
                    if is_big_endian {
                        CODEC_TYPE_PCM_S16BE
                    }
                    else {
                        CODEC_TYPE_PCM_S16LE
                    }
                }
                24 => {
                    if is_big_endian {
                        CODEC_TYPE_PCM_S24BE
                    }
                    else {
                        CODEC_TYPE_PCM_S24LE
                    }
                }
                32 => {
                    if is_big_endian {
                        CODEC_TYPE_PCM_S32BE
                    }
                    else {
                        CODEC_TYPE_PCM_S32LE
                    }
                }
                _ => CODEC_TYPE_NULL,
            }
        }
        else {
            // Unsigned-integer sample format.
            match bits_per_sample {
                8 => CODEC_TYPE_PCM_U8,
                16 => {
                    if is_big_endian {
                        CODEC_TYPE_PCM_U16BE
                    }
                    else {
                        CODEC_TYPE_PCM_U16LE
                    }
                }
                24 => {
                    if is_big_endian {
                        CODEC_TYPE_PCM_U24BE
                    }
                    else {
                        CODEC_TYPE_PCM_U24LE
                    }
                }
                32 => {
                    if is_big_endian {
                        CODEC_TYPE_PCM_U32BE
                    }
                    else {
                        CODEC_TYPE_PCM_U32LE
                    }
                }
                _ => CODEC_TYPE_NULL,
            }
        }
    }
}

/// Gets the audio channels for a version 0 or 1 sample entry.
fn pcm_channels(num_channels: u32) -> Result<Channels> {
    match num_channels {
        1 => Ok(Channels::FRONT_LEFT),
        2 => Ok(Channels::FRONT_LEFT | Channels::FRONT_RIGHT),
        _ => decode_error("isomp4: invalid number of channels"),
    }
}

/// Gets the audio channels for a version 2 LPCM sample entry.
fn lpcm_channels(num_channels: u32) -> Result<Channels> {
    if num_channels < 1 {
        return decode_error("isomp4: invalid number of channels");
    }

    if num_channels > 32 {
        return unsupported_error("isomp4: maximum 32 channels");
    }

    // TODO: For LPCM, the channels are "auxilary". They do not have a speaker assignment. Symphonia
    // does not have a way to represent this yet.
    let channel_mask = !((!0 << 1) << (num_channels - 1));

    match Channels::from_bits(channel_mask) {
        Some(channels) => Ok(channels),
        _ => unsupported_error("isomp4: unsupported number of channels"),
    }
}

fn read_audio_sample_entry<B: ReadBytes>(
    reader: &mut B,
    mut header: AtomHeader,
) -> Result<SampleEntry> {
    // An audio sample entry atom is derived from a base sample entry atom. The audio sample entry
    // atom contains the fields of the base sample entry first, then the audio sample entry fields
    // next. After those fields, a number of other atoms are nested, including the mandatory
    // codec-specific atom. Though the codec-specific atom is nested within the (audio) sample entry
    // atom, the (audio) sample entry atom uses the atom type of the codec-specific atom. This is
    // odd in-that the final structure will appear to have the codec-specific atom nested within
    // itself, which is not actually the case.

    let data_start_pos = reader.pos();

    // First 6 bytes of all sample entries should be all 0.
    reader.ignore_bytes(6)?;

    // Sample entry data reference.
    let _ = reader.read_be_u16()?;

    // The version of the audio sample entry.
    let version = reader.read_be_u16()?;

    // Skip revision and vendor.
    reader.ignore_bytes(6)?;

    let mut num_channels = u32::from(reader.read_be_u16()?);
    let sample_size = reader.read_be_u16()?;

    // Skip compression ID and packet size.
    reader.ignore_bytes(4)?;

    let mut sample_rate = f64::from(FpU16::parse_raw(reader.read_be_u32()?));

    let is_pcm_codec = is_pcm_codec(header.atype);

    let mut codec_specific = match version {
        0 => {
            // Version 0.
            if is_pcm_codec {
                let codec_type = pcm_codec_type(header.atype);
                let bits_per_sample = 8 * bytes_per_pcm_sample(codec_type);

                // Validate the codec-derived bytes-per-sample equals the declared bytes-per-sample.
                if u32::from(sample_size) != bits_per_sample {
                    return decode_error("isomp4: invalid pcm sample size");
                }

                // The original fields describe the PCM sample format.
                Some(AudioCodecSpecific::Pcm(Pcm {
                    codec_type: pcm_codec_type(header.atype),
                    bits_per_sample,
                    bits_per_coded_sample: bits_per_sample,
                    frames_per_packet: 1,
                    channels: pcm_channels(num_channels)?,
                }))
            }
            else {
                None
            }
        }
        1 => {
            // Version 1.

            // The number of frames (ISO/MP4 samples) per packet. For PCM codecs, this is always 1.
            let _frames_per_packet = reader.read_be_u32()?;

            // The number of bytes per PCM audio sample. This value supersedes sample_size. For
            // non-PCM codecs, this value is not useful.
            let bytes_per_audio_sample = reader.read_be_u32()?;

            // The number of bytes per PCM audio frame (ISO/MP4 sample). For non-PCM codecs, this
            // value is not useful.
            let _bytes_per_frame = reader.read_be_u32()?;

            // The next value, as defined, is seemingly non-sensical.
            let _ = reader.read_be_u32()?;

            if is_pcm_codec {
                let codec_type = pcm_codec_type(header.atype);
                let codec_bytes_per_sample = bytes_per_pcm_sample(codec_type);

                // Validate the codec-derived bytes-per-sample equals the declared bytes-per-sample.
                if bytes_per_audio_sample != codec_bytes_per_sample {
                    return decode_error("isomp4: invalid pcm bytes per sample");
                }

                // The new fields describe the PCM sample format and supersede the original version
                // 0 fields.
                Some(AudioCodecSpecific::Pcm(Pcm {
                    codec_type,
                    bits_per_sample: 8 * codec_bytes_per_sample,
                    bits_per_coded_sample: 8 * codec_bytes_per_sample,
                    frames_per_packet: 1,
                    channels: pcm_channels(num_channels)?,
                }))
            }
            else {
                None
            }
        }
        2 => {
            // Version 2.
            reader.ignore_bytes(4)?;

            sample_rate = reader.read_be_f64()?;
            num_channels = reader.read_be_u32()?;

            if reader.read_be_u32()? != 0x7f00_0000 {
                return decode_error("isomp4: audio sample entry v2 reserved must be 0x7f00_0000");
            }

            // The following fields are only useful for PCM codecs.
            let bits_per_sample = reader.read_be_u32()?;
            let lpcm_flags = reader.read_be_u32()?;
            let _bytes_per_packet = reader.read_be_u32()?;
            let lpcm_frames_per_packet = reader.read_be_u32()?;

            // This is only valid if this is a PCM codec.
            let codec_type = lpcm_codec_type(bits_per_sample, lpcm_flags);

            if is_pcm_codec && codec_type != CODEC_TYPE_NULL {
                // Like version 1, the new fields describe the PCM sample format and supersede the
                // original version 0 fields.
                Some(AudioCodecSpecific::Pcm(Pcm {
                    codec_type,
                    bits_per_sample,
                    bits_per_coded_sample: bits_per_sample,
                    frames_per_packet: u64::from(lpcm_frames_per_packet),
                    channels: lpcm_channels(num_channels)?,
                }))
            }
            else {
                None
            }
        }
        _ => {
            return unsupported_error("isomp4: unknown sample entry version");
        }
    };

    // Need to account for the data already read from the atom.
    header.data_len -= reader.pos() - data_start_pos;

    let mut iter = AtomIterator::new(reader, header);

    while let Some(entry_header) = iter.next()? {
        match entry_header.atype {
            AtomType::Esds => {
                // MP4A/ESDS codec-specific atom.
                if header.atype != AtomType::Mp4a || codec_specific.is_some() {
                    return decode_error("isomp4: invalid sample entry");
                }

                codec_specific = Some(AudioCodecSpecific::Esds(iter.read_atom::<EsdsAtom>()?));
            }
            AtomType::Alac => {
                // ALAC codec-specific atom.
                if header.atype != AtomType::Alac || codec_specific.is_some() {
                    return decode_error("isomp4: invalid sample entry");
                }

                codec_specific = Some(AudioCodecSpecific::Alac(iter.read_atom::<AlacAtom>()?));
            }
            AtomType::FlacDsConfig => {
                // FLAC codec-specific atom.
                if header.atype != AtomType::Flac || codec_specific.is_some() {
                    return decode_error("isomp4: invalid sample entry");
                }

                codec_specific = Some(AudioCodecSpecific::Flac(iter.read_atom::<FlacAtom>()?));
            }
            AtomType::OpusDsConfig => {
                // Opus codec-specific atom.
                if header.atype != AtomType::Opus || codec_specific.is_some() {
                    return decode_error("isomp4: invalid sample entry");
                }

                codec_specific = Some(AudioCodecSpecific::Opus(iter.read_atom::<OpusAtom>()?));
            }
            AtomType::QtWave => {
                // The QuickTime WAVE (aka. siDecompressionParam) atom may contain many different
                // types of sub-atoms to store decoder parameters.
                let wave = iter.read_atom::<WaveAtom>()?;

                if let Some(esds) = wave.esds {
                    if codec_specific.is_some() {
                        return decode_error("isomp4: invalid sample entry");
                    }

                    codec_specific = Some(AudioCodecSpecific::Esds(esds));
                }
            }
            _ => (),
        }
    }

    // A MP3 sample entry has no codec-specific atom.
    if header.atype == AtomType::Mp3 {
        if codec_specific.is_some() {
            return decode_error("isomp4: invalid sample entry");
        }

        codec_specific = Some(AudioCodecSpecific::Mp3);
    }

    Ok(SampleEntry::Audio(AudioSampleEntry {
        num_channels,
        sample_size,
        sample_rate,
        codec_specific,
    }))
}
