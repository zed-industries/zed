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

#ifndef GRPC_TEST_CORE_PROMISE_TEST_WAKEUP_SCHEDULERS_H
#define GRPC_TEST_CORE_PROMISE_TEST_WAKEUP_SCHEDULERS_H

#include <stdlib.h>

#include <functional>

#include "gmock/gmock.h"

namespace grpc_core {

// A wakeup scheduler that simply crashes.
// Useful for very limited tests.
struct NoWakeupScheduler {
  template <typename ActivityType>
  class BoundScheduler {
   public:
    explicit BoundScheduler(NoWakeupScheduler) {}
    void ScheduleWakeup() { abort(); }
  };
};

// A wakeup scheduler that simply runs the callback immediately.
// Useful for unit testing, probably not so much for real systems due to lock
// ordering problems.
struct InlineWakeupScheduler {
  template <typename ActivityType>
  class BoundScheduler {
   public:
    explicit BoundScheduler(InlineWakeupScheduler) {}
    void ScheduleWakeup() {
      static_cast<ActivityType*>(this)->RunScheduledWakeup();
    }
  };
};

// Mock for something that can schedule callbacks.
class MockCallbackScheduler {
 public:
  MOCK_METHOD(void, Schedule, (std::function<void()>));
};

// WakeupScheduler that schedules wakeups against a MockCallbackScheduler.
// Usage:
// TEST(..., ...) {
//   MockCallbackScheduler scheduler;
//   auto activity = MakeActivity(...,
//                                UseMockCallbackScheduler(&scheduler),
//                                ...);
struct UseMockCallbackScheduler {
  MockCallbackScheduler* scheduler;
  template <typename ActivityType>
  class BoundScheduler {
   public:
    explicit BoundScheduler(UseMockCallbackScheduler use_scheduler)
        : scheduler(use_scheduler.scheduler) {}
    void ScheduleWakeup() {
      scheduler->Schedule(
          [this] { static_cast<ActivityType*>(this)->RunScheduledWakeup(); });
    }

   private:
    MockCallbackScheduler* scheduler;
  };
};

}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_PROMISE_TEST_WAKEUP_SCHEDULERS_H
