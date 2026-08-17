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

#ifndef MEDIAPIPE_TASKS_CC_TEXT_CUSTOM_OPS_SENTENCEPIECE_SENTENCEPIECE_CONSTANTS_H_
#define MEDIAPIPE_TASKS_CC_TEXT_CUSTOM_OPS_SENTENCEPIECE_SENTENCEPIECE_CONSTANTS_H_

namespace mediapipe::tflite_operations::sentencepiece {

// The constant is copied from
// https://github.com/google/sentencepiece/blob/master/src/unigram_model.cc
inline constexpr float kUnkPenalty = 10.0;

// These constants are copied from
// https://github.com/google/sentencepiece/blob/master/src/sentencepiece_processor.cc
//
// Replaces white space with U+2581 (LOWER ONE EIGHT BLOCK).
inline constexpr char kSpaceSymbol[] = "\xe2\x96\x81";

// Encodes <unk> into U+2047 (DOUBLE QUESTION MARK),
// since this character can be useful both for user and
// developer. We can easily figure out that <unk> is emitted.
inline constexpr char kDefaultUnknownSymbol[] = " \xE2\x81\x87 ";

}  // namespace mediapipe::tflite_operations::sentencepiece

#endif  // MEDIAPIPE_TASKS_CC_TEXT_CUSTOM_OPS_SENTENCEPIECE_SENTENCEPIECE_CONSTANTS_H_
