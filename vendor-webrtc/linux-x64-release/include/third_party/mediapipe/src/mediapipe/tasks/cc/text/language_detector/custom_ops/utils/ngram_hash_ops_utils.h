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

#ifndef MEDIAPIPE_TASKS_CC_TEXT_LANGUAGE_DETECTOR_CUSTOM_OPS_UTILS_NGRAM_HASH_OPS_UTILS_H_
#define MEDIAPIPE_TASKS_CC_TEXT_LANGUAGE_DETECTOR_CUSTOM_OPS_UTILS_NGRAM_HASH_OPS_UTILS_H_

#include <string>
#include <utility>
#include <vector>

namespace mediapipe::tasks::text::language_detector::custom_ops {

struct TokenizedOutput {
  // The processed string (with necessary prefix, suffix, skipped tokens, etc.).
  std::string str;

  // This vector contains pairs, where each pair has two members. The first
  // denoting the starting index of the token in the `str` string, and the
  // second denoting the length of that token in bytes.
  std::vector<std::pair<const size_t, const size_t>> tokens;
};

// Tokenizes the given input string on Unicode token boundaries, with a maximum
// of `max_tokens` tokens.
//
// If `exclude_nonalphaspace_tokens` is enabled, the tokenization ignores
// non-alphanumeric tokens, and replaces them with a replacement token (" ").
//
// The method returns the output in the `TokenizedOutput` struct, which stores
// both, the processed input string, and the indices and sizes of each token
// within that string.
TokenizedOutput Tokenize(const char* input_str, int len, int max_tokens,
                         bool exclude_nonalphaspace_tokens);

// Converts the given unicode string (`input_str`) with the specified length
// (`len`) to a lowercase string.
//
// The method populates the lowercased string in `output_str`.
void LowercaseUnicodeStr(const char* input_str, int len,
                         std::string* output_str);

}  // namespace mediapipe::tasks::text::language_detector::custom_ops

#endif  // MEDIAPIPE_TASKS_CC_TEXT_LANGUAGE_DETECTOR_CUSTOM_OPS_UTILS_NGRAM_HASH_OPS_UTILS_H_
