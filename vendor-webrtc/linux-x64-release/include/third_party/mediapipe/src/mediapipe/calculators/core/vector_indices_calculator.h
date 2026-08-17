// Copyright 2022 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_CALCULATORS_CORE_VECTOR_INDICES_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_CORE_VECTOR_INDICES_CALCULATOR_H_

#include <optional>

#include "mediapipe/framework/api2/node.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/port/status.h"

namespace mediapipe {
namespace api2 {
// Calculator that takes a vector and constructs an index range vector based on
// the size of the input vector.
//
// Inputs:
//   VECTOR - std::vector<T>
//     Vector whose range of indices to return.
//
// Outputs:
//   INDICES - std::vector<int>
//     Indices vector of the input vector.
//
// Example config:
//  node {
//    calculator: "{SpecificType}VectorIndicesCalculator"
//    input_stream: "VECTOR:vector"
//    output_stream: "INDICES:indices"
//  }
//
template <typename T>
class VectorIndicesCalculator : public Node {
 public:
  static constexpr Input<std::vector<T>> kVector{"VECTOR"};
  static constexpr Output<std::vector<int>> kRange{"INDICES"};

  MEDIAPIPE_NODE_CONTRACT(kVector, kRange);

  absl::Status Process(CalculatorContext* cc) final {
    // Get the size of the input vector.
    const int vector_size = kVector(cc).Get().size();
    std::vector<int> out_idxs(vector_size);
    std::iota(out_idxs.begin(), out_idxs.end(), 0);
    kRange(cc).Send(out_idxs);
    return absl::OkStatus();
  }
};

}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_CORE_VECTOR_INDICES_CALCULATOR_H_
