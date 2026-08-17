//
// Copyright 2022 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_WEIGHTED_ROUND_ROBIN_STATIC_STRIDE_SCHEDULER_H
#define GRPC_SRC_CORE_LOAD_BALANCING_WEIGHTED_ROUND_ROBIN_STATIC_STRIDE_SCHEDULER_H

#include <grpc/support/port_platform.h>
#include <stddef.h>
#include <stdint.h>

#include <optional>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "absl/types/span.h"

namespace grpc_core {

// StaticStrideScheduler implements a stride scheduler without the ability to
// add, remove, or modify elements after construction. In exchange, not only is
// it cheaper to construct and batch-update weights than a traditional dynamic
// stride scheduler, it can also be used to make concurrent picks without any
// locking.
//
// Construction is O(|weights|).  Picking is O(1) if weights are similar, or
// O(|weights|) if the mean of the non-zero weights is a small fraction of the
// max. Stores two bytes per weight.
class StaticStrideScheduler final {
 public:
  // Constructs and returns a new StaticStrideScheduler, or nullopt if all
  // weights are zero or |weights| <= 1. All weights must be >=0.
  // `next_sequence_func` should return a rate monotonically increasing sequence
  // number, which may wrap. `float_weights` does not need to live beyond the
  // function. Caller is responsible for ensuring `next_sequence_func` remains
  // valid for all calls to `Pick()`.
  static std::optional<StaticStrideScheduler> Make(
      absl::Span<const float> float_weights,
      absl::AnyInvocable<uint32_t()> next_sequence_func);

  // Returns the index of the next pick. May invoke `next_sequence_func`
  // multiple times. The returned value is guaranteed to be in [0, |weights|).
  // Can be called concurrently iff `next_sequence_func` can.
  size_t Pick() const;

 private:
  StaticStrideScheduler(std::vector<uint16_t> weights,
                        absl::AnyInvocable<uint32_t()> next_sequence_func);

  mutable absl::AnyInvocable<uint32_t()> next_sequence_func_;

  // List of backend weights scaled such that the max(weights_) == kMaxWeight.
  std::vector<uint16_t> weights_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_WEIGHTED_ROUND_ROBIN_STATIC_STRIDE_SCHEDULER_H
