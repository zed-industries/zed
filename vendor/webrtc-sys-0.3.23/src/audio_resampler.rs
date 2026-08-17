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
    unsafe extern "C++" {
        include!("livekit/audio_resampler.h");

        type AudioResampler;

        unsafe fn remix_and_resample(
            self: Pin<&mut AudioResampler>,
            src: *const i16,
            samples_per_channel: usize,
            num_channels: usize,
            sample_rate: i32,
            dst_num_channels: usize,
            dst_sample_rate: i32,
        ) -> usize;

        unsafe fn data(self: &AudioResampler) -> *const i16;

        fn create_audio_resampler() -> UniquePtr<AudioResampler>;
    }
}

impl_thread_safety!(ffi::AudioResampler, Send + Sync);
