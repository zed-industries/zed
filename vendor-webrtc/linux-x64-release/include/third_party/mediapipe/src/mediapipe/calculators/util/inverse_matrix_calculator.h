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

#ifndef MEDIAPIPE_CALCULATORS_UTIL_INVERSE_MATRIX_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_UTIL_INVERSE_MATRIX_CALCULATOR_H_

#include "mediapipe/framework/api2/node.h"
#include "mediapipe/framework/api2/port.h"

namespace mediapipe {

// Runs affine transformation.
//
// Input:
//   MATRIX - std::array<float, 16>
//     Row major 4x4 matrix to inverse.
//
// Output:
//   MATRIX - std::array<float, 16>
//     Row major 4x4 inversed matrix.
//
// Usage example:
//   node {
//     calculator: "dishti.aimatter.InverseMatrixCalculator"
//     input_stream: "MATRIX:input_matrix"
//     output_stream: "MATRIX:output_matrix"
//   }
class InverseMatrixCalculator : public mediapipe::api2::NodeIntf {
 public:
  static constexpr mediapipe::api2::Input<std::array<float, 16>> kInputMatrix{
      "MATRIX"};
  static constexpr mediapipe::api2::Output<std::array<float, 16>> kOutputMatrix{
      "MATRIX"};
  MEDIAPIPE_NODE_INTERFACE(InverseMatrixCalculator, kInputMatrix,
                           kOutputMatrix);
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_UTIL_INVERSE_MATRIX_CALCULATOR_H_
