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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_NLCLASSIFIER_NL_CLASSIFIER_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_NLCLASSIFIER_NL_CLASSIFIER_H_

#include <stddef.h>
#include <string.h>

#include <memory>
#include <string>
#include <vector>

#include "absl/base/macros.h"  // from @com_google_absl
#include "absl/status/status.h"  // from @com_google_absl
#include "flatbuffers/flatbuffers.h"  // from @flatbuffers
#include "tensorflow/lite/c/common.h"
#include "tensorflow/lite/core/api/op_resolver.h"
#include "tensorflow/lite/kernels/register.h"
#include "tensorflow/lite/string_type.h"
#include "tensorflow_lite_support/cc/common.h"
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/core/base_task_api.h"
#include "tensorflow_lite_support/cc/task/core/category.h"
#include "tensorflow_lite_support/cc/task/processor/regex_preprocessor.h"
#include "tensorflow_lite_support/cc/task/text/proto/nl_classifier_options_proto_inc.h"

namespace tflite {
namespace task {
namespace text {
namespace nlclassifier {

// Options to identify input and output tensors of the model
struct NLClassifierOptions {
  int input_tensor_index = 0;
  int output_score_tensor_index = 0;
  // By default there is no output label tensor. The label file can be attached
  // to the output score tensor metadata.
  int output_label_tensor_index = -1;
  std::string input_tensor_name = "INPUT";
  std::string output_score_tensor_name = "OUTPUT_SCORE";
  std::string output_label_tensor_name = "OUTPUT_LABEL";
};

// Classifier API for NLClassification tasks, categorizes string into different
// classes.
//
// The API expects a TFLite model with the following input/output tensor:
// Input tensor:
//   (kTfLiteString) - input of the model, accepts a string.
//      or
//   (kTfLiteInt32) - input of the model, accepts a tokenized
//   indices of a string input. A RegexTokenizer needs to be set up in the input
//   tensor's metadata.
// Output score tensor:
//   (kTfLiteUInt8/kTfLiteInt8/kTfLiteInt16/kTfLiteFloat32/
//    kTfLiteFloat64/kTfLiteBool)
//    - output scores for each class, if type is one of the Int types,
//      dequantize it to double, if type is kTfLiteBool, convert the values to
//      0.0 and 1.0 respectively
//    - can have an optional associated file in metadata for labels, the file
//      should be a plain text file with one label per line, the number of
//      labels should match the number of categories the model outputs.
// Output label tensor: optional
//   (kTfLiteString/kTfLiteInt32)
//    - output classname for each class, should be of the same length with
//      scores. If this tensor is not present, the API uses score indices as
//      classnames.
//    - will be ignored if output score tensor already has an associated label
//      file.
//
// By default the API tries to find the input/output tensors with default
// configurations in NLClassifierOptions, with tensor name prioritized over
// tensor index. The option is configurable for different TFLite models.
class NLClassifier : public core::BaseTaskApi<std::vector<core::Category>,
                                              const std::string&> {
 public:
  using BaseTaskApi::BaseTaskApi;

  // Creates an NLClassifier from the provided options. A non-default
  // OpResolver can be specified in order to support custom Ops or specify a
  // subset of built-in Ops.
  //
  // This is a forward compatible method that uses
  // `tflite::task::text::NLClassifierOptions`. Factory create methods with
  // `tflite::task::text::nlclassifier::NLClassifierOptions` will be deprecated.
  //
  // TODO(b/182537114): unify the classification options (support the common
  // classification options) and results across vision/text/audio.
  static tflite::support::StatusOr<std::unique_ptr<NLClassifier>>
  CreateFromOptions(
      const tflite::task::text::NLClassifierOptions& options,
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>());

  // Creates an NLClassifier from TFLite model buffer.
  ABSL_DEPRECATED("Prefer using `CreateFromOptions`")
  static tflite::support::StatusOr<std::unique_ptr<NLClassifier>>
  CreateFromBufferAndOptions(
      const char* model_buffer_data, size_t model_buffer_size,
      const NLClassifierOptions& options = {},
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>());

  // Creates an NLClassifier from TFLite model file.
  ABSL_DEPRECATED("Prefer using `CreateFromOptions`")
  static tflite::support::StatusOr<std::unique_ptr<NLClassifier>>
  CreateFromFileAndOptions(
      const std::string& path_to_model, const NLClassifierOptions& options = {},
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>());

  // Creates an NLClassifier from TFLite model file descriptor.
  ABSL_DEPRECATED("Prefer using `CreateFromOptions`")
  static tflite::support::StatusOr<std::unique_ptr<NLClassifier>>
  CreateFromFdAndOptions(
      int fd, const NLClassifierOptions& options = {},
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>());

  ABSL_DEPRECATED("Prefer using `ClassifyText`")
  std::vector<core::Category> Classify(const std::string& text);

  // Performs classification on a string input, returns classified results or an
  // error.
  tflite::support::StatusOr<std::vector<core::Category>> ClassifyText(
      const std::string& text);

 protected:
  static constexpr int kOutputTensorIndex = 0;
  static constexpr int kOutputTensorLabelFileIndex = 0;

  // Initialize NLClassifier with the proto NLClassifierOptions.
  absl::Status Initialize(
      std::unique_ptr<tflite::task::text::NLClassifierOptions> options);

  ABSL_DEPRECATED(
      "Prefer using `tflite::task::text::NLClassifierOptions` and "
      "`Initialize(std::unique_ptr<tflite::task::text::NLClassifierOptions> "
      "options)`")
  absl::Status Initialize(const NLClassifierOptions& options = {});
  ABSL_DEPRECATED(
      "Prefer using `tflite::task::text::NLClassifierOptions` and "
      "`CreateFromOptions`")
  const NLClassifierOptions& GetOptions() const;

  // Try to extract attached label file from metadata and initialize
  // labels_vector_, return error if metadata type is incorrect or no label file
  // is attached in metadata.
  absl::Status TrySetLabelFromMetadata(const TensorMetadata* metadata);

  // Pass through the input text into model's input tensor.
  absl::Status Preprocess(const std::vector<TfLiteTensor*>& input_tensors,
                          const std::string& input) override;

  // Extract model output and create results with output label tensor or label
  // file attached in metadata. If no output label tensor or label file is
  // found, use output score index as labels.
  tflite::support::StatusOr<std::vector<core::Category>> Postprocess(
      const std::vector<const TfLiteTensor*>& output_tensors,
      const std::string& input) override;

  std::vector<core::Category> BuildResults(const TfLiteTensor* scores,
                                           const TfLiteTensor* labels);

  // Gets the tensor from a vector of tensors by checking tensor name first and
  // tensor index second, return nullptr if no tensor is found.
  template <typename TensorType>
  static TensorType* FindTensorWithNameOrIndex(
      const std::vector<TensorType*>& tensors,
      const flatbuffers::Vector<flatbuffers::Offset<TensorMetadata>>*
          metadata_array,
      const std::string& name, int index) {
    int tensor_index = FindTensorIndex(tensors, metadata_array, name, index);
    return tensor_index >= 0 && tensor_index < tensors.size()
               ? tensors[tensor_index]
               : nullptr;
  }

  // Gets the tensor index of the specified tensor name from a vector of tensors
  // Return nullptr if no tensor is found by name (metadata tensor name or model
  // tensor name).
  template <typename TensorType>
  static int FindTensorIndex(
      const std::vector<TensorType*>& tensors,
      const flatbuffers::Vector<flatbuffers::Offset<TensorMetadata>>*
          metadata_array,
      const std::string& name, int default_index) {
    if (metadata_array != nullptr && metadata_array->size() == tensors.size()) {
      for (size_t i = 0; i < metadata_array->size(); i++) {
        if (strcmp(name.data(), metadata_array->Get(i)->name()->c_str()) == 0) {
          return i;
        }
      }
    }

    for (int i = 0; i < tensors.size(); i++) {
      TensorType* tensor = tensors[i];
      if (tensor->name == name) {
        return i;
      }
    }
    return default_index;
  }

 private:
  std::unique_ptr<tflite::task::processor::RegexPreprocessor> preprocessor_ =
      nullptr;

  std::unique_ptr<tflite::task::text::NLClassifierOptions> proto_options_;

  // labels vector initialized from output tensor's associated file, if one
  // exists.
  std::unique_ptr<std::vector<std::string>> labels_vector_;

  // Deprecated: using the proto_options_
  // (tflite::task::text::NLClassifierOptions).
  NLClassifierOptions struct_options_;
};

}  // namespace nlclassifier
}  // namespace text
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_NLCLASSIFIER_NL_CLASSIFIER_H_
