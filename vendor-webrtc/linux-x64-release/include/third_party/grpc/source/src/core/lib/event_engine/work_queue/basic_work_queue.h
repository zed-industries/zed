// Copyright 2023 The gRPC Authors
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
#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_WORK_QUEUE_BASIC_WORK_QUEUE_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_WORK_QUEUE_BASIC_WORK_QUEUE_H
#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <deque>

#include "absl/base/thread_annotations.h"
#include "absl/functional/any_invocable.h"
#include "src/core/lib/event_engine/work_queue/work_queue.h"
#include "src/core/util/sync.h"

namespace grpc_event_engine::experimental {

// A basic WorkQueue implementation that guards an std::deque with a Mutex
//
// Implementation note: q_.back is the most recent. q_.front is the oldest. New
// closures are added to the back.
class BasicWorkQueue : public WorkQueue {
 public:
  BasicWorkQueue() : owner_(nullptr) {}
  explicit BasicWorkQueue(void* owner);
  // Returns whether the queue is empty
  bool Empty() const override ABSL_LOCKS_EXCLUDED(mu_);
  // Returns the size of the queue.
  size_t Size() const override ABSL_LOCKS_EXCLUDED(mu_);
  // Returns the most recent element from the queue, or nullptr if either empty
  // or the queue is under contention. This is the fastest way to retrieve
  // elements from the queue.
  //
  // This method may return nullptr even if the queue is not empty.
  EventEngine::Closure* PopMostRecent() override ABSL_LOCKS_EXCLUDED(mu_);
  // Returns the most recent element from the queue, or nullptr if either empty
  // or the queue is under contention.
  // This is expected to be the slower of the two ways to retrieve closures from
  // the queue.
  //
  // This method may return nullptr even if the queue is not empty.
  EventEngine::Closure* PopOldest() override ABSL_LOCKS_EXCLUDED(mu_);
  // Adds a closure to the queue.
  void Add(EventEngine::Closure* closure) override ABSL_LOCKS_EXCLUDED(mu_);
  // Wraps an AnyInvocable and adds it to the the queue.
  void Add(absl::AnyInvocable<void()> invocable) override
      ABSL_LOCKS_EXCLUDED(mu_);
  const void* owner() override { return owner_; }

 private:
  mutable grpc_core::Mutex mu_;
  std::deque<EventEngine::Closure*> q_ ABSL_GUARDED_BY(mu_);
  const void* const owner_ = nullptr;
};

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_WORK_QUEUE_BASIC_WORK_QUEUE_H
