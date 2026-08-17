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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_EXEC_CTX_WAKEUP_SCHEDULER_H
#define GRPC_SRC_CORE_LIB_PROMISE_EXEC_CTX_WAKEUP_SCHEDULER_H

#include <grpc/support/port_platform.h>

#include "absl/status/status.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/exec_ctx.h"
#include "src/core/util/debug_location.h"

namespace grpc_core {

// A callback scheduler for activities that works by scheduling callbacks on the
// exec ctx.
class ExecCtxWakeupScheduler {
 public:
  template <typename ActivityType>
  class BoundScheduler {
   protected:
    explicit BoundScheduler(ExecCtxWakeupScheduler) {}
    BoundScheduler(const BoundScheduler&) = delete;
    BoundScheduler& operator=(const BoundScheduler&) = delete;
    void ScheduleWakeup() {
      GRPC_CLOSURE_INIT(
          &closure_,
          [](void* arg, grpc_error_handle) {
            static_cast<ActivityType*>(arg)->RunScheduledWakeup();
          },
          static_cast<ActivityType*>(this), nullptr);
      ExecCtx::Run(DEBUG_LOCATION, &closure_, absl::OkStatus());
    }

   private:
    grpc_closure closure_;
  };
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_EXEC_CTX_WAKEUP_SCHEDULER_H
