// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// A "timer" takes care of invoking a callback in the future, once or
// repeatedly. The callback is invoked:
// - OneShotTimer: Once after a `TimeDelta` delay has elapsed.
// - RetainingOneShotTimer: Same as OneShotTimer, but the callback is retained
//    after being executed, allowing another invocation to be scheduled with
//    Reset() without specifying the callback again.
// - DeadlineTimer: Once at the specified `TimeTicks` time.
// - RepeatingTimer: Repeatedly, with a specified `TimeDelta` delay before the
//    first invocation and between invocations.
// - MetronomeTimer: Repeatedly, with a specified `TimeDelta` delay between the
//    beginning of each invocations such that a constant phase is respected.
// (Retaining)OneShotTimer and RepeatingTimer automatically apply some leeway to
// the delay whereas DeadlineTimer and MetronomeTimer allow more control over
// the requested time. As a result, the former are generally more
// power-efficient.
// Prefer using (Retaining)OneShotTimer and RepeatingTimer because they
// automatically apply some leeway to the delay which enables power-efficient
// scheduling.

// Scheduled invocations can be cancelled with Stop() or by deleting the
// Timer. The latter makes it easy to ensure that an object is not accessed by a
// Timer after it has been deleted: just make the Timer a member of the object
// which receives Timer events (see example below).
//
// Sample RepeatingTimer usage:
//
//   class MyClass {
//    public:
//     void StartDoingStuff() {
//       timer_.Start(FROM_HERE, base::Seconds(1),
//                    this, &MyClass::DoStuff);
//       // Alternative form if the callback is not bound to `this` or
//       // requires arguments:
//       //    timer_.Start(FROM_HERE, base::Seconds(1),
//       //                 base::BindRepeating(&MyFunction, 42));
//     }
//     void StopDoingStuff() {
//       timer_.Stop();
//     }
//    private:
//     void DoStuff() {
//       // This method is called every second to do stuff.
//       ...
//     }
//     base::RepeatingTimer timer_;
//   };
//
// These APIs are not thread safe. When a method is called (except the
// constructor), all further method calls must be on the same sequence until
// Stop(). Once stopped, it may be destroyed or restarted on another sequence.
//
// By default, the scheduled tasks will be run on the same sequence that the
// Timer was *started on*. To mock time in unit tests, some old tests used
// SetTaskRunner() to schedule the delay on a test-controlled TaskRunner. The
// modern and preferred approach to mock time is to use TaskEnvironment's
// MOCK_TIME mode.

#ifndef BASE_TIMER_TIMER_H_
#define BASE_TIMER_TIMER_H_

// IMPORTANT: If you change timer code, make sure that all tests (including
// disabled ones) from timer_unittests.cc pass locally. Some are disabled
// because they're flaky on the buildbot, but when you run them locally you
// should be able to tell the difference.

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/functional/bind.h"
#include "base/functional/callback.h"
#include "base/functional/callback_helpers.h"
#include "base/location.h"
#include "base/memory/raw_ptr.h"
#include "base/sequence_checker.h"
#include "base/task/delayed_task_handle.h"
#include "base/task/sequenced_task_runner.h"
#include "base/time/time.h"
#include "base/types/strong_alias.h"

namespace base {

class TickClock;

namespace internal {

// This class wraps logic shared by all timers.
class BASE_EXPORT TimerBase {
 public:
  TimerBase(const TimerBase&) = delete;
  TimerBase& operator=(const TimerBase&) = delete;

  virtual ~TimerBase();

  // Returns true if the timer is running (i.e., not stopped).
  bool IsRunning() const;

  // Sets the task runner on which the delayed task should be scheduled when
  // this Timer is running. This method can only be called while this Timer
  // isn't running. If this is used to mock time in tests, the modern and
  // preferred approach is to use TaskEnvironment::TimeSource::MOCK_TIME. To
  // avoid racy usage of Timer, |task_runner| must run tasks on the same
  // sequence which this Timer is bound to (started from). TODO(gab): Migrate
  // callers using this as a test seam to
  // TaskEnvironment::TimeSource::MOCK_TIME.
  virtual void SetTaskRunner(scoped_refptr<SequencedTaskRunner> task_runner);

  // Call this method to stop the timer and cancel all previously scheduled
  // tasks. It is a no-op if the timer is not running.
  virtual void Stop();

 protected:
  // Constructs a timer. Start must be called later to set task info.
  explicit TimerBase(const Location& posted_from = Location());

  virtual void OnStop() = 0;

  // Disables the scheduled task and abandons it so that it no longer refers
  // back to this object.
  void AbandonScheduledTask();

