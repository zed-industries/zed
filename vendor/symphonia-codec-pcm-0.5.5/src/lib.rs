// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]
// The following lints are allowed in all Symphonia crates. Please see clippy.toml for their
// justification.
#![allow(clippy::comparison_chain)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::identity_op)]
#![allow(clippy::manual_range_contains)]

use symphonia_core::support_codec;

use symphonia_core::audio::{AsAudioBufferRef, AudioBuffer, AudioBufferRef, Signal, SignalSpec};
use symphonia_core::codecs::{CodecDescriptor, CodecParameters, CodecType};
use symphonia_core::codecs::{Decoder, DecoderOptions, FinalizeResult};
// Signed Int PCM codecs
use symphonia_core::codecs::{CODEC_TYPE_PCM_S16BE, CODEC_TYPE_PCM_S24BE, CODEC_TYPE_PCM_S32BE};
use symphonia_core::codecs::{CODEC_TYPE_PCM_S16LE, CODEC_TYPE_PCM_S8};
use symphonia_core::codecs::{CODEC_TYPE_PCM_S24LE, CODEC_TYPE_PCM_S32LE};
// Unsigned Int PCM codecs
use symphonia_core::codecs::{CODEC_TYPE_PCM_U16BE, CODEC_TYPE_PCM_U24BE, CODEC_TYPE_PCM_U32BE};
use symphonia_core::codecs::{CODEC_TYPE_PCM_U16LE, CODEC_TYPE_PCM_U8};
use symphonia_core::codecs::{CODEC_TYPE_PCM_U24LE, CODEC_TYPE_PCM_U32LE};
// Floating point PCM codecs
use symphonia_core::codecs::{CODEC_TYPE_PCM_F32BE, CODEC_TYPE_PCM_F32LE};
use symphonia_core::codecs::{CODEC_TYPE_PCM_F64BE, CODEC_TYPE_PCM_F64LE};
// G711 ALaw and MuLaw PCM codecs
use symphonia_core::codecs::{CODEC_TYPE_PCM_ALAW, CODEC_TYPE_PCM_MULAW};
use symphonia_core::conv::IntoSample;
use symphonia_core::errors::{decode_error, unsupported_error, Result};
use symphonia_core::formats::Packet;
use symphonia_core::io::ReadBytes;
use symphonia_core::sample::{i24, u24, SampleFormat};
use symphonia_core::units::Duration;

macro_rules! impl_generic_audio_buffer_func {
    ($generic:expr, $buf:ident, $expr:expr) => {
        match $generic {
            GenericAudioBuffer::U8($buf) => $expr,
            GenericAudioBuffer::U16($buf) => $expr,
            GenericAudioBuffer::U24($buf) => $expr,
            GenericAudioBuffer::U32($buf) => $expr,
            GenericAudioBuffer::S8($buf) => $expr,
            GenericAudioBuffer::S16($buf) => $expr,
            GenericAudioBuffer::S24($buf) => $expr,
            GenericAudioBuffer::S32($buf) => $expr,
            GenericAudioBuffer::F32($buf) => $expr,
            GenericAudioBuffer::F64($buf) => $expr,
        }
    };
}

/// A generic audio buffer.
/// TODO: Move to core library with a full API.
pub enum GenericAudioBuffer {
    U8(AudioBuffer<u8>),
    U16(AudioBuffer<u16>),
    U24(AudioBuffer<u24>),
    U32(AudioBuffer<u32>),
    S8(AudioBuffer<i8>),
    S16(AudioBuffer<i16>),
    S24(AudioBuffer<i24>),
    S32(AudioBuffer<i32>),
    F32(AudioBuffer<f32>),
    F64(AudioBuffer<f64>),
}

