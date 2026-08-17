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

#ifndef THIRD_PARTY_TENSORFLOW_LITE_SUPPORT_CC_TEXT_TOKENIZERS_TOKENIZER_UTILS_H_
#define THIRD_PARTY_TENSORFLOW_LITE_SUPPORT_CC_TEXT_TOKENIZERS_TOKENIZER_UTILS_H_

#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/text/tokenizers/tokenizer.h"
#include "tensorflow_lite_support/metadata/cc/metadata_extractor.h"
#include "tensorflow_lite_support/metadata/metadata_schema_generated.h"

namespace tflite {
namespace support {
namespace text {
namespace tokenizer {


// Create a Tokenizer from model metadata by extracting
tflite::support::StatusOr<std::unique_ptr<Tokenizer>>
CreateTokenizerFromProcessUnit(
    const tflite::ProcessUnit* tokenizer_process_unit,
    const tflite::metadata::ModelMetadataExtractor* metadata_extractor);

}  // namespace tokenizer
}  // namespace text
}  // namespace support
}  // namespace tflite

#endif  // THIRD_PARTY_TENSORFLOW_LITE_SUPPORT_CC_TEXT_TOKENIZERS_TOKENIZER_UTILS_H_
