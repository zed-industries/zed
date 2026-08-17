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

#ifndef THIRD_PARTY_TENSORFLOW_LITE_SUPPORT_CC_TASK_AUDIO_UTILS_AUDIO_UTILS_H_
#define THIRD_PARTY_TENSORFLOW_LITE_SUPPORT_CC_TASK_AUDIO_UTILS_AUDIO_UTILS_H_

#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/audio/core/audio_buffer.h"
#include "tensorflow_lite_support/cc/task/audio/utils/wav_io.h"

namespace tflite {
namespace task {
namespace audio {

// Loads a WAV file into an AudioBuffer object.
//
// - buffer_size: The number of samples that the AudioBuffer object can
// store. If the WAV file contains more samples than buffer_size, only the
// samples at the beginning of the WAV file will be loaded.
// - IMPORTANT: wav_data is the actual data backing the returned AudioBuffer
// object. As the AudioBuffer object doesn't take ownership of the wav_data
// object, the user of this function has to make sure that wav_data outlives the
// returned AudioBuffer object.
tflite::support::StatusOr<AudioBuffer> LoadAudioBufferFromFile(
    const std::string& wav_file_path, uint32_t* buffer_size, uint32_t* offset,
    std::vector<float>* wav_data);

}  // namespace audio
}  // namespace task
}  // namespace tflite

#endif  // THIRD_PARTY_TENSORFLOW_LITE_SUPPORT_CC_TASK_AUDIO_UTILS_AUDIO_UTILS_H_
