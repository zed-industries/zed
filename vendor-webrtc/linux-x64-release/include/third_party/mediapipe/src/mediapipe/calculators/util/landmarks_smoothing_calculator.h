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

#ifndef MEDIAPIPE_CALCULATORS_UTIL_LANDMARKS_SMOOTHING_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_UTIL_LANDMARKS_SMOOTHING_CALCULATOR_H_

#include "mediapipe/framework/api2/node.h"
#include "mediapipe/framework/formats/landmark.pb.h"
#include "mediapipe/framework/formats/rect.pb.h"
#include "mediapipe/framework/port/ret_check.h"

namespace mediapipe {
namespace api2 {

// A calculator to smooth landmarks over time.
//
// Inputs:
//   NORM_LANDMARKS (optional): A NormalizedLandmarkList of landmarks you want
//     to smooth.
//   LANDMARKS (optional): A LandmarkList of landmarks you want to smooth.
//   IMAGE_SIZE (optional): A std::pair<int, int> represention of image width
//     and height. Required to perform all computations in absolute coordinates
//     when smoothing NORM_LANDMARKS to avoid any influence of normalized
//     values.
//   OBJECT_SCALE_ROI (optional): A NormRect or Rect (depending on the format of
//     input landmarks) used to determine the object scale for some of the
//     filters. If not provided - object scale will be calculated from
//     landmarks.
//
// Outputs:
//   NORM_FILTERED_LANDMARKS (optional): A NormalizedLandmarkList of smoothed
//     landmarks.
//   FILTERED_LANDMARKS (optional): A LandmarkList of smoothed landmarks.
//
// Example config:
//   node {
//     calculator: "LandmarksSmoothingCalculator"
//     input_stream: "NORM_LANDMARKS:landmarks"
//     input_stream: "IMAGE_SIZE:image_size"
//     input_stream: "OBJECT_SCALE_ROI:roi"
//     output_stream: "NORM_FILTERED_LANDMARKS:landmarks_filtered"
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
class LandmarksSmoothingCalculator : public NodeIntf {
 public:
  static constexpr Input<mediapipe::NormalizedLandmarkList>::Optional
      kInNormLandmarks{"NORM_LANDMARKS"};
  static constexpr Input<mediapipe::LandmarkList>::Optional kInLandmarks{
      "LANDMARKS"};
  static constexpr Input<std::pair<int, int>>::Optional kImageSize{
      "IMAGE_SIZE"};
  static constexpr Input<OneOf<NormalizedRect, Rect>>::Optional kObjectScaleRoi{
      "OBJECT_SCALE_ROI"};
  static constexpr Output<mediapipe::NormalizedLandmarkList>::Optional
      kOutNormLandmarks{"NORM_FILTERED_LANDMARKS"};
  static constexpr Output<mediapipe::LandmarkList>::Optional kOutLandmarks{
      "FILTERED_LANDMARKS"};
  MEDIAPIPE_NODE_INTERFACE(LandmarksSmoothingCalculator, kInNormLandmarks,
                           kInLandmarks, kImageSize, kObjectScaleRoi,
                           kOutNormLandmarks, kOutLandmarks);

  static absl::Status UpdateContract(CalculatorContract* cc) {
    RET_CHECK(kInNormLandmarks(cc).IsConnected() ^
              kInLandmarks(cc).IsConnected())
        << "One and only one of NORM_LANDMARKS and LANDMARKS input is allowed";

    // TODO: Verify scale ROI is of the same type as landmarks
    // that are being smoothed.

    if (kInNormLandmarks(cc).IsConnected()) {
      RET_CHECK(kImageSize(cc).IsConnected());
      RET_CHECK(kOutNormLandmarks(cc).IsConnected());
      RET_CHECK(!kOutLandmarks(cc).IsConnected());
    } else {
      RET_CHECK(!kImageSize(cc).IsConnected());
      RET_CHECK(kOutLandmarks(cc).IsConnected());
      RET_CHECK(!kOutNormLandmarks(cc).IsConnected());
    }

    return absl::OkStatus();
  }
};

}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_UTIL_LANDMARKS_SMOOTHING_CALCULATOR_H_
