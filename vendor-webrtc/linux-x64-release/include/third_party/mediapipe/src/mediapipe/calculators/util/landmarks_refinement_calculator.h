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

#ifndef MEDIAPIPE_CALCULATORS_UTIL_LANDMARKS_REFINEMENT_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_UTIL_LANDMARKS_REFINEMENT_CALCULATOR_H_

#include "mediapipe/framework/api2/node.h"
#include "mediapipe/framework/api2/port.h"
#include "mediapipe/framework/formats/landmark.pb.h"

namespace mediapipe {

namespace api2 {

// A calculator to refine one set of landmarks with another.
//
// Inputs:
//   LANDMARKS: Multiple NormalizedLandmarkList to use for
//     refinement. They will be applied to the resulting REFINED_LANDMARKS in
//     the provided order. Each list should be non empty and contain the same
//     amount of landmarks as indexes in mapping. Number of lists should be the
//     same as number of refinements in options.
//
// Outputs:
//   REFINED_LANDMARKS: A NormalizedLandmarkList with refined landmarks. Number
//     of produced landmarks is equal to to the maximum index mapping number in
//     calculator options (calculator verifies that there are no gaps in the
//     mapping).
//
// Examples config:
//   node {
//     calculator: "LandmarksRefinementCalculator"
//     input_stream: "LANDMARKS:0:mesh_landmarks"
//     input_stream: "LANDMARKS:1:lips_landmarks"
//     input_stream: "LANDMARKS:2:left_eye_landmarks"
//     input_stream: "LANDMARKS:3:right_eye_landmarks"
//     output_stream: "REFINED_LANDMARKS:landmarks"
//     options: {
//       [mediapipe.LandmarksRefinementCalculatorOptions.ext] {
//         refinement: {
//           indexes_mapping: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
//           z_refinement: { copy {} }
//         }
//         refinement: {
//           indexes_mapping: [0, 1, 2, 3]
//           z_refinement: { none {} }
//         }
//         refinement: {
//           indexes_mapping: [4, 5]
//           z_refinement: { none {} }
//         }
//         refinement: {
//           indexes_mapping: [6, 7]
//           z_refinement: { none {} }
//         }
//       }
//     }
//   }
//
class LandmarksRefinementCalculator : public NodeIntf {
 public:
  static constexpr Input<::mediapipe::NormalizedLandmarkList>::Multiple
      kLandmarks{"LANDMARKS"};
  static constexpr Output<::mediapipe::NormalizedLandmarkList>
      kRefinedLandmarks{"REFINED_LANDMARKS"};

  MEDIAPIPE_NODE_INTERFACE(LandmarksRefinementCalculator, kLandmarks,
                           kRefinedLandmarks);
};

}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_UTIL_LANDMARKS_REFINEMENT_CALCULATOR_H_
