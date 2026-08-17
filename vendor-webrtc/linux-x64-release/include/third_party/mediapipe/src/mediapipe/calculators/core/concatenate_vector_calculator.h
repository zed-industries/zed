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

#ifndef MEDIAPIPE_CALCULATORS_CORE_CONCATENATE_VECTOR_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_CORE_CONCATENATE_VECTOR_CALCULATOR_H_

#include <string>
#include <type_traits>
#include <vector>

#include "mediapipe/calculators/core/concatenate_vector_calculator.pb.h"
#include "mediapipe/framework/api2/node.h"
#include "mediapipe/framework/api2/port.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/port/canonical_errors.h"
#include "mediapipe/framework/port/ret_check.h"
#include "mediapipe/framework/port/status.h"

namespace mediapipe {
// Note: since this is a calculator template that can be included by other
// source files, we do not place this in namespace api2 directly, but qualify
// the api2 names below, to avoid changing the visible name of the class.
// We cannot simply write "using mediapipe::api2" since it's a header file.
// This distinction will go away once api2 is finalized.

// Concatenates several objects of type T or std::vector<T> following stream
// index order. This class assumes that every input stream contains either T or
// vector<T> type. To use this class for a particular type T, regisiter a
// calculator using ConcatenateVectorCalculator<T>.
template <typename T>
class ConcatenateVectorCalculator : public api2::Node {
 public:
  static constexpr
      typename api2::Input<api2::OneOf<T, std::vector<T>>>::Multiple kIn{""};
  static constexpr api2::Output<std::vector<T>> kOut{""};

  MEDIAPIPE_NODE_CONTRACT(kIn, kOut);

  static absl::Status UpdateContract(CalculatorContract* cc) {
    RET_CHECK_GE(kIn(cc).Count(), 1);
    return absl::OkStatus();
  }

  absl::Status Open(CalculatorContext* cc) override {
    only_emit_if_all_present_ =
        cc->Options<::mediapipe::ConcatenateVectorCalculatorOptions>()
            .only_emit_if_all_present();
    return absl::OkStatus();
  }

  absl::Status Process(CalculatorContext* cc) override {
    if (only_emit_if_all_present_) {
      for (const auto& input : kIn(cc)) {
        if (input.IsEmpty()) return ::absl::OkStatus();
      }
    }
    return ConcatenateVectors<T>(std::is_copy_constructible<T>(), cc);
  }

  template <typename U>
  absl::Status ConcatenateVectors(std::true_type, CalculatorContext* cc) {
    auto output = std::vector<U>();
    for (const auto& input : kIn(cc)) {
      if (input.IsEmpty()) continue;
      input.Visit([&output](const U& value) { output.push_back(value); },
                  [&output](const std::vector<U>& value) {
                    output.insert(output.end(), value.begin(), value.end());
                  });
    }
    kOut(cc).Send(std::move(output));
    return absl::OkStatus();
  }

  template <typename U>
  absl::Status ConcatenateVectors(std::false_type, CalculatorContext* cc) {
    return ConsumeAndConcatenateVectors<T>(std::is_move_constructible<U>(), cc);
  }

  template <typename U>
  absl::Status ConsumeAndConcatenateVectors(std::true_type,
                                            CalculatorContext* cc) {
    auto output = std::vector<U>();
    for (auto input : kIn(cc)) {
      if (input.IsEmpty()) continue;
      MP_RETURN_IF_ERROR(input.ConsumeAndVisit(
          [&output](std::unique_ptr<U> value) {
            output.push_back(std::move(*value));
          },
          [&output](std::unique_ptr<std::vector<U>> value) {
            output.insert(output.end(), std::make_move_iterator(value->begin()),
                          std::make_move_iterator(value->end()));
          }));
    }
    kOut(cc).Send(std::move(output));
    return absl::OkStatus();
  }

  template <typename U>
  absl::Status ConsumeAndConcatenateVectors(std::false_type,
                                            CalculatorContext* cc) {
    return absl::InternalError(
        "Cannot copy or move inputs to concatenate them");
  }

 private:
  bool only_emit_if_all_present_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_CORE_CONCATENATE_VECTOR_CALCULATOR_H_
