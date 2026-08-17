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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_UNIVERSAL_SENTENCE_ENCODER_PREPROCESSOR_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_UNIVERSAL_SENTENCE_ENCODER_PREPROCESSOR_H_

#include <initializer_list>
#include <memory>

#include "absl/status/status.h"  // from @com_google_absl
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/core/tflite_engine.h"
#include "tensorflow_lite_support/cc/task/processor/text_preprocessor.h"

namespace tflite {
namespace task {
namespace processor {

// Processes input text and populates the associated Universal Sentence Encoder
// input tensors.
// Requirements for the input tensors:
//   Exactly 3 string input tensors of type, kTfLiteString: contains
//   respectively the query text, response context and response text.
//
// Utils to help locate the 3 input tensors for models conforming to certain
// metadata requirements are available in:
// https://github.com/tensorflow/tflite-support/tree/master/tensorflow_lite_support/cc/task/text/utils/universal_sentence_encoder_utils.h
class UniversalSentenceEncoderPreprocessor : public TextPreprocessor {
 public:
  static tflite::support::StatusOr<
      std::unique_ptr<UniversalSentenceEncoderPreprocessor>>
  Create(tflite::task::core::TfLiteEngine* engine,
         std::initializer_list<int> input_tensor_indices);

  // Note that this only fills the response text input tensor. As a consequence,
  // only the corresponding response encoding output tensor will be filled at
  // inference time.
  absl::Status Preprocess(const std::string& text);

 private:
  using TextPreprocessor::TextPreprocessor;
};

}  // namespace processor
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_UNIVERSAL_SENTENCE_ENCODER_PREPROCESSOR_H_
