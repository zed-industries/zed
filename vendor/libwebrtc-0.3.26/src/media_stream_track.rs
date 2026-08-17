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

use livekit_protocol::enum_dispatch;

use crate::{audio_track::RtcAudioTrack, video_track::RtcVideoTrack};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RtcTrackState {
    Live,
    Ended,
}

#[derive(Debug, Clone)]
pub enum MediaStreamTrack {
    Video(RtcVideoTrack),
    Audio(RtcAudioTrack),
}

#[cfg(not(target_arch = "wasm32"))]
impl MediaStreamTrack {
    enum_dispatch!(
        [Video, Audio];
        pub(crate) fn sys_handle(self: &Self) -> cxx::SharedPtr<webrtc_sys::media_stream::ffi::MediaStreamTrack>;
    );
}

impl MediaStreamTrack {
    enum_dispatch!(
        [Video, Audio];
        pub fn id(self: &Self) -> String;
        pub fn enabled(self: &Self) -> bool;
        pub fn set_enabled(self: &Self, enabled: bool) -> bool;
        pub fn state(self: &Self) -> RtcTrackState;
    );
}

macro_rules! media_stream_track {
    () => {
        pub fn id(&self) -> String {
            self.handle.id()
        }

        pub fn enabled(&self) -> bool {
            self.handle.enabled()
        }

        pub fn set_enabled(&self, enabled: bool) -> bool {
            self.handle.set_enabled(enabled)
        }

        pub fn state(&self) -> RtcTrackState {
            self.handle.state().into()
        }

        #[cfg(not(target_arch = "wasm32"))]
        pub(crate) fn sys_handle(
            &self,
        ) -> cxx::SharedPtr<webrtc_sys::media_stream::ffi::MediaStreamTrack> {
            self.handle.sys_handle()
        }
    };
}

pub(crate) use media_stream_track;

impl From<RtcAudioTrack> for MediaStreamTrack {
    fn from(track: RtcAudioTrack) -> Self {
        Self::Audio(track)
    }
}

impl From<RtcVideoTrack> for MediaStreamTrack {
    fn from(track: RtcVideoTrack) -> Self {
        Self::Video(track)
    }
}
