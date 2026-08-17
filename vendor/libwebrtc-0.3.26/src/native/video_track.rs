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
use sys_vt::ffi::video_to_media;
use webrtc_sys::video_track as sys_vt;

use super::media_stream_track::impl_media_stream_track;
use crate::media_stream_track::RtcTrackState;

#[derive(Clone)]
pub struct RtcVideoTrack {
    pub(crate) sys_handle: SharedPtr<sys_vt::ffi::VideoTrack>,
}

impl RtcVideoTrack {
    impl_media_stream_track!(video_to_media);

    pub fn sys_handle(&self) -> SharedPtr<sys_vt::ffi::MediaStreamTrack> {
        video_to_media(self.sys_handle.clone())
    }
}
