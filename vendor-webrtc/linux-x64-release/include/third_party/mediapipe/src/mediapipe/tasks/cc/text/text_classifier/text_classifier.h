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

#ifndef MEDIAPIPE_TASKS_CC_TEXT_TEXT_CLASSIFIER_TEXT_CLASSIFIER_H_
#define MEDIAPIPE_TASKS_CC_TEXT_TEXT_CLASSIFIER_TEXT_CLASSIFIER_H_

#include <memory>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "mediapipe/tasks/cc/components/containers/classification_result.h"
#include "mediapipe/tasks/cc/components/processors/classifier_options.h"
#include "mediapipe/tasks/cc/core/base_options.h"
#include "mediapipe/tasks/cc/core/base_task_api.h"

namespace mediapipe {
namespace tasks {
namespace text {
namespace text_classifier {

// Alias the shared ClassificationResult struct as result type.
using TextClassifierResult =
    ::mediapipe::tasks::components::containers::ClassificationResult;

// The options for configuring a MediaPipe text classifier task.
struct TextClassifierOptions {
  // Base options for configuring MediaPipe Tasks, such as specifying the model
  // file with metadata, accelerator options, op resolver, etc.
  tasks::core::BaseOptions base_options;

  // Options for configuring the classifier behavior, such as score threshold,
  // number of results, etc.
  components::processors::ClassifierOptions classifier_options;
};

// Performs classification on text.
//
// This API expects a TFLite model with (optional) TFLite Model Metadata that
// contains the mandatory (described below) input tensors, output tensor,
// and the optional (but recommended) label items as AssociatedFiles with type
// TENSOR_AXIS_LABELS per output classification tensor. Metadata is required for
// models with int32 input tensors because it contains the input process unit
// for the model's Tokenizer. No metadata is required for models with string
// input tensors.
//
// Input tensors:
//   (kTfLiteInt32)
//    - 3 input tensors of size `[batch_size x bert_max_seq_len]` representing
//      the input ids, segment ids, and mask ids
//    - or 1 input tensor of size `[batch_size x max_seq_len]` representing the
//      input ids
//   or (kTfLiteString)
//    - 1 input tensor that is shapeless or has shape [1] containing the input
//      string
// At least one output tensor with:
//   (kTfLiteFloat32/kBool)
//    - `[1 x N]` array with `N` represents the number of categories.
//    - optional (but recommended) label items as AssociatedFiles with type
//      TENSOR_AXIS_LABELS, containing one label per line. The first such
//      AssociatedFile (if any) is used to fill the `category_name` field of the
//      results. The `display_name` field is filled from the AssociatedFile (if
//      any) whose locale matches the `display_names_locale` field of the
//      `TextClassifierOptions` used at creation time ("en" by default, i.e.
//      English). If none of these are available, only the `index` field of the
//      results will be filled.
class TextClassifier : core::BaseTaskApi {
 public:
  using BaseTaskApi::BaseTaskApi;

  // Creates a TextClassifier from the provided `options`.
  static absl::StatusOr<std::unique_ptr<TextClassifier>> Create(
      std::unique_ptr<TextClassifierOptions> options);

  // Performs classification on the input `text`.
  absl::StatusOr<TextClassifierResult> Classify(absl::string_view text);

  // Shuts down the TextClassifier when all the work is done.
  absl::Status Close() { return runner_->Close(); }
};

}  // namespace text_classifier
}  // namespace text
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_TEXT_TEXT_CLASSIFIER_TEXT_CLASSIFIER_H_
