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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TEXT_TOKENIZERS_REGEX_TOKENIZER_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TEXT_TOKENIZERS_REGEX_TOKENIZER_H_

#include "absl/container/node_hash_map.h"  // from @com_google_absl
#include "re2/re2.h"
#include "tensorflow_lite_support/cc/text/tokenizers/tokenizer.h"

namespace tflite {
namespace support {
namespace text {
namespace tokenizer {

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

}  // namespace tokenizer
}  // namespace text
}  // namespace support
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TEXT_TOKENIZERS_REGEX_TOKENIZER_H_