impl GenericAudioBuffer {
    fn new(format: SampleFormat, duration: Duration, spec: SignalSpec) -> Self {
        match format {
            SampleFormat::U8 => GenericAudioBuffer::U8(AudioBuffer::new(duration, spec)),
            SampleFormat::U16 => GenericAudioBuffer::U16(AudioBuffer::new(duration, spec)),
            SampleFormat::U24 => GenericAudioBuffer::U24(AudioBuffer::new(duration, spec)),
            SampleFormat::U32 => GenericAudioBuffer::U32(AudioBuffer::new(duration, spec)),
            SampleFormat::S8 => GenericAudioBuffer::S8(AudioBuffer::new(duration, spec)),
            SampleFormat::S16 => GenericAudioBuffer::S16(AudioBuffer::new(duration, spec)),
            SampleFormat::S24 => GenericAudioBuffer::S24(AudioBuffer::new(duration, spec)),
            SampleFormat::S32 => GenericAudioBuffer::S32(AudioBuffer::new(duration, spec)),
            SampleFormat::F32 => GenericAudioBuffer::F32(AudioBuffer::new(duration, spec)),
            SampleFormat::F64 => GenericAudioBuffer::F64(AudioBuffer::new(duration, spec)),
        }
    }

    fn clear(&mut self) {
        impl_generic_audio_buffer_func!(self, buf, buf.clear());
    }
}

impl AsAudioBufferRef for GenericAudioBuffer {
    fn as_audio_buffer_ref(&self) -> AudioBufferRef<'_> {
        impl_generic_audio_buffer_func!(self, buf, buf.as_audio_buffer_ref())
    }
}

macro_rules! read_pcm_signed {
    ($buf:expr, $fmt:tt, $read:expr, $width:expr, $coded_width:expr) => {
        // Get buffer of the correct sample format.
        match $buf {
            GenericAudioBuffer::$fmt(ref mut buf) => {
                // Read samples.
                let shift = $width - $coded_width;
                buf.fill(|audio_planes, idx| -> Result<()> {
                    for plane in audio_planes.planes() {
                        plane[idx] = ($read << shift).into_sample();
                    }
                    Ok(())
                })
            }
            _ => unreachable!(),
        }
    };
}

macro_rules! read_pcm_unsigned {
    ($buf:expr, $fmt:tt, $read:expr, $width:expr, $coded_width:expr) => {
        // Get buffer of the correct sample format.
        match $buf {
            GenericAudioBuffer::$fmt(ref mut buf) => {
                // Read samples.
                let shift = $width - $coded_width;
                buf.fill(|audio_planes, idx| -> Result<()> {
                    for plane in audio_planes.planes() {
                        plane[idx] = ($read << shift).into_sample();
                    }
                    Ok(())
                })
            }
            _ => unreachable!(),
        }
    };
}

macro_rules! read_pcm_floating {
    ($buf:expr, $fmt:tt, $read:expr) => {
        // Get buffer of the correct sample format.
        match $buf {
            GenericAudioBuffer::$fmt(ref mut buf) => {
                // Read samples.
                buf.fill(|audio_planes, idx| -> Result<()> {
                    for plane in audio_planes.planes() {
                        plane[idx] = $read;
                    }
                    Ok(())
                })
            }
            _ => unreachable!(),
        }
    };
}

macro_rules! read_pcm_transfer_func {
    ($buf:expr, $fmt:tt, $func:expr) => {
        // Get buffer of the correct sample format.
        match $buf {
            GenericAudioBuffer::$fmt(ref mut buf) => {
                // Read samples.
                buf.fill(|audio_planes, idx| -> Result<()> {
                    for plane in audio_planes.planes() {
                        plane[idx] = $func;
                    }
                    Ok(())
                })
            }
            _ => unreachable!(),
        }
    };
}

