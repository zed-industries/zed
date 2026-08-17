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

#[cfg(target_os = "android")]
pub mod android;
pub mod apm;
pub mod audio_mixer;
pub mod audio_resampler;
pub mod audio_track;
pub mod candidate;
pub mod data_channel;
#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
pub mod desktop_capturer;
pub mod frame_cryptor;
pub mod helper;
pub mod jsep;
pub mod media_stream;
pub mod media_stream_track;
pub mod peer_connection;
pub mod peer_connection_factory;
pub mod prohibit_libsrtp_initialization;
pub mod rtc_error;
pub mod rtp_parameters;
pub mod rtp_receiver;
pub mod rtp_sender;
pub mod rtp_transceiver;
pub mod video_frame;
pub mod video_frame_buffer;
pub mod video_track;
pub mod webrtc;
pub mod yuv_helper;

pub const MEDIA_TYPE_VIDEO: &str = "video";
pub const MEDIA_TYPE_AUDIO: &str = "audio";
pub const MEDIA_TYPE_DATA: &str = "data";

macro_rules! impl_thread_safety {
    ($obj:ty, Send) => {
        unsafe impl Send for $obj {}
    };

    ($obj:ty, Send + Sync) => {
        unsafe impl Send for $obj {}
        unsafe impl Sync for $obj {}
    };
}

pub(crate) use impl_thread_safety;
