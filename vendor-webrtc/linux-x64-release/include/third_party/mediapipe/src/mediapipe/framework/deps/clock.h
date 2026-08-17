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

#ifndef MEDIAPIPE_DEPS_CLOCK_H_
#define MEDIAPIPE_DEPS_CLOCK_H_

#include "absl/time/time.h"

namespace mediapipe {

// An abstract interface representing a Clock, which is an object that can
// tell you the current time, and sleep.
//
// This interface allows decoupling code that uses time from the code that
// creates a point in time.  You can use this to your advantage by injecting
// Clocks into interfaces rather than having implementations call absl::Now()
// directly.
//
// The Clock::RealClock() function returns a pointer (that you do not own)
// to the global realtime clock.
//
// Example:
//
//   bool IsWeekend(Clock* clock) {
//     absl::Time now = clock->TimeNow();
//     // ... code to check if 'now' is a weekend.
//   }
//
//   // Production code.
//   IsWeekend(Clock::RealClock());
//
//   // Test code:
//   MyTestClock test_clock(SATURDAY);
//   IsWeekend(&test_clock);
//
class Clock {
 public:
  // Returns a pointer to the global realtime clock.  The caller does not
  // own the returned pointer and should not delete it.  The returned clock
  // is thread-safe.
  static Clock* RealClock();

  virtual ~Clock();

  // Returns the current time.
  virtual absl::Time TimeNow() = 0;

  // Sleeps for the specified duration.
  virtual void Sleep(absl::Duration d) = 0;

  // Sleeps until the specified time.
  virtual void SleepUntil(absl::Time wakeup_time) = 0;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_DEPS_CLOCK_H_