// alaw_to_linear and mulaw_to_linear are adaptations of alaw2linear and ulaw2linear from g711.c by
// SUN Microsystems (unrestricted use license), license text for these parts follow below.
//
// ---
//
// This source code is a product of Sun Microsystems, Inc. and is provided for
// unrestricted use.  Users may copy or modify this source code without
// charge.
//
// SUN SOURCE CODE IS PROVIDED AS IS WITH NO WARRANTIES OF ANY KIND INCLUDING THE
// WARRANTIES OF DESIGN, MERCHANTIBILITY AND FITNESS FOR A PARTICULAR
// PURPOSE, OR ARISING FROM A COURSE OF DEALING, USAGE OR TRADE PRACTICE.
//
// Sun source code is provided with no support and without any obligation on the
// part of Sun Microsystems, Inc. to assist in its use, correction,
// modification or enhancement.
//
// SUN MICROSYSTEMS, INC. SHALL HAVE NO LIABILITY WITH RESPECT TO THE
// INFRINGEMENT OF COPYRIGHTS, TRADE SECRETS OR ANY PATENTS BY THIS SOFTWARE
// OR ANY PART THEREOF.
//
// In no event will Sun Microsystems, Inc. be liable for any lost revenue
// or profits or other special, indirect and consequential damages, even if
// Sun has been advised of the possibility of such damages.
//
// Sun Microsystems, Inc.
// 2550 Garcia Avenue
// Mountain View, California  94043
const XLAW_QUANT_MASK: u8 = 0x0f;
const XLAW_SEG_MASK: u8 = 0x70;
const XLAW_SEG_SHIFT: u32 = 4;

fn alaw_to_linear(mut a_val: u8) -> i16 {
    a_val ^= 0x55;

    let mut t = i16::from((a_val & XLAW_QUANT_MASK) << 4);
    let seg = (a_val & XLAW_SEG_MASK) >> XLAW_SEG_SHIFT;

    match seg {
        0 => t += 0x8,
        1 => t += 0x108,
        _ => t = (t + 0x108) << (seg - 1),
    }

    if a_val & 0x80 == 0x80 {
        t
    }
    else {
        -t
    }
}

fn mulaw_to_linear(mut mu_val: u8) -> i16 {
    const BIAS: i16 = 0x84;

    // Complement to obtain normal u-law value.
    mu_val = !mu_val;

    // Extract and bias the quantization bits. Then shift up by the segment number and subtract out
    // the bias.
    let mut t = i16::from((mu_val & XLAW_QUANT_MASK) << 3) + BIAS;
    t <<= (mu_val & XLAW_SEG_MASK) >> XLAW_SEG_SHIFT;

    if mu_val & 0x80 == 0x80 {
        BIAS - t
    }
    else {
        t - BIAS
    }
}

fn is_supported_pcm_codec(codec_type: CodecType) -> bool {
    matches!(
        codec_type,
        CODEC_TYPE_PCM_S32LE
            | CODEC_TYPE_PCM_S32BE
            | CODEC_TYPE_PCM_S24LE
            | CODEC_TYPE_PCM_S24BE
            | CODEC_TYPE_PCM_S16LE
            | CODEC_TYPE_PCM_S16BE
            | CODEC_TYPE_PCM_S8
            | CODEC_TYPE_PCM_U32LE
            | CODEC_TYPE_PCM_U32BE
            | CODEC_TYPE_PCM_U24LE
            | CODEC_TYPE_PCM_U24BE
            | CODEC_TYPE_PCM_U16LE
            | CODEC_TYPE_PCM_U16BE
            | CODEC_TYPE_PCM_U8
            | CODEC_TYPE_PCM_F32LE
            | CODEC_TYPE_PCM_F32BE
            | CODEC_TYPE_PCM_F64LE
            | CODEC_TYPE_PCM_F64BE
            | CODEC_TYPE_PCM_ALAW
            | CODEC_TYPE_PCM_MULAW
    )
}

/// Pulse Code Modulation (PCM) decoder for all raw PCM, and log-PCM codecs.
pub struct PcmDecoder {
    params: CodecParameters,
    coded_width: u32,
    buf: GenericAudioBuffer,
}

