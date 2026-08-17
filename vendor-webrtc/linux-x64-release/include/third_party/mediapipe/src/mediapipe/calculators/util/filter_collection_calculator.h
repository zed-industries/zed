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

#ifndef MEDIAPIPE_CALCULATORS_UTIL_FILTER_VECTOR_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_UTIL_FILTER_VECTOR_CALCULATOR_H_

#include <vector>

#include "absl/strings/str_cat.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/port/canonical_errors.h"
#include "mediapipe/framework/port/ret_check.h"
#include "mediapipe/framework/port/status.h"

namespace mediapipe {

// A calculator that gates elements of an input collection based on
// corresponding boolean values of the "CONDITION" vector. If there is no input
// collection or "CONDITION" vector, the calculator forwards timestamp bounds
// for downstream calculators. If the "CONDITION" vector has false values for
// all elements of the input collection, the calculator outputs a packet
// containing an empty collection.
// Example usage:
// node {
//   calculator: "FilterCollectionCalculator"
//   input_stream: "ITERABLE:input_collection"
//   input_stream: "CONDITION:condition_vector"
//   output_stream: "ITERABLE:output_collection"
// }
// This calculator is able to handle collections of copyable types T.
template <typename IterableT>
class FilterCollectionCalculator : public CalculatorBase {
 public:
  static absl::Status GetContract(CalculatorContract* cc) {
    RET_CHECK(cc->Inputs().HasTag("ITERABLE"));
    RET_CHECK(cc->Inputs().HasTag("CONDITION"));
    RET_CHECK(cc->Outputs().HasTag("ITERABLE"));

    cc->Inputs().Tag("ITERABLE").Set<IterableT>();
    cc->Inputs().Tag("CONDITION").Set<std::vector<bool>>();

    cc->Outputs().Tag("ITERABLE").Set<IterableT>();

    return absl::OkStatus();
  }

  absl::Status Open(CalculatorContext* cc) override {
    cc->SetOffset(TimestampDiff(0));
    return absl::OkStatus();
  }

  absl::Status Process(CalculatorContext* cc) override {
    if (cc->Inputs().Tag("ITERABLE").IsEmpty()) {
      return absl::OkStatus();
    }
    if (cc->Inputs().Tag("CONDITION").IsEmpty()) {
      return absl::OkStatus();
    }

    const std::vector<bool>& filter_by =
        cc->Inputs().Tag("CONDITION").Get<std::vector<bool>>();

    return FilterCollection<IterableT>(
        std::is_copy_constructible<typename IterableT::value_type>(), cc,
        filter_by);
  }

  template <typename IterableU>
  absl::Status FilterCollection(std::true_type, CalculatorContext* cc,
                                const std::vector<bool>& filter_by) {
    const IterableU& input = cc->Inputs().Tag("ITERABLE").Get<IterableU>();
    if (input.size() != filter_by.size()) {
      return absl::InternalError(absl::StrCat(
          "Input vector size: ", input.size(),
          " doesn't mach condition vector size: ", filter_by.size()));
    }

    auto output = absl::make_unique<IterableU>();
    for (int i = 0; i < input.size(); ++i) {
      if (filter_by[i]) {
        output->push_back(input[i]);
      }
    }
    cc->Outputs().Tag("ITERABLE").Add(output.release(), cc->InputTimestamp());
    return absl::OkStatus();
  }

  template <typename IterableU>
  absl::Status FilterCollection(std::false_type, CalculatorContext* cc,
                                const std::vector<bool>& filter_by) {
    return absl::InternalError("Cannot copy input collection to filter it.");
  }
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_UTIL_FILTER_VECTOR_CALCULATOR_H_
