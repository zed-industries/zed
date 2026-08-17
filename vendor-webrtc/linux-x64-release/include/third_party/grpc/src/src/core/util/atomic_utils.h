//
//
// Copyright 2017 gRPC authors.
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
//

#ifndef GRPC_SRC_CORE_UTIL_ATOMIC_UTILS_H
#define GRPC_SRC_CORE_UTIL_ATOMIC_UTILS_H

#include <grpc/support/port_platform.h>

#include <atomic>

namespace grpc_core {

// Atomically increment a counter only if the counter value is not zero.
// Returns true if increment took place; false if counter is zero.
template <typename T>
inline bool IncrementIfNonzero(std::atomic<T>* p) {
  T count = p->load(std::memory_order_acquire);
  do {
    // If zero, we are done (without an increment). If not, we must do a CAS
    // to maintain the contract: do not increment the counter if it is already
    // zero
    if (count == 0) {
      return false;
    }
  } while (!p->compare_exchange_weak(
      count, count + 1, std::memory_order_acq_rel, std::memory_order_acquire));
  return true;
}

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_ATOMIC_UTILS_H
