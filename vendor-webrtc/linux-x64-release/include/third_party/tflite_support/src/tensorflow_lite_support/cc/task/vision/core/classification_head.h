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
#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_CORE_CLASSIFICATION_HEAD_ITEM_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_CORE_CLASSIFICATION_HEAD_ITEM_H_

#include <string>
#include <vector>

#include "absl/memory/memory.h"  // from @com_google_absl
#include "absl/strings/string_view.h"  // from @com_google_absl
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/vision/core/label_map_item.h"
#include "tensorflow_lite_support/cc/task/vision/utils/score_calibration.h"
#include "tensorflow_lite_support/metadata/cc/metadata_extractor.h"
#include "tensorflow_lite_support/metadata/metadata_schema_generated.h"

namespace tflite {
namespace task {
namespace vision {

// A single classifier head for an image classifier model, associated with a
// corresponding output tensor.
struct ClassificationHead {
  ClassificationHead() : score_threshold(0) {}

  explicit ClassificationHead(
      const std::vector<tflite::task::vision::LabelMapItem>&& label_map_items)
      : label_map_items(label_map_items), score_threshold(0) {}

  // An optional name that usually indicates what this set of classes represent,
  // e.g. "flowers".
  std::string name;
  // The label map representing the list of supported classes, aka labels.
  //
  // This must be in direct correspondence with the associated output tensor,
  // i.e.:
  //
  // - The number of classes must match with the dimension of the corresponding
  // output tensor,
  // - The i-th item in the label map is assumed to correspond to the i-th
  // output value in the output tensor.
  //
  // This requires to put in place dedicated sanity checks before running
  // inference.
  std::vector<tflite::task::vision::LabelMapItem> label_map_items;
  // Recommended score threshold typically in [0,1[. Classification results with
  // a score below this value are considered low-confidence and should be
  // rejected from returned results.
  float score_threshold;
  // Optional score calibration parameters (one set of parameters per class in
  // the label map). This is primarily meant for multi-label classifiers made of
  // independent sigmoids.
  //
  // Such parameters are usually tuned so that calibrated scores can be compared
  // to a default threshold common to all classes to achieve a given amount of
  // precision.
  //
  // Example: 60% precision for threshold = 0.5.
  absl::optional<tflite::task::vision::SigmoidCalibrationParameters>
      calibration_params;
};

// Builds a classification head using the provided metadata extractor, for the
// given output tensor metadata. Returns an error in case the head cannot be
// built (e.g. missing associated file for score calibration parameters).
//
// Optionally it is possible to specify which locale should be used (e.g. "en")
// to fill the label map display names, if any, and provided the corresponding
// associated file is present in the metadata. If no locale is specified, or if
// there is no associated file for the provided locale, display names are just
// left empty and no error is returned.
//
// E.g. (metatada displayed in JSON format below):
//
// ...
// "associated_files": [
//  {
//    "name": "labels.txt",
//    "type": "TENSOR_AXIS_LABELS"
//  },
//  {
//    "name": "labels-en.txt",
//    "type": "TENSOR_AXIS_LABELS",
//    "locale": "en"
//  },
// ...
//
// See metadata schema TENSOR_AXIS_LABELS for more details.
tflite::support::StatusOr<ClassificationHead> BuildClassificationHead(
    const tflite::metadata::ModelMetadataExtractor& metadata_extractor,
    const tflite::TensorMetadata& output_tensor_metadata,
    absl::string_view display_names_locale = absl::string_view());

}  // namespace vision
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_CORE_CLASSIFICATION_HEAD_ITEM_H_