  // Returns the task runner on which the task should be scheduled. If the
  // corresponding |task_runner_| field is null, the task runner for the current
  // sequence is returned.
  scoped_refptr<SequencedTaskRunner> GetTaskRunner();

  // The task runner on which the task should be scheduled. If it is null, the
  // task runner for the current sequence will be used.
  scoped_refptr<SequencedTaskRunner> task_runner_;

  // Timer isn't thread-safe and while it is running, it must only be used on
  // the same sequence until fully Stop()'ed. Once stopped, it may be destroyed
  // or restarted on another sequence.
  SEQUENCE_CHECKER(sequence_checker_);

  // Location in user code.
  Location posted_from_ GUARDED_BY_CONTEXT(sequence_checker_);

  // The handle to the posted delayed task.
  DelayedTaskHandle delayed_task_handle_ GUARDED_BY_CONTEXT(sequence_checker_);

  // Callback invoked when the timer is ready. This is saved as a member to
  // avoid rebinding every time the Timer fires. Lazy initialized the first time
  // the Timer is started.
  RepeatingClosure timer_callback_;
};

//-----------------------------------------------------------------------------
// This class wraps logic shared by (Retaining)OneShotTimer and RepeatingTimer.
class BASE_EXPORT DelayTimerBase : public TimerBase {
 public:
  DelayTimerBase(const DelayTimerBase&) = delete;
  DelayTimerBase& operator=(const DelayTimerBase&) = delete;

  ~DelayTimerBase() override;

  // Returns the current delay for this timer.
  TimeDelta GetCurrentDelay() const;

  // Call this method to reset the timer delay. The user task must be set. If
  // the timer is not running, this will start it by posting a task.
  virtual void Reset();

  TimeTicks desired_run_time() const {
    DCHECK_CALLED_ON_VALID_SEQUENCE(sequence_checker_);
    return desired_run_time_;
  }

 protected:
  // Constructs a timer. Start must be called later to set task info.
  // If |tick_clock| is provided, it is used instead of TimeTicks::Now() to get
  // TimeTicks when scheduling tasks.
  explicit DelayTimerBase(const TickClock* tick_clock = nullptr);

  // Construct a timer with task info.
  // If |tick_clock| is provided, it is used instead of TimeTicks::Now() to get
  // TimeTicks when scheduling tasks.
  DelayTimerBase(const Location& posted_from,
                 TimeDelta delay,
                 const TickClock* tick_clock = nullptr);

  virtual void RunUserTask() = 0;

  // Schedules |OnScheduledTaskInvoked()| to run on the current sequence with
  // the given |delay|. |desired_run_time_| is reset to Now() + delay.
  void ScheduleNewTask(TimeDelta delay);

  void StartInternal(const Location& posted_from, TimeDelta delay);

 private:
  // DCHECKs that the user task is not null. Used to diagnose a recurring bug
  // where Reset() is called on a OneShotTimer that has already fired.
  virtual void EnsureNonNullUserTask() = 0;

  // Returns the current tick count.
  TimeTicks Now() const;

  // Called when the scheduled task is invoked. Will run the  |user_task| if the
  // timer is still running and |desired_run_time_| was reached.
  void OnScheduledTaskInvoked();

  // Delay requested by user.
  TimeDelta delay_ GUARDED_BY_CONTEXT(sequence_checker_);

  // The desired run time of |user_task_|. The user may update this at any time,
  // even if their previous request has not run yet. This time can be a "zero"
  // TimeTicks if the task must be run immediately.
  TimeTicks desired_run_time_ GUARDED_BY_CONTEXT(sequence_checker_);

  // The tick clock used to calculate the run time for scheduled tasks.
  const raw_ptr<const TickClock> tick_clock_
      GUARDED_BY_CONTEXT(sequence_checker_);
};

}  // namespace internal

//-----------------------------------------------------------------------------
// A simple, one-shot timer.  See usage notes at the top of the file.
class BASE_EXPORT OneShotTimer : public internal::DelayTimerBase {
 public:
  OneShotTimer();
  explicit OneShotTimer(const TickClock* tick_clock);

  OneShotTimer(const OneShotTimer&) = delete;
  OneShotTimer& operator=(const OneShotTimer&) = delete;

  ~OneShotTimer() override;

  // Start the timer to run at the given |delay| from now. If the timer is
  // already running, it will be replaced to call the given |user_task|.
  virtual void Start(const Location& posted_from,
                     TimeDelta delay,
                     OnceClosure user_task);

