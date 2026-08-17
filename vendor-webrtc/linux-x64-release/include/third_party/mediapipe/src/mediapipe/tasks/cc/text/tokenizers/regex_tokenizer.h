/* Copyright 2022 The MediaPipe Authors.

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

#ifndef MEDIAPIPE_TASKS_CC_TEXT_TOKENIZERS_REGEX_TOKENIZER_H_
#define MEDIAPIPE_TASKS_CC_TEXT_TOKENIZERS_REGEX_TOKENIZER_H_

#include <cstddef>
#include <string>

#include "absl/container/node_hash_map.h"
#include "absl/strings/string_view.h"
#include "mediapipe/tasks/cc/text/tokenizers/tokenizer.h"
#include "re2/re2.h"

namespace mediapipe {
namespace tasks {
namespace text {
namespace tokenizers {

// Tokenizer to load a vocabulary and split text by regular expressions.
class RegexTokenizer : public Tokenizer {
 public:
  explicit RegexTokenizer(const std::string& regex_pattern,
                          const std::string& path_to_vocab);

  explicit RegexTokenizer(const std::string& regex_pattern,
                          const char* vocab_buffer_data,
                          size_t vocab_buffer_size);

  TokenizerResult Tokenize(const std::string& input) override;

  bool LookupId(absl::string_view key, int* result) const override;

  bool LookupWord(int vocab_id, absl::string_view* result) const override;

  bool GetStartToken(int* start_token);
  bool GetPadToken(int* pad_token);
  bool GetUnknownToken(int* unknown_token);

 private:
  RE2 delim_re_;
  absl::node_hash_map<std::string, int> token_index_map_;
  absl::node_hash_map<int, absl::string_view> index_token_map_;
};

}  // namespace tokenizers
}  // namespace text
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_TEXT_TOKENIZERS_REGEX_TOKENIZER_H_
