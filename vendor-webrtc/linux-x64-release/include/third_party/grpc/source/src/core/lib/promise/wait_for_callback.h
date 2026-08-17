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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_WAIT_FOR_CALLBACK_H
#define GRPC_SRC_CORE_LIB_PROMISE_WAIT_FOR_CALLBACK_H

#include <grpc/support/port_platform.h>

#include <memory>
#include <utility>

#include "absl/base/thread_annotations.h"
#include "src/core/lib/promise/activity.h"
#include "src/core/lib/promise/poll.h"
#include "src/core/util/sync.h"

namespace grpc_core {

// Bridge callback interfaces and promise interfaces.
// This class helps bridge older callback interfaces with promises:
// MakeWaitPromise() returns a promise that will wait until a callback created
// by MakeCallback() has been invoked.
class WaitForCallback {
 public:
  // Creates a promise that blocks until the callback is invoked.
  auto MakeWaitPromise() {
    return [state = state_]() -> Poll<Empty> {
      MutexLock lock(&state->mutex);
      if (state->done) return Empty{};
      state->waker = GetContext<Activity>()->MakeNonOwningWaker();
      return Pending{};
    };
  }

  // Creates a callback that unblocks the promise.
  auto MakeCallback() {
    return [state = state_]() {
      ReleasableMutexLock lock(&state->mutex);
      state->done = true;
      auto waker = std::move(state->waker);
      lock.Release();
      waker.Wakeup();
    };
  }

 private:
  struct State {
    Mutex mutex;
    bool done ABSL_GUARDED_BY(mutex) = false;
    Waker waker ABSL_GUARDED_BY(mutex);
  };
  const std::shared_ptr<State> state_{std::make_shared<State>()};
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_WAIT_FOR_CALLBACK_H
