// Copyright 2021 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_CALCULATORS_UTIL_REFINE_LANDMARKS_FROM_HEATMAP_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_UTIL_REFINE_LANDMARKS_FROM_HEATMAP_CALCULATOR_H_

#include <vector>

#include "mediapipe/framework/api2/node.h"
#include "mediapipe/framework/formats/landmark.pb.h"
#include "mediapipe/framework/formats/tensor.h"
#include "mediapipe/framework/port/statusor.h"

namespace mediapipe {
namespace api2 {

class RefineLandmarksFromHeatmapCalculator : public NodeIntf {
 public:
  static constexpr Input<mediapipe::NormalizedLandmarkList> kInLandmarks{
      "NORM_LANDMARKS"};
  static constexpr Input<std::vector<Tensor>> kInTensors{"TENSORS"};
  static constexpr Output<mediapipe::NormalizedLandmarkList> kOutLandmarks{
      "NORM_LANDMARKS"};

  MEDIAPIPE_NODE_INTERFACE(RefineLandmarksFromHeatmapCalculator, kInLandmarks,
                           kInTensors, kOutLandmarks);
};

}  // namespace api2

// Exposed for testing.
absl::StatusOr<mediapipe::NormalizedLandmarkList> RefineLandmarksFromHeatMap(
    const mediapipe::NormalizedLandmarkList& in_lms,
    const float* heatmap_raw_data, const std::vector<int>& heatmap_dims,
    int kernel_size, float min_confidence_to_refine, bool refine_presence,
    bool refine_visibility);

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_UTIL_REFINE_LANDMARKS_FROM_HEATMAP_CALCULATOR_H_
