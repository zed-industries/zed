// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use libwebrtc::prelude::*;
use livekit_protocol as proto;

use crate::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VideoCodec {
    VP8,
    H264,
    VP9,
    AV1,
    H265,
}

impl VideoCodec {
    pub fn as_str(&self) -> &'static str {
        match self {
            VideoCodec::VP8 => "vp8",
            VideoCodec::H264 => "h264",
            VideoCodec::VP9 => "vp9",
            VideoCodec::AV1 => "av1",
            VideoCodec::H265 => "h265",
        }
    }
}

#[derive(Debug, Clone)]
pub struct VideoResolution {
    pub width: u32,
    pub height: u32,
    pub frame_rate: f64,
    pub aspect_ratio: f32,
}

#[derive(Debug, Clone)]
pub struct VideoEncoding {
    pub max_bitrate: u64,
    pub max_framerate: f64,
}

#[derive(Debug, Clone)]
pub struct VideoPreset {
    pub encoding: VideoEncoding,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct AudioEncoding {
    pub max_bitrate: u64,
}

#[derive(Debug, Clone)]
pub struct AudioPreset {
    pub encoding: AudioEncoding,
}

impl AudioPreset {
    pub const fn new(max_bitrate: u64) -> Self {
        Self { encoding: AudioEncoding { max_bitrate } }
    }
}

#[derive(Clone, Debug)]
pub struct TrackPublishOptions {
    // If the encodings aren't set, LiveKit will compute the most appropriate ones
    pub video_encoding: Option<VideoEncoding>,
    pub audio_encoding: Option<AudioEncoding>,
    pub video_codec: VideoCodec,
    pub dtx: bool,
    pub red: bool,
    pub simulcast: bool,
    // pub name: String,
    pub source: TrackSource,
    pub stream: String,
    pub preconnect_buffer: bool,
}

impl Default for TrackPublishOptions {
    fn default() -> Self {
        Self {
            video_encoding: None,
            audio_encoding: None,
            video_codec: VideoCodec::VP8,
            dtx: true,
            red: true,
            simulcast: true,
            source: TrackSource::Unknown,
            stream: "".to_string(),
            preconnect_buffer: false,
        }
    }
}

impl VideoPreset {
    pub const fn new(width: u32, height: u32, max_bitrate: u64, max_framerate: f64) -> Self {
        Self { width, height, encoding: VideoEncoding { max_bitrate, max_framerate } }
    }

    pub fn resolution(&self) -> VideoResolution {
        VideoResolution {
            width: self.width,
            height: self.height,
            frame_rate: self.encoding.max_framerate,
            aspect_ratio: self.width as f32 / self.height as f32,
        }
    }
}

/// Compute appropriate RtpEncodingParameters from the video resolution.
/// TrackPublishOptions helps to find the most appropriate encodings
pub fn compute_video_encodings(
    width: u32,
    height: u32,
    options: &TrackPublishOptions,
) -> Vec<RtpEncodingParameters> {
    let screenshare = options.source == TrackSource::Screenshare;
    let encoding = match options.video_encoding.clone() {
        Some(encoding) => encoding,
        None => compute_appropriate_encoding(screenshare, width, height, options.video_codec),
    };

    let initial_preset = VideoPreset {
        width,
        height,
        encoding: VideoEncoding {
            max_bitrate: encoding.max_bitrate,
            max_framerate: encoding.max_framerate,
        },
    };

    if !options.simulcast {
        return into_rtp_encodings(width, height, &[initial_preset]);
    }

    let mut simulcast_presets = compute_default_simulcast_presets(screenshare, &initial_preset);

    let mid_preset = simulcast_presets.pop();
    let low_preset = simulcast_presets.pop();

    let size = u32::max(width, height);

    if size >= 960 && low_preset.is_some() {
        #[allow(clippy::unnecessary_unwrap)]
        return into_rtp_encodings(
            width,
            height,
            &[low_preset.unwrap(), mid_preset.unwrap(), initial_preset],
        );
    } else if size >= 480 {
        return into_rtp_encodings(width, height, &[mid_preset.unwrap(), initial_preset]);
    }

    // Other layers not needed
    into_rtp_encodings(width, height, &[initial_preset])
}

/// Return an appropriate VideoEncdoding for the specified resolution based on our presets
pub fn compute_appropriate_encoding(
    is_screenshare: bool,
    width: u32,
    height: u32,
    codec: VideoCodec,
) -> VideoEncoding {
    let presets = compute_presets_for_resolution(is_screenshare, width, height);
    let size = u32::max(width, height);

    let mut encoding = presets.first().unwrap().encoding.clone();

    for preset in presets {
        encoding = preset.encoding.clone();
        if preset.width > size {
            break;
        }
    }

    match codec {
        VideoCodec::VP9 => encoding.max_bitrate = (encoding.max_bitrate as f32 * 0.85) as u64,
        VideoCodec::AV1 => encoding.max_bitrate = (encoding.max_bitrate as f32 * 0.7) as u64,
        _ => {}
    }

    encoding
}

pub fn compute_presets_for_resolution(
    is_screenshare: bool,
    width: u32,
    height: u32,
) -> &'static [VideoPreset] {
    if is_screenshare {
        return screenshare::PRESETS;
    }

