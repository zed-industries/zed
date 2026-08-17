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

#ifndef MEDIAPIPE_CALCULATORS_UTIL_SET_JOINTS_VISIBILITY_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_UTIL_SET_JOINTS_VISIBILITY_CALCULATOR_H_

#include "mediapipe/framework/api2/node.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/formats/body_rig.pb.h"
#include "mediapipe/framework/formats/landmark.pb.h"

namespace mediapipe {
namespace api2 {

// A calculator set Joints visibility from Landmarks.
//
// Calculator allows to either copy visibility right from the landmark or
// somehow combine visibilities of several landmarks.
//
// Input:
//   JOINTS - JointList
//     Joints to to update visibility.
//   LANDMARKS - LandmarkList
//     Landmarks to take visibility from.
//
// Output:
//   JOINTS - JointList
//     Joints with updated visibility.
//
// Example:
//   node {
//     calculator: "SetJointsVisibilityCalculator"
//     input_stream: "JOINTS:joints"
//     input_stream: "LANDMARKS:landmarks"
//     output_stream: "JOINTS:joints_with_visibility"
//     options: {
//       [mediapipe.SetJointsVisibilityCalculatorOptions.ext] {
//         mapping: [
//           { copy: { idx: 0 } },
//           { highest: { idx: [5, 6] } }
//         ]
//       }
//     }
//   }
class SetJointsVisibilityCalculator : public NodeIntf {
 public:
  static constexpr Input<mediapipe::JointList> kInJoints{"JOINTS"};
  static constexpr Input<mediapipe::LandmarkList> kInLandmarks{"LANDMARKS"};
  static constexpr Output<mediapipe::JointList> kOutJoints{"JOINTS"};
  MEDIAPIPE_NODE_INTERFACE(SetJointsVisibilityCalculator, kInJoints,
                           kInLandmarks, kOutJoints);
};

}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_UTIL_SET_JOINTS_VISIBILITY_CALCULATOR_H_
