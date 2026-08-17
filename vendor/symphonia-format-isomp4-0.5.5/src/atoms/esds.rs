// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::codecs::{
    CodecParameters, CodecType, CODEC_TYPE_AAC, CODEC_TYPE_MP3, CODEC_TYPE_NULL,
};
use symphonia_core::errors::{decode_error, unsupported_error, Result};
use symphonia_core::io::{FiniteStream, ReadBytes, ScopedStream};

use crate::atoms::{Atom, AtomHeader};

use log::{debug, warn};

const ES_DESCRIPTOR: u8 = 0x03;
const DECODER_CONFIG_DESCRIPTOR: u8 = 0x04;
const DECODER_SPECIFIC_DESCRIPTOR: u8 = 0x05;
const SL_CONFIG_DESCRIPTOR: u8 = 0x06;

const MIN_DESCRIPTOR_SIZE: u64 = 2;

fn read_descriptor_header<B: ReadBytes>(reader: &mut B) -> Result<(u8, u32)> {
    let tag = reader.read_u8()?;

    let mut size = 0;

    for _ in 0..4 {
        let val = reader.read_u8()?;
        size = (size << 7) | u32::from(val & 0x7f);
        if val & 0x80 == 0 {
            break;
        }
    }

    Ok((tag, size))
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct EsdsAtom {
    /// Atom header.
    header: AtomHeader,
    /// Elementary stream descriptor.
    descriptor: ESDescriptor,
}

impl Atom for EsdsAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let (_, _) = AtomHeader::read_extra(reader)?;

        let mut descriptor = None;

        let mut scoped = ScopedStream::new(reader, header.data_len - AtomHeader::EXTRA_DATA_SIZE);

        while scoped.bytes_available() > MIN_DESCRIPTOR_SIZE {
            let (desc, desc_len) = read_descriptor_header(&mut scoped)?;

            match desc {
                ES_DESCRIPTOR => {
                    descriptor = Some(ESDescriptor::read(&mut scoped, desc_len)?);
                }
                _ => {
                    warn!("unknown descriptor in esds atom, desc={}", desc);
                    scoped.ignore_bytes(desc_len as u64)?;
                }
            }
        }

        // Ignore remainder of the atom.
        scoped.ignore()?;

        Ok(EsdsAtom { header, descriptor: descriptor.unwrap() })
    }
}

impl EsdsAtom {
    pub fn fill_codec_params(&self, codec_params: &mut CodecParameters) {
        codec_params.for_codec(self.descriptor.dec_config.codec_type);

        if let Some(ds_config) = &self.descriptor.dec_config.dec_specific_info {
            codec_params.with_extra_data(ds_config.extra_data.clone());
        }
    }
}

pub trait ObjectDescriptor: Sized {
    fn read<B: ReadBytes>(reader: &mut B, len: u32) -> Result<Self>;
}

/*
class ES_Descriptor extends BaseDescriptor : bit(8) tag=ES_DescrTag {
    bit(16) ES_ID;
    bit(1) streamDependenceFlag;
    bit(1) URL_Flag;
    bit(1) OCRstreamFlag;
    bit(5) streamPriority;
    if (streamDependenceFlag)
        bit(16) dependsOn_ES_ID;
    if (URL_Flag) {
        bit(8) URLlength;
        bit(8) URLstring[URLlength];
    }
    if (OCRstreamFlag)
        bit(16) OCR_ES_Id;
    DecoderConfigDescriptor decConfigDescr;
    SLConfigDescriptor slConfigDescr;
    IPI_DescrPointer ipiPtr[0 .. 1];
    IP_IdentificationDataSet ipIDS[0 .. 255];
    IPMP_DescriptorPointer ipmpDescrPtr[0 .. 255];
    LanguageDescriptor langDescr[0 .. 255];
    QoS_Descriptor qosDescr[0 .. 1];
    RegistrationDescriptor regDescr[0 .. 1];
    ExtensionDescriptor extDescr[0 .. 255];
}
*/

