//
// Copyright 2019 gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_WORK_SERIALIZER_H
#define GRPC_SRC_CORE_UTIL_WORK_SERIALIZER_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>

#include <functional>
#include <memory>

#include "absl/base/thread_annotations.h"
#include "absl/functional/any_invocable.h"
#include "src/core/util/debug_location.h"
#include "src/core/util/orphanable.h"

namespace grpc_core {

// WorkSerializer is a mechanism to schedule callbacks in a synchronized manner.
// All callbacks scheduled on a WorkSerializer instance will be executed
// serially in a borrowed thread. The API provides a FIFO guarantee to the
// execution of callbacks scheduled on the thread.
// When a thread calls Run() with a callback the callback runs asynchronously.
class ABSL_LOCKABLE WorkSerializer {
 public:
  explicit WorkSerializer(
      std::shared_ptr<grpc_event_engine::experimental::EventEngine>
          event_engine);
  ~WorkSerializer();

  WorkSerializer(const WorkSerializer&) = delete;
  WorkSerializer& operator=(const WorkSerializer&) = delete;
  WorkSerializer(WorkSerializer&&) noexcept = default;
  WorkSerializer& operator=(WorkSerializer&&) noexcept = default;

  // Runs a given callback on the work serializer.
  //
  // The callback will be executed as an EventEngine callback, that then
  // arranges for the next callback in the queue to execute.
  //
  // If you want to use clang thread annotation to make sure that callback is
  // called by WorkSerializer only, you need to add the annotation to both the
  // lambda function given to Run and the actual callback function like;
  //
  //   void run_callback() {
  //     work_serializer.Run(
  //         []() ABSL_EXCLUSIVE_LOCKS_REQUIRED(work_serializer) {
  //            callback();
  //         }, DEBUG_LOCATION);
  //   }
  //   void callback() ABSL_EXCLUSIVE_LOCKS_REQUIRED(work_serializer) { ... }
  void Run(absl::AnyInvocable<void()> callback, DebugLocation location = {});

#ifndef NDEBUG
  // Returns true if the current thread is running in the WorkSerializer.
  bool RunningInWorkSerializer() const;
#endif

 private:
  class WorkSerializerImpl;

  OrphanablePtr<WorkSerializerImpl> impl_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_WORK_SERIALIZER_H
