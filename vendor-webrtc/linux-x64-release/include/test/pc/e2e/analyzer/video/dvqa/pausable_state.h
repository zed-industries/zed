/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_ANALYZER_VIDEO_DVQA_PAUSABLE_STATE_H_
#define TEST_PC_E2E_ANALYZER_VIDEO_DVQA_PAUSABLE_STATE_H_

#include <cstdint>
#include <vector>

#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "system_wrappers/include/clock.h"

namespace webrtc {

// Provides ability to pause and resume and tell at any point was state paused
// or active.
class PausableState {
 public:
  // Creates a state as active.
  explicit PausableState(Clock* clock) : clock_(clock) {}
  PausableState(const PausableState&) = delete;
  PausableState& operator=(const PausableState&) = delete;
  PausableState(PausableState&&) = default;
  PausableState& operator=(PausableState&&) = default;

  // Pauses current state. States MUST be active.
  //
  // Complexity: O(1)
  void Pause();

  // Activates current state. State MUST be paused.
  //
  // Complexity: O(1)
  void Resume();

  // Returns is state is paused right now.
  //
  // Complexity: O(1)
  bool IsPaused() const;

  // Returns if last event before `time` was "pause".
  //
  // Complexity: O(log(n))
  bool WasPausedAt(Timestamp time) const;

  // Returns if next event after `time` was "resume".
  //
  // Complexity: O(log(n))
  bool WasResumedAfter(Timestamp time) const;

  // Returns time of last event or plus infinity if no events happened.
  //
  // Complexity O(1)
  Timestamp GetLastEventTime() const;

  // Returns sum of durations during which state was active starting from
  // time `time`.
  //
  // Complexity O(n)
  TimeDelta GetActiveDurationFrom(Timestamp time) const;

 private:
  struct Event {
    Timestamp time;
    bool is_paused;
  };

  // Returns position in `events_` which has time:
  // 1. Most right of the equals
  // 2. The biggest which is smaller
  // 3. -1 otherwise (first time is bigger than `time`)
  int64_t GetPos(Timestamp time) const;

  Clock* clock_;

  std::vector<Event> events_;
};

}  // namespace webrtc

#endif  // TEST_PC_E2E_ANALYZER_VIDEO_DVQA_PAUSABLE_STATE_H_
