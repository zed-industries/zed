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

#ifndef MEDIAPIPE_CALCULATORS_TENSOR_TENSOR_TO_JOINTS_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_TENSOR_TENSOR_TO_JOINTS_CALCULATOR_H_

#include <memory>

#include "mediapipe/framework/api2/node.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/formats/body_rig.pb.h"
#include "mediapipe/framework/formats/tensor.h"

namespace mediapipe {
namespace api2 {

// A calculator to convert Tensors to JointList.
//
// Calculator fills in only rotation of the joints leaving visibility undefined.
//
// Input:
//   TENSOR - std::vector<Tensor> with kFloat32 values
//     Vector of tensors to be converted to joints. Only the first tensor will
//     be used. Number of values is expected to be multiple of six.
//
// Output:
//   JOINTS - JointList
//     List of joints with rotations extracted from given tensor and undefined
//     visibility.
//
// Example:
//   node {
//     calculator: "TensorToJointsCalculator"
//     input_stream: "TENSOR:tensor"
//     output_stream: "JOINTS:joints"
//     options: {
//       [mediapipe.TensorToJointsCalculatorOptions.ext] {
//         num_joints: 56
//         start_index: 3
//       }
//     }
//   }
class TensorToJointsCalculator : public NodeIntf {
 public:
  static constexpr Input<mediapipe::Tensor> kInTensor{"TENSOR"};
  static constexpr Output<mediapipe::JointList> kOutJoints{"JOINTS"};
  MEDIAPIPE_NODE_INTERFACE(TensorToJointsCalculator, kInTensor, kOutJoints);
};

}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_TENSOR_TENSOR_TO_JOINTS_CALCULATOR_H_