  // Start the timer to run at the given |delay| from now. If the timer is
  // already running, it will be replaced to call a task formed from
  // |receiver->*method|.
  template <class Receiver>
  void Start(const Location& posted_from,
             TimeDelta delay,
             Receiver* receiver,
             void (Receiver::*method)()) {
    // Explicitly qualify calls, in case this is used inside Blink (which has
    // similar methods in WTF).
    Start(posted_from, delay,
          ::base::BindOnce(method, ::base::Unretained(receiver)));
  }

  // Run the scheduled task immediately, and stop the timer. The timer needs to
  // be running.
  virtual void FireNow();

 private:
  void OnStop() final;
  void RunUserTask() final;
  void EnsureNonNullUserTask() final;

  OnceClosure user_task_;
};

//-----------------------------------------------------------------------------
// A simple, repeating timer.  See usage notes at the top of the file.
class BASE_EXPORT RepeatingTimer : public internal::DelayTimerBase {
 public:
  RepeatingTimer();
  explicit RepeatingTimer(const TickClock* tick_clock);

  RepeatingTimer(const RepeatingTimer&) = delete;
  RepeatingTimer& operator=(const RepeatingTimer&) = delete;

  ~RepeatingTimer() override;

  RepeatingTimer(const Location& posted_from,
                 TimeDelta delay,
                 RepeatingClosure user_task);
  RepeatingTimer(const Location& posted_from,
                 TimeDelta delay,
                 RepeatingClosure user_task,
                 const TickClock* tick_clock);

  // Start the timer to run at the given |delay| from now. If the timer is
  // already running, it will be replaced to call the given |user_task|.
  virtual void Start(const Location& posted_from,
                     TimeDelta delay,
                     RepeatingClosure user_task);

  // Start the timer to run at the given |delay| from now. If the timer is
  // already running, it will be replaced to call a task formed from
  // |receiver->*method|.
  template <class Receiver>
  void Start(const Location& posted_from,
             TimeDelta delay,
             Receiver* receiver,
             void (Receiver::*method)()) {
    Start(posted_from, delay, BindRepeating(method, Unretained(receiver)));
  }

  const RepeatingClosure& user_task() const LIFETIME_BOUND {
    return user_task_;
  }

 private:
  // Mark this final, so that the destructor can call this safely.
  void OnStop() final;
  void RunUserTask() override;
  void EnsureNonNullUserTask() final;

  RepeatingClosure user_task_;
};

//-----------------------------------------------------------------------------
// A simple, one-shot timer with the retained |user_task| which is reused when
// Reset() is invoked. See usage notes at the top of the file.
class BASE_EXPORT RetainingOneShotTimer : public internal::DelayTimerBase {
 public:
  RetainingOneShotTimer();
  explicit RetainingOneShotTimer(const TickClock* tick_clock);

  RetainingOneShotTimer(const RetainingOneShotTimer&) = delete;
  RetainingOneShotTimer& operator=(const RetainingOneShotTimer&) = delete;

  ~RetainingOneShotTimer() override;

  RetainingOneShotTimer(const Location& posted_from,
                        TimeDelta delay,
                        RepeatingClosure user_task);
  RetainingOneShotTimer(const Location& posted_from,
                        TimeDelta delay,
                        RepeatingClosure user_task,
                        const TickClock* tick_clock);

  // Start the timer to run at the given |delay| from now. If the timer is
  // already running, it will be replaced to call the given |user_task|.
  virtual void Start(const Location& posted_from,
                     TimeDelta delay,
                     RepeatingClosure user_task);

  // Start the timer to run at the given |delay| from now. If the timer is
  // already running, it will be replaced to call a task formed from
  // |receiver->*method|.
  template <class Receiver>
  void Start(const Location& posted_from,
             TimeDelta delay,
             Receiver* receiver,
             void (Receiver::*method)()) {
    Start(posted_from, delay, BindRepeating(method, Unretained(receiver)));
  }

  const RepeatingClosure& user_task() const LIFETIME_BOUND {
    return user_task_;
  }

 private:
  // Mark this final, so that the destructor can call this safely.
  void OnStop() final;
  void RunUserTask() override;
  void EnsureNonNullUserTask() final;

  RepeatingClosure user_task_;
};

//-----------------------------------------------------------------------------
// A Delay timer is like The Button from Lost. Once started, you have to keep
// calling Reset otherwise it will call the given method on the sequence it was
// initially Reset() from.
//
// Once created, it is inactive until Reset is called. Once |delay| seconds have
// passed since the last call to Reset, the callback is made. Once the callback
// has been made, it's inactive until Reset is called again.
//
// If destroyed, the timeout is canceled and will not occur even if already
// inflight.
class DelayTimer {
 public:
  template <class Receiver>
  DelayTimer(const Location& posted_from,
             TimeDelta delay,
             Receiver* receiver,
             void (Receiver::*method)())
      : DelayTimer(posted_from, delay, receiver, method, nullptr) {}

