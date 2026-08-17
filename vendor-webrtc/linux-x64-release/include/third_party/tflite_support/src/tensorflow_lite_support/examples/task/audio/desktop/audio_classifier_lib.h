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
#ifndef TENSORFLOW_LITE_SUPPORT_EXAMPLES_TASK_AUDIO_DESKTOP_AUDIO_CLASSIFIER_LIB_H_
#define TENSORFLOW_LITE_SUPPORT_EXAMPLES_TASK_AUDIO_DESKTOP_AUDIO_CLASSIFIER_LIB_H_

#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/audio/core/audio_buffer.h"
#include "tensorflow_lite_support/cc/task/audio/proto/classifications_proto_inc.h"

namespace tflite {
namespace task {
namespace audio {

// Loads `wav_file` from filesystem and runs classification using TFLite model
// in `model_path` with default options. If the content of `wav_file` is longer
// than what the model requires, only the beginning section is used for
// inference.
tflite::support::StatusOr<ClassificationResult> Classify(
    const std::string& model_path, const std::string& wav_file,
    bool use_coral = false);

// Prints the output classification result in the standard output. It only
// displays classes whose score is higher than the `score_threshold`.
void Display(const ClassificationResult& result, float score_threshold);

}  // namespace audio
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_EXAMPLES_TASK_AUDIO_DESKTOP_AUDIO_CLASSIFIER_LIB_H_
