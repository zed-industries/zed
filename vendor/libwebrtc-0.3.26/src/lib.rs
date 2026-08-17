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

use thiserror::Error;

#[cfg_attr(target_arch = "wasm32", path = "web/mod.rs")]
#[cfg_attr(not(target_arch = "wasm32"), path = "native/mod.rs")]
mod imp;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MediaType {
    Audio,
    Video,
    Data,
    Unsupported,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RtcErrorType {
    Internal,
    InvalidSdp,
    InvalidState,
}

#[derive(Error, Debug)]
#[error("an RtcError occured: {error_type:?} - {message}")]
pub struct RtcError {
    pub error_type: RtcErrorType,
    pub message: String,
}

pub mod audio_frame;
pub mod audio_source;
pub mod audio_stream;
pub mod audio_track;
pub mod data_channel;
#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
pub mod desktop_capturer;
pub mod ice_candidate;
pub mod media_stream;
pub mod media_stream_track;
pub mod peer_connection;
pub mod peer_connection_factory;
pub mod prelude;
pub mod rtp_parameters;
pub mod rtp_receiver;
pub mod rtp_sender;
pub mod rtp_transceiver;
pub mod session_description;
pub mod stats;
pub mod video_frame;
pub mod video_source;
pub mod video_stream;
pub mod video_track;

#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    pub use webrtc_sys::webrtc::ffi::create_random_uuid;

    pub use crate::imp::{apm, audio_mixer, audio_resampler, frame_cryptor, yuv_helper};
}

#[cfg(target_os = "android")]
pub mod android {
    pub use crate::imp::android::*;
}