impl PcmDecoder {
    fn decode_inner(&mut self, packet: &Packet) -> Result<()> {
        let mut reader = packet.as_buf_reader();

        let _ = match self.params.codec {
            CODEC_TYPE_PCM_S32LE => {
                read_pcm_signed!(self.buf, S32, reader.read_i32()?, 32, self.coded_width)
            }
            CODEC_TYPE_PCM_S32BE => {
                read_pcm_signed!(self.buf, S32, reader.read_be_i32()?, 32, self.coded_width)
            }
            CODEC_TYPE_PCM_S24LE => {
                read_pcm_signed!(self.buf, S24, reader.read_i24()? << 8, 24, self.coded_width)
            }
            CODEC_TYPE_PCM_S24BE => {
                read_pcm_signed!(self.buf, S24, reader.read_be_i24()? << 8, 24, self.coded_width)
            }
            CODEC_TYPE_PCM_S16LE => {
                read_pcm_signed!(self.buf, S16, reader.read_i16()?, 16, self.coded_width)
            }
            CODEC_TYPE_PCM_S16BE => {
                read_pcm_signed!(self.buf, S16, reader.read_be_i16()?, 16, self.coded_width)
            }
            CODEC_TYPE_PCM_S8 => {
                read_pcm_signed!(self.buf, S8, reader.read_i8()?, 8, self.coded_width)
            }
            CODEC_TYPE_PCM_U32LE => {
                read_pcm_unsigned!(self.buf, U32, reader.read_u32()?, 32, self.coded_width)
            }
            CODEC_TYPE_PCM_U32BE => {
                read_pcm_unsigned!(self.buf, U32, reader.read_be_u32()?, 32, self.coded_width)
            }
            CODEC_TYPE_PCM_U24LE => {
                read_pcm_unsigned!(self.buf, U24, reader.read_u24()? << 8, 24, self.coded_width)
            }
            CODEC_TYPE_PCM_U24BE => {
                read_pcm_unsigned!(self.buf, U24, reader.read_be_u24()? << 8, 24, self.coded_width)
            }
            CODEC_TYPE_PCM_U16LE => {
                read_pcm_unsigned!(self.buf, U16, reader.read_u16()?, 16, self.coded_width)
            }
            CODEC_TYPE_PCM_U16BE => {
                read_pcm_unsigned!(self.buf, U16, reader.read_be_u16()?, 16, self.coded_width)
            }
            CODEC_TYPE_PCM_U8 => {
                read_pcm_unsigned!(self.buf, U8, reader.read_u8()?, 8, self.coded_width)
            }
            CODEC_TYPE_PCM_F32LE => {
                read_pcm_floating!(self.buf, F32, reader.read_f32()?)
            }
            CODEC_TYPE_PCM_F32BE => {
                read_pcm_floating!(self.buf, F32, reader.read_be_f32()?)
            }
            CODEC_TYPE_PCM_F64LE => {
                read_pcm_floating!(self.buf, F64, reader.read_f64()?)
            }
            CODEC_TYPE_PCM_F64BE => {
                read_pcm_floating!(self.buf, F64, reader.read_be_f64()?)
            }
            CODEC_TYPE_PCM_ALAW => {
                read_pcm_transfer_func!(self.buf, S16, alaw_to_linear(reader.read_u8()?))
            }
            CODEC_TYPE_PCM_MULAW => {
                read_pcm_transfer_func!(self.buf, S16, mulaw_to_linear(reader.read_u8()?))
            }
            // CODEC_TYPE_PCM_S32LE_PLANAR =>
            // CODEC_TYPE_PCM_S32BE_PLANAR =>
            // CODEC_TYPE_PCM_S24LE_PLANAR =>
            // CODEC_TYPE_PCM_S24BE_PLANAR =>
            // CODEC_TYPE_PCM_S16LE_PLANAR =>
            // CODEC_TYPE_PCM_S16BE_PLANAR =>
            // CODEC_TYPE_PCM_S8_PLANAR    =>
            // CODEC_TYPE_PCM_U32LE_PLANAR =>
            // CODEC_TYPE_PCM_U32BE_PLANAR =>
            // CODEC_TYPE_PCM_U24LE_PLANAR =>
            // CODEC_TYPE_PCM_U24BE_PLANAR =>
            // CODEC_TYPE_PCM_U16LE_PLANAR =>
            // CODEC_TYPE_PCM_U16BE_PLANAR =>
            // CODEC_TYPE_PCM_U8_PLANAR    =>
            // CODEC_TYPE_PCM_F32LE_PLANAR =>
            // CODEC_TYPE_PCM_F32BE_PLANAR =>
            // CODEC_TYPE_PCM_F64LE_PLANAR =>
            // CODEC_TYPE_PCM_F64BE_PLANAR =>
            _ => unsupported_error("pcm: codec is unsupported"),
        };

        Ok(())
    }
}

