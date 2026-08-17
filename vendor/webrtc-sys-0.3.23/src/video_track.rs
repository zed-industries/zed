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

use std::sync::Arc;

use cxx::UniquePtr;

use crate::{impl_thread_safety, video_frame::ffi::VideoFrame};

#[cxx::bridge(namespace = "livekit_ffi")]
pub mod ffi {
    #[repr(i32)]
    pub enum ContentHint {
        None,
        Fluid,
        Detailed,
        Text,
    }

    #[derive(Debug)]
    pub struct VideoTrackSourceConstraints {
        pub has_min_fps: bool,
        pub min_fps: f64,
        pub has_max_fps: bool,
        pub max_fps: f64,
    }

    #[derive(Debug)]
    pub struct VideoResolution {
        pub width: u32,
        pub height: u32,
    }

    extern "C++" {
        include!("livekit/video_frame.h");
        include!("livekit/media_stream_track.h");

        type VideoFrame = crate::video_frame::ffi::VideoFrame;
        type MediaStreamTrack = crate::media_stream_track::ffi::MediaStreamTrack;
    }

    unsafe extern "C++" {
        include!("livekit/video_track.h");

        type VideoTrack;
        type NativeVideoSink;
        type VideoTrackSource;

        fn add_sink(self: &VideoTrack, sink: &SharedPtr<NativeVideoSink>);
        fn remove_sink(self: &VideoTrack, sink: &SharedPtr<NativeVideoSink>);
        fn set_should_receive(self: &VideoTrack, should_receive: bool);
        fn should_receive(self: &VideoTrack) -> bool;
        fn content_hint(self: &VideoTrack) -> ContentHint;
        fn set_content_hint(self: &VideoTrack, hint: ContentHint);
        fn new_native_video_sink(observer: Box<VideoSinkWrapper>) -> SharedPtr<NativeVideoSink>;

        fn video_resolution(self: &VideoTrackSource) -> VideoResolution;
        fn on_captured_frame(self: &VideoTrackSource, frame: &UniquePtr<VideoFrame>) -> bool;
        fn new_video_track_source(
            resolution: &VideoResolution,
            is_screencast: bool,
        ) -> SharedPtr<VideoTrackSource>;
        fn video_to_media(track: SharedPtr<VideoTrack>) -> SharedPtr<MediaStreamTrack>;
        unsafe fn media_to_video(track: SharedPtr<MediaStreamTrack>) -> SharedPtr<VideoTrack>;
        fn _shared_video_track() -> SharedPtr<VideoTrack>;
    }

    extern "Rust" {
        type VideoSinkWrapper;

        fn on_frame(self: &VideoSinkWrapper, frame: UniquePtr<VideoFrame>);
        fn on_discarded_frame(self: &VideoSinkWrapper);
        fn on_constraints_changed(
            self: &VideoSinkWrapper,
            constraints: VideoTrackSourceConstraints,
        );
    }
}

impl_thread_safety!(ffi::VideoTrack, Send + Sync);
impl_thread_safety!(ffi::NativeVideoSink, Send + Sync);
impl_thread_safety!(ffi::VideoTrackSource, Send + Sync);

pub trait VideoSink: Send {
    fn on_frame(&self, frame: UniquePtr<VideoFrame>);
    fn on_discarded_frame(&self);
    fn on_constraints_changed(&self, constraints: ffi::VideoTrackSourceConstraints);
}

pub struct VideoSinkWrapper {
    observer: Arc<dyn VideoSink>,
}

impl VideoSinkWrapper {
    pub fn new(observer: Arc<dyn VideoSink>) -> Self {
        Self { observer }
    }

    fn on_frame(&self, frame: UniquePtr<VideoFrame>) {
        self.observer.on_frame(frame);
    }

    fn on_discarded_frame(&self) {
        self.observer.on_discarded_frame();
    }

    fn on_constraints_changed(&self, constraints: ffi::VideoTrackSourceConstraints) {
        self.observer.on_constraints_changed(constraints);
    }
}
