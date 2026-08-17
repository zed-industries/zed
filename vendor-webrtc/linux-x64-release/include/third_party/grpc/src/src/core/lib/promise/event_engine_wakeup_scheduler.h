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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_EVENT_ENGINE_WAKEUP_SCHEDULER_H
#define GRPC_SRC_CORE_LIB_PROMISE_EVENT_ENGINE_WAKEUP_SCHEDULER_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>

#include <memory>
#include <utility>

#include "absl/log/check.h"
#include "src/core/lib/iomgr/exec_ctx.h"

namespace grpc_core {

// A callback scheduler for activities that works by scheduling callbacks on the
// exec ctx.
class EventEngineWakeupScheduler {
 public:
  explicit EventEngineWakeupScheduler(
      std::shared_ptr<grpc_event_engine::experimental::EventEngine>
          event_engine)
      : event_engine_(std::move(event_engine)) {
    CHECK_NE(event_engine_, nullptr);
  }

  template <typename ActivityType>
  class BoundScheduler
      : public grpc_event_engine::experimental::EventEngine::Closure {
   protected:
    explicit BoundScheduler(EventEngineWakeupScheduler scheduler)
        : event_engine_(std::move(scheduler.event_engine_)) {}
    BoundScheduler(const BoundScheduler&) = delete;
    BoundScheduler& operator=(const BoundScheduler&) = delete;
    void ScheduleWakeup() { event_engine_->Run(this); }
    void Run() final {
      ExecCtx exec_ctx;
      static_cast<ActivityType*>(this)->RunScheduledWakeup();
    }

   private:
    std::shared_ptr<grpc_event_engine::experimental::EventEngine> event_engine_;
  };

 private:
  std::shared_ptr<grpc_event_engine::experimental::EventEngine> event_engine_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_EVENT_ENGINE_WAKEUP_SCHEDULER_H
