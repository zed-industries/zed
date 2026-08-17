// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THROTTLING_WAKE_UP_BUDGET_POOL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THROTTLING_WAKE_UP_BUDGET_POOL_H_

#include <optional>

#include "base/time/time.h"
#include "third_party/blink/renderer/platform/scheduler/common/throttling/budget_pool.h"

namespace blink {
namespace scheduler {

// WakeUpBudgetPool represents a collection of task queues which run for a
// limited time at regular intervals.
class PLATFORM_EXPORT WakeUpBudgetPool : public BudgetPool {
 public:
  explicit WakeUpBudgetPool(const char* name);
  WakeUpBudgetPool(const WakeUpBudgetPool&) = delete;
  WakeUpBudgetPool& operator=(const WakeUpBudgetPool&) = delete;
  ~WakeUpBudgetPool() override;

  // Sets the interval between wake ups. This can be invoked at any time. If a
  // next wake up is already scheduled, it is rescheduled according to the new
  // |interval| as part of this call.
  void SetWakeUpInterval(base::TimeTicks now, base::TimeDelta interval);

  // Sets the duration of wake ups. This does not have an immediate effect and
  // should be called only during initialization of a WakeUpBudgetPool.
  void SetWakeUpDuration(base::TimeDelta duration);

  // Sets a lower wake up alignment. If non-zero, the budget pool will ignore
  // the |wake_up_interval_| and allow a wake up aligned on |alignment| if there
  // hasn't been a wake up in the last |wake_up_interval_|.
  //
  // This does not have an immediate effect and should be called only during
  // initialization of a WakeUpBudgetPool.
  void AllowLowerAlignmentIfNoRecentWakeUp(base::TimeDelta alignment);

  // BudgetPool implementation:
  void RecordTaskRunTime(base::TimeTicks start_time,
                         base::TimeTicks end_time) final {}
  bool CanRunTasksAt(base::TimeTicks moment) const final;
  base::TimeTicks GetTimeTasksCanRunUntil(base::TimeTicks now) const final;
  base::TimeTicks GetNextAllowedRunTime(
      base::TimeTicks desired_run_time) const final;
  void OnWakeUp(base::TimeTicks now) final;
  void WriteIntoTrace(perfetto::TracedValue context,
                      base::TimeTicks now) const final;

  std::optional<base::TimeTicks> last_wake_up_for_testing() const {
    return last_wake_up_;
  }

 protected:
  QueueBlockType GetBlockType() const final;

 private:
  base::TimeDelta wake_up_interval_;
  base::TimeDelta wake_up_duration_;
  base::TimeDelta wake_up_alignment_if_no_recent_wake_up_;

  std::optional<base::TimeTicks> last_wake_up_;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THROTTLING_WAKE_UP_BUDGET_POOL_H_