impl Decoder for PcmDecoder {
    fn try_new(params: &CodecParameters, _options: &DecoderOptions) -> Result<Self> {
        // This decoder only supports certain PCM codecs.
        if !is_supported_pcm_codec(params.codec) {
            return unsupported_error("pcm: invalid codec type");
        }

        let frames = match params.max_frames_per_packet {
            Some(frames) => frames,
            _ => return unsupported_error("pcm: maximum frames per packet is required"),
        };

        let rate = match params.sample_rate {
            Some(rate) => rate,
            _ => return unsupported_error("pcm: sample rate is required"),
        };

        let spec = if let Some(channels) = params.channels {
            // Atleast one channel is required.
            if channels.count() < 1 {
                return unsupported_error("pcm: number of channels cannot be 0");
            }

            SignalSpec::new(rate, channels)
        }
        else if let Some(layout) = params.channel_layout {
            SignalSpec::new_with_layout(rate, layout)
        }
        else {
            return unsupported_error("pcm: channels or channel_layout is required");
        };

        // Determine the sample format for the audio buffer based on the codec type.
        let (sample_format, sample_format_width) = match params.codec {
            CODEC_TYPE_PCM_S32LE | CODEC_TYPE_PCM_S32BE => (SampleFormat::S32, 32),
            CODEC_TYPE_PCM_S24LE | CODEC_TYPE_PCM_S24BE => (SampleFormat::S24, 24),
            CODEC_TYPE_PCM_S16LE | CODEC_TYPE_PCM_S16BE => (SampleFormat::S16, 16),
            CODEC_TYPE_PCM_S8 => (SampleFormat::S8, 8),
            CODEC_TYPE_PCM_U32LE | CODEC_TYPE_PCM_U32BE => (SampleFormat::U32, 32),
            CODEC_TYPE_PCM_U24LE | CODEC_TYPE_PCM_U24BE => (SampleFormat::U24, 24),
            CODEC_TYPE_PCM_U16LE | CODEC_TYPE_PCM_U16BE => (SampleFormat::U16, 16),
            CODEC_TYPE_PCM_U8 => (SampleFormat::U8, 8),
            CODEC_TYPE_PCM_F32LE | CODEC_TYPE_PCM_F32BE => (SampleFormat::F32, 32),
            CODEC_TYPE_PCM_F64LE | CODEC_TYPE_PCM_F64BE => (SampleFormat::F64, 64),
            CODEC_TYPE_PCM_ALAW => (SampleFormat::S16, 16),
            CODEC_TYPE_PCM_MULAW => (SampleFormat::S16, 16),
            _ => unreachable!(),
        };

        // Signed and unsigned integer PCM codecs may code fewer bits per sample than the actual
        // sample format. The number of coded bits is therefore required to shift the decoded
        // samples into the range of the sample format. Try to get the bits per coded sample
        // parameter, or, if failing that, the bits per sample parameter.
        let coded_width =
            params.bits_per_coded_sample.unwrap_or_else(|| params.bits_per_sample.unwrap_or(0));

        // If the coded sample width is unknown, then the bits per coded sample may be constant and
        // implicit to the codec.
        if coded_width == 0 {
            // A-Law, Mu-Law, and floating point codecs have an implicit coded sample bit-width. If
            // the codec is none of those, then decoding is not possible.
            match params.codec {
                CODEC_TYPE_PCM_F32LE | CODEC_TYPE_PCM_F32BE => (),
                CODEC_TYPE_PCM_F64LE | CODEC_TYPE_PCM_F64BE => (),
                CODEC_TYPE_PCM_ALAW | CODEC_TYPE_PCM_MULAW => (),
                _ => return unsupported_error("pcm: unknown bits per (coded) sample"),
            }
        }
        else if coded_width > sample_format_width {
            // If the coded sample width is greater than the width of the sample format, then the
            // stream has incorrect parameters.
            return decode_error("pcm: coded bits per sample is greater than the sample format");
        }

        // Create an audio buffer of the correct format.
        let buf = GenericAudioBuffer::new(sample_format, frames, spec);

        Ok(PcmDecoder { params: params.clone(), coded_width, buf })
    }

