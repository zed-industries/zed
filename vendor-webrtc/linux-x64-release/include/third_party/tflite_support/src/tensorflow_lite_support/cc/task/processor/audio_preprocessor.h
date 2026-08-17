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
#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_AUDIO_PREPROCESSOR_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_AUDIO_PREPROCESSOR_H_

#include <initializer_list>

#include "absl/memory/memory.h"  // from @com_google_absl
#include "tensorflow_lite_support/cc/port/status_macros.h"
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/audio/core/audio_buffer.h"
#include "tensorflow_lite_support/cc/task/core/tflite_engine.h"
#include "tensorflow_lite_support/cc/task/processor/processor.h"

namespace tflite {
namespace task {
namespace processor {

// Processes input audio and populates the associate input tensor.
// Requirement for the input tensor:
//
// Input tensor:
//   (kTfLiteFloat32)
//    - input audio buffer of size `[batch * samples]`.
//    - batch inference is not supported (`batch` is required to be 1).
//    - for multi-channel models, the channels need be interleaved.
class AudioPreprocessor : public Preprocessor {
 public:
  static tflite::support::StatusOr<std::unique_ptr<AudioPreprocessor>> Create(
      tflite::task::core::TfLiteEngine* engine,
      const std::initializer_list<int> input_indices);

  // Processes the provided AudioBuffer and populates tensor values.
  //
  // The input `audio_buffer` are the raw buffer captured by the required format
  // which can retrieved by GetRequiredAudioFormat().
  ::absl::Status Preprocess(
      const tflite::task::audio::AudioBuffer& audio_buffer);

  // Returns the required input audio format if it is set. Otherwise, returns
  // kMetadataNotFoundError.
  tflite::task::audio::AudioBuffer::AudioFormat GetRequiredAudioFormat() {
    return audio_format_;
  }

  // Returns the required input buffer size in number of float elements.
  int GetRequiredInputBufferSize() { return input_buffer_size_; }

 private:
  using Preprocessor::Preprocessor;

  ::absl::Status Init();
  ::absl::Status SetAudioFormatFromMetadata();
  ::absl::Status CheckAndSetInputs();

  // Expected input audio format by the model.
  tflite::task::audio::AudioBuffer::AudioFormat audio_format_;

  // Expected input audio buffer size in number of float elements.
  int input_buffer_size_;
};

}  // namespace processor
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_AUDIO_PREPROCESSOR_H_
