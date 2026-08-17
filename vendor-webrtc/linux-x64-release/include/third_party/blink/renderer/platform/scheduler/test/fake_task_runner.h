// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FAKE_TASK_RUNNER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FAKE_TASK_RUNNER_H_

#include "base/functional/callback.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/tick_clock.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"

namespace blink {
namespace scheduler {

// A dummy task runner for tests.
class FakeTaskRunner : public base::SingleThreadTaskRunner {
 public:
  FakeTaskRunner();
  FakeTaskRunner(const FakeTaskRunner&) = delete;
  FakeTaskRunner& operator=(const FakeTaskRunner&) = delete;

  void SetTime(base::TimeTicks new_time);
  void SetTime(double new_time) {
    SetTime(base::TimeTicks() + base::Seconds(new_time));
  }

  // base::SingleThreadTaskRunner implementation:
  bool RunsTasksInCurrentSequence() const override;

  void RunUntilIdle();
  void AdvanceTimeAndRun(base::TimeDelta delta);
  void AdvanceTimeAndRun(double delta_seconds) {
    AdvanceTimeAndRun(base::Seconds(delta_seconds));
  }

  const base::TickClock* GetMockTickClock() const;

  using PendingTask = std::pair<base::OnceClosure, base::TimeTicks>;
  Deque<PendingTask> TakePendingTasksForTesting();

 protected:
  ~FakeTaskRunner() override;

  bool PostDelayedTask(const base::Location& location,
                       base::OnceClosure task,
                       base::TimeDelta delay) override;
  bool PostDelayedTaskAt(base::subtle::PostDelayedTaskPassKey,
                         const base::Location& from_here,
                         base::OnceClosure task,
                         base::TimeTicks delayed_run_time,
                         base::subtle::DelayPolicy deadline_policy) override;
  bool PostNonNestableDelayedTask(const base::Location&,
                                  base::OnceClosure task,
                                  base::TimeDelta delay) override;

 private:
  class Data;
  class BaseTaskRunner;
  scoped_refptr<Data> data_;

  explicit FakeTaskRunner(scoped_refptr<Data> data);
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FAKE_TASK_RUNNER_H_
