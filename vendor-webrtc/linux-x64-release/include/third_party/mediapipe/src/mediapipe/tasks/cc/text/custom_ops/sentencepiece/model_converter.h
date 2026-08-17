/* Copyright 2023 The MediaPipe Authors.

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

#ifndef MEDIAPIPE_TASKS_CC_TEXT_CUSTOM_OPS_SENTENCEPIECE_MODEL_CONVERTER_H_
#define MEDIAPIPE_TASKS_CC_TEXT_CUSTOM_OPS_SENTENCEPIECE_MODEL_CONVERTER_H_

#include <string>

#include "absl/status/statusor.h"

namespace mediapipe::tflite_operations::sentencepiece {

// Converts Sentencepiece configuration to flatbuffer format.
// encoding_offset is used by some encoders that combine different encodings.
absl::StatusOr<std::string> ConvertSentencepieceModelToFlatBuffer(
    const std::string& model_config_str, int encoding_offset = 0);
std::string ConvertSentencepieceModel(const std::string& model_string);

}  // namespace mediapipe::tflite_operations::sentencepiece

#endif  // MEDIAPIPE_TASKS_CC_TEXT_CUSTOM_OPS_SENTENCEPIECE_MODEL_CONVERTER_H_
