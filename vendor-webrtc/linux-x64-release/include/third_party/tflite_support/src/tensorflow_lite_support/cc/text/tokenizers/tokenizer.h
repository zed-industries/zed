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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TEXT_TOKENIZERS_TOKENIZER_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TEXT_TOKENIZERS_TOKENIZER_H_

#include <fstream>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"  // from @com_google_absl

namespace tflite {
namespace support {
namespace text {
namespace tokenizer {

struct TokenizerResult {
  std::vector<std::string> subwords;
};

// Interface of general tokenizer.
class Tokenizer {
 public:
  // Perform tokenization to get tokenized results.
  virtual TokenizerResult Tokenize(const std::string& input) = 0;

  // Find the id of a string token.
  virtual bool LookupId(absl::string_view key, int* result) const = 0;

  // Find the string token from an id.
  virtual bool LookupWord(int vocab_id, absl::string_view* result) const = 0;

  // Destructor.
  virtual ~Tokenizer() = default;
};

}  // namespace tokenizer
}  // namespace text
}  // namespace support
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TEXT_TOKENIZERS_TOKENIZER_H_
