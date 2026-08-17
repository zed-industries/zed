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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_CLASSIFICATION_POSTPROCESSOR_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_CLASSIFICATION_POSTPROCESSOR_H_

#include <initializer_list>

#include "tensorflow_lite_support/cc/port/status_macros.h"
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/core/classification_head.h"
#include "tensorflow_lite_support/cc/task/core/label_map_item.h"
#include "tensorflow_lite_support/cc/task/core/score_calibration.h"
#include "tensorflow_lite_support/cc/task/core/task_utils.h"
#include "tensorflow_lite_support/cc/task/processor/processor.h"
#include "tensorflow_lite_support/cc/task/processor/proto/class.pb.h"
#include "tensorflow_lite_support/cc/task/processor/proto/classification_options.pb.h"
#include "tensorflow_lite_support/cc/task/processor/proto/classifications.pb.h"

namespace tflite {
namespace task {
namespace processor {

// This Postprocessor expects one output tensor with:
//   (kTfLiteUInt8/kTfLiteFloat32)
//    -  `N `classes and either 2 or 4 dimensions, i.e. `[1 x N]` or
//       `[1 x 1 x 1 x N]`
//    - optional (but recommended) label map(s) as AssociatedFile-s with type
//      TENSOR_AXIS_LABELS, containing one label per line. The first such
//      AssociatedFile (if any) is used to fill the `class_name` field of the
//      results. The `display_name` field is filled from the AssociatedFile (if
//      any) whose locale matches the `display_names_locale` field of the
//      `ImageClassifierOptions` used at creation time ("en" by default, i.e.
//      English). If none of these are available, only the `index` field of the
//      results will be filled.
class ClassificationPostprocessor : public Postprocessor {
 public:
  static tflite::support::StatusOr<std::unique_ptr<ClassificationPostprocessor>>
  Create(core::TfLiteEngine* engine,
         const std::initializer_list<int> output_indices,
         std::unique_ptr<ClassificationOptions> options);

  // Convert the tensor output to classification class.
  // Note that this method doesn't add head_name for backward compatibility.
  // Head name can be retrieved by `GetHeadName` method.
  template <typename T>
  absl::Status Postprocess(T* classifications);

  const std::string GetHeadName() const { return classification_head_.name; }

 private:
  using Postprocessor::Postprocessor;

  absl::Status Init(std::unique_ptr<ClassificationOptions> options);
  // Given a ClassificationResult object containing class indices, fills the
  // name and display name from the label map(s).
  template <typename T>
  absl::Status FillResultsFromLabelMaps(T* classifications);

  // The list of classification heads associated with the corresponding output
  // tensors. Built from TFLite Model Metadata.
  ::tflite::task::core::ClassificationHead classification_head_{};

  // Set of allowlisted or denylisted class names.
  struct ClassNameSet {
    absl::flat_hash_set<std::string> values;
    bool is_allowlist;
  };

  // Allowlisted or denylisted class names based on provided options at
  // construction time. These are used to filter out results during
  // post-processing.
  ClassNameSet class_name_set_;

  // Score calibration parameters, if any. Built from TFLite Model
  // Metadata.
  std::unique_ptr<core::ScoreCalibration> score_calibration_;

  // Number of classes returned by `Postprocess` method.
  int num_results_;

