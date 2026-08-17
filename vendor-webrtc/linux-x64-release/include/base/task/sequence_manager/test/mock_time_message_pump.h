// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_TEST_MOCK_TIME_MESSAGE_PUMP_H_
#define BASE_TASK_SEQUENCE_MANAGER_TEST_MOCK_TIME_MESSAGE_PUMP_H_

#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/message_loop/message_pump.h"
#include "base/synchronization/waitable_event.h"
#include "base/time/time.h"

namespace base {

class SimpleTestTickClock;

namespace sequence_manager {

// MessagePump implementation that uses a SimpleTestTickClock to keep track of
// time and will advance it as needed to keep running tasks.
//
// This pump will actually check fail if it ever has to go to sleep as this
// would indicate that the unit test might block indefinitely.
// TODO(carlscab): In the future we could consider sleeping if there is no
// outstanding |delayed_work_time_|, because we could be woken up by concurrent
// ScheduleWork() calls.
class MockTimeMessagePump : public MessagePump {
 public:
  explicit MockTimeMessagePump(SimpleTestTickClock* clock);
  ~MockTimeMessagePump() override;

  // MessagePump implementation
  void Run(Delegate* delegate) override;
  void Quit() override;
  void ScheduleWork() override;
  void ScheduleDelayedWork(
      const Delegate::NextWorkInfo& next_work_info) override;

  // Returns the time at which the pump would have to wake up to be perform
  // work.
  TimeTicks next_wake_up_time() const { return next_wake_up_time_; }

  // Quits after the first call to Delegate::DoWork(). Useful
  // for tests that want to make sure certain things happen during a DoWork
  // call.
  void SetQuitAfterDoWork(bool quit_after_do_some_work) {
    quit_after_do_some_work_ = quit_after_do_some_work;
  }

  // Allows this instance to advance the SimpleTestTickClock up to but not over
  // |advance_until| when idle (i.e. when a regular pump would go to sleep).
  // The clock will allways be advanced to |advance_until|, even if there are no
  // tasks requiring it (i.e. delayed tasks to be run after
  // |advance_until|) except for a value of TimeTicks::Max() which will advance
  // the clock as long as there is pending delayed work.
  void SetAllowTimeToAutoAdvanceUntil(TimeTicks advance_until) {
    allow_advance_until_ = advance_until;
  }

  // Quit when this pump's Delegate is out of work (i.e. when a regular pump
  // would go to sleep) and we are not allowed to advance the clock anymore.
  void SetStopWhenMessagePumpIsIdle(bool stop_when_message_pump_is_idle) {
    stop_when_message_pump_is_idle_ = stop_when_message_pump_is_idle;
  }

 private:
  // Returns true if the clock was indeed advanced and thus we should attempt
  // another iteration of the DoWork-DoIdleWork-loop.
  bool MaybeAdvanceTime(TimeTicks target_time);

  const raw_ptr<SimpleTestTickClock> clock_;
  // This flag is set to false when Run should return.
  bool keep_running_ = true;

  bool stop_when_message_pump_is_idle_ = false;
  bool quit_after_do_some_work_ = false;

  TimeTicks next_wake_up_time_{TimeTicks::Max()};

  TimeTicks allow_advance_until_ = TimeTicks::Min();
};

}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_TEST_MOCK_TIME_MESSAGE_PUMP_H_
