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

use cxx::UniquePtr;
use webrtc_sys::apm::ffi as sys_apm;

pub use webrtc_sys::apm::ffi::{
    AdaptiveDigitalConfig, AudioProcessingConfig, GainController2Config,
};

use crate::{RtcError, RtcErrorType};

pub struct AudioProcessingModule {
    sys_handle: UniquePtr<sys_apm::AudioProcessingModule>,
}

impl AudioProcessingModule {
    /// Constructs an APM from a full config struct.
    pub fn new(config: AudioProcessingConfig) -> Self {
        Self { sys_handle: sys_apm::create_apm(&config) }
    }

    /// Backward-compatible four-flag constructor.
    ///
    /// Each flag toggles only the corresponding component's `.enabled`
    /// field; every other config value stays at WebRTC's default. In
    /// particular, `gain_controller_enabled = true` does *not* enable
    /// AGC2's `adaptive_digital` controller, matching the behavior of
    /// the previous binding. Use [`AudioProcessingModule::new`] with an
    /// explicit `AudioProcessingConfig` to opt into adaptive gain.
    pub fn from_flags(
        echo_canceller_enabled: bool,
        gain_controller_enabled: bool,
        high_pass_filter_enabled: bool,
        noise_suppression_enabled: bool,
    ) -> Self {
        Self::new(AudioProcessingConfig {
            echo_canceller_enabled,
            gain_controller2: GainController2Config {
                enabled: gain_controller_enabled,
                ..Default::default()
            },
            high_pass_filter_enabled,
            noise_suppression_enabled,
        })
    }

    pub fn process_stream(
        &mut self,
        data: &mut [i16],
        sample_rate: i32,
        num_channels: i32,
    ) -> Result<(), RtcError> {
        let samples_per_10ms = (sample_rate as usize / 100) * num_channels as usize;
        assert!(
            data.len() % samples_per_10ms == 0 && data.len() >= samples_per_10ms,
            "slice must have a multiple of 10ms worth of samples"
        );

        for chunk in data.chunks_mut(samples_per_10ms) {
            if unsafe {
                self.sys_handle.pin_mut().process_stream(
                    chunk.as_mut_ptr(),
                    chunk.len(),
                    chunk.as_mut_ptr(),
                    chunk.len(),
                    sample_rate,
                    num_channels,
                )
            } != 0
            {
                return Err(RtcError {
                    error_type: RtcErrorType::Internal,
                    message: "Failed to process stream".to_string(),
                });
            }
        }

        Ok(())
    }

    pub fn process_reverse_stream(
        &mut self,
        data: &mut [i16],
        sample_rate: i32,
        num_channels: i32,
    ) -> Result<(), RtcError> {
        let samples_per_10ms = (sample_rate as usize / 100) * num_channels as usize;
        assert!(
            data.len() % samples_per_10ms == 0 && data.len() >= samples_per_10ms,
            "slice must have a multiple of 10ms worth of samples"
        );

        for chunk in data.chunks_mut(samples_per_10ms) {
            if unsafe {
                self.sys_handle.pin_mut().process_reverse_stream(
                    chunk.as_mut_ptr(),
                    chunk.len(),
                    chunk.as_mut_ptr(),
                    chunk.len(),
                    sample_rate,
                    num_channels,
                )
            } != 0
            {
                return Err(RtcError {
                    error_type: RtcErrorType::Internal,
                    message: "Failed to process reverse stream".to_string(),
                });
            }
        }

        Ok(())
    }

    pub fn set_stream_delay_ms(&mut self, delay_ms: i32) -> Result<(), RtcError> {
        if self.sys_handle.pin_mut().set_stream_delay_ms(delay_ms) == 0 {
            Ok(())
        } else {
            Err(RtcError {
                error_type: RtcErrorType::Internal,
                message: "Failed to set stream delay".to_string(),
            })
        }
    }
}
