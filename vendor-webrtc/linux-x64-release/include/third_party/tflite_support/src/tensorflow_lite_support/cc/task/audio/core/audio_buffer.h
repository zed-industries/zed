/* Copyright 2021 The TensorFlow Authors. All Rights Reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/
#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_AUDIO_CORE_AUDIO_BUFFER_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_AUDIO_CORE_AUDIO_BUFFER_H_

#include <memory>

#include "absl/memory/memory.h"  // from @com_google_absl
#include "absl/status/status.h"  // from @com_google_absl
#include "absl/strings/str_format.h"  // from @com_google_absl
#include "tensorflow_lite_support/cc/common.h"
#include "tensorflow_lite_support/cc/port/statusor.h"

namespace tflite {
namespace task {
namespace audio {

// Provides a view into the provided backing buffer and the audio format
// metadata.
// TODO(b/182675479): Support quantized input format.
class AudioBuffer {
 public:
  // Audio format metadata.
  struct AudioFormat {
    int channels;
    int sample_rate;
  };

  // Factory method for creating an AudioBuffer object. The internal buffer does
  // not take the ownership of the input backing buffer.
  static tflite::support::StatusOr<std::unique_ptr<AudioBuffer>> Create(
      const float* audio_buffer, int buffer_size,
      const AudioFormat& audio_format) {
    return absl::make_unique<AudioBuffer>(audio_buffer, buffer_size,
                                          audio_format);
  }

  // AudioBuffer for internal use only. Uses the factory method to construct
  // AudioBuffer instance. The internal buffer does not take the ownership of
  // the input backing buffer.
  AudioBuffer(const float* audio_buffer, int buffer_size,
              const AudioFormat& audio_format)
      : audio_buffer_(audio_buffer),
        buffer_size_(buffer_size),
        audio_format_(audio_format) {}

  // Accessors
  AudioFormat GetAudioFormat() const { return audio_format_; }
  int GetBufferSize() const { return buffer_size_; }
  const float* GetFloatBuffer() const { return audio_buffer_; }

 private:
  const float* audio_buffer_;
  int buffer_size_;
  AudioFormat audio_format_;
};

}  // namespace audio
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_AUDIO_CORE_AUDIO_BUFFER_H_
