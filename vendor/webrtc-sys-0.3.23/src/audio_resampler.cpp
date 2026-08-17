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

#include "livekit/audio_resampler.h"

#include <memory>

#include "audio/remix_resample.h"
#include "api/audio/audio_view.h"
#include "api/audio/audio_frame.h"

namespace livekit_ffi {

size_t AudioResampler::remix_and_resample(const int16_t* src,
                                          size_t samples_per_channel,
                                          size_t num_channels,
                                          int sample_rate,
                                          size_t dest_num_channels,
                                          int dest_sample_rate) {
  frame_.num_channels_ = dest_num_channels;
  frame_.sample_rate_hz_ = dest_sample_rate;
  frame_.samples_per_channel_ = webrtc::SampleRateToDefaultChannelSize(dest_sample_rate);
  webrtc::InterleavedView<const int16_t> source(static_cast<const int16_t*>(src),
                                         samples_per_channel,
                                         num_channels);
  webrtc::voe::RemixAndResample(source, sample_rate, &resampler_, &frame_);

  return frame_.num_channels() * frame_.samples_per_channel() * sizeof(int16_t);
}

const int16_t* AudioResampler::data() const {
  return frame_.data();
}

std::unique_ptr<AudioResampler> create_audio_resampler() {
  return std::make_unique<AudioResampler>();
}

}  // namespace livekit_ffi
