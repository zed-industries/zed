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
#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_THREAD_POOL_THREAD_POOL_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_THREAD_POOL_THREAD_POOL_H
#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <memory>

#include "absl/functional/any_invocable.h"
#include "src/core/lib/event_engine/forkable.h"

namespace grpc_event_engine::experimental {

// Interface for all EventEngine ThreadPool implementations
class ThreadPool : public Forkable {
 public:
  // Asserts Quiesce was called.
  ~ThreadPool() override = default;
  // Shut down the pool, and wait for all threads to exit.
  // This method is safe to call from within a ThreadPool thread.
  virtual void Quiesce() = 0;
  // Run must not be called after Quiesce completes
  virtual void Run(absl::AnyInvocable<void()> callback) = 0;
  virtual void Run(EventEngine::Closure* closure) = 0;
};

// Creates a default thread pool.
std::shared_ptr<ThreadPool> MakeThreadPool(size_t reserve_threads);

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_THREAD_POOL_THREAD_POOL_H
