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
#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_UTILS_SCORE_CALIBRATION_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_UTILS_SCORE_CALIBRATION_H_

#include <iostream>
#include <map>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/container/flat_hash_map.h"  // from @com_google_absl
#include "absl/status/status.h"  // from @com_google_absl
#include "absl/strings/string_view.h"  // from @com_google_absl
#include "absl/types/optional.h"  // from @com_google_absl
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/vision/core/label_map_item.h"
#include "tensorflow_lite_support/metadata/metadata_schema_generated.h"

namespace tflite {
namespace task {
namespace vision {

// Sigmoid structure.
struct Sigmoid {
  Sigmoid() : scale(1.0) {}
  Sigmoid(std::string label, float slope, float offset, float scale = 1.0,
          absl::optional<float> min_uncalibrated_score = absl::nullopt)
      : label(label),
        slope(slope),
        offset(offset),
        scale(scale),
        min_uncalibrated_score(min_uncalibrated_score) {}

  bool operator==(const Sigmoid& other) const {
    return label == other.label && slope == other.slope &&
           offset == other.offset && scale == other.scale &&
           min_uncalibrated_score == other.min_uncalibrated_score;
  }

  // Unique label corresponding to the sigmoid parameters.
  std::string label;
  float slope;
  float offset;
  float scale;
  absl::optional<float> min_uncalibrated_score;
};

std::ostream& operator<<(std::ostream& os, const Sigmoid& s);

// Transformation function to use for computing transformation scores.
enum class ScoreTransformation {
  kIDENTITY,         // f(x) = x
  kLOG,              // f(x) = log(x)
  kINVERSE_LOGISTIC  // f(x) = log(x) - log(1 - x)
};

// Sigmoid calibration parameters.
struct SigmoidCalibrationParameters {
  SigmoidCalibrationParameters()
      : default_score(0.0),
        score_transformation(ScoreTransformation::kIDENTITY) {}
  explicit SigmoidCalibrationParameters(
      std::vector<Sigmoid> sigmoid,
      ScoreTransformation score_transformation = ScoreTransformation::kIDENTITY,
      absl::optional<Sigmoid> default_sigmoid = absl::nullopt,
      float default_score = 0.0)
      : sigmoid(sigmoid),
        default_sigmoid(default_sigmoid),
        default_score(default_score),
        score_transformation(score_transformation) {}
  // A vector of Sigmoid associated to the ScoreCalibration instance.
  std::vector<Sigmoid> sigmoid;
  // If set, this sigmoid will be applied to any non-matching labels.
  absl::optional<Sigmoid> default_sigmoid;
  // The default score for non-matching labels. Only used if default_sigmoid
  // isn't set.
  float default_score;
  // Function for computing a transformation score prior to sigmoid fitting.
  ScoreTransformation score_transformation;
};

// This class is used to calibrate predicted scores so that scores are
// comparable across labels. Depending on the particular calibration parameters
// being used, the calibrated scores can also be approximately interpreted as a
// likelihood of being correct. For a given TF Lite model, such parameters are
// typically obtained from TF Lite Metadata (see ScoreCalibrationOptions).
class ScoreCalibration {
 public:
  ScoreCalibration();
  ~ScoreCalibration();

  // Transfers input parameters and construct a label to sigmoid map.
  absl::Status InitializeFromParameters(
      const SigmoidCalibrationParameters& params);

  // Returns a calibrated score given a label string and uncalibrated score. The
  // calibrated score will be in the range [0.0, 1.0] and can loosely be
  // interpreted as a likelihood of the label being correct.
  float ComputeCalibratedScore(const std::string& label,
                               float uncalibrated_score) const;

 private:
  // Finds the sigmoid parameters corresponding to the provided label.
  absl::optional<Sigmoid> FindSigmoidParameters(const std::string& label) const;

  // Parameters for internal states.
  SigmoidCalibrationParameters sigmoid_parameters_;

  // Maps label strings to the particular sigmoid stored in sigmoid_parameters_.
  absl::flat_hash_map<std::string, Sigmoid> sigmoid_parameters_map_;
};

// Builds SigmoidCalibrationParameters using data obtained from TF Lite Metadata
// (see ScoreCalibrationOptions in metadata schema).
//
// The provided `score_calibration_file` represents the contents of the score
// calibration associated file (TENSOR_AXIS_SCORE_CALIBRATION), i.e. one set of
// parameters (scale, slope, etc) per line. Each line must be in 1:1
// correspondence with `label_map_items`, so as to associate each sigmoid to its
// corresponding label name. Returns an error if no valid parameters could be
// built (e.g. malformed parameters).
tflite::support::StatusOr<SigmoidCalibrationParameters>
BuildSigmoidCalibrationParams(
    const tflite::ScoreCalibrationOptions& score_calibration_options,
    absl::string_view score_calibration_file,
    const std::vector<LabelMapItem>& label_map_items);

}  // namespace vision
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_UTILS_SCORE_CALIBRATION_H_
