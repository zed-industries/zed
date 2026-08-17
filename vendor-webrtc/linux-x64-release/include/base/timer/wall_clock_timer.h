// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TIMER_WALL_CLOCK_TIMER_H_
#define BASE_TIMER_WALL_CLOCK_TIMER_H_

#include "base/base_export.h"
#include "base/functional/bind.h"
#include "base/functional/callback.h"
#include "base/location.h"
#include "base/memory/raw_ptr.h"
#include "base/power_monitor/power_observer.h"
#include "base/sequence_checker.h"
#include "base/time/time.h"
#include "base/timer/timer.h"

namespace base {

class Clock;
class TickClock;

// `WallClockTimer` is based on `OneShotTimer` and provides a similar API.
// Where `OneShotTimer` uses `TimeTicks`, however, `WallClockTimer` uses `Time`.
// On some platforms, `TimeTicks` freezes during suspend; `Time` does not.
// `WallClockTimer` recomputes the desired delay after resuming from suspend, so
// on these platforms, suspends will "delay" `OneShotTimer` but not
// `WallClockTimer`.
//
// This does not attempt to observe and accommodate other `Time` changes, e.g.
// `Time` moving backwards in response to a DST change. `WallClockTimer` will
// only notice such changes if the system is subsequently suspended (which will
// cause a recalculation on resume that will coincidentally take them into
// account).
//
// After construction, the timer will become bound to the first sequence any
// method is called on. All subsequent calls must happen on that sequence until
// the task runs or is canceled via `Stop()`, after which the timer may be
// destroyed or restarted on another sequence.
class BASE_EXPORT WallClockTimer : public PowerSuspendObserver {
 public:
  // Constructs a timer. Start() must be called later to start the timer.
  // If |clock| is provided, it's used instead of
  // DefaultClock::GetInstance() to calulate timer's delay. If |tick_clock|
  // is provided, it's used instead of TimeTicks::Now() to get TimeTicks when
  // scheduling tasks.
  WallClockTimer();
  WallClockTimer(const Clock* clock, const TickClock* tick_clock);
  WallClockTimer(const WallClockTimer&) = delete;
  WallClockTimer& operator=(const WallClockTimer&) = delete;
  ~WallClockTimer() override;

  // Starts the timer to run at the given |desired_run_time|. If the timer is
  // already running, it will be replaced to call the given |user_task|.
  virtual void Start(const Location& posted_from,
                     Time desired_run_time,
                     OnceClosure user_task);

  // Starts the timer to run at the given |desired_run_time|. If the timer is
  // already running, it will be replaced to call a task formed from
  // |receiver|->*|method|.
  template <class Receiver>
  void Start(const Location& posted_from,
             Time desired_run_time,
             Receiver* receiver,
             void (Receiver::*method)()) {
    Start(posted_from, desired_run_time,
          BindOnce(method, Unretained(receiver)));
  }

  // Stops the timer. No-op if the timer is not running.
  void Stop();

  // Returns whether the timer is running.
  bool IsRunning() const;

  // PowerSuspendObserver:
  void OnResume() override;

  Time desired_run_time() const {
    DCHECK_CALLED_ON_VALID_SEQUENCE(sequence_checker_);
    return desired_run_time_;
  }

 private:
  void RunUserTask();

  SEQUENCE_CHECKER(sequence_checker_);

  // Location in user code.
  Location posted_from_ GUARDED_BY_CONTEXT(sequence_checker_);

  // The desired run time of |user_task_|.
  Time desired_run_time_ GUARDED_BY_CONTEXT(sequence_checker_);

  OnceClosure user_task_ GUARDED_BY_CONTEXT(sequence_checker_);

  OneShotTimer timer_;

  // The clock used to calculate the run time for scheduled tasks.
  const raw_ptr<const Clock> clock_;
};

}  // namespace base

#endif  // BASE_TIMER_WALL_CLOCK_TIMER_H_
