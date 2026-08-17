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

#include "api/scoped_refptr.h"
#include "api/video_codecs/video_decoder_factory.h"
#include "api/video_codecs/video_encoder_factory.h"
#include "modules/audio_processing/aec3/echo_canceller3.h"
#include "modules/audio_processing/audio_buffer.h"
#include "rust/cxx.h"

namespace livekit_ffi {

// Forward declarations so the cxx-generated header below can reference
// the C++ class while this header in turn pulls in the cxx-generated
// definitions of `AudioProcessingConfig` and its nested structs.
class AudioProcessingModule;
struct AudioProcessingConfig;

}  // namespace livekit_ffi

#include "webrtc-sys/src/apm.rs.h"

namespace livekit_ffi {

class AudioProcessingModule {
 public:
  explicit AudioProcessingModule(const AudioProcessingConfig& config);

  int process_stream(const int16_t* src,
                     size_t src_len,
                     int16_t* dst,
                     size_t dst_len,
                     int sample_rate,
                     int num_channels);

  int process_reverse_stream(const int16_t* src,
                             size_t src_len,
                             int16_t* dst,
                             size_t dst_len,
                             int sample_rate,
                             int num_channels);

  int set_stream_delay_ms(int delay_ms);

 private:
  webrtc::scoped_refptr<webrtc::AudioProcessing> apm_;
};

std::unique_ptr<AudioProcessingModule> create_apm(
    const AudioProcessingConfig& config);

}  // namespace livekit_ffi