  template <class Receiver>
  DelayTimer(const Location& posted_from,
             TimeDelta delay,
             Receiver* receiver,
             void (Receiver::*method)(),
             const TickClock* tick_clock)
      : timer_(posted_from,
               delay,
               BindRepeating(method, Unretained(receiver)),
               tick_clock) {}

  DelayTimer(const DelayTimer&) = delete;
  DelayTimer& operator=(const DelayTimer&) = delete;

  void Reset() { timer_.Reset(); }

 private:
  RetainingOneShotTimer timer_;
};

//-----------------------------------------------------------------------------
// A one-shot timer that attempts to run |user_task| some time near specified
// deadline. See usage notes at the top of the file.
class BASE_EXPORT DeadlineTimer : public internal::TimerBase {
 public:
  DeadlineTimer();
  ~DeadlineTimer() override;

  DeadlineTimer(const DeadlineTimer&) = delete;
  DeadlineTimer& operator=(const DeadlineTimer&) = delete;

  // Start the timer to run |user_task| near the specified |deadline| following
  // |delay_policy| If the timer is already running, it will be replaced to call
  // the given |user_task|.
  void Start(const Location& posted_from,
             TimeTicks deadline,
             OnceClosure user_task,
             subtle::DelayPolicy delay_policy =
                 subtle::DelayPolicy::kFlexiblePreferEarly);

  // Start the timer to run |user_task| near the specified |deadline|. If the
  // timer is already running, it will be replaced to call a task formed from
  // |receiver->*method|.
  template <class Receiver>
  void Start(const Location& posted_from,
             TimeTicks deadline,
             Receiver* receiver,
             void (Receiver::*method)(),
             subtle::DelayPolicy delay_policy =
                 subtle::DelayPolicy::kFlexiblePreferEarly) {
    Start(posted_from, deadline, BindOnce(method, Unretained(receiver)),
          delay_policy);
  }

 protected:
  void OnStop() override;

  // Schedules |OnScheduledTaskInvoked()| to run on the current sequence at
  // the given |deadline|.
  void ScheduleNewTask(TimeTicks deadline, subtle::DelayPolicy delay_policy);

 private:
  // Called when the scheduled task is invoked to run the |user_task|.
  void OnScheduledTaskInvoked();

  OnceClosure user_task_;
};

//-----------------------------------------------------------------------------
// Repeatedly invokes a callback, waiting for a precise delay between the
// beginning of each invocation. See usage notes at the top of the file.
class BASE_EXPORT MetronomeTimer : public internal::TimerBase {
 public:
  MetronomeTimer();
  ~MetronomeTimer() override;

  MetronomeTimer(const MetronomeTimer&) = delete;
  MetronomeTimer& operator=(const MetronomeTimer&) = delete;

  MetronomeTimer(const Location& posted_from,
                 TimeDelta interval,
                 RepeatingClosure user_task,
                 TimeTicks phase = TimeTicks());

  // Start the timer to repeatedly run |user_task| at the specified |interval|;
  // If not specified, the phase is up to the scheduler, otherwise each
  // invocation starts as close as possible to `phase + n * delay` for some
  // integer n. If the timer is already running, it will be replaced to call the
  // given |user_task|.
  void Start(const Location& posted_from,
             TimeDelta interval,
             RepeatingClosure user_task,
             TimeTicks phase = TimeTicks());

  // Same as the previous overload, except that the user task is specified by
  // `receiver` and `method`.
  template <class Receiver>
  void Start(const Location& posted_from,
             TimeDelta interval,
             Receiver* receiver,
             void (Receiver::*method)(),
             TimeTicks phase = TimeTicks()) {
    Start(posted_from, interval, BindRepeating(method, Unretained(receiver)),
          phase);
  }

  // Call this method to reset the timer delay. The user task must be set. If
  // the timer is not running, this will start it by posting a task.
  void Reset();

 protected:
  void OnStop() override;

  // Schedules |OnScheduledTaskInvoked()| to run on the current sequence at
  // the next tick.
  void ScheduleNewTask();

 private:
  // Called when the scheduled task is invoked to run the |user_task|.
  void OnScheduledTaskInvoked();

  TimeDelta interval_;
  RepeatingClosure user_task_;
  TimeTicks phase_;
};

}  // namespace base

#endif  // BASE_TIMER_TIMER_H_
