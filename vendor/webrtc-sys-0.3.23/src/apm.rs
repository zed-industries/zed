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
    /// Mirrors `webrtc::AudioProcessing::Config::GainController2::AdaptiveDigital`.
    #[derive(Clone, Copy, Debug)]
    pub struct AdaptiveDigitalConfig {
        pub enabled: bool,
        pub headroom_db: f32,
        pub max_gain_db: f32,
        pub initial_gain_db: f32,
        pub max_gain_change_db_per_second: f32,
        pub max_output_noise_level_dbfs: f32,
    }

    /// Mirrors `webrtc::AudioProcessing::Config::GainController2` (AGC2).
    #[derive(Clone, Copy, Debug)]
    pub struct GainController2Config {
        pub enabled: bool,
        pub input_volume_controller_enabled: bool,
        pub adaptive_digital: AdaptiveDigitalConfig,
        pub fixed_digital_gain_db: f32,
    }

    /// Mirrors `webrtc::AudioProcessing::Config`.
    #[derive(Clone, Copy, Debug)]
    pub struct AudioProcessingConfig {
        pub echo_canceller_enabled: bool,
        pub gain_controller2: GainController2Config,
        pub high_pass_filter_enabled: bool,
        pub noise_suppression_enabled: bool,
    }

    unsafe extern "C++" {
        include!("livekit/apm.h");

        type AudioProcessingModule;

        unsafe fn process_stream(
            self: Pin<&mut AudioProcessingModule>,
            src: *const i16,
            src_len: usize,
            dst: *mut i16,
            dst_len: usize,
            sample_rate: i32,
            num_channels: i32,
        ) -> i32;

        unsafe fn process_reverse_stream(
            self: Pin<&mut AudioProcessingModule>,
            src: *const i16,
            src_len: usize,
            dst: *mut i16,
            dst_len: usize,
            sample_rate: i32,
            num_channels: i32,
        ) -> i32;

        fn set_stream_delay_ms(self: Pin<&mut AudioProcessingModule>, delay: i32) -> i32;

        fn create_apm(config: &AudioProcessingConfig) -> UniquePtr<AudioProcessingModule>;
    }
}

impl_thread_safety!(ffi::AudioProcessingModule, Send + Sync);

// `Default` lives in this crate because the structs do; downstream crates
// can't add custom `impl Default` for foreign types due to the orphan rule.
// The values mirror the field-initializer defaults of
// `webrtc::AudioProcessing::Config` exactly, so an
// `AudioProcessingConfig::default()` round-trips through the C++ layer to
// the same `webrtc::Config` an unconfigured `webrtc::AudioProcessing::Config`
// would produce.
impl Default for ffi::AdaptiveDigitalConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            headroom_db: 5.0,
            max_gain_db: 50.0,
            initial_gain_db: 15.0,
            max_gain_change_db_per_second: 6.0,
            max_output_noise_level_dbfs: -50.0,
        }
    }
}

impl Default for ffi::GainController2Config {
    fn default() -> Self {
        Self {
            enabled: false,
            input_volume_controller_enabled: false,
            adaptive_digital: ffi::AdaptiveDigitalConfig::default(),
            fixed_digital_gain_db: 0.0,
        }
    }
}

impl Default for ffi::AudioProcessingConfig {
    fn default() -> Self {
        Self {
            echo_canceller_enabled: false,
            gain_controller2: ffi::GainController2Config::default(),
            high_pass_filter_enabled: false,
            noise_suppression_enabled: false,
        }
    }
}
