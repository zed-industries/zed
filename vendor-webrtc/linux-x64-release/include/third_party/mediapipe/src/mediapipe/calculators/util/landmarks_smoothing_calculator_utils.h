// Copyright 2023 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_CALCULATORS_UTIL_LANDMARKS_SMOOTHING_CALCULATOR_UTILS_H_
#define MEDIAPIPE_CALCULATORS_UTIL_LANDMARKS_SMOOTHING_CALCULATOR_UTILS_H_

#include "mediapipe/calculators/util/landmarks_smoothing_calculator.pb.h"
#include "mediapipe/framework/calculator_context.h"
#include "mediapipe/framework/formats/landmark.pb.h"
#include "mediapipe/framework/formats/rect.pb.h"
#include "mediapipe/util/filtering/one_euro_filter.h"
#include "mediapipe/util/filtering/relative_velocity_filter.h"

namespace mediapipe {
namespace landmarks_smoothing {

void NormalizedLandmarksToLandmarks(
    const mediapipe::NormalizedLandmarkList& norm_landmarks,
    const int image_width, const int image_height,
    mediapipe::LandmarkList& landmarks);

void LandmarksToNormalizedLandmarks(
    const mediapipe::LandmarkList& landmarks, const int image_width,
    const int image_height, mediapipe::NormalizedLandmarkList& norm_landmarks);

float GetObjectScale(const NormalizedRect& roi, const int image_width,
                     const int image_height);

float GetObjectScale(const Rect& roi);

// Abstract class for various landmarks filters.
class LandmarksFilter {
 public:
  virtual ~LandmarksFilter() = default;

  virtual absl::Status Reset() { return absl::OkStatus(); }

  virtual absl::Status Apply(const mediapipe::LandmarkList& in_landmarks,
                             const absl::Duration& timestamp,
                             const absl::optional<float> object_scale_opt,
                             mediapipe::LandmarkList& out_landmarks) = 0;
};

absl::StatusOr<std::unique_ptr<LandmarksFilter>> InitializeLandmarksFilter(
    const mediapipe::LandmarksSmoothingCalculatorOptions& options);

class MultiLandmarkFilters {
 public:
  virtual ~MultiLandmarkFilters() = default;

  virtual absl::StatusOr<LandmarksFilter*> GetOrCreate(
      const int64_t tracking_id,
      const mediapipe::LandmarksSmoothingCalculatorOptions& options);

  virtual void ClearUnused(const std::vector<int64_t>& tracking_ids);

  virtual void Clear();

 private:
  std::map<int64_t, std::unique_ptr<LandmarksFilter>> filters_;
};

}  // namespace landmarks_smoothing
}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_UTIL_LANDMARKS_SMOOTHING_CALCULATOR_UTILS_H_
