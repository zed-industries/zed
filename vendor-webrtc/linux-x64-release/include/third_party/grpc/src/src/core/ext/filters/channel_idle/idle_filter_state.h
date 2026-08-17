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

#ifndef GRPC_SRC_CORE_EXT_FILTERS_CHANNEL_IDLE_IDLE_FILTER_STATE_H
#define GRPC_SRC_CORE_EXT_FILTERS_CHANNEL_IDLE_IDLE_FILTER_STATE_H

#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <atomic>

namespace grpc_core {

// State machine for the idle filter.
// Keeps track of how many calls are in progress, whether there is a timer
// started, and whether we've seen calls since the previous timer fired.
class IdleFilterState {
 public:
  explicit IdleFilterState(bool start_timer);
  ~IdleFilterState() = default;

  IdleFilterState(const IdleFilterState&) = delete;
  IdleFilterState& operator=(const IdleFilterState&) = delete;

  // Increment the number of calls in progress.
  void IncreaseCallCount();

  // Decrement the number of calls in progress.
  // Return true if we reached idle with no timer started.
  GRPC_MUST_USE_RESULT bool DecreaseCallCount();

  // Check if there's been any activity since the last timer check.
  // If there was, reset the activity flag and return true to indicated that
  // a new timer should be started.
  // If there was not, reset the timer flag and return false - in this case
  // we know that the channel is idle and has been for one full cycle.
  GRPC_MUST_USE_RESULT bool CheckTimer();

 private:
  // Bit in state_ indicating that the timer has been started.
  static constexpr uintptr_t kTimerStarted = 1;
  // Bit in state_ indicating that we've seen a call start or stop since the
  // last timer.
  static constexpr uintptr_t kCallsStartedSinceLastTimerCheck = 2;
  // How much should we shift to get the number of calls in progress.
  static constexpr uintptr_t kCallsInProgressShift = 2;
  // How much to increment/decrement the state_ when a call is started/stopped.
  // Ensures we don't clobber the preceding bits.
  static constexpr uintptr_t kCallIncrement = uintptr_t{1}
                                              << kCallsInProgressShift;
  std::atomic<uintptr_t> state_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_FILTERS_CHANNEL_IDLE_IDLE_FILTER_STATE_H
