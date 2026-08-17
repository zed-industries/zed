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

#ifndef MEDIAPIPE_CALCULATORS_UTIL_LANDMARKS_TRANSFORMATION_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_UTIL_LANDMARKS_TRANSFORMATION_CALCULATOR_H_

#include "mediapipe/calculators/util/landmarks_transformation_calculator.pb.h"
#include "mediapipe/framework/api2/node.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/formats/landmark.pb.h"

namespace mediapipe {
namespace api2 {

// A calculator to transform landmarks.
//
// Input:
//   LANDMARKS - LandmarkList
//     Landmarks to transform.
//
// Output:
//   LANDMARKS - LandmarkList
//     Transformed landmarks.
//
// Example:
//   node {
//     calculator: "LandmarksTransformationCalculator"
//     input_stream: "LANDMARKS:in_landmarks"
//     output_stream: "LANDMARKS:out_landmarks"
//     options: {
//       [mediapipe.LandmarksTransformationCalculatorOptions.ext] {
//         transformation: { normalize_translation: {} }
//         transformation: { flip_axis: { flip_x: true } }
//       }
//     }
//   }
class LandmarksTransformationCalculator : public NodeIntf {
 public:
  static constexpr Input<mediapipe::LandmarkList> kInLandmarks{"LANDMARKS"};
  static constexpr Input<
      mediapipe::LandmarksTransformationCalculatorOptions>::Optional kInOptions{
      "OPTIONS"};
  static constexpr Output<mediapipe::LandmarkList> kOutLandmarks{"LANDMARKS"};
  MEDIAPIPE_NODE_INTERFACE(LandmarksTransformationCalculator, kInLandmarks,
                           kInOptions, kOutLandmarks);
};

}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_UTIL_LANDMARKS_TRANSFORMATION_CALCULATOR_H_