  // Recommended score threshold typically in [0,1[. Classification results with
  // a score below this value are considered low-confidence and should be
  // rejected from returned results.
  float score_threshold_;
};

template <typename T>
absl::Status ClassificationPostprocessor::Postprocess(T* classifications) {
  const auto& head = classification_head_;
  classifications->set_head_index(tensor_indices_.at(0));

  std::vector<std::pair<int, float>> score_pairs;
  score_pairs.reserve(head.label_map_items.size());

  const TfLiteTensor* output_tensor = GetTensor();
  if (output_tensor->type == kTfLiteUInt8) {
    TFLITE_ASSIGN_OR_RETURN(const uint8* output_data,
                     core::AssertAndReturnTypedTensor<uint8>(output_tensor));
    for (int j = 0; j < head.label_map_items.size(); ++j) {
      score_pairs.emplace_back(
          j, output_tensor->params.scale * (static_cast<int>(output_data[j]) -
                                            output_tensor->params.zero_point));
    }
  } else {
    TFLITE_ASSIGN_OR_RETURN(const float* output_data,
                     core::AssertAndReturnTypedTensor<float>(output_tensor));
    for (int j = 0; j < head.label_map_items.size(); ++j) {
      score_pairs.emplace_back(j, output_data[j]);
    }
  }

  // Optional score calibration.
  if (score_calibration_ != nullptr) {
    for (auto& score_pair : score_pairs) {
      const std::string& class_name =
          head.label_map_items[score_pair.first].name;

      // In ComputeCalibratedScore, score_pair.second is set to the
      // default_score value from metadata [1] if the category (1) has no
      // score calibration data or (2) has a very low confident uncalibrated
      // score, i.e. lower than the `min_uncalibrated_score` threshold.
      // Otherwise, score_pair.second is calculated based on the selected
      // score transformation function, and the value is guaranteed to be in
      // the range of [0, scale], where scale is a label-dependent sigmoid
      // parameter.
      //
      // [1]:
      // https://github.com/tensorflow/tflite-support/blob/af26cb6952ccdeee0e849df2b93dbe7e57f6bc48/tensorflow_lite_support/metadata/metadata_schema.fbs#L453
      score_pair.second = score_calibration_->ComputeCalibratedScore(
          class_name, score_pair.second);
    }
  }

  if (class_name_set_.values.empty()) {
    // Partially sort in descending order (higher score is better).
    absl::c_partial_sort(
        score_pairs, score_pairs.begin() + num_results_,
        [](const std::pair<int, float>& a, const std::pair<int, float>& b) {
          return a.second > b.second;
        });

    for (int j = 0; j < num_results_; ++j) {
      float score = score_pairs[j].second;
      if (score < score_threshold_) {
        break;
      }
      auto* cl = classifications->add_classes();
      cl->set_index(score_pairs[j].first);
      cl->set_score(score);
    }
  } else {
    // Sort in descending order (higher score is better).
    absl::c_sort(score_pairs, [](const std::pair<int, float>& a,
                                 const std::pair<int, float>& b) {
      return a.second > b.second;
    });

    for (int j = 0; j < head.label_map_items.size(); ++j) {
      float score = score_pairs[j].second;
      if (score < score_threshold_ ||
          classifications->classes_size() >= num_results_) {
        break;
      }

      const int class_index = score_pairs[j].first;
      const std::string& class_name = head.label_map_items[class_index].name;

      bool class_name_found = class_name_set_.values.contains(class_name);

      if ((!class_name_found && class_name_set_.is_allowlist) ||
          (class_name_found && !class_name_set_.is_allowlist)) {
        continue;
      }

      auto* cl = classifications->add_classes();
      cl->set_index(class_index);
      cl->set_score(score);
    }
  }
  return FillResultsFromLabelMaps(classifications);
}

template <typename T>
absl::Status ClassificationPostprocessor::FillResultsFromLabelMaps(
    T* classifications) {
  int head_index = classifications->head_index();
  const auto& label_map_items = classification_head_.label_map_items;
  for (int j = 0; j < classifications->classes_size(); ++j) {
    auto* current_class = classifications->mutable_classes(j);
    int current_class_index = current_class->index();
    if (current_class_index < 0 ||
        current_class_index >= label_map_items.size()) {
      return CreateStatusWithPayload(
          absl::StatusCode::kInvalidArgument,
          absl::StrFormat("Invalid class index (%d) with respect to label "
                          "map size (%d) for head #%d.",
                          current_class_index, label_map_items.size(),
                          head_index),
          support::TfLiteSupportStatus::kMetadataInconsistencyError);
    }
    const std::string& name = label_map_items[current_class_index].name;
    if (!name.empty()) {
      current_class->set_class_name(name);
    }
    const std::string& display_name =
        label_map_items[current_class_index].display_name;
    if (!display_name.empty()) {
      current_class->set_display_name(display_name);
    }
  }
  return absl::OkStatus();
}

}  // namespace processor
}  // namespace task
}  // namespace tflite
#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_CLASSIFICATION_POSTPROCESSOR_H_
