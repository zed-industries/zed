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

#ifndef GRPC_SRC_CORE_LIB_RESOURCE_QUOTA_PERIODIC_UPDATE_H
#define GRPC_SRC_CORE_LIB_RESOURCE_QUOTA_PERIODIC_UPDATE_H

#include <grpc/support/port_platform.h>
#include <inttypes.h>

#include <atomic>

#include "absl/functional/function_ref.h"
#include "src/core/util/time.h"

namespace grpc_core {

// Lightweight timer-like mechanism for periodic updates.
// Fast path only decrements an atomic int64.
// Slow path runs corrections and estimates how many ticks are required to hit
// the target period.
// This is super inaccurate of course, but for places where we can't run timers,
// or places where continuous registration/unregistration would cause problems
// it can be quite useful.
class PeriodicUpdate {
 public:
  explicit PeriodicUpdate(Duration period) : period_(period) {}

  // Tick the update, call f and return true if we think the period expired.
  bool Tick(absl::FunctionRef<void(Duration)> f) {
    // Atomically decrement the remaining ticks counter.
    // If we hit 0 our estimate of period length has expired.
    // See the comment next to the data members for a description of thread
    // safety.
    if (updates_remaining_.fetch_sub(1, std::memory_order_acquire) == 1) {
      return MaybeEndPeriod(f);
    }
    return false;
  }

 private:
  bool MaybeEndPeriod(absl::FunctionRef<void(Duration)> f);

  // Thread safety:
  // When updates_remaining_ reaches 0 the thread that decremented becomes
  // responsible for updating any mutable variables and then setting
  // updates_remaining_ to a value greater than zero.
  // Whilst in this state other threads *may* decrement updates_remaining_, but
  // this is fine because they'll observe an ignorable negative value.

  std::atomic<int64_t> updates_remaining_{1};
  const Duration period_;
  Timestamp period_start_ = Timestamp::ProcessEpoch();
  int64_t expected_updates_per_period_ = 1;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_RESOURCE_QUOTA_PERIODIC_UPDATE_H
