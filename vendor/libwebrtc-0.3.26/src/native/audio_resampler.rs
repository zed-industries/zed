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
use webrtc_sys::audio_resampler as sys_ar;

pub struct AudioResampler {
    sys_handle: UniquePtr<sys_ar::ffi::AudioResampler>,
}

impl Default for AudioResampler {
    fn default() -> Self {
        Self { sys_handle: sys_ar::ffi::create_audio_resampler() }
    }
}

impl AudioResampler {
    pub fn remix_and_resample<'a>(
        &'a mut self,
        src: &[i16],
        samples_per_channel: u32,
        num_channels: u32,
        sample_rate: u32,
        dst_num_channels: u32,
        dst_sample_rate: u32,
    ) -> &'a [i16] {
        assert!(src.len() >= (samples_per_channel * num_channels) as usize, "src buffer too small");

        unsafe {
            let len = self.sys_handle.pin_mut().remix_and_resample(
                src.as_ptr(),
                samples_per_channel as usize,
                num_channels as usize,
                sample_rate as i32,
                dst_num_channels as usize,
                dst_sample_rate as i32,
            );

            std::slice::from_raw_parts(self.sys_handle.data(), len / 2)
        }
    }
}
