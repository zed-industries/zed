// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::audio::{AudioBuffer, Channels, Layout, SignalSpec};
use symphonia_core::codecs::{CodecType, CODEC_TYPE_MP1, CODEC_TYPE_MP2, CODEC_TYPE_MP3};
use symphonia_core::errors::Result;

use symphonia_core::io::BufReader;

/// The MPEG audio version.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MpegVersion {
    /// Version 2.5
    Mpeg2p5,
    /// Version 2
    Mpeg2,
    /// Version 1
    Mpeg1,
}

/// The MPEG audio layer.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MpegLayer {
    /// Layer 1
    Layer1,
    /// Layer 2
    Layer2,
    /// Layer 3
    Layer3,
}

/// For Joint Stereo channel mode, the mode extension describes the features and parameters of the
/// stereo encoding.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    /// Joint Stereo in layer 3 may use both Mid-Side and Intensity encoding.
    Layer3 { mid_side: bool, intensity: bool },
    /// Joint Stereo in layers 1 and 2 may only use Intensity encoding on a set of bands. The range
    /// of bands using intensity encoding is `bound..32`.
    Intensity { bound: u32 },
}

/// The channel mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ChannelMode {
    /// Single mono audio channel.
    Mono,
    /// Dual mono audio channels.
    DualMono,
    /// Stereo channels.
    Stereo,
    /// Joint Stereo encoded channels (decodes to Stereo).
    JointStereo(Mode),
}

impl ChannelMode {
    /// Gets the number of channels.
    #[inline(always)]
    pub fn count(&self) -> usize {
        match self {
            ChannelMode::Mono => 1,
            _ => 2,
        }
    }

    /// Gets the the channel map.
    #[inline(always)]
    pub fn channels(&self) -> Channels {
        match self {
            ChannelMode::Mono => Channels::FRONT_LEFT,
            _ => Channels::FRONT_LEFT | Channels::FRONT_RIGHT,
        }
    }
}

/// The emphasis applied during encoding.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Emphasis {
    /// No emphasis
    None,
    /// 50/15us
    Fifty15,
    /// CCIT J.17
    CcitJ17,
}

/// A MPEG 1, 2, or 2.5 audio frame header.
#[derive(Debug)]
pub struct FrameHeader {
    pub version: MpegVersion,
    pub layer: MpegLayer,
    pub bitrate: u32,
    pub sample_rate: u32,
    pub sample_rate_idx: usize,
    pub channel_mode: ChannelMode,
    #[allow(dead_code)]
    pub emphasis: Emphasis,
    #[allow(dead_code)]
    pub is_copyrighted: bool,
    #[allow(dead_code)]
    pub is_original: bool,
    #[allow(dead_code)]
    pub has_padding: bool,
    pub has_crc: bool,
    pub frame_size: usize,
}

impl FrameHeader {
    /// Returns true if this a MPEG1 frame, false otherwise.
    #[inline(always)]
    pub fn is_mpeg1(&self) -> bool {
        self.version == MpegVersion::Mpeg1
    }

    /// Returns true if this a MPEG2.5 frame, false otherwise.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn is_mpeg2p5(&self) -> bool {
        self.version == MpegVersion::Mpeg2p5
    }

    /// Returns the codec type for the frame.
    pub fn codec(&self) -> CodecType {
        match self.layer {
            MpegLayer::Layer1 => CODEC_TYPE_MP1,
            MpegLayer::Layer2 => CODEC_TYPE_MP2,
            MpegLayer::Layer3 => CODEC_TYPE_MP3,
        }
    }

    /// Returns a signal specification for the frame.
    #[allow(dead_code)]
    pub fn spec(&self) -> SignalSpec {
        let layout = match self.n_channels() {
            1 => Layout::Mono,
            2 => Layout::Stereo,
            _ => unreachable!(),
        };

        SignalSpec::new_with_layout(self.sample_rate, layout)
    }

    /// Returns the number of audio samples in the frame per channel.
    pub fn duration(&self) -> u64 {
        match self.layer {
            MpegLayer::Layer1 => 384,
            MpegLayer::Layer2 => 1152,
            MpegLayer::Layer3 => 576 * self.n_granules() as u64,
        }
    }

    /// Returns the number of granules in the frame.
    #[inline(always)]
    pub fn n_granules(&self) -> usize {
        match self.version {
            MpegVersion::Mpeg1 => 2,
            _ => 1,
        }
    }

    /// Returns the number of channels per granule.
    #[inline(always)]
    pub fn n_channels(&self) -> usize {
        self.channel_mode.count()
    }

    /// Returns true if Intensity Stereo encoding is used, false otherwise.
    #[allow(dead_code)]
    #[inline(always)]
    pub fn is_intensity_stereo(&self) -> bool {
        match self.channel_mode {
            ChannelMode::JointStereo(Mode::Intensity { .. }) => true,
            ChannelMode::JointStereo(Mode::Layer3 { intensity, .. }) => intensity,
            _ => false,
        }
    }

    /// Get the side information length.
    #[inline(always)]
    pub fn side_info_len(&self) -> usize {
        match (self.version, self.channel_mode) {
            (MpegVersion::Mpeg1, ChannelMode::Mono) => 17,
            (MpegVersion::Mpeg1, _) => 32,
            (_, ChannelMode::Mono) => 9,
            (_, _) => 17,
        }
    }
}

pub trait Layer {
    fn decode(
        &mut self,
        reader: &mut BufReader<'_>,
        header: &FrameHeader,
        out: &mut AudioBuffer<f32>,
    ) -> Result<()>;
}
