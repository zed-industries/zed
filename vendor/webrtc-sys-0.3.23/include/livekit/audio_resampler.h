/*
 * Copyright 2025 LiveKit, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#pragma once

#include <memory>

#include "api/audio/audio_frame.h"
#include "api/data_channel_interface.h"
#include "common_audio/resampler/include/push_resampler.h"
#include "livekit/webrtc.h"
#include "rust/cxx.h"

namespace livekit_ffi {

class AudioResampler {
 public:
  size_t remix_and_resample(const int16_t* src,
                            size_t samples_per_channel,
                            size_t num_channels,
                            int sample_rate_hz,
                            size_t dest_num_channels,
                            int dest_sample_rate_hz);

  const int16_t* data() const;

 private:
  webrtc::AudioFrame frame_;
  webrtc::PushResampler<int16_t> resampler_;
};

std::unique_ptr<AudioResampler> create_audio_resampler();

}  // namespace livekit_ffi
