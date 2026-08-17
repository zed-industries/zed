// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_WEBRTC_OVERRIDES_LOW_PRECISION_TIMER_H_
#define THIRD_PARTY_BLINK_WEBRTC_OVERRIDES_LOW_PRECISION_TIMER_H_

#include "base/functional/callback.h"
#include "base/memory/ref_counted.h"
#include "base/memory/scoped_refptr.h"
#include "base/synchronization/lock.h"
#include "base/task/sequenced_task_runner.h"
#include "base/thread_annotations.h"
#include "base/time/time.h"
#include "third_party/webrtc/rtc_base/system/rtc_export.h"

namespace blink {

// Implements a low precision timer, expect it to fire up to ~16 ms late (plus
// any OS or workload related delays).
//
// This timer only fires on metronome ticks as specified by
// MetronomeSource::TimeSnappedToNextTick(). This allows running numerous timers
// without increasing the Idle Wake Ups frequency beyond the metronome tick
// frequency, i.e. when each tick is already scheduled adding a
// LowPrecisionTimer should not add any Idle Wake Ups.
//
// TODO(https://crbug.com/1267874): Can this timer be moved to base/ or replaced
// by a low precision base/ timer without causing excessive Idle Wake Up
// frequencies (e.g. WebRTC use case with 50 incoming videos)?
class RTC_EXPORT LowPrecisionTimer final {
 public:
  LowPrecisionTimer(scoped_refptr<base::SequencedTaskRunner> task_runner,
                    base::RepeatingCallback<void()> callback);
  ~LowPrecisionTimer();
  // Must be called prior to destruction. Unregisters from the metronome
  // provider.
  void Shutdown();

  // Schedules to invoke the callback |delay| time from now.
  void StartOneShot(base::TimeDelta delay);
  // Scheduldes to repeat unconditionally until the timer is stopped. This has
  // the same behavior as calling StartOneShot(delay) inside each callback.
  void StartRepeating(base::TimeDelta delay);
  // True if there is currently activity scheduled.
  bool IsActive();
  // Cancels any scheduled callbacks.
  void Stop();

  // Change which task runner to fire callbacks on. Seamlessy re-schedules
  // pending callbacks. Must not be called from inside the callback.
  void MoveToNewTaskRunner(
      scoped_refptr<base::SequencedTaskRunner> task_runner);

 private:
  // Handles the scheduling and cancellation of a repeating callback.
  // The callback can be re-scheduled after it has fired, but all other settings
  // are "const". To change settings the SchedulableCallback has to be
  // inactivated and replaced by a new SchedulableCallback instance.
  class SchedulableCallback
      : public base::RefCountedThreadSafe<SchedulableCallback> {
   public:
    SchedulableCallback(scoped_refptr<base::SequencedTaskRunner> task_runner,
                        base::RepeatingCallback<void()> callback,
                        base::TimeDelta repeated_delay);
    ~SchedulableCallback();

    // The task can be re-scheduled after each run.
    void Schedule(base::TimeTicks scheduled_time);
    bool IsScheduled();
    // Inactivate the callback. Returns the cancelled scheduled time, or
    // base::TimeTicks::Max() if nothing was scheduled when cancelled.
    base::TimeTicks Inactivate();

   private:
    void MaybeRun();
    void RemoveMetronomeListener();

    const scoped_refptr<base::SequencedTaskRunner> task_runner_;
    const base::RepeatingCallback<void()> callback_;

    // Only accessed on |task_runner_|.
    bool is_currently_running_ = false;
    base::Lock active_lock_;
    // Guarded by |active_lock_|, but to avoid deadlock we do not acquire the
    // lock when Inactivate() is being called from inside the callback. In this
    // case the lock is already being held by MaybeRun().
    bool is_active_ = true;
    base::TimeDelta repeated_delay_;

    base::Lock scheduled_time_lock_;
    base::TimeTicks scheduled_time_ GUARDED_BY(scheduled_time_lock_) =
        base::TimeTicks::Max();  // Max represents forever, i.e. not scheduled.
  };

  // Lazy-constructs |schedulable_callback_| and schedules the callback on the
  // specified time.
  void ScheduleCallback(base::TimeTicks scheduled_time)
      EXCLUSIVE_LOCKS_REQUIRED(lock_);
  // Inactivates in-flight callbacks and re-schedulels the callback using the
  // latest settings. Used e.g. when we start or stop using the metronome.
  void RescheduleCallback() EXCLUSIVE_LOCKS_REQUIRED(lock_);

  const base::RepeatingCallback<void()> callback_;
  base::Lock lock_;
  bool is_shutdown_ GUARDED_BY(lock_) = false;
  scoped_refptr<base::SequencedTaskRunner> task_runner_ GUARDED_BY(lock_);
  scoped_refptr<SchedulableCallback> schedulable_callback_ GUARDED_BY(lock_);
  // If not repeating this is zero.
  base::TimeDelta repeated_delay_ GUARDED_BY(lock_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_WEBRTC_OVERRIDES_LOW_PRECISION_TIMER_H_
