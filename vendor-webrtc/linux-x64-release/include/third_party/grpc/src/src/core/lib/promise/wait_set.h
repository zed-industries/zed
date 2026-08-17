// Copyright 2021 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_WAIT_SET_H
#define GRPC_SRC_CORE_LIB_PROMISE_WAIT_SET_H

#include <grpc/support/port_platform.h>

#include <utility>

#include "absl/container/flat_hash_set.h"
#include "absl/hash/hash.h"
#include "src/core/lib/promise/activity.h"
#include "src/core/lib/promise/poll.h"

namespace grpc_core {

// Helper type that can be used to enqueue many Activities waiting for some
// external state.
// Typically the external state should be guarded by mu_, and a call to
// WakeAllAndUnlock should be made when the state changes.
// Promises should bottom out polling inside pending(), which will register for
// wakeup and return Pending().
// Queues handles to Activities, and not Activities themselves, meaning that if
// an Activity is destroyed prior to wakeup we end up holding only a small
// amount of memory (around 16 bytes + malloc overhead) until the next wakeup
// occurs.
class WaitSet final {
  using WakerSet = absl::flat_hash_set<Waker>;

 public:
  // Register for wakeup, return Pending(). If state is not ready to proceed,
  // Promises should bottom out here.
  Pending AddPending(Waker waker) {
    pending_.emplace(std::move(waker));
    return Pending();
  }

  class WakeupSet {
   public:
    void Wakeup() {
      while (!wakeup_.empty()) {
        wakeup_.extract(wakeup_.begin()).value().Wakeup();
      }
    }

   private:
    friend class WaitSet;
    explicit WakeupSet(WakerSet&& wakeup)
        : wakeup_(std::forward<WakerSet>(wakeup)) {}
    WakerSet wakeup_;
  };

  GRPC_MUST_USE_RESULT WakeupSet TakeWakeupSet() {
    auto ret = WakeupSet(std::move(pending_));
    pending_.clear();  // reinitialize after move.
    return ret;
  }

  void WakeupAsync() {
    while (!pending_.empty()) {
      pending_.extract(pending_.begin()).value().WakeupAsync();
    }
  }

  std::string ToString();

 private:
  // Handles to activities that need to be awoken.
  WakerSet pending_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_WAIT_SET_H
