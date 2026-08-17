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

#ifndef MEDIAPIPE_CALCULATORS_UTIL_MULTI_LANDMARKS_SMOOTHING_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_UTIL_MULTI_LANDMARKS_SMOOTHING_CALCULATOR_H_

#include "mediapipe/framework/api2/node.h"
#include "mediapipe/framework/formats/landmark.pb.h"
#include "mediapipe/framework/formats/rect.pb.h"

namespace mediapipe {
namespace api2 {

// A calculator to smooth landmarks over time.
//
// Inputs:
//   NORM_LANDMARKS: A std::vector<NormalizedLandmarkList> of landmarks you want
//     to smooth.
//   TRACKING_IDS: A std<int64_t> vector of tracking IDs used to associate
//     landmarks over time. When new ID arrives - calculator will initialize new
//     filter. When tracking ID is no longer provided - calculator will forget
//     smoothing state.
//   IMAGE_SIZE: A std::pair<int, int> represention of image width and height.
//     Required to perform all computations in absolute coordinates to avoid any
//     influence of normalized values.
//   OBJECT_SCALE_ROI (optional): A std::vector<NormRect> used to determine the
//     object scale for some of the filters. If not provided - object scale will
//     be calculated from landmarks.
//
// Outputs:
//   NORM_FILTERED_LANDMARKS: A std::vector<NormalizedLandmarkList> of smoothed
//     landmarks.
//
// Example config:
//   node {
//     calculator: "MultiLandmarksSmoothingCalculator"
//     input_stream: "NORM_LANDMARKS:pose_landmarks"
//     input_stream: "IMAGE_SIZE:image_size"
//     input_stream: "OBJECT_SCALE_ROI:roi"
//     output_stream: "NORM_FILTERED_LANDMARKS:pose_landmarks_filtered"
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
class MultiLandmarksSmoothingCalculator : public NodeIntf {
 public:
  static constexpr Input<std::vector<mediapipe::NormalizedLandmarkList>>
      kInNormLandmarks{"NORM_LANDMARKS"};
  static constexpr Input<std::vector<int64_t>> kTrackingIds{"TRACKING_IDS"};
  static constexpr Input<std::pair<int, int>> kImageSize{"IMAGE_SIZE"};
  static constexpr Input<std::vector<NormalizedRect>>::Optional kObjectScaleRoi{
      "OBJECT_SCALE_ROI"};
  static constexpr Output<std::vector<mediapipe::NormalizedLandmarkList>>
      kOutNormLandmarks{"NORM_FILTERED_LANDMARKS"};

  MEDIAPIPE_NODE_INTERFACE(MultiLandmarksSmoothingCalculator, kInNormLandmarks,
                           kTrackingIds, kImageSize, kObjectScaleRoi,
                           kOutNormLandmarks);
};

}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_UTIL_MULTI_LANDMARKS_SMOOTHING_CALCULATOR_H_
