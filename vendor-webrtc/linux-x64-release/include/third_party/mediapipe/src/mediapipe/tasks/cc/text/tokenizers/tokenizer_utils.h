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

#ifndef MEDIAPIPE_TASKS_CC_TEXT_TOKENIZERS_TOKENIZER_UTILS_H_
#define MEDIAPIPE_TASKS_CC_TEXT_TOKENIZERS_TOKENIZER_UTILS_H_

#include <memory>

#include "absl/status/statusor.h"
#include "mediapipe/tasks/cc/metadata/metadata_extractor.h"
#include "mediapipe/tasks/cc/text/tokenizers/regex_tokenizer.h"
#include "mediapipe/tasks/cc/text/tokenizers/tokenizer.h"
#include "mediapipe/tasks/metadata/metadata_schema_generated.h"

namespace mediapipe {
namespace tasks {
namespace text {
namespace tokenizers {

// Creates a RegexTokenizer by extracting vocab files from the metadata.
absl::StatusOr<std::unique_ptr<RegexTokenizer>> CreateRegexTokenizerFromOptions(
    const tflite::RegexTokenizerOptions* options,
    const metadata::ModelMetadataExtractor* metadata_extractor);

// Create a Tokenizer by extracting vocab / model files from the metadata.
absl::StatusOr<std::unique_ptr<Tokenizer>> CreateTokenizerFromProcessUnit(
    const tflite::ProcessUnit* tokenizer_process_unit,
    const metadata::ModelMetadataExtractor* metadata_extractor);

}  // namespace tokenizers
}  // namespace text
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_TEXT_TOKENIZERS_TOKENIZER_UTILS_H_
