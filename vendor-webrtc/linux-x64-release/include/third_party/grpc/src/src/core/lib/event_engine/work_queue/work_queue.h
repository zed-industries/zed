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
#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_WORK_QUEUE_WORK_QUEUE_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_WORK_QUEUE_WORK_QUEUE_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include "absl/functional/any_invocable.h"

namespace grpc_event_engine::experimental {

// An interface for thread-safe EventEngine callback work queues.
//
// Implementations should be optimized for LIFO operations using PopMostRecent.
// All methods must be guaranteed thread-safe.
class WorkQueue {
 public:
  virtual ~WorkQueue() = default;
  // Returns whether the queue is empty.
  virtual bool Empty() const = 0;
  // Returns the size of the queue.
  virtual size_t Size() const = 0;
  // Returns the most recent element from the queue. This is the fastest way to
  // retrieve elements from the queue.
  //
  // Implementations are permitted to return nullptr even if the queue is not
  // empty. This is to support potential optimizations.
  virtual EventEngine::Closure* PopMostRecent() = 0;
  // Returns the most recent element from the queue, or nullptr if either empty
  // or the queue is under contention.
  // This is expected to be the slower of the two ways to retrieve closures from
  // the queue.
  //
  // Implementations are permitted to return nullptr even if the queue is not
  // empty. This is to support potential optimizations.
  virtual EventEngine::Closure* PopOldest() = 0;
  // Adds a closure to the queue.
  virtual void Add(EventEngine::Closure* closure) = 0;
  // Wraps an AnyInvocable and adds it to the the queue.
  virtual void Add(absl::AnyInvocable<void()> invocable) = 0;
  // Returns an optional owner id for queue identification.
  // TODO(hork): revisit if this can be moved to the thread pool implementation
  // if dynamic queue type experiments are warranted.
  virtual const void* owner() = 0;
};

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_WORK_QUEUE_WORK_QUEUE_H
