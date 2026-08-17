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

#ifndef MEDIAPIPE_CALCULATORS_UTIL_COMBINE_JOINTS_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_UTIL_COMBINE_JOINTS_CALCULATOR_H_

#include "mediapipe/framework/api2/node.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/formats/body_rig.pb.h"

namespace mediapipe {
namespace api2 {

// A calculator to combine several joint sets into one.
//
// Input:
//   JOINTS - Multiple JointList
//     Joint sets to combine into one. Subsets are applied in provided order and
//     overwrite each other.
//
// Output:
//   JOINTS - JointList
//     Combined joints.
//
// Example:
//   node {
//     calculator: "CombineJointsCalculator"
//     input_stream: "JOINTS:0:joints_0"
//     input_stream: "JOINTS:1:joints_1"
//     output_stream: "JOINTS:combined_joints"
//     options: {
//       [mediapipe.CombineJointsCalculatorOptions.ext] {
//         num_joints: 63
//         joints_mapping: { idx: [0, 1, 2] }
//         joints_mapping: { idx: [2, 3] }
//         default_joint: {
//           rotation_6d: [1, 0, 0, 1, 0, 0]
//           visibility: 1.0
//         }
//       }
//     }
//   }
class CombineJointsCalculator : public NodeIntf {
 public:
  static constexpr Input<mediapipe::JointList>::Multiple kInJoints{"JOINTS"};
  static constexpr Output<mediapipe::JointList> kOutJoints{"JOINTS"};
  MEDIAPIPE_NODE_INTERFACE(CombineJointsCalculator, kInJoints, kOutJoints);
};

}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_UTIL_COMBINE_JOINTS_CALCULATOR_H_