    // Check how close width & height are from 16/9 or 4/3
    let ar = landscape_aspect_ratio(width, height);
    if f32::abs(ar - 16.0 / 9.0) < f32::abs(ar - 4.0 / 3.0) {
        return video::PRESETS;
    }

    video43::PRESETS
}

/// Returns our most appropriate default presets
pub fn compute_default_simulcast_presets(
    is_screenshare: bool,
    initial: &VideoPreset,
) -> Vec<VideoPreset> {
    if is_screenshare {
        return vec![screenshare::compute_default_simulcast_preset(initial)];
    }

    let ar = landscape_aspect_ratio(initial.width, initial.height);
    if f32::abs(ar - 16.0 / 9.0) < f32::abs(ar - 4.0 / 3.0) {
        return video::DEFAULT_SIMULCAST_PRESETS.to_owned();
    }

    video43::DEFAULT_SIMULCAST_PRESETS.to_owned()
}

pub fn landscape_aspect_ratio(width: u32, height: u32) -> f32 {
    if width > height {
        width as f32 / height as f32
    } else {
        height as f32 / width as f32
    }
}

/// Presets must be ordered
pub fn into_rtp_encodings(
    initial_width: u32,
    initial_height: u32,
    presets: &[VideoPreset],
) -> Vec<RtpEncodingParameters> {
    let mut encodings = Vec::with_capacity(presets.len());
    let size = u32::min(initial_width, initial_height);
    for (i, preset) in presets.iter().enumerate() {
        encodings.push(RtpEncodingParameters {
            rid: VIDEO_RIDS[i].to_string(),
            scale_resolution_down_by: Some(f64::max(
                1.0,
                size as f64 / u32::min(preset.width, preset.height) as f64,
            )),
            max_bitrate: Some(preset.encoding.max_bitrate),
            max_framerate: Some(preset.encoding.max_framerate),
            ..Default::default()
        })
    }

    encodings.reverse();
    encodings
}

pub fn video_quality_for_rid(rid: &str) -> Option<proto::VideoQuality> {
    match rid {
        "f" => Some(proto::VideoQuality::High),
        "h" => Some(proto::VideoQuality::Medium),
        "q" => Some(proto::VideoQuality::Low),
        _ => None,
    }
}

pub fn video_layers_from_encodings(
    width: u32,
    height: u32,
    encodings: &[RtpEncodingParameters],
) -> Vec<proto::VideoLayer> {
    if encodings.is_empty() {
        return vec![proto::VideoLayer {
            quality: proto::VideoQuality::High as i32,
            width,
            height,
            bitrate: 0,
            ssrc: 0,
            ..Default::default()
        }];
    }

    let mut layers = Vec::with_capacity(encodings.len());
    for encoding in encodings {
        let scale = encoding.scale_resolution_down_by.unwrap_or(1.0);
        let quality = video_quality_for_rid(&encoding.rid).unwrap_or(proto::VideoQuality::High);

        layers.push(proto::VideoLayer {
            quality: quality as i32,
            width: (width as f64 / scale) as u32,
            height: (height as f64 / scale) as u32,
            bitrate: encoding.max_bitrate.unwrap_or(0) as u32,
            ssrc: 0,
            ..Default::default()
        });
    }

    layers
}

const VIDEO_RIDS: &[char] = &['q', 'h', 'f'];

pub mod audio {
    use super::AudioPreset;

    pub const TELEPHONE: AudioPreset = AudioPreset::new(12_000);
    pub const SPEECH: AudioPreset = AudioPreset::new(24_000);
    pub const MUSIC: AudioPreset = AudioPreset::new(48_000);
    pub const MUSIC_STEREO: AudioPreset = AudioPreset::new(64_000);
    pub const MUSIC_HIGH_QUALITY: AudioPreset = AudioPreset::new(96_000);
    pub const MUSIC_HIGH_QUALITY_STEREO: AudioPreset = AudioPreset::new(128_000);

