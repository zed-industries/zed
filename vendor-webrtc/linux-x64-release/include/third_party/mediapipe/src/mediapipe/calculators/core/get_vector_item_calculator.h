// Copyright 2019 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_CALCULATORS_CORE_GET_VECTOR_ITEM_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_CORE_GET_VECTOR_ITEM_CALCULATOR_H_

#include <optional>

#include "mediapipe/calculators/core/get_vector_item_calculator.pb.h"
#include "mediapipe/framework/api2/node.h"
#include "mediapipe/framework/api2/packet.h"
#include "mediapipe/framework/api2/port.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/port/ret_check.h"
#include "mediapipe/framework/port/status.h"

namespace mediapipe {
namespace api2 {

// A calculator to return an item from the vector by its index.
// Item index can be specified through INDEX stream and/or calculator options.
// INDEX stream takes precedence over options.
//
// Inputs:
//   VECTOR - std::vector<T>
//     Vector to take an item from.
//   INDEX [OPTIONAL] - int
//     Index of the item to return.
//
// Outputs:
//   ITEM - T
//     Item from the vector at given index.
//
// Example config:
//   node {
//     calculator: "Get{SpecificType}VectorItemCalculator"
//     input_stream: "VECTOR:vector"
//     input_stream: "INDEX:index"
//     output_stream: "ITEM:item"
//     options {
//       [mediapipe.GetVectorItemCalculatorOptions.ext] {
//         item_index: 5
//       }
//     }
//   }
//
template <typename T>
class GetVectorItemCalculator : public Node {
 public:
  static constexpr Input<std::vector<T>> kIn{"VECTOR"};
  static constexpr Input<OneOf<int, uint64_t>>::Optional kIdx{"INDEX"};
  static constexpr Output<T> kOut{"ITEM"};

  MEDIAPIPE_NODE_CONTRACT(kIn, kIdx, kOut);

  absl::Status Open(CalculatorContext* cc) final {
    cc->SetOffset(mediapipe::TimestampDiff(0));
    auto& options = cc->Options<mediapipe::GetVectorItemCalculatorOptions>();
    RET_CHECK(kIdx(cc).IsConnected() || options.has_item_index());
    return absl::OkStatus();
  }

  absl::Status Process(CalculatorContext* cc) final {
    if (kIn(cc).IsEmpty()) {
      return absl::OkStatus();
    }

    const std::vector<T>& items = kIn(cc).Get();
    const auto& options =
        cc->Options<mediapipe::GetVectorItemCalculatorOptions>();

    int idx = 0;
    if (kIdx(cc).IsConnected() && !kIdx(cc).IsEmpty()) {
      idx = kIdx(cc).Visit(
          [](uint64_t idx_uint64_t) { return static_cast<int>(idx_uint64_t); },
          [](int idx_int) { return idx_int; });
    } else if (options.has_item_index()) {
      idx = options.item_index();
    } else {
      return absl::OkStatus();
    }

    RET_CHECK(idx >= 0);
    RET_CHECK(options.output_empty_on_oob() || idx < items.size());

    if (idx < items.size()) {
      kOut(cc).Send(items[idx]);
    }

    return absl::OkStatus();
  }
};

}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_CORE_GET_VECTOR_ITEM_CALCULATOR_H_
