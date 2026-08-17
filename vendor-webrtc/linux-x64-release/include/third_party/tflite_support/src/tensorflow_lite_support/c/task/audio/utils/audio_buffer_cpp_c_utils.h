/* Copyright 2022 The TensorFlow Authors. All Rights Reserved.

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
#ifndef TENSORFLOW_LITE_SUPPORT_C_TASK_AUDIO_AUDIO_BUFFER_CPP_C_UTILS_H_
#define TENSORFLOW_LITE_SUPPORT_C_TASK_AUDIO_AUDIO_BUFFER_CPP_C_UTILS_H_

#include "tensorflow_lite_support/c/task/audio/core/audio_buffer.h"
#include "tensorflow_lite_support/cc/task/audio/core/audio_buffer.h"

// Utils for Conversions between C and C++ AudioBuffer
// -----------------------------------------------------------------
// Meant to be used with audio C apis.

// Creates the C++ AudioBuffer from the C AudioBuffer
namespace tflite {
namespace task {
namespace audio {

tflite::support::StatusOr<std::unique_ptr<tflite::task::audio::AudioBuffer>>
CreateCppAudioBuffer(const TfLiteAudioBuffer* audio_buffer);

tflite::support::StatusOr<TfLiteAudioFormat*> CreateCAudioFormat(
    tflite::support::StatusOr<tflite::task::audio::AudioBuffer::AudioFormat>
        cpp_audio_format);

}  // namespace audio
}  // namespace task
}  // namespace tflite
#endif  // TENSORFLOW_LITE_SUPPORT_C_TASK_AUDIO_AUDIO_BUFFER_CPP_C_UTILS_H_
