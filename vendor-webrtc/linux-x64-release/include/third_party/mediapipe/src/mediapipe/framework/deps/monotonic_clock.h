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

#ifndef MEDIAPIPE_DEPS_MONOTONIC_CLOCK_H_
#define MEDIAPIPE_DEPS_MONOTONIC_CLOCK_H_

#include "absl/time/time.h"
#include "mediapipe/framework/deps/clock.h"

namespace mediapipe {

// MonotonicClock is an interface for a Clock that never goes backward.
// Successive returned values from Now() are guaranteed to be monotonically
// non-decreasing, although they may not be monotonic with respect to values
// returned from other instances of MonotonicClock.
//
// You can wrap any Clock object in a MonotonicClock using the
// CreateMonotonicClock() factory method, including Clock::RealClock().
// However, if you want a monotonic version of real time, it is strongly
// recommended that you use the CreateSynchronizedMonotonicClock() factory
// method, which wraps Clock::RealClock() and guarantees that values returned
// from Now() are monotonic ACROSS instances of the class that are created by
// CreateSynchronizedMonotonicClock().
//
// All methods support concurrent access.
class MonotonicClock : public Clock {
 public:
  // The MonotonicClock state, which may be shared between MonotonicClocks.
  struct State;

  ~MonotonicClock() override {}

  // The Clock interface (see util/time/clock.h).
  //
  // Return a monotonically non-decreasing time.
  absl::Time TimeNow() override = 0;
  // Sleep and SleepUntil guarantee only that the caller will sleep for at
  // least as long as specified in monotonic time.  The caller may sleep for
  // much longer (in monotonic time) if monotonic time jumps far into the
  // future.  Whether or not this happens depends on the behavior of the raw
  // clock.
  void Sleep(absl::Duration d) override = 0;
  void SleepUntil(absl::Time wakeup_time) override = 0;

  // Get metrics about time corrections.
  virtual void GetCorrectionMetrics(int* correction_count,
                                    double* max_correction) = 0;
  // Reset values returned by GetCorrectionMetrics().
  virtual void ResetCorrectionMetrics() = 0;

  // Factory methods.
  //
  // Create a MonotonicClock based on the given raw_clock.  This clock will
  // return monotonically non-decreasing values from Now(), but may not behave
  // monotonically with respect to other instances created by this function,
  // even if they are based on the same raw_clock.  Caller owns raw_clock.
  static MonotonicClock* CreateMonotonicClock(Clock* raw_clock);

  // Create an instance of MonotonicClock that is based on Clock::RealClock().
  // All such instance are synced with each other such that return values from
  // Now() are monotonic across instances.  This allows independently developed
  // code bases to have private instances of the synchronized MonotonicClock
  // and know that they will never see time anomalies when calling from one
  // code base to another.  Each instance can have its own correction callback.
  // Unlike Clock::RealClock(), caller owns this object and should delete it
  // when no longer needed.
  static MonotonicClock* CreateSynchronizedMonotonicClock();
};

class MonotonicClockTest;

// Provides access to MonotonicClock::State for unit-testing.
class MonotonicClockAccess {
 private:
  using State = MonotonicClock::State;

  // Reset internal global state.  Should only be called by test code.
  static void SynchronizedMonotonicClockReset();
  static State* CreateMonotonicClockState(Clock* raw_clock);
  static void DeleteMonotonicClockState(State* state);
  // Create a monotonic clock based on the given state.  Caller owns state
  // so that multiple such clocks can be created from the same state.
  static MonotonicClock* CreateMonotonicClock(State* state);
  friend class mediapipe::MonotonicClockTest;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_DEPS_MONOTONIC_CLOCK_H_
