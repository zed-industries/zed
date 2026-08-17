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

use cxx::type_id;
use cxx::ExternType;
use std::any::Any;
use std::sync::Arc;

use crate::impl_thread_safety;

#[cxx::bridge(namespace = "livekit_ffi")]
pub mod ffi {

    pub struct AudioSourceOptions {
        pub echo_cancellation: bool,
        pub noise_suppression: bool,
        pub auto_gain_control: bool,
    }

    extern "C++" {
        include!("livekit/media_stream_track.h");

        type MediaStreamTrack = crate::media_stream_track::ffi::MediaStreamTrack;
        type CompleteCallback = crate::audio_track::CompleteCallback;
    }

    unsafe extern "C++" {
        include!("livekit/audio_track.h");

        type AudioTrack;
        type NativeAudioSink;
        type AudioTrackSource;

        fn add_sink(self: &AudioTrack, sink: &SharedPtr<NativeAudioSink>);
        fn remove_sink(self: &AudioTrack, sink: &SharedPtr<NativeAudioSink>);
        fn new_native_audio_sink(
            observer: Box<AudioSinkWrapper>,
            sample_rate: i32,
            num_channels: i32,
        ) -> SharedPtr<NativeAudioSink>;

        unsafe fn capture_frame(
            self: &AudioTrackSource,
            data: &[i16],
            sample_rate: u32,
            nb_channels: u32,
            nb_frames: usize,
            userdata: *const SourceContext,
            on_complete: CompleteCallback,
        ) -> bool;
        fn clear_buffer(self: &AudioTrackSource);
        fn audio_options(self: &AudioTrackSource) -> AudioSourceOptions;
        fn set_audio_options(self: &AudioTrackSource, options: &AudioSourceOptions);

        fn new_audio_track_source(
            options: AudioSourceOptions,
            sample_rate: i32,
            num_channels: i32,
            queue_size_ms: i32,
        ) -> SharedPtr<AudioTrackSource>;

        fn audio_to_media(track: SharedPtr<AudioTrack>) -> SharedPtr<MediaStreamTrack>;
        unsafe fn media_to_audio(track: SharedPtr<MediaStreamTrack>) -> SharedPtr<AudioTrack>;
        fn _shared_audio_track() -> SharedPtr<AudioTrack>;
        fn _shared_audio_track_source() -> SharedPtr<AudioTrackSource>;
    }

    extern "Rust" {
        type AudioSinkWrapper;
        type SourceContext;

        fn on_data(
            self: &AudioSinkWrapper,
            data: &[i16],
            sample_rate: i32,
            nb_channels: usize,
            nb_frames: usize,
        );
    }
}

impl_thread_safety!(ffi::AudioTrack, Send + Sync);
impl_thread_safety!(ffi::NativeAudioSink, Send + Sync);
impl_thread_safety!(ffi::AudioTrackSource, Send + Sync);

#[repr(transparent)]
pub struct SourceContext(pub Box<dyn Any + Send>);

#[repr(transparent)]
pub struct CompleteCallback(pub extern "C" fn(ctx: *const SourceContext));

unsafe impl ExternType for CompleteCallback {
    type Id = type_id!("livekit_ffi::CompleteCallback");
    type Kind = cxx::kind::Trivial;
}

pub trait AudioSink: Send {
    fn on_data(&self, data: &[i16], sample_rate: i32, nb_channels: usize, nb_frames: usize);
}

pub struct AudioSinkWrapper {
    observer: Arc<dyn AudioSink>,
}

impl AudioSinkWrapper {
    pub fn new(observer: Arc<dyn AudioSink>) -> Self {
        Self { observer }
    }

    fn on_data(&self, data: &[i16], sample_rate: i32, nb_channels: usize, nb_frames: usize) {
        self.observer.on_data(data, sample_rate, nb_channels, nb_frames);
    }
}
