/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_METRONOME_TEST_FAKE_METRONOME_H_
#define API_METRONOME_TEST_FAKE_METRONOME_H_

#include <cstddef>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "api/metronome/metronome.h"
#include "api/units/time_delta.h"

namespace webrtc::test {

// ForcedTickMetronome is a Metronome that ticks when `Tick()` is invoked.
// The constructor argument `tick_period` returned in `TickPeriod()`.
class ForcedTickMetronome : public Metronome {
 public:
  explicit ForcedTickMetronome(TimeDelta tick_period);

  // Forces all TickListeners to run `OnTick`.
  void Tick();
  size_t NumListeners();

  // Metronome implementation.
  void RequestCallOnNextTick(absl::AnyInvocable<void() &&> callback) override;
  TimeDelta TickPeriod() const override;

 private:
  const TimeDelta tick_period_;
  std::vector<absl::AnyInvocable<void() &&>> callbacks_;
};

// FakeMetronome is a metronome that ticks based on a repeating task at the
// `tick_period` provided in the constructor. It is designed for use with
// simulated task queues for unit tests.
class FakeMetronome : public Metronome {
 public:
  explicit FakeMetronome(TimeDelta tick_period);

  void SetTickPeriod(TimeDelta tick_period);

  // Metronome implementation.
  void RequestCallOnNextTick(absl::AnyInvocable<void() &&> callback) override;
  TimeDelta TickPeriod() const override;

 private:
  TimeDelta tick_period_;
  std::vector<absl::AnyInvocable<void() &&>> callbacks_;
};

}  // namespace webrtc::test

#endif  // API_METRONOME_TEST_FAKE_METRONOME_H_
