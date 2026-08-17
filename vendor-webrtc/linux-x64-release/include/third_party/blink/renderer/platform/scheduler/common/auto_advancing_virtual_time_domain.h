// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_AUTO_ADVANCING_VIRTUAL_TIME_DOMAIN_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_AUTO_ADVANCING_VIRTUAL_TIME_DOMAIN_H_

#include "base/memory/raw_ptr.h"
#include "base/task/sequence_manager/time_domain.h"
#include "base/task/task_observer.h"
#include "base/time/tick_clock.h"
#include "base/time/time.h"
#include "base/time/time_override.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/common/process_time_override_coordinator.h"

namespace blink {
namespace scheduler {
class SchedulerHelper;

// A time domain that runs tasks sequentially in time order but doesn't sleep
// between delayed tasks. Because AutoAdvancingVirtualTimeDomain may override
// Time/TimeTicks in a multi-threaded context, it must outlive any thread that
// may call Time::Now() or TimeTicks::Now(). In practice, this means
// AutoAdvancingVirtualTimeDomain can never be destroyed in production and acts
// as a one-way switch. In tests, it should only be destroyed after all threads
// have been joined.
//
// KEY: A-E are delayed tasks
// |    A   B C  D           E  (Execution with RealTimeDomain)
// |-----------------------------> time
//
// |ABCDE                       (Execution with AutoAdvancingVirtualTimeDomain)
// |-----------------------------> time
class PLATFORM_EXPORT AutoAdvancingVirtualTimeDomain
    : public base::sequence_manager::TimeDomain,
      public base::TaskObserver {
 public:
  AutoAdvancingVirtualTimeDomain(base::Time initial_time,
                                 base::TimeTicks initial_time_ticks,
                                 SchedulerHelper* helper);
  AutoAdvancingVirtualTimeDomain(const AutoAdvancingVirtualTimeDomain&) =
      delete;
  AutoAdvancingVirtualTimeDomain& operator=(
      const AutoAdvancingVirtualTimeDomain&) = delete;
  ~AutoAdvancingVirtualTimeDomain() override;

  // Controls whether or not virtual time is allowed to advance, when the
  // SequenceManager runs out of immediate work to do.
  void SetCanAdvanceVirtualTime(bool can_advance_virtual_time);

  // If non-null, virtual time may not advance past |virtual_time_fence|.
  void SetVirtualTimeFence(base::TimeTicks virtual_time_fence);

  // The maximum number of tasks we will run before advancing virtual time in
  // order to avoid starving delayed tasks. NB a value of 0 allows infinite
  // starvation. A reasonable value for this in practice is around 1000 tasks,
  // which should only affect rendering of the heaviest pages.
  void SetMaxVirtualTimeTaskStarvationCount(int max_task_starvation_count);

  // Updates to min(NextDelayedTaskTime, |new_virtual_time|) if thats ahead of
  // the current virtual time.  Returns true if time was advanced.
  bool MaybeAdvanceVirtualTime(base::TimeTicks new_virtual_time);

  // base::PendingTask implementation:
  void WillProcessTask(const base::PendingTask& pending_task,
                       bool was_blocked_or_low_priority) override;
  void DidProcessTask(const base::PendingTask& pending_task) override;

  int task_starvation_count() const { return task_starvation_count_; }

  base::TimeTicks InitialTicks() const { return initial_time_ticks_; }

 private:
  // TickClock implementation:
  base::TimeTicks NowTicks() const override;

  // TimeDomain implementation:
  bool MaybeFastForwardToWakeUp(
      std::optional<base::sequence_manager::WakeUp> wakeup,
      bool quit_when_idle_requested) override;
  const char* GetName() const override;

  // The number of tasks that have been run since the last time VirtualTime
  // advanced. Used to detect excessive starvation of delayed tasks.
  int task_starvation_count_;

  // The maximum number amount of delayed task starvation we will allow.
  // NB a value of 0 allows infinite starvation.
  int max_task_starvation_count_;

  bool can_advance_virtual_time_;
  raw_ptr<SchedulerHelper> helper_;  // NOT OWNED

  // VirtualTime is usually doled out in 100ms intervals using fences and this
  // variable let us honor a request to MaybeAdvanceVirtualTime that straddles
  // one of these boundaries.
  base::TimeTicks requested_next_virtual_time_;

  // Upper limit on how far virtual time is allowed to advance.
  base::TimeTicks virtual_time_fence_;

  std::unique_ptr<ProcessTimeOverrideCoordinator::ScopedOverride>
      time_override_;
  const base::TimeTicks initial_time_ticks_;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_AUTO_ADVANCING_VIRTUAL_TIME_DOMAIN_H_
