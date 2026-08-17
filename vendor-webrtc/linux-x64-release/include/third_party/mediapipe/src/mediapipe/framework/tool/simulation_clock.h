// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_FRAMEWORK_TOOL_SIMULATION_CLOCK_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_SIMULATION_CLOCK_H_

#include <map>
#include <set>

#include "absl/base/thread_annotations.h"
#include "absl/synchronization/mutex.h"
#include "absl/time/time.h"
#include "mediapipe/framework/deps/clock.h"

namespace mediapipe {

// A Clock that simulates the passage of time as quickly as possible.
// Parallel threads can call Sleep() and be woken in the proper order.
// Simulated time can be advanced by calling SleepUntil for the desired
// clock time.  Clock time does not advance until all woken threads have
// finished or gone back to sleep.
//
// In case multiple threads are scheduled to wake at the same time, they
// wake in the order in which they called Sleep().  No thread is permitted
// to continue until all earlier threads have finished or entered Sleep.
// The result is a single well-defined order of events.  Any desired
// order of events can be defined by adjusting the precise sleep times.
class SimulationClock : public mediapipe::Clock {
 public:
  SimulationClock() {}
  ~SimulationClock() override;

  // Returns the simulated time.
  absl::Time TimeNow() override;

  // Sleeps until the specified duration has elapsed according to this clock.
  void Sleep(absl::Duration d) override;

  // Sleeps until the specifed wakeup_time.
  void SleepUntil(absl::Time wakeup_time) override;

  // Informs this clock that a woken thread has started running.
  void ThreadStart();

  // Informs this clock that a woken thread has finished running.
  void ThreadFinish();

 protected:
  // Queue up wake up waiter.
  void SleepInternal(absl::Time wakeup_time)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(time_mutex_);
  // Advances to the next wake up time if no related threads are running.
  void TryAdvanceTime() ABSL_EXCLUSIVE_LOCKS_REQUIRED(time_mutex_);

  // Represents a thread blocked in SleepUntil.
  struct Waiter {
    bool sleeping = true;
    absl::CondVar cond;
  };

 protected:
  absl::Mutex time_mutex_;
  absl::Time time_ ABSL_GUARDED_BY(time_mutex_);
  std::multimap<absl::Time, Waiter*> waiters_ ABSL_GUARDED_BY(time_mutex_);
  int num_running_ ABSL_GUARDED_BY(time_mutex_) = 0;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_SIMULATION_CLOCK_H_
