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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_NLCLASSIFIER_BERT_NL_CLASSIFIER_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_NLCLASSIFIER_BERT_NL_CLASSIFIER_H_

#include <stddef.h>

#include <memory>
#include <string>
#include <vector>

#include "absl/base/macros.h"  // from @com_google_absl
#include "absl/status/status.h"  // from @com_google_absl
#include "tensorflow/lite/c/common.h"
#include "tensorflow/lite/core/api/op_resolver.h"
#include "tensorflow/lite/kernels/register.h"
#include "tensorflow/lite/string_type.h"
#include "tensorflow_lite_support/cc/task/core/category.h"
#include "tensorflow_lite_support/cc/task/processor/bert_preprocessor.h"
#include "tensorflow_lite_support/cc/task/text/nlclassifier/nl_classifier.h"
#include "tensorflow_lite_support/cc/task/text/proto/bert_nl_classifier_options_proto_inc.h"

namespace tflite {
namespace task {
namespace text {

// Classifier API for NLClassification tasks with Bert models, categorizes
// string into different classes.
//
// The API expects a Bert based TFLite model with metadata populated.
// The metadata should contain the following information:
//   - input_process_units for Wordpiece/Sentencepiece Tokenizer
//   - 3 input tensors with names "ids", "mask" and "segment_ids"
//   - 1 output tensor of type float32[1, 2], with a optionally attached label
//     file. If a label file is attached, the file should be a plain text file
//     with one label per line, the number of labels should match the number of
//     categories the model outputs.

class BertNLClassifier : public tflite::task::text::nlclassifier::NLClassifier {
 public:
  using tflite::task::text::nlclassifier::NLClassifier::NLClassifier;

  // Factory function to create a BertNLClassifier from BertNLClassifierOptions.
  static tflite::support::StatusOr<std::unique_ptr<BertNLClassifier>>
  CreateFromOptions(
      const BertNLClassifierOptions& options,
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>());

  // Factory function to create a BertNLClassifier from TFLite model with
  // metadata.
  ABSL_DEPRECATED("Prefer using `CreateFromOptions`")
  static tflite::support::StatusOr<std::unique_ptr<BertNLClassifier>>
  CreateFromFile(
      const std::string& path_to_model_with_metadata,
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>()) {
    BertNLClassifierOptions options;
    options.mutable_base_options()->mutable_model_file()->set_file_name(
        path_to_model_with_metadata);
    return CreateFromOptions(options, std::move(resolver));
  }

  // Factory function to create a BertNLClassifier from in memory buffer of a
  // TFLite model with metadata.
  ABSL_DEPRECATED("Prefer using `CreateFromOptions`")
  static tflite::support::StatusOr<std::unique_ptr<BertNLClassifier>>
  CreateFromBuffer(
      const char* model_with_metadata_buffer_data,
      size_t model_with_metadata_buffer_size,
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>()) {
    BertNLClassifierOptions options;
    options.mutable_base_options()->mutable_model_file()->set_file_content(
        model_with_metadata_buffer_data, model_with_metadata_buffer_size);
    return CreateFromOptions(options, std::move(resolver));
  }

  // Factory function to create a BertNLClassifier from the file descriptor of a
  // TFLite model with metadata.
  ABSL_DEPRECATED("Prefer using `CreateFromOptions`")
  static tflite::support::StatusOr<std::unique_ptr<BertNLClassifier>>
  CreateFromFd(
      int fd,
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>()) {
    BertNLClassifierOptions options;
    options.mutable_base_options()
        ->mutable_model_file()
        ->mutable_file_descriptor_meta()
        ->set_fd(fd);
    return CreateFromOptions(options, std::move(resolver));
  }

 protected:
  // Run tokenization on input text and construct three input tensors ids, mask
  // and segment_ids for the model input.
  absl::Status Preprocess(const std::vector<TfLiteTensor*>& input_tensors,
                          const std::string& input) override;

  // Extract model output and create results with label file attached in
  // metadata. If no label file is attached, use output score index as labels.
  tflite::support::StatusOr<std::vector<core::Category>> Postprocess(
      const std::vector<const TfLiteTensor*>& output_tensors,
      const std::string& input) override;

 private:
  // Initialize the API with the tokenizer and label files set in the metadata.
  absl::Status Initialize(std::unique_ptr<BertNLClassifierOptions> options);

  std::unique_ptr<tflite::task::processor::BertPreprocessor> preprocessor_ =
      nullptr;

  std::unique_ptr<BertNLClassifierOptions> options_;
};

}  // namespace text
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_NLCLASSIFIER_BERT_NL_CLASSIFIER_H_
