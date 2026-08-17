/* Copyright 2020 The TensorFlow Authors. All Rights Reserved.

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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_UTILS_COMMON_UTILS_H_
#define TENSORFLOW_LITE_SUPPORT_CC_UTILS_COMMON_UTILS_H_

#include <string>
#include <vector>

#include "absl/container/node_hash_map.h"  // from @com_google_absl

namespace tflite {
namespace support {
namespace utils {

// Read a vocab file with one vocabulary on each line, create a vector of
// strings.
std::vector<std::string> LoadVocabFromFile(const std::string& path_to_vocab);

// read a vocab buffer with one vocab one each line, create a vector of strings
std::vector<std::string> LoadVocabFromBuffer(const char* vocab_buffer_data,
                                             const size_t vocab_buffer_size);

// Read a vocab file with one vocabulary and its corresponding index on each
// line separated by space, create a map of <vocab, index>.
absl::node_hash_map<std::string, int> LoadVocabAndIndexFromFile(
    const std::string& path_to_vocab);

// Read a vocab buffer with one vocabulary and its corresponding index on each
// line separated by space, create a map of <vocab, index>.
absl::node_hash_map<std::string, int> LoadVocabAndIndexFromBuffer(
    const char* vocab_buffer_data, const size_t vocab_buffer_size);
}  // namespace utils
}  // namespace support
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_UTILS_COMMON_UTILS_H_
