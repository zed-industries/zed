// Copyright 2023 gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_RANDOM_EARLY_DETECTION_H
#define GRPC_SRC_CORE_UTIL_RANDOM_EARLY_DETECTION_H

#include <grpc/support/port_platform.h>
#include <limits.h>

#include <cstdint>

#include "absl/random/bit_gen_ref.h"

namespace grpc_core {

// Implements the random early detection algorithm - allows items to be rejected
// or accepted based upon their size.
class RandomEarlyDetection {
 public:
  RandomEarlyDetection() : soft_limit_(INT_MAX), hard_limit_(INT_MAX) {}
  RandomEarlyDetection(uint64_t soft_limit, uint64_t hard_limit)
      : soft_limit_(soft_limit), hard_limit_(hard_limit) {}

  // Returns true if the size is greater than or equal to the hard limit - ie if
  // this item must be rejected.
  bool MustReject(uint64_t size) { return size >= hard_limit_; }

  // Returns true if the item should be rejected.
  bool Reject(uint64_t size, absl::BitGenRef bitsrc) const;

  uint64_t soft_limit() const { return soft_limit_; }
  uint64_t hard_limit() const { return hard_limit_; }

  void SetLimits(uint64_t soft_limit, uint64_t hard_limit) {
    soft_limit_ = soft_limit;
    hard_limit_ = hard_limit;
  }

 private:
  // The soft limit is the size at which we start rejecting items with a
  // probability that increases linearly to 1 as the size approaches the hard
  // limit.
  uint64_t soft_limit_;
  // The hard limit is the size at which we reject all items.
  uint64_t hard_limit_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_RANDOM_EARLY_DETECTION_H