#[allow(dead_code)]
#[derive(Debug)]
pub struct ESDescriptor {
    pub es_id: u16,
    pub dec_config: DecoderConfigDescriptor,
    pub sl_config: SLDescriptor,
}

impl ObjectDescriptor for ESDescriptor {
    fn read<B: ReadBytes>(reader: &mut B, len: u32) -> Result<Self> {
        let es_id = reader.read_be_u16()?;
        let es_flags = reader.read_u8()?;

        // Stream dependence flag.
        if es_flags & 0x80 != 0 {
            let _depends_on_es_id = reader.read_u16()?;
        }

        // URL flag.
        if es_flags & 0x40 != 0 {
            let url_len = reader.read_u8()?;
            reader.ignore_bytes(u64::from(url_len))?;
        }

        // OCR stream flag.
        if es_flags & 0x20 != 0 {
            let _ocr_es_id = reader.read_u16()?;
        }

        let mut dec_config = None;
        let mut sl_config = None;

        let mut scoped = ScopedStream::new(reader, u64::from(len) - 3);

        // Multiple descriptors follow, but only the decoder configuration descriptor is useful.
        while scoped.bytes_available() > MIN_DESCRIPTOR_SIZE {
            let (desc, desc_len) = read_descriptor_header(&mut scoped)?;

            match desc {
                DECODER_CONFIG_DESCRIPTOR => {
                    dec_config = Some(DecoderConfigDescriptor::read(&mut scoped, desc_len)?);
                }
                SL_CONFIG_DESCRIPTOR => {
                    sl_config = Some(SLDescriptor::read(&mut scoped, desc_len)?);
                }
                _ => {
                    debug!("skipping {} object in es descriptor", desc);
                    scoped.ignore_bytes(u64::from(desc_len))?;
                }
            }
        }

        // Consume remaining bytes.
        scoped.ignore()?;

        // Decoder configuration descriptor is mandatory.
        if dec_config.is_none() {
            return decode_error("isomp4: missing decoder config descriptor");
        }

        // SL descriptor is mandatory.
        if sl_config.is_none() {
            return decode_error("isomp4: missing sl config descriptor");
        }

        Ok(ESDescriptor { es_id, dec_config: dec_config.unwrap(), sl_config: sl_config.unwrap() })
    }
}

/*
class DecoderConfigDescriptor extends BaseDescriptor : bit(8) tag=DecoderConfigDescrTag {
    bit(8) objectTypeIndication;
    bit(6) streamType;
    bit(1) upStream;
    const bit(1) reserved=1;
    bit(24) bufferSizeDB;
    bit(32) maxBitrate;
    bit(32) avgBitrate;
    DecoderSpecificInfo decSpecificInfo[0 .. 1];
    profileLevelIndicationIndexDescriptor profileLevelIndicationIndexDescr [0..255];
}
*/

#[allow(dead_code)]
#[derive(Debug)]
pub struct DecoderConfigDescriptor {
    pub codec_type: CodecType,
    pub object_type_indication: u8,
    pub dec_specific_info: Option<DecoderSpecificInfo>,
}

