// Copyright 2024 The Centipede Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// `PeriodicAction` runs a separate thread that invokes a user-provided callback
// at the specified interval. The user can request an out-of-schedule invocation
// of the callback by "nudging" the action object.
//
// Example:
//   MyStats stats = ...;
//   PeriodicAction stats_logger{
//       [&stats]() { LOG(INFO) << "Current stats are: " << stats; },
//       {.delay = absl::Minutes(5), .interval = absl::Minutes(1)}
//   };
//   while (true) {
//     Foo();
//     Bar();
//     if (HaveUpdate()) {
//       stats_logger.Nudge();
//     }
//   }

#ifndef FUZZTEST_CENTIPEDE_PERIODIC_ACTION_H_
#define FUZZTEST_CENTIPEDE_PERIODIC_ACTION_H_

#include <cstdint>
#include <memory>

#include "absl/functional/any_invocable.h"
#include "absl/time/time.h"

namespace fuzztest::internal {

class PeriodicAction {
 public:
  struct Options {
    // The interval to sleep for before a given iteration. Iteration numbers are
    // 0-based.
    //
    // Thus, the interval before `iteration == 0` is the delay before the first
    // invocation of the action, the interval before `iteration == 1` is the
    // interval between the first and the second invocation, etc.
    //
    // This is a functor and not a fixed value to enable dynamic intervals (the
    // caller can use static functor state for that). Note that
    // `PeriodicAction::Nudge()` calls trigger out-of-schedule invocations and
    // count as iterations (therefore incrementing the internal iteration
    // counter and resetting the timer).
    //
    // If `sleep_before_each()` ever returns an `absl::InfiniteDuration()`, then
    // periodic action execution will be paused and resumed only by the next
    // `Nudge()` call.
    absl::AnyInvocable<absl::Duration(uint64_t iter_num)> sleep_before_each;
  };

  // Convenience factory methods for common options.
  static Options ConstDelayConstInterval(  //
      absl::Duration delay, absl::Duration interval) {
    return {
        [delay, interval](uint64_t i) { return i == 0 ? delay : interval; },
    };
  }
  static Options ZeroDelayZeroInterval() {
    return ConstDelayConstInterval(absl::ZeroDuration(), absl::ZeroDuration());
  }
  static Options ZeroDelayConstInterval(absl::Duration interval) {
    return ConstDelayConstInterval(absl::ZeroDuration(), interval);
  }
  static Options ConstDelayZeroInterval(absl::Duration delay) {
    return ConstDelayConstInterval(delay, absl::ZeroDuration());
  }

  PeriodicAction(absl::AnyInvocable<void()> action, Options options);

  // Movable, but not copyable.
  PeriodicAction(PeriodicAction&&);
  PeriodicAction& operator=(PeriodicAction&&);

  // Stops the periodic action via RAII. May block: waits for any currently
  // active invocation of the action to finish first before returning.
  ~PeriodicAction();

  // Stops the periodic action explicitly. May block: waits for any currently
  // active invocation of the action to finish first before returning.
  void Stop();
  // The same as `Stop()`, but returns immediately without waiting for any
  // currently active invocation to finish.
  void StopAsync();

  // Triggers an out-of-schedule invocation of the action and resets the
  // timer. If a previously scheduled or nudged invocation of the action is
  // currently active, it will be allowed to finish before the nudged one
  // starts. However, the `Nudge()` call itself returns immediately without
  // waiting for either one to finish.
  void Nudge();

 private:
  // Use the "pointer to implementation" idiom to make the class movable and
  // move-constructible.
  class Impl;
  std::unique_ptr<Impl> pimpl_;
};

}  // namespace fuzztest::internal

#endif  // FUZZTEST_CENTIPEDE_PERIODIC_ACTION_H_
