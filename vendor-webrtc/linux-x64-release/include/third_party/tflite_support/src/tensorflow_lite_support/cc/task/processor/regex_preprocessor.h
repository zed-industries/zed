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
#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_REGEX_PREPROCESSOR_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_REGEX_PREPROCESSOR_H_

#include "absl/status/status.h"  // from @com_google_absl
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/processor/text_preprocessor.h"
#include "tensorflow_lite_support/cc/text/tokenizers/regex_tokenizer.h"

namespace tflite {
namespace task {
namespace processor {

// Processes input text and populates the associated input tensor.
// Requirements for the input tensor:
//   A string tensor of type, kTfLiteString
//      or
//   An int32 tensor of type, kTfLiteInt32: contains the tokenized indices of
//   a string input. A RegexTokenizer needs to be set up in the input tensor's
//   metadata.
class RegexPreprocessor : public TextPreprocessor {
 public:
  static tflite::support::StatusOr<std::unique_ptr<RegexPreprocessor>> Create(
      tflite::task::core::TfLiteEngine* engine, int input_tensor_index);

  absl::Status Preprocess(const std::string& text);

 private:
  using TextPreprocessor::TextPreprocessor;

  absl::Status Init();

  tflite::support::StatusOr<const tflite::ProcessUnit*>
  TryFindRegexTokenizerMetadata();

  absl::Status RegexPreprocess(const std::string& input_text);

  tflite::support::StatusOr<
      std::unique_ptr<tflite::support::text::tokenizer::RegexTokenizer>>
  CreateTokenizerFromMetadata(
      const tflite::ProcessUnit* tokenizer_metadata,
      const tflite::metadata::ModelMetadataExtractor* metadata_extractor);

  std::unique_ptr<tflite::support::text::tokenizer::RegexTokenizer> tokenizer_;
};

}  // namespace processor
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_REGEX_PREPROCESSOR_H_
