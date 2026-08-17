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

#ifndef MEDIAPIPE_CALCULATORS_UTIL_MULTI_WORLD_LANDMARKS_SMOOTHING_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_UTIL_MULTI_WORLD_LANDMARKS_SMOOTHING_CALCULATOR_H_

#include "mediapipe/framework/api2/node.h"
#include "mediapipe/framework/formats/landmark.pb.h"
#include "mediapipe/framework/formats/rect.pb.h"

namespace mediapipe {
namespace api2 {

// A calculator to smooth landmarks over time.
//
// Inputs:
//   LANDMARKS: A std::vector<LandmarkList> of landmarks you want to
//     smooth.
//   TRACKING_IDS: A std<int64_t> vector of tracking IDs used to associate
//     landmarks over time. When new ID arrives - calculator will initialize new
//     filter. When tracking ID is no longer provided - calculator will forget
//     smoothing state.
//   OBJECT_SCALE_ROI (optional): A std::vector<Rect> used to determine the
//     object scale for some of the filters. If not provided - object scale will
//     be calculated from landmarks.
//
// Outputs:
//   FILTERED_LANDMARKS: A std::vector<LandmarkList> of smoothed landmarks.
//
// Example config:
//   node {
//     calculator: "MultiWorldLandmarksSmoothingCalculator"
//     input_stream: "LANDMARKS:landmarks"
//     input_stream: "OBJECT_SCALE_ROI:roi"
//     output_stream: "FILTERED_LANDMARKS:landmarks_filtered"
//     options: {
//       [mediapipe.LandmarksSmoothingCalculatorOptions.ext] {
//         velocity_filter: {
//           window_size: 5
//           velocity_scale: 10.0
//         }
//       }
//     }
//   }
//
class MultiWorldLandmarksSmoothingCalculator : public NodeIntf {
 public:
  static constexpr Input<std::vector<mediapipe::LandmarkList>> kInLandmarks{
      "LANDMARKS"};
  static constexpr Input<std::vector<int64_t>> kTrackingIds{"TRACKING_IDS"};
  static constexpr Input<std::vector<Rect>>::Optional kObjectScaleRoi{
      "OBJECT_SCALE_ROI"};
  static constexpr Output<std::vector<mediapipe::LandmarkList>> kOutLandmarks{
      "FILTERED_LANDMARKS"};

  MEDIAPIPE_NODE_INTERFACE(MultiWorldLandmarksSmoothingCalculator, kInLandmarks,
                           kTrackingIds, kObjectScaleRoi, kOutLandmarks);
};

}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_UTIL_MULTI_WORLD_LANDMARKS_SMOOTHING_CALCULATOR_H_
