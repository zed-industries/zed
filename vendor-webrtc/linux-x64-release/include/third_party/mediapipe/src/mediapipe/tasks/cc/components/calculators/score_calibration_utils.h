// Copyright 2022 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_TASKS_CC_COMPONENTS_CALCULATORS_SCORE_CALIBRATION_UTILS_H_
#define MEDIAPIPE_TASKS_CC_COMPONENTS_CALCULATORS_SCORE_CALIBRATION_UTILS_H_

#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "mediapipe/tasks/cc/components/calculators/score_calibration_calculator.pb.h"
#include "mediapipe/tasks/metadata/metadata_schema_generated.h"

namespace mediapipe {
namespace tasks {

// Populates ScoreCalibrationCalculatorOptions given a TFLite Metadata score
// transformation type, default threshold and score calibration AssociatedFile
// contents, as specified in `TENSOR_AXIS_SCORE_CALIBRATION`:
// https://github.com/google/mediapipe/blob/master/mediapipe/tasks/metadata/metadata_schema.fbs
absl::Status ConfigureScoreCalibration(
    tflite::ScoreTransformationType score_transformation, float default_score,
    absl::string_view score_calibration_file,
    ScoreCalibrationCalculatorOptions* options);

}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_COMPONENTS_CALCULATORS_SCORE_CALIBRATION_UTILS_H_
