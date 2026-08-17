// Copyright 2022 The gRPC Authors
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

#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_LOCKFREE_EVENT_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_LOCKFREE_EVENT_H
#include <grpc/support/port_platform.h>

#include <atomic>
#include <cstdint>

#include "absl/status/status.h"
#include "src/core/lib/event_engine/posix_engine/posix_engine_closure.h"

namespace grpc_event_engine::experimental {

class Scheduler;

class LockfreeEvent {
 public:
  explicit LockfreeEvent(Scheduler* scheduler) : scheduler_(scheduler) {}

  LockfreeEvent(const LockfreeEvent&) = delete;
  LockfreeEvent& operator=(const LockfreeEvent&) = delete;

  // These methods are used to initialize and destroy the internal state. These
  // cannot be done in constructor and destructor because SetReady may be called
  // when the event is destroyed and put in a freelist.
  void InitEvent();
  void DestroyEvent();

  // Returns true if fd has been shutdown, false otherwise.
  bool IsShutdown() const {
    return (state_.load(std::memory_order_relaxed) & kShutdownBit) != 0;
  }

  // Schedules \a closure when the event is received (see SetReady()) or the
  // shutdown state has been set. Note that the event may have already been
  // received, in which case the closure would be scheduled immediately.
  // If the shutdown state has already been set, then \a closure is scheduled
  // with the shutdown error.
  void NotifyOn(PosixEngineClosure* closure);

  // Sets the shutdown state. If a closure had been provided by NotifyOn and has
  // not yet been scheduled, it will be scheduled with \a shutdown_error.
  bool SetShutdown(absl::Status shutdown_error);

  // Signals that the event has been received.
  void SetReady();

 private:
  enum State { kClosureNotReady = 0, kClosureReady = 2, kShutdownBit = 1 };

  std::atomic<intptr_t> state_;
  Scheduler* scheduler_;
};

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_LOCKFREE_EVENT_H