    pub const PRESETS: &[AudioPreset] =
        &[TELEPHONE, SPEECH, MUSIC, MUSIC_STEREO, MUSIC_HIGH_QUALITY, MUSIC_HIGH_QUALITY_STEREO];
}

pub mod video {
    use super::VideoPreset;

    pub const H90: VideoPreset = VideoPreset::new(160, 90, 90_000, 15.0);
    pub const H180: VideoPreset = VideoPreset::new(320, 180, 160_000, 15.0);
    pub const H216: VideoPreset = VideoPreset::new(384, 216, 180_000, 15.0);
    pub const H360: VideoPreset = VideoPreset::new(640, 360, 450_000, 20.0);
    pub const H540: VideoPreset = VideoPreset::new(960, 540, 800_000, 25.0);
    pub const H720: VideoPreset = VideoPreset::new(1280, 720, 1_700_000, 30.0);
    pub const H1080: VideoPreset = VideoPreset::new(1920, 1080, 3_000_000, 30.0);
    pub const H1440: VideoPreset = VideoPreset::new(2560, 1440, 5_000_000, 30.0);
    pub const H2160: VideoPreset = VideoPreset::new(3840, 2160, 8_000_000, 30.0);

    pub const PRESETS: &[VideoPreset] = &[H90, H180, H216, H360, H540, H720, H1080, H1440, H2160];
    pub const DEFAULT_SIMULCAST_PRESETS: &[VideoPreset] = &[H180, H360];
}

pub mod video43 {
    use super::VideoPreset;

    pub const H120: VideoPreset = VideoPreset::new(160, 120, 80_000, 15.0);
    pub const H180: VideoPreset = VideoPreset::new(240, 180, 100_000, 15.0);
    pub const H240: VideoPreset = VideoPreset::new(320, 240, 150_000, 15.0);
    pub const H360: VideoPreset = VideoPreset::new(480, 360, 225_000, 20.0);
    pub const H480: VideoPreset = VideoPreset::new(640, 480, 300_000, 20.0);
    pub const H540: VideoPreset = VideoPreset::new(720, 540, 450_000, 25.0);
    pub const H720: VideoPreset = VideoPreset::new(960, 720, 1_500_000, 30.0);
    pub const H1080: VideoPreset = VideoPreset::new(1440, 1080, 2_500_000, 30.0);
    pub const H1440: VideoPreset = VideoPreset::new(1920, 1440, 3_500_000, 30.0);

    pub const PRESETS: &[VideoPreset] = &[H120, H180, H240, H360, H480, H540, H720, H1080, H1440];
    pub const DEFAULT_SIMULCAST_PRESETS: &[VideoPreset] = &[H180, H360];
}

pub mod screenshare {
    /// The screenshare presets are optimized for quality.
    /// When simulcasting, we prefer to reduce the FPS.
    use super::VideoPreset;

    pub const H360_FPS3: VideoPreset = VideoPreset::new(640, 360, 200_000, 3.0);
    pub const H720_FPS5: VideoPreset = VideoPreset::new(1280, 720, 400_000, 5.0);
    pub const H720_FPS15: VideoPreset = VideoPreset::new(1280, 720, 1_000_000, 15.0);
    pub const H1080_FPS15: VideoPreset = VideoPreset::new(1920, 1080, 1_500_000, 15.0);
    pub const H1080_FPS30: VideoPreset = VideoPreset::new(1920, 1080, 3_000_000, 30.0);

    pub const PRESETS: &[VideoPreset] =
        &[H360_FPS3, H720_FPS5, H720_FPS15, H1080_FPS15, H1080_FPS30];

    /// Only one additional layer for screenshares. (Prioritize quality)
    pub fn compute_default_simulcast_preset(initial: &VideoPreset) -> VideoPreset {
        const SCALE_DOWN_FACTOR: u32 = 2;
        const FPS: f64 = 3.0;

        VideoPreset::new(
            initial.width / SCALE_DOWN_FACTOR,
            initial.height / SCALE_DOWN_FACTOR,
            u64::max(
                150_000,
                initial.encoding.max_bitrate
                    / (SCALE_DOWN_FACTOR.pow(2) as u64
                        * (initial.encoding.max_framerate / FPS) as u64),
            ),
            FPS,
        )
    }
}
