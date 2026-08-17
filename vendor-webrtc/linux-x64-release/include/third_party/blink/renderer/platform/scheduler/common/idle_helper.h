// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_IDLE_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_IDLE_HELPER_H_

#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/task_observer.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/common/cancelable_closure_holder.h"
#include "third_party/blink/renderer/platform/scheduler/common/scheduler_helper.h"
#include "third_party/blink/renderer/platform/scheduler/common/single_thread_idle_task_runner.h"

namespace blink {
namespace scheduler {
namespace idle_helper_unittest {
class BaseIdleHelperTest;
class IdleHelperTest;
}  // namespace idle_helper_unittest

class SchedulerHelper;

// The job of the IdleHelper is to run idle tasks when the system is otherwise
// idle. Idle tasks should be optional work, with no guarantee they will be run
// at all. Idle tasks are subject to three levels of throttling:
//
//   1. Both idle queues are run a BEST_EFFORT priority (i.e. only selected if
//      there is nothing else to do.
//   2. The idle queues are only enabled during an idle period.
//   3. Idle tasks posted from within an idle task run in the next idle period.
//      This is achieved by inserting a fence into the queue.
//
// There are two types of idle periods:
//   1. Short idle period - typically less than 10ms run after begin main frame
//      has finished, with the idle period ending at the compositor provided
//      deadline.
//   2. Long idle periods - typically up to 50ms when no frames are being
//      produced.
//
// Idle tasks are supplied a deadline, and should endeavor to finished before it
// ends to avoid jank.
class PLATFORM_EXPORT IdleHelper : public base::TaskObserver,
                                   public SingleThreadIdleTaskRunner::Delegate {
 public:
  // Used to by scheduler implementations to customize idle behaviour.
  class PLATFORM_EXPORT Delegate {
   public:
    Delegate();
    Delegate(const Delegate&) = delete;
    Delegate& operator=(const Delegate&) = delete;
    virtual ~Delegate();

    // If it's ok to enter a long idle period, return true.  Otherwise return
    // false and set next_long_idle_period_delay_out so we know when to try
    // again.
    virtual bool CanEnterLongIdlePeriod(
        base::TimeTicks now,
        base::TimeDelta* next_long_idle_period_delay_out) = 0;

    // Signals that the Long Idle Period hasn't started yet because the system
    // isn't quiescent.
    virtual void IsNotQuiescent() = 0;

    // Signals that the task list has changed.
    virtual void OnPendingTasksChanged(bool has_tasks) = 0;
  };

  // The minimum duration of an idle period.
  static constexpr base::TimeDelta kMinimumIdlePeriodDuration =
      base::Milliseconds(1);

  // The maximum duration of an idle period.
  static constexpr base::TimeDelta kMaximumIdlePeriodDuration =
      base::Milliseconds(50);

  // The minimum delay to wait between retrying to initiate a long idle time.
  static constexpr base::TimeDelta kRetryEnableLongIdlePeriodDelay =
      base::Milliseconds(1);

  // |helper|, |delegate|, and |idle_queue| are not owned by IdleHelper object
  // and must outlive it.
  IdleHelper(
      SchedulerHelper* helper,
      Delegate* delegate,
      const char* idle_period_tracing_name,
      base::TimeDelta required_quiescence_duration_before_long_idle_period,
      base::sequence_manager::TaskQueue* idle_queue);
  IdleHelper(const IdleHelper&) = delete;
  IdleHelper& operator=(const IdleHelper&) = delete;
  ~IdleHelper() override;

  // Prevents any further idle tasks from running.
  void Shutdown();

  // Releases memory associated with cancelled idle tasks (best effort).
  void RemoveCancelledIdleTasks();

  // Returns the idle task runner. Tasks posted to this runner may be reordered
  // relative to other task types and may be starved for an arbitrarily long
  // time if no idle time is available.
  scoped_refptr<SingleThreadIdleTaskRunner> IdleTaskRunner();

  // Start a short idle period with a given idle period deadline. Unlike long
  // idle periods, a new idle period will not start after this idle period
  // finishes unless this method is called again or long idle periods are
  // enabled.
  void StartShortIdlePeriod(base::TimeTicks now,
                            base::TimeTicks idle_period_deadline);

  // If |required_quiescence_duration_before_long_idle_period_| is zero then
  // immediately initiate a long idle period, otherwise check if any tasks have
  // run recently and if so, check again after a delay of
  // |required_quiescence_duration_before_long_idle_period_|.
  // Calling this function will end any previous idle period immediately, and
  // potentially again later if
  // |required_quiescence_duration_before_long_idle_period_| is non-zero.
  // NOTE EndIdlePeriod will disable the long idle periods.
  void EnableLongIdlePeriod();

  // This will end an idle period either started with StartIdlePeriod or
  // EnableLongIdlePeriod.
  void EndIdlePeriod();

  bool IsInIdlePeriod() const;
  bool IsInLongIdlePeriod() const;

  // SingleThreadIdleTaskRunner::Delegate implementation:
  void OnIdleTaskPosted() override;
  base::TimeTicks WillProcessIdleTask() override;
  void DidProcessIdleTask() override;
  base::TimeTicks NowTicks() override;

  // base::TaskObserver implementation:
  void WillProcessTask(const base::PendingTask& pending_task,
                       bool was_blocked_or_low_priority) override;
  void DidProcessTask(const base::PendingTask& pending_task) override;

  const char* IdlePeriodStateForTracing() const;

  // Returns the deadline for the current idle task.
  base::TimeTicks CurrentIdleTaskDeadlineForTesting() const;

 private:
  friend class idle_helper_unittest::BaseIdleHelperTest;
  friend class idle_helper_unittest::IdleHelperTest;

  enum class IdlePeriodState {
    kNotInIdlePeriod,
    kInShortIdlePeriod,
    kInLongIdlePeriod,
    kInLongIdlePeriodPaused,
  };

  // Start an idle period with a given idle period deadline.
  void StartIdlePeriod(IdlePeriodState new_idle_period_state,
                       base::TimeTicks now,
                       base::TimeTicks idle_period_deadline);

  // Returns the new idle period state for the next long idle period. Fills in
  // |next_long_idle_period_delay_out| with the next time we should try to
  // initiate the next idle period.
  IdlePeriodState ComputeNewLongIdlePeriodState(
      const base::TimeTicks now,
      base::TimeDelta* next_long_idle_period_delay_out);

  bool ShouldWaitForQuiescence();
  void OnIdleTaskPostedOnMainThread();
  void UpdateLongIdlePeriodStateAfterIdleTask();

  void SetIdlePeriodState(IdlePeriodState new_state,
                          base::TimeTicks new_deadline,
                          base::TimeTicks optional_now);

  void TraceIdleIdleTaskStart();
  void TraceIdleIdleTaskEnd();
  void TraceEventIdlePeriodStateChange(IdlePeriodState new_state,
                                       bool new_running_idle_task,
                                       base::TimeTicks new_deadline,
                                       base::TimeTicks optional_now);

  static const char* IdlePeriodStateToString(IdlePeriodState state);

  raw_ptr<SchedulerHelper> helper_;  // NOT OWNED
  raw_ptr<Delegate> delegate_;       // NOT OWNED
  raw_ptr<base::sequence_manager::TaskQueue, DanglingUntriaged>
      idle_queue_;  // NOT OWNED
  scoped_refptr<SingleThreadIdleTaskRunner> idle_task_runner_;

  CancelableClosureHolder enable_next_long_idle_period_closure_;
  CancelableClosureHolder on_idle_task_posted_closure_;

  IdlePeriodState idle_period_state_ = IdlePeriodState::kNotInIdlePeriod;
  base::TimeTicks idle_period_deadline_;
  base::TimeTicks last_idle_task_trace_time_;
  bool idle_period_trace_event_started_ = false;
  bool running_idle_task_for_tracing_ = false;
  const char* idle_period_tracing_name_;
  const char* last_sub_trace_event_name_ = nullptr;

  base::TimeDelta required_quiescence_duration_before_long_idle_period_;
  bool is_shutdown_ = false;
  base::WeakPtrFactory<IdleHelper> weak_factory_{this};
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_IDLE_HELPER_H_
