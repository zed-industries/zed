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

use crate::impl_thread_safety;

#[cxx::bridge(namespace = "livekit_ffi")]
pub mod ffi {
    #[repr(i32)]
    pub enum TrackState {
        Live,
        Ended,
    }

    unsafe extern "C++" {
        include!("livekit/media_stream_track.h");

        type MediaStreamTrack;

        fn kind(self: &MediaStreamTrack) -> String;
        fn id(self: &MediaStreamTrack) -> String;
        fn enabled(self: &MediaStreamTrack) -> bool;
        fn set_enabled(self: &MediaStreamTrack, enable: bool) -> bool;
        fn state(self: &MediaStreamTrack) -> TrackState;

        fn _shared_media_stream_track() -> SharedPtr<MediaStreamTrack>;
    }
}

impl_thread_safety!(ffi::MediaStreamTrack, Send + Sync);
