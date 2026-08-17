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

#ifndef MEDIAPIPE_TASKS_CC_TEXT_TOKENIZERS_SENTENCEPIECE_TOKENIZER_H_
#define MEDIAPIPE_TASKS_CC_TEXT_TOKENIZERS_SENTENCEPIECE_TOKENIZER_H_

#include <cstddef>
#include <fstream>
#include <string>
#include <vector>

#include "absl/log/absl_check.h"
#include "absl/strings/string_view.h"
#include "mediapipe/framework/port/logging.h"
#include "mediapipe/tasks/cc/text/tokenizers/tokenizer.h"
#include "src/sentencepiece_processor.h"  // from @com_google_sentencepiece

namespace mediapipe {
namespace tasks {
namespace text {
namespace tokenizers {

// SentencePiece tokenizer. Initialized with a model file.
class SentencePieceTokenizer : public Tokenizer {
 public:
  // Initialize the SentencePiece tokenizer from model file path.
  explicit SentencePieceTokenizer(const std::string& path_to_model) {
    // Can't use ABSL_CHECK_OK here because in internal builds
    // the return type is absl::Status while the open source builds
    // use sentencepiece/src/deps/status.h's util::Status which
    // doesn't work with the absl CHECK macros.
    const auto status = sp_.Load(path_to_model);
    ABSL_CHECK(status.ok()) << status.ToString();
  }

  explicit SentencePieceTokenizer(const char* spmodel_buffer_data,
                                  size_t spmodel_buffer_size) {
    absl::string_view buffer_binary(spmodel_buffer_data, spmodel_buffer_size);
    const auto status = sp_.LoadFromSerializedProto(buffer_binary);
    ABSL_CHECK(status.ok()) << status.ToString();
  }

  // Perform tokenization, return tokenized results.
  TokenizerResult Tokenize(const std::string& input) override {
    TokenizerResult result;
    std::vector<std::string>& subwords = result.subwords;
    const auto status = sp_.Encode(input, &subwords);
    ABSL_CHECK(status.ok()) << status.ToString();
    return result;
  }

  // Find the id of a string token.
  bool LookupId(absl::string_view key, int* result) const override {
    *result = sp_.PieceToId(key);
    return true;
  }

  // Find the string token of an id.
  bool LookupWord(int vocab_id, absl::string_view* result) const override {
    *result = sp_.IdToPiece(vocab_id);
    return true;
  }

 private:
  sentencepiece::SentencePieceProcessor sp_;
};

}  // namespace tokenizers
}  // namespace text
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_TEXT_TOKENIZERS_SENTENCEPIECE_TOKENIZER_H_
