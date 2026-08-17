/* Copyright 2022 The MediaPipe Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/

#ifndef MEDIAPIPE_CALCULATORS_CORE_MERGE_TO_VECTOR_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_CORE_MERGE_TO_VECTOR_CALCULATOR_H_

#include <algorithm>
#include <memory>
#include <utility>
#include <vector>

#include "absl/status/status.h"
#include "mediapipe/framework/api2/node.h"
#include "mediapipe/framework/api2/port.h"
#include "mediapipe/framework/calculator_framework.h"

namespace mediapipe {
namespace api2 {

template <typename T>
class MergeToVectorCalculator : public Node {
 public:
  static constexpr typename Input<T>::Multiple kIn{""};
  static constexpr Output<std::vector<T>> kOut{""};

  MEDIAPIPE_NODE_CONTRACT(kIn, kOut);

  static absl::Status UpdateContract(CalculatorContract* cc) {
    RET_CHECK_GT(kIn(cc).Count(), 0) << "Needs at least one input stream";
    return absl::OkStatus();
  }

  absl::Status Open(::mediapipe::CalculatorContext* cc) {
    cc->SetOffset(::mediapipe::TimestampDiff(0));
    return absl::OkStatus();
  }

  absl::Status Process(CalculatorContext* cc) {
    std::vector<T> output_vector;
    for (auto it = kIn(cc).begin(); it != kIn(cc).end(); it++) {
      const auto& elem = *it;
      if (!elem.IsEmpty()) {
        output_vector.push_back(elem.Get());
      }
    }
    kOut(cc).Send(output_vector);
    return absl::OkStatus();
  }
};

}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_CORE_MERGE_TO_VECTOR_CALCULATOR_H_
