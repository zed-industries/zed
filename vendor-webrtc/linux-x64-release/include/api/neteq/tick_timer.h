/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_NETEQ_TICK_TIMER_H_
#define API_NETEQ_TICK_TIMER_H_

#include <stdint.h>

#include <memory>

#include "rtc_base/checks.h"

namespace webrtc {

// Implements a time counter. The counter is advanced with the Increment()
// methods, and is queried with the ticks() accessor. It is assumed that one
// "tick" of the counter corresponds to 10 ms.
// A TickTimer object can provide two types of associated time-measuring
// objects: Stopwatch and Countdown.
class TickTimer {
 public:
  // Stopwatch measures time elapsed since it was started, by querying the
  // associated TickTimer for the current time. The intended use is to request a
  // new Stopwatch object from a TickTimer object with the GetNewStopwatch()
  // method. Note: since the Stopwatch object contains a reference to the
  // TickTimer it is associated with, it cannot outlive the TickTimer.
  class Stopwatch {
   public:
    explicit Stopwatch(const TickTimer& ticktimer);

    uint64_t ElapsedTicks() const { return ticktimer_.ticks() - starttick_; }

    uint64_t ElapsedMs() const {
      const uint64_t elapsed_ticks = ticktimer_.ticks() - starttick_;
      const int ms_per_tick = ticktimer_.ms_per_tick();
      return elapsed_ticks < UINT64_MAX / ms_per_tick
                 ? elapsed_ticks * ms_per_tick
                 : UINT64_MAX;
    }

   private:
    const TickTimer& ticktimer_;
    const uint64_t starttick_;
  };

  // Countdown counts down from a given start value with each tick of the
  // associated TickTimer, until zero is reached. The Finished() method will
  // return true if zero has been reached, false otherwise. The intended use is
  // to request a new Countdown object from a TickTimer object with the
  // GetNewCountdown() method. Note: since the Countdown object contains a
  // reference to the TickTimer it is associated with, it cannot outlive the
  // TickTimer.
  class Countdown {
   public:
    Countdown(const TickTimer& ticktimer, uint64_t ticks_to_count);

    ~Countdown();

    bool Finished() const {
      return stopwatch_->ElapsedTicks() >= ticks_to_count_;
    }

   private:
    const std::unique_ptr<Stopwatch> stopwatch_;
    const uint64_t ticks_to_count_;
  };

  TickTimer() : TickTimer(10) {}
  explicit TickTimer(int ms_per_tick) : ms_per_tick_(ms_per_tick) {
    RTC_DCHECK_GT(ms_per_tick_, 0);
  }

  TickTimer(const TickTimer&) = delete;
  TickTimer& operator=(const TickTimer&) = delete;

  void Increment() { ++ticks_; }

  // Mainly intended for testing.
  void Increment(uint64_t x) { ticks_ += x; }

  uint64_t ticks() const { return ticks_; }

  int ms_per_tick() const { return ms_per_tick_; }

  // Returns a new Stopwatch object, based on the current TickTimer. Note that
  // the new Stopwatch object contains a reference to the current TickTimer,
  // and must therefore not outlive the TickTimer.
  std::unique_ptr<Stopwatch> GetNewStopwatch() const {
    return std::unique_ptr<Stopwatch>(new Stopwatch(*this));
  }

  // Returns a new Countdown object, based on the current TickTimer. Note that
  // the new Countdown object contains a reference to the current TickTimer,
  // and must therefore not outlive the TickTimer.
  std::unique_ptr<Countdown> GetNewCountdown(uint64_t ticks_to_count) const {
    return std::unique_ptr<Countdown>(new Countdown(*this, ticks_to_count));
  }

 private:
  uint64_t ticks_ = 0;
  const int ms_per_tick_;
};

}  // namespace webrtc
#endif  // API_NETEQ_TICK_TIMER_H_
