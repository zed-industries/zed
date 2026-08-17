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
    extern "C++" {
        include!("livekit/helper.h");
        include!("livekit/media_stream_track.h");
        include!("livekit/audio_track.h");
        include!("livekit/video_track.h");

        type MediaStreamTrack = crate::media_stream_track::ffi::MediaStreamTrack;
        type AudioTrack = crate::audio_track::ffi::AudioTrack;
        type VideoTrack = crate::video_track::ffi::VideoTrack;
        type VideoTrackPtr = crate::helper::ffi::VideoTrackPtr;
        type AudioTrackPtr = crate::helper::ffi::AudioTrackPtr;
    }

    unsafe extern "C++" {
        include!("livekit/media_stream.h");

        type MediaStream;

        fn id(self: &MediaStream) -> String;
        fn get_audio_tracks(self: &MediaStream) -> Vec<AudioTrackPtr>;
        fn get_video_tracks(self: &MediaStream) -> Vec<VideoTrackPtr>;
        fn find_audio_track(self: &MediaStream, track_id: String) -> SharedPtr<AudioTrack>;
        fn find_video_track(self: &MediaStream, track_id: String) -> SharedPtr<VideoTrack>;
        fn add_track(self: &MediaStream, audio_track: SharedPtr<MediaStreamTrack>) -> bool;
        fn remove_track(self: &MediaStream, audio_track: SharedPtr<MediaStreamTrack>) -> bool;

        fn _shared_media_stream() -> SharedPtr<MediaStream>;
    }
}

impl_thread_safety!(ffi::MediaStream, Send + Sync);