    fn supported_codecs() -> &'static [CodecDescriptor] {
        &[
            support_codec!(
                CODEC_TYPE_PCM_S32LE,
                "pcm_s32le",
                "PCM Signed 32-bit Little-Endian Interleaved"
            ),
            support_codec!(
                CODEC_TYPE_PCM_S32BE,
                "pcm_s32be",
                "PCM Signed 32-bit Big-Endian Interleaved"
            ),
            support_codec!(
                CODEC_TYPE_PCM_S24LE,
                "pcm_s24le",
                "PCM Signed 24-bit Little-Endian Interleaved"
            ),
            support_codec!(
                CODEC_TYPE_PCM_S24BE,
                "pcm_s24be",
                "PCM Signed 24-bit Big-Endian Interleaved"
            ),
            support_codec!(
                CODEC_TYPE_PCM_S16LE,
                "pcm_s16le",
                "PCM Signed 16-bit Little-Endian Interleaved"
            ),
            support_codec!(
                CODEC_TYPE_PCM_S16BE,
                "pcm_s16be",
                "PCM Signed 16-bit Big-Endian Interleaved"
            ),
            support_codec!(CODEC_TYPE_PCM_S8, "pcm_s8", "PCM Signed 8-bit Interleaved"),
            support_codec!(
                CODEC_TYPE_PCM_U32LE,
                "pcm_u32le",
                "PCM Unsigned 32-bit Little-Endian Interleaved"
            ),
            support_codec!(
                CODEC_TYPE_PCM_U32BE,
                "pcm_u32be",
                "PCM Unsigned 32-bit Big-Endian Interleaved"
            ),
            support_codec!(
                CODEC_TYPE_PCM_U24LE,
                "pcm_u24le",
                "PCM Unsigned 24-bit Little-Endian Interleaved"
            ),
            support_codec!(
                CODEC_TYPE_PCM_U24BE,
                "pcm_u24be",
                "PCM Unsigned 24-bit Big-Endian Interleaved"
            ),
            support_codec!(
                CODEC_TYPE_PCM_U16LE,
                "pcm_u16le",
                "PCM Unsigned 16-bit Little-Endian Interleaved"
            ),
            support_codec!(
                CODEC_TYPE_PCM_U16BE,
                "pcm_u16be",
                "PCM Unsigned 16-bit Big-Endian Interleaved"
            ),
            support_codec!(CODEC_TYPE_PCM_U8, "pcm_u8", "PCM Unsigned 8-bit Interleaved"),
            support_codec!(
                CODEC_TYPE_PCM_F32LE,
                "pcm_f32le",
                "PCM 32-bit Little-Endian Floating Point Interleaved"
            ),
            support_codec!(
                CODEC_TYPE_PCM_F32BE,
                "pcm_f32be",
                "PCM 32-bit Big-Endian Floating Point Interleaved"
            ),
            support_codec!(
                CODEC_TYPE_PCM_F64LE,
                "pcm_f64le",
                "PCM 64-bit Little-Endian Floating Point Interleaved"
            ),
            support_codec!(
                CODEC_TYPE_PCM_F64BE,
                "pcm_f64be",
                "PCM 64-bit Big-Endian Floating Point Interleaved"
            ),
            support_codec!(CODEC_TYPE_PCM_ALAW, "pcm_alaw", "PCM A-law"),
            support_codec!(CODEC_TYPE_PCM_MULAW, "pcm_mulaw", "PCM Mu-law"),
            // support_codec!(
            //     CODEC_TYPE_PCM_S32LE_PLANAR,
            //     "pcm_s32le_planar",
            //     "PCM Signed 32-bit Little-Endian Planar"
            // ),
            // support_codec!(
            //     CODEC_TYPE_PCM_S32BE_PLANAR,
            //     "pcm_s32be_planar",
            //     "PCM Signed 32-bit Big-Endian Planar"
            // ),
            // support_codec!(
            //     CODEC_TYPE_PCM_S24LE_PLANAR,
            //     "pcm_s24le_planar",
            //     "PCM Signed 24-bit Little-Endian Planar"
            // ),
            // support_codec!(
            //     CODEC_TYPE_PCM_S24BE_PLANAR,
            //     "pcm_s24be_planar",
            //     "PCM Signed 24-bit Big-Endian Planar"
            // ),
            // support_codec!(
            //     CODEC_TYPE_PCM_S16LE_PLANAR,
            //     "pcm_s16le_planar",
            //     "PCM Signed 16-bit Little-Endian Planar"
            // ),
            // support_codec!(
            //     CODEC_TYPE_PCM_S16BE_PLANAR,
            //     "pcm_s16be_planar",
            //     "PCM Signed 16-bit Big-Endian Planar"
            // ),
            // support_codec!(
            //     CODEC_TYPE_PCM_S8_PLANAR   ,
            //     "pcm_s8_planar"   ,
            //     "PCM Signed 8-bit Planar"
            // ),
            // support_codec!(
            //     CODEC_TYPE_PCM_U32LE_PLANAR,
            //     "pcm_u32le_planar",
            //     "PCM Unsigned 32-bit Little-Endian Planar"
            // ),
            // support_codec!(
            //     CODEC_TYPE_PCM_U32BE_PLANAR,
            //     "pcm_u32be_planar",
            //     "PCM Unsigned 32-bit Big-Endian Planar"
            // ),
            // support_codec!(
            //     CODEC_TYPE_PCM_U24LE_PLANAR,
            //     "pcm_u24le_planar",
            //     "PCM Unsigned 24-bit Little-Endian Planar"
            // ),
            // support_codec!(
            //     CODEC_TYPE_PCM_U24BE_PLANAR,
            //     "pcm_u24be_planar",
            //     "PCM Unsigned 24-bit Big-Endian Planar"
            // ),
            // support_codec!(
            //     CODEC_TYPE_PCM_U16LE_PLANAR,
            //     "pcm_u16le_planar",
            //     "PCM Unsigned 16-bit Little-Endian Planar"
            // ),
            // support_codec!(
            //     CODEC_TYPE_PCM_U16BE_PLANAR,
            //     "pcm_u16be_planar",
            //     "PCM Unsigned 16-bit Big-Endian Planar"
            // ),
            // support_codec!(
            //     CODEC_TYPE_PCM_U8_PLANAR   ,
            //     "pcm_u8_planar"   ,
            //     "PCM Unsigned 8-bit Planar"
            // ),
            // support_codec!(
            //     CODEC_TYPE_PCM_F32LE_PLANAR,
            //     "pcm_f32le_planar",
            //     "PCM 32-bit Little-Endian Floating Point Planar"
            // ),
            // support_codec!(
            //     CODEC_TYPE_PCM_F32BE_PLANAR,
            //     "pcm_f32be_planar",
            //     "PCM 32-bit Big-Endian Floating Point Planar"
            // ),
            // support_codec!(
            //     CODEC_TYPE_PCM_F64LE_PLANAR,
            //     "pcm_f64le_planar",
            //     "PCM 64-bit Little-Endian Floating Point Planar"
            // ),
            // support_codec!(
            //     CODEC_TYPE_PCM_F64BE_PLANAR,
            //     "pcm_f64be_planar",
            //     "PCM 64-bit Big-Endian Floating Point Planar"
            // ),
        ]
    }

    fn reset(&mut self) {
        // No state is stored between packets, therefore do nothing.
    }

    fn codec_params(&self) -> &CodecParameters {
        &self.params
    }

    fn decode(&mut self, packet: &Packet) -> Result<AudioBufferRef<'_>> {
        if let Err(e) = self.decode_inner(packet) {
            self.buf.clear();
            Err(e)
        }
        else {
            Ok(self.buf.as_audio_buffer_ref())
        }
    }

    fn finalize(&mut self) -> FinalizeResult {
        Default::default()
    }

    fn last_decoded(&self) -> AudioBufferRef<'_> {
        self.buf.as_audio_buffer_ref()
    }
}