impl ObjectDescriptor for DecoderConfigDescriptor {
    fn read<B: ReadBytes>(reader: &mut B, len: u32) -> Result<Self> {
        // AAC
        const OBJECT_TYPE_ISO14496_3: u8 = 0x40;
        const OBJECT_TYPE_ISO13818_7_MAIN: u8 = 0x66;
        const OBJECT_TYPE_ISO13818_7_LC: u8 = 0x67;
        // MP3
        const OBJECT_TYPE_ISO13818_3: u8 = 0x69;
        const OBJECT_TYPE_ISO11172_3: u8 = 0x6b;

        let object_type_indication = reader.read_u8()?;

        let (_stream_type, _upstream) = {
            let val = reader.read_u8()?;

            if val & 0x1 != 1 {
                debug!("decoder config descriptor reserved bit is not 1");
            }

            ((val & 0xfc) >> 2, (val & 0x2) >> 1)
        };

        let _buffer_size = reader.read_be_u24()?;
        let _max_bitrate = reader.read_be_u32()?;
        let _avg_bitrate = reader.read_be_u32()?;

        let mut dec_specific_config = None;

        let mut scoped = ScopedStream::new(reader, u64::from(len) - 13);

        // Multiple descriptors follow, but only the decoder specific info descriptor is useful.
        while scoped.bytes_available() > MIN_DESCRIPTOR_SIZE {
            let (desc, desc_len) = read_descriptor_header(&mut scoped)?;

            match desc {
                DECODER_SPECIFIC_DESCRIPTOR => {
                    dec_specific_config = Some(DecoderSpecificInfo::read(&mut scoped, desc_len)?);
                }
                _ => {
                    debug!("skipping {} object in decoder config descriptor", desc);
                    scoped.ignore_bytes(u64::from(desc_len))?;
                }
            }
        }

        let codec_type = match object_type_indication {
            OBJECT_TYPE_ISO14496_3 | OBJECT_TYPE_ISO13818_7_LC | OBJECT_TYPE_ISO13818_7_MAIN => {
                CODEC_TYPE_AAC
            }
            OBJECT_TYPE_ISO13818_3 | OBJECT_TYPE_ISO11172_3 => CODEC_TYPE_MP3,
            _ => {
                debug!(
                    "unknown object type indication {:#x} for decoder config descriptor",
                    object_type_indication
                );

                CODEC_TYPE_NULL
            }
        };

        // Consume remaining bytes.
        scoped.ignore()?;

        Ok(DecoderConfigDescriptor {
            codec_type,
            object_type_indication,
            dec_specific_info: dec_specific_config,
        })
    }
}

#[derive(Debug)]
pub struct DecoderSpecificInfo {
    pub extra_data: Box<[u8]>,
}

impl ObjectDescriptor for DecoderSpecificInfo {
    fn read<B: ReadBytes>(reader: &mut B, len: u32) -> Result<Self> {
        Ok(DecoderSpecificInfo { extra_data: reader.read_boxed_slice_exact(len as usize)? })
    }
}

/*
class SLConfigDescriptor extends BaseDescriptor : bit(8) tag=SLConfigDescrTag {
    bit(8) predefined;
    if (predefined==0) {
        bit(1) useAccessUnitStartFlag;
        bit(1) useAccessUnitEndFlag;
        bit(1) useRandomAccessPointFlag;
        bit(1) hasRandomAccessUnitsOnlyFlag;
        bit(1) usePaddingFlag;
        bit(1) useTimeStampsFlag;
        bit(1) useIdleFlag;
        bit(1) durationFlag;
        bit(32) timeStampResolution;
        bit(32) OCRResolution;
        bit(8) timeStampLength; // must be  64
        bit(8) OCRLength; // must be  64
        bit(8) AU_Length; // must be  32
        bit(8) instantBitrateLength;
        bit(4) degradationPriorityLength;
        bit(5) AU_seqNumLength; // must be  16
        bit(5) packetSeqNumLength; // must be  16
        bit(2) reserved=0b11;
    }
    if (durationFlag) {
        bit(32) timeScale;
        bit(16) accessUnitDuration;
        bit(16) compositionUnitDuration;
    }
    if (!useTimeStampsFlag) {
        bit(timeStampLength) startDecodingTimeStamp;
        bit(timeStampLength) startCompositionTimeStamp;
    }
}

timeStampLength == 32, for predefined == 0x1
timeStampLength == 0,  for predefined == 0x2
*/
#[derive(Debug)]
pub struct SLDescriptor;

impl ObjectDescriptor for SLDescriptor {
    fn read<B: ReadBytes>(reader: &mut B, _len: u32) -> Result<Self> {
        // const SLCONFIG_PREDEFINED_CUSTOM: u8 = 0x0;
        // const SLCONFIG_PREDEFINED_NULL: u8 = 0x1;
        const SLCONFIG_PREDEFINED_MP4: u8 = 0x2;

        let predefined = reader.read_u8()?;

        if predefined != SLCONFIG_PREDEFINED_MP4 {
            return unsupported_error("isomp4: sl descriptor predefined not mp4");
        }

        Ok(SLDescriptor {})
    }
}
