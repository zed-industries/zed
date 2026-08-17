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

#ifndef MEDIAPIPE_TASKS_CC_TEXT_CUSTOM_OPS_SENTENCEPIECE_DOUBLE_ARRAY_TRIE_BUILDER_H_
#define MEDIAPIPE_TASKS_CC_TEXT_CUSTOM_OPS_SENTENCEPIECE_DOUBLE_ARRAY_TRIE_BUILDER_H_

#include <cstdint>
#include <string>
#include <vector>

namespace mediapipe::tflite_operations::sentencepiece {

std::vector<uint32_t> BuildTrie(const std::vector<std::string>& data,
                                const std::vector<int>& ids);

// A variant where ids are indexes in data.
std::vector<uint32_t> BuildTrie(const std::vector<std::string>& data);

}  // namespace mediapipe::tflite_operations::sentencepiece

#endif  // MEDIAPIPE_TASKS_CC_TEXT_CUSTOM_OPS_SENTENCEPIECE_DOUBLE_ARRAY_TRIE_BUILDER_H_
