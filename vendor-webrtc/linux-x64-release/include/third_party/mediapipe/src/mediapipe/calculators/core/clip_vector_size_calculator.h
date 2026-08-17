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

#ifndef MEDIAPIPE_CALCULATORS_CORE_CLIP_VECTOR_SIZE_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_CORE_CLIP_VECTOR_SIZE_CALCULATOR_H_

#include <type_traits>
#include <vector>

#include "mediapipe/calculators/core/clip_vector_size_calculator.pb.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/port/canonical_errors.h"
#include "mediapipe/framework/port/ret_check.h"
#include "mediapipe/framework/port/status.h"

namespace mediapipe {

// Clips the size of the input vector of type T to a specified max_vec_size.
// In a graph it will be used as:
// node {
//   calculator: "ClipIntVectorSizeCalculator"
//   input_stream: "input_vector"
//   output_stream: "output_vector"
//   options {
//     [mediapipe.ClipVectorSizeCalculatorOptions.ext] {
//       max_vec_size: 5
//     }
//   }
// }
// Optionally, you can pass in a side packet that will override `max_vec_size`
// that is specified in the options.
template <typename T>
class ClipVectorSizeCalculator : public CalculatorBase {
 public:
  static absl::Status GetContract(CalculatorContract* cc) {
    RET_CHECK(cc->Inputs().NumEntries() == 1);
    RET_CHECK(cc->Outputs().NumEntries() == 1);

    if (cc->Options<::mediapipe::ClipVectorSizeCalculatorOptions>()
            .max_vec_size() < 1) {
      return absl::InternalError(
          "max_vec_size should be greater than or equal to 1.");
    }

    cc->Inputs().Index(0).Set<std::vector<T>>();
    cc->Outputs().Index(0).Set<std::vector<T>>();
    // Optional input side packet that determines `max_vec_size`.
    if (cc->InputSidePackets().NumEntries() > 0) {
      cc->InputSidePackets().Index(0).Set<int>();
    }

    return absl::OkStatus();
  }

  absl::Status Open(CalculatorContext* cc) override {
    cc->SetOffset(TimestampDiff(0));
    max_vec_size_ = cc->Options<::mediapipe::ClipVectorSizeCalculatorOptions>()
                        .max_vec_size();
    // Override `max_vec_size` if passed as side packet.
    if (cc->InputSidePackets().NumEntries() > 0 &&
        !cc->InputSidePackets().Index(0).IsEmpty()) {
      max_vec_size_ = cc->InputSidePackets().Index(0).Get<int>();
    }
    return absl::OkStatus();
  }

  absl::Status Process(CalculatorContext* cc) override {
    if (max_vec_size_ < 1) {
      return absl::InternalError(
          "max_vec_size should be greater than or equal to 1.");
    }
    if (cc->Inputs().Index(0).IsEmpty()) {
      return absl::OkStatus();
    }

    return ClipVectorSize<T>(std::is_copy_constructible<T>(), cc);
  }

  template <typename U>
  absl::Status ClipVectorSize(std::true_type, CalculatorContext* cc) {
    auto output = absl::make_unique<std::vector<U>>();
    const std::vector<U>& input_vector =
        cc->Inputs().Index(0).Get<std::vector<U>>();
    if (max_vec_size_ >= input_vector.size()) {
      output->insert(output->end(), input_vector.begin(), input_vector.end());
    } else {
      for (int i = 0; i < max_vec_size_; ++i) {
        output->push_back(input_vector[i]);
      }
    }
    cc->Outputs().Index(0).Add(output.release(), cc->InputTimestamp());
    return absl::OkStatus();
  }

  template <typename U>
  absl::Status ClipVectorSize(std::false_type, CalculatorContext* cc) {
    return ConsumeAndClipVectorSize<T>(std::is_move_constructible<U>(), cc);
  }

  template <typename U>
  absl::Status ConsumeAndClipVectorSize(std::true_type, CalculatorContext* cc) {
    auto output = absl::make_unique<std::vector<U>>();
    absl::StatusOr<std::unique_ptr<std::vector<U>>> input_status =
        cc->Inputs().Index(0).Value().Consume<std::vector<U>>();

    if (input_status.ok()) {
      std::unique_ptr<std::vector<U>> input_vector =
          std::move(input_status).value();
      auto begin_it = input_vector->begin();
      auto end_it = input_vector->end();
      if (max_vec_size_ < input_vector->size()) {
        end_it = input_vector->begin() + max_vec_size_;
      }
      output->insert(output->end(), std::make_move_iterator(begin_it),
                     std::make_move_iterator(end_it));
    } else {
      return input_status.status();
    }
    cc->Outputs().Index(0).Add(output.release(), cc->InputTimestamp());
    return absl::OkStatus();
  }

  template <typename U>
  absl::Status ConsumeAndClipVectorSize(std::false_type,
                                        CalculatorContext* cc) {
    return absl::InternalError(
        "Cannot copy or move input vectors and clip their size.");
  }

 private:
  int max_vec_size_ = 0;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_CORE_CLIP_VECTOR_SIZE_CALCULATOR_H_
