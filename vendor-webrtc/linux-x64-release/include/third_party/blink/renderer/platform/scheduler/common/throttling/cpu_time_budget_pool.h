// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THROTTLING_CPU_TIME_BUDGET_POOL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THROTTLING_CPU_TIME_BUDGET_POOL_H_

#include <optional>

#include "base/gtest_prod_util.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/scheduler/common/throttling/budget_pool.h"
#include "third_party/blink/renderer/platform/scheduler/common/tracing_helper.h"

namespace blink {
namespace scheduler {

// CPUTimeBudgetPool represents a collection of task queues which share a limit
// on total cpu time.
class PLATFORM_EXPORT CPUTimeBudgetPool : public BudgetPool {
 public:
  CPUTimeBudgetPool(const char* name,
                    TraceableVariableController* tracing_controller,
                    base::TimeTicks now);
  CPUTimeBudgetPool(const CPUTimeBudgetPool&) = delete;
  CPUTimeBudgetPool& operator=(const CPUTimeBudgetPool&) = delete;

  ~CPUTimeBudgetPool() override;

  // Set max budget level, std::nullopt represent absence of max level.
  // Max budget level prevents accumulating arbitrary large budgets when
  // page is inactive for a very long time.
  void SetMaxBudgetLevel(base::TimeTicks now,
                         std::optional<base::TimeDelta> max_budget_level);

  // Set max throttling duration, std::nullopt represents absense of it.
  // Max throttling duration prevents page from being throttled for
  // a very long period after a single long task.
  void SetMaxThrottlingDelay(
      base::TimeTicks now,
      std::optional<base::TimeDelta> max_throttling_delay);

  // Throttle task queues from this time budget pool if tasks are running
  // for more than |cpu_percentage| per cent of wall time.
  // This function does not affect internal time budget level.
  void SetTimeBudgetRecoveryRate(base::TimeTicks now, double cpu_percentage);

  // Increase budget level by given value. This function DOES NOT unblock
  // queues even if they are allowed to run with increased budget level.
  void GrantAdditionalBudget(base::TimeTicks now, base::TimeDelta budget_level);

  // Set callback which will be called every time when this budget pool
  // is throttled. Throttling duration (time until the queue is allowed
  // to run again) is passed as a parameter to callback.
  void SetReportingCallback(
      base::RepeatingCallback<void(base::TimeDelta)> reporting_callback);

  // BudgetPool implementation:
  void RecordTaskRunTime(base::TimeTicks start_time,
                         base::TimeTicks end_time) final;
  bool CanRunTasksAt(base::TimeTicks moment) const final;
  base::TimeTicks GetTimeTasksCanRunUntil(base::TimeTicks now) const final;
  base::TimeTicks GetNextAllowedRunTime(
      base::TimeTicks desired_run_time) const final;
  void OnWakeUp(base::TimeTicks now) final;
  void WriteIntoTrace(perfetto::TracedValue context,
                      base::TimeTicks) const final;

 protected:
  QueueBlockType GetBlockType() const final;

 private:
  FRIEND_TEST_ALL_PREFIXES(TaskQueueThrottlerTest, CPUTimeBudgetPool);

  // Advances |last_checkpoint_| to |now| if needed and recalculates
  // budget level.
  void Advance(base::TimeTicks now);

  // Increase |current_budget_level_| to satisfy max throttling duration
  // condition if necessary.
  // Decrease |current_budget_level_| to satisfy max budget level
  // condition if necessary.
  void EnforceBudgetLevelRestrictions();

  // Max budget level which we can accrue.
  // Tasks will be allowed to run for this time before being throttled
  // after a very long period of inactivity.
  std::optional<base::TimeDelta> max_budget_level_;
  // Max throttling delay places a lower limit on time budget level,
  // ensuring that one long task does not cause extremely long throttling.
  // Note that this is not a guarantee that every task will run
  // after desired run time + max throttling duration, but a guarantee
  // that at least one task will be run every max_throttling_delay.
  std::optional<base::TimeDelta> max_throttling_delay_;

  TraceableCounter<base::TimeDelta,
                   TRACE_DISABLED_BY_DEFAULT("renderer.scheduler")>
      current_budget_level_;
  base::TimeTicks last_checkpoint_;
  double cpu_percentage_;

  base::RepeatingCallback<void(base::TimeDelta)> reporting_callback_;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THROTTLING_CPU_TIME_BUDGET_POOL_H_
