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

use cxx::SharedPtr;
use webrtc_sys::media_stream as sys_ms;

use crate::{
    audio_track,
    imp::{audio_track::RtcAudioTrack, video_track::RtcVideoTrack},
    video_track,
};

#[derive(Clone)]
pub struct MediaStream {
    pub(crate) sys_handle: SharedPtr<sys_ms::ffi::MediaStream>,
}

impl MediaStream {
    pub fn id(&self) -> String {
        self.sys_handle.id()
    }

    pub fn audio_tracks(&self) -> Vec<audio_track::RtcAudioTrack> {
        self.sys_handle
            .get_audio_tracks()
            .into_iter()
            .map(|t| audio_track::RtcAudioTrack { handle: RtcAudioTrack { sys_handle: t.ptr } })
            .collect()
    }

    pub fn video_tracks(&self) -> Vec<video_track::RtcVideoTrack> {
        self.sys_handle
            .get_video_tracks()
            .into_iter()
            .map(|t| video_track::RtcVideoTrack { handle: RtcVideoTrack { sys_handle: t.ptr } })
            .collect()
    }
}
