// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::audio::{AsAudioBufferRef, AudioBuffer, AudioBufferRef, Signal};
use symphonia_core::codecs::{CodecDescriptor, CodecParameters, CodecType};
use symphonia_core::codecs::{Decoder, DecoderOptions, FinalizeResult};
use symphonia_core::errors::{decode_error, unsupported_error, Result};
use symphonia_core::formats::Packet;
use symphonia_core::io::FiniteStream;
use symphonia_core::support_codec;

#[cfg(feature = "mp1")]
use symphonia_core::codecs::CODEC_TYPE_MP1;
#[cfg(feature = "mp2")]
use symphonia_core::codecs::CODEC_TYPE_MP2;
#[cfg(feature = "mp3")]
use symphonia_core::codecs::CODEC_TYPE_MP3;

use super::{common::*, header};

#[cfg(feature = "mp1")]
use crate::layer1;
#[cfg(feature = "mp2")]
use crate::layer2;
#[cfg(feature = "mp3")]
use crate::layer3;

enum State {
    #[cfg(feature = "mp1")]
    Layer1(layer1::Layer1),
    #[cfg(feature = "mp2")]
    Layer2(layer2::Layer2),
    #[cfg(feature = "mp3")]
    Layer3(Box<layer3::Layer3>),
}

impl State {
    fn new(codec: CodecType) -> Self {
        match codec {
            #[cfg(feature = "mp1")]
            CODEC_TYPE_MP1 => State::Layer1(layer1::Layer1::new()),
            #[cfg(feature = "mp2")]
            CODEC_TYPE_MP2 => State::Layer2(layer2::Layer2::new()),
            #[cfg(feature = "mp3")]
            CODEC_TYPE_MP3 => State::Layer3(Box::new(layer3::Layer3::new())),
            _ => unreachable!(),
        }
    }
}

/// MPEG1 and MPEG2 audio layer 1, 2, and 3 decoder.
pub struct MpaDecoder {
    params: CodecParameters,
    state: State,
    buf: AudioBuffer<f32>,
}

impl MpaDecoder {
    fn decode_inner(&mut self, packet: &Packet) -> Result<()> {
        let mut reader = packet.as_buf_reader();

        let header = header::read_frame_header(&mut reader)?;

        // The packet should be the size stated in the header.
        if header.frame_size != reader.bytes_available() as usize {
            return decode_error("mpa: invalid packet length");
        }

        // The audio buffer can only be created after the first frame is decoded.
        if self.buf.is_unused() {
            self.buf = AudioBuffer::new(1152, header.spec());
        }
        else {
            // Ensure the packet contains an audio frame with the same signal specification as the
            // buffer.
            //
            // TODO: Is it worth it to support changing signal specifications?
            if self.buf.spec() != &header.spec() {
                return decode_error("mpa: invalid audio buffer signal spec for packet");
            }
        }

        // Clear the audio buffer.
        self.buf.clear();

        // Choose the decode step based on the MPEG layer and the current codec type.
        match &mut self.state {
            #[cfg(feature = "mp1")]
            State::Layer1(layer) if header.layer == MpegLayer::Layer1 => {
                layer.decode(&mut reader, &header, &mut self.buf)?;
            }
            #[cfg(feature = "mp2")]
            State::Layer2(layer) if header.layer == MpegLayer::Layer2 => {
                layer.decode(&mut reader, &header, &mut self.buf)?;
            }
            #[cfg(feature = "mp3")]
            State::Layer3(layer) if header.layer == MpegLayer::Layer3 => {
                layer.decode(&mut reader, &header, &mut self.buf)?;
            }
            _ => return decode_error("mpa: invalid mpeg audio layer"),
        }

        self.buf.trim(packet.trim_start() as usize, packet.trim_end() as usize);

        Ok(())
    }
}

impl Decoder for MpaDecoder {
    fn try_new(params: &CodecParameters, _: &DecoderOptions) -> Result<Self> {
        // This decoder only supports MP1, MP2, and MP3.
        match params.codec {
            #[cfg(feature = "mp1")]
            CODEC_TYPE_MP1 => (),
            #[cfg(feature = "mp2")]
            CODEC_TYPE_MP2 => (),
            #[cfg(feature = "mp3")]
            CODEC_TYPE_MP3 => (),
            _ => return unsupported_error("mpa: invalid codec type"),
        }

        // Create decoder state.
        let state = State::new(params.codec);

        Ok(MpaDecoder { params: params.clone(), state, buf: AudioBuffer::unused() })
    }

    fn supported_codecs() -> &'static [CodecDescriptor] {
        &[
            #[cfg(feature = "mp1")]
            support_codec!(CODEC_TYPE_MP1, "mp1", "MPEG Audio Layer 1"),
            #[cfg(feature = "mp2")]
            support_codec!(CODEC_TYPE_MP2, "mp2", "MPEG Audio Layer 2"),
            #[cfg(feature = "mp3")]
            support_codec!(CODEC_TYPE_MP3, "mp3", "MPEG Audio Layer 3"),
        ]
    }

    fn codec_params(&self) -> &CodecParameters {
        &self.params
    }

    fn reset(&mut self) {
        // Fully reset the decoder state.
        self.state = State::new(self.params.codec);
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
