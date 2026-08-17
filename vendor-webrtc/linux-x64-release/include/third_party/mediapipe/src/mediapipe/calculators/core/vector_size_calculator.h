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

#ifndef MEDIAPIPE_CALCULATORS_CORE_VECTOR_SIZE_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_CORE_VECTOR_SIZE_CALCULATOR_H_

#include <optional>

#include "mediapipe/framework/api2/node.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/port/status.h"

namespace mediapipe {
namespace api2 {

// A calcutlator to return vector size.
//
// Inputs:
//   VECTOR - std::vector<T>
//     Vector which size to return.
//
// Outputs:
//   SIZE - int
//     Size of the input vector.
//
// Example config:
//  node {
//    calculator: "{SpecificType}VectorSizeCalculator"
//    input_stream: "VECTOR:vector"
//    output_stream: "SIZE:vector_size"
//  }
//
template <typename T>
class VectorSizeCalculator : public Node {
 public:
  static constexpr Input<std::vector<T>> kIn{"VECTOR"};
  static constexpr Output<int> kOut{"SIZE"};

  MEDIAPIPE_NODE_CONTRACT(kIn, kOut);

  absl::Status Process(CalculatorContext* cc) final {
    if (kIn(cc).IsEmpty()) {
      return absl::OkStatus();
    }
    kOut(cc).Send(kIn(cc).Get().size());
    return absl::OkStatus();
  }
};

}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_CORE_VECTOR_SIZE_CALCULATOR_H_
