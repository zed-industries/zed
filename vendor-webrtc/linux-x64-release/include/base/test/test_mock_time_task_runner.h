// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_TEST_MOCK_TIME_TASK_RUNNER_H_
#define BASE_TEST_TEST_MOCK_TIME_TASK_RUNNER_H_

#include <stddef.h>

#include <memory>
#include <queue>
#include <vector>

#include "base/containers/circular_deque.h"
#include "base/functional/callback.h"
#include "base/functional/callback_helpers.h"
#include "base/memory/raw_ptr.h"
#include "base/run_loop.h"
#include "base/synchronization/condition_variable.h"
#include "base/synchronization/lock.h"
#include "base/task/single_thread_task_runner.h"
#include "base/test/test_pending_task.h"
#include "base/threading/thread_checker_impl.h"
#include "base/time/clock.h"
#include "base/time/tick_clock.h"
#include "base/time/time.h"

namespace base {

// ATTENTION: Prefer using base::test::SingleThreadTaskEnvironment with a
// base::test::SingleThreadTaskEnvironment::TimeSource::MOCK_TIME trait instead.
// The only case where TestMockTimeTaskRunner is necessary is when instantiating
// multiple TestMockTimeTaskRunners in the same test to deterministically
// exercise the result of a race between two simulated threads.
//
// Runs pending tasks in the order of the tasks' post time + delay, and keeps
// track of a mock (virtual) tick clock time that can be fast-forwarded.
//
// TestMockTimeTaskRunner has the following properties:
//
//   - Methods RunsTasksInCurrentSequence() and Post[Delayed]Task() can be
//     called from any thread, but the rest of the methods must be called on
//     the same thread the TestMockTimeTaskRunner was created on unless a call
//     is made to DetachFromThread(), in which case usage can switch to a
//     different thread.
//   - It allows for reentrancy, in that it handles the running of tasks that in
//     turn call back into it (e.g., to post more tasks).
//   - Tasks are stored in a priority queue, and executed in the increasing
//     order of post time + delay, but ignoring nestability.
//   - It does not check for overflow when doing time arithmetic. A sufficient
//     condition for preventing overflows is to make sure that the sum of all
//     posted task delays and fast-forward increments is still representable by
//     a TimeDelta, and that adding this delta to the starting values of Time
//     and TickTime is still within their respective range.
//
// A TestMockTimeTaskRunner of Type::kBoundToThread has the following additional
// properties:
//   - Thread/SequencedTaskRunner::CurrentDefaultHandle refers to it on its
//     thread.
//   - It can be driven by a RunLoop on the thread it was created on.
//     RunLoop::Run() will result in running non-delayed tasks until idle and
//     then, if RunLoop::QuitWhenIdle() wasn't invoked, fast-forwarding time to
//     the next delayed task and looping again. And so on, until either
//     RunLoop::Quit() is invoked (quits immediately after the current task) or
//     RunLoop::QuitWhenIdle() is invoked (quits before having to fast forward
//     time once again). Should RunLoop::Run() process all tasks (including
//     delayed ones), it will block until more are posted. As usual,
//     RunLoop::RunUntilIdle() is equivalent to RunLoop::Run() followed by an
//     immediate RunLoop::QuitWhenIdle().
//
// This is a slightly more sophisticated version of TestSimpleTaskRunner, in
// that it supports running delayed tasks in the correct temporal order.
class TestMockTimeTaskRunner : public SingleThreadTaskRunner,
                               public RunLoop::Delegate {
 public:
  // Everything that is executed in the scope of a ScopedContext will behave as
  // though it ran under |scope|
  // (i.e. SingleThreadTaskRunner::CurrentDefaultHandle,
  // RunsTasksInCurrentSequence, etc.). This allows the test body to be all in
  // one block when multiple TestMockTimeTaskRunners share the main thread.
  // Note: RunLoop isn't supported: will DCHECK if used inside a ScopedContext.
  //
  // For example:
  //
  //   class ExampleFixture {
  //    protected:
  //     DoBarOnFoo() {
  //       DCHECK(foo_task_runner_->RunsOnCurrentThread());
  //       EXPECT_EQ(
  //           foo_task_runner_, SingleThreadTaskRunner::GetCurrentDefault());
  //       DoBar();
  //     }
  //
  //     // Mock main task runner.
  //     base::MessageLoop message_loop_;
  //     base::ScopedMockTimeMessageLoopTaskRunner main_task_runner_;
  //
  //     // Mock foo task runner.
  //     scoped_refptr<TestMockTimeTaskRunner> foo_task_runner_ =
  //         new TestMockTimeTaskRunner();
  //   };
  //
  //   TEST_F(ExampleFixture, DoBarOnFoo) {
  //     DoThingsOnMain();
  //     {
  //       TestMockTimeTaskRunner::ScopedContext scoped_context(
  //           foo_task_runner_.get());
  //       DoBarOnFoo();
  //     }
  //     DoMoreThingsOnMain();
  //   }
  //
  class ScopedContext {
   public:
    // Note: |scope| is ran until idle as part of this constructor to ensure
    // that anything which runs in the underlying scope runs after any already
    // pending tasks (the contrary would break the SequencedTaskRunner
    // contract).
    explicit ScopedContext(scoped_refptr<TestMockTimeTaskRunner> scope);

    ScopedContext(const ScopedContext&) = delete;
    ScopedContext& operator=(const ScopedContext&) = delete;

    ~ScopedContext();

   private:
    SingleThreadTaskRunner::CurrentHandleOverrideForTesting
        single_thread_task_runner_current_default_handle_override_;
  };

  enum class Type {
    // A TestMockTimeTaskRunner which can only be driven directly through its
    // API. SingleThread/SequencedTaskRunner::CurrentDefaultHandle will refer to
    // it only in the scope of its tasks.
    kStandalone,
    // A TestMockTimeTaskRunner which will associate to the thread it is created
    // on, enabling RunLoop to drive it and making
    // Thread/SequencedTaskRunner::CurrentDefaultHandle refer to it on that
    // thread.
    kBoundToThread,
  };

  // Constructs an instance whose virtual time will start at the Unix epoch, and
  // whose time ticks will start at zero.
  TestMockTimeTaskRunner(Type type = Type::kStandalone);

  // Constructs an instance starting at the given virtual time and time ticks.
  TestMockTimeTaskRunner(Time start_time,
                         TimeTicks start_ticks,
                         Type type = Type::kStandalone);

  TestMockTimeTaskRunner(const TestMockTimeTaskRunner&) = delete;
  TestMockTimeTaskRunner& operator=(const TestMockTimeTaskRunner&) = delete;

  // Fast-forwards virtual time by |delta|, causing all tasks with a remaining
  // delay less than or equal to |delta| to be executed. |delta| must be
  // non-negative.
  void FastForwardBy(TimeDelta delta);

  // Fast-forwards virtual time by |delta| but not causing any task execution.
  void AdvanceMockTickClock(TimeDelta delta);

  // Fast-forward virtual time, but not tick time. May be useful for testing
  // timers when simulating suspend/resume or time adjustments. As it doesn't
  // advance tick time, no tasks are automatically processed
  // (ProcessAllTasksNoLaterThan is not called).
  void AdvanceWallClock(TimeDelta delta);

  // Fast-forwards virtual time just until all tasks are executed.
  void FastForwardUntilNoTasksRemain();

  // Executes all tasks that have no remaining delay. Tasks with a remaining
  // delay greater than zero will remain enqueued, and no virtual time will
  // elapse.
  void RunUntilIdle();

  // Processes the next |n| pending tasks in the order that they would normally
  // be processed advancing the virtual time as needed. Cancelled tasks are not
  // run but they still count towards |n|. If |n| is negative, this is
  // equivalent to FastForwardUntilNoTasksRemain(). If we run out of pending
  // tasks before reaching |n|, we early out.
  void ProcessNextNTasks(int n);

  // Clears the queue of pending tasks without running them.
  void ClearPendingTasks();

  // Returns the current virtual time (initially starting at the Unix epoch).
  Time Now() const;

  // Returns the current virtual tick time (initially starting at 0).
  TimeTicks NowTicks() const;

  // Returns a Clock that uses the virtual time of |this| as its time source.
  // The returned Clock will hold a reference to |this|.
  Clock* GetMockClock() const;

  // Returns a TickClock that uses the virtual time ticks of |this| as its tick
  // source. The returned TickClock will hold a reference to |this|.
  const TickClock* GetMockTickClock() const;

  // Cancelled pending tasks get pruned automatically.
  base::circular_deque<TestPendingTask> TakePendingTasks();
  bool HasPendingTask();
  size_t GetPendingTaskCount();
  TimeDelta NextPendingTaskDelay();

  // Allow invoking methods from different threads.
  // It is the caller's responsibility to ensure there are no data races.
  void DetachFromThread();

  // SingleThreadTaskRunner:
  bool RunsTasksInCurrentSequence() const override;
  bool PostDelayedTask(const Location& from_here,
                       OnceClosure task,
                       TimeDelta delay) override;
  bool PostDelayedTaskAt(subtle::PostDelayedTaskPassKey,
                         const Location& from_here,
                         OnceClosure task,
                         TimeTicks delayed_run_time,
                         subtle::DelayPolicy deadline_policy) override;
  bool PostNonNestableDelayedTask(const Location& from_here,
                                  OnceClosure task,
                                  TimeDelta delay) override;

 protected:
  ~TestMockTimeTaskRunner() override;

  // Called before the next task to run is selected, so that subclasses have a
  // last chance to make sure all tasks are posted.
  virtual void OnBeforeSelectingTask();

  // Called after the current mock time has been incremented so that subclasses
  // can react to the passing of time.
  virtual void OnAfterTimePassed();

  // Called after each task is run so that subclasses may perform additional
  // activities, e.g., pump additional task runners.
  virtual void OnAfterTaskRun();

 private:
  class NonOwningProxyTaskRunner;

  // MockClock implements TickClock and Clock. Always returns the then-current
  // mock time of |task_runner| as the current time or time ticks.
  class MockClock : public TickClock, public Clock {
   public:
    explicit MockClock(TestMockTimeTaskRunner* task_runner)
        : task_runner_(task_runner) {}

    MockClock(const MockClock&) = delete;
    MockClock& operator=(const MockClock&) = delete;

    // TickClock:
    TimeTicks NowTicks() const override;

    // Clock:
    Time Now() const override;

   private:
    raw_ptr<TestMockTimeTaskRunner> task_runner_;
  };

  struct TestOrderedPendingTask;

  // Predicate that defines a strict weak temporal ordering of tasks.
  class TemporalOrder {
   public:
    bool operator()(const TestOrderedPendingTask& first_task,
                    const TestOrderedPendingTask& second_task) const;
  };

  typedef std::priority_queue<TestOrderedPendingTask,
                              std::vector<TestOrderedPendingTask>,
                              TemporalOrder>
      TaskPriorityQueue;

  // Core of the implementation for all flavors of fast-forward methods. Given a
  // non-negative |max_delta|, processes up to |limit| tasks with a remaining
  // delay less than or equal to |max_delta|, and moves virtual time forward as
  // needed for each processed task. Cancelled tasks count towards |limit|. If
  // |limit| is negative, no limit on the number of processed tasks is imposed.
  // Pass in TimeDelta::Max() as |max_delta| and a negative |limit| to run all
  // tasks.
  void ProcessTasksNoLaterThan(TimeDelta max_delta, int limit = -1);

  // Forwards |now_ticks_| until it equals |later_ticks|, and forwards |now_| by
  // the same amount. Calls OnAfterTimePassed() if |later_ticks| > |now_ticks_|.
  // Does nothing if |later_ticks| <= |now_ticks_|.
  void ForwardClocksUntilTickTime(TimeTicks later_ticks);

  // Returns the |next_task| to run if there is any with a running time that is
  // at most |reference| + |max_delta|. This additional complexity is required
  // so that |max_delta| == TimeDelta::Max() can be supported.
  bool DequeueNextTask(const TimeTicks& reference,
                       const TimeDelta& max_delta,
                       TestPendingTask* next_task);

  // RunLoop::Delegate:
  void Run(bool application_tasks_allowed, TimeDelta timeout) override;
  void Quit() override;
  void EnsureWorkScheduled() override;

  // Also used for non-dcheck logic (RunsTasksInCurrentSequence()) and as such
  // needs to be a ThreadCheckerImpl.
  ThreadCheckerImpl thread_checker_;

  Time now_;
  TimeTicks now_ticks_;

  // Temporally ordered heap of pending tasks. Must only be accessed while the
  // |tasks_lock_| is held.
  TaskPriorityQueue tasks_;

  // The ordinal to use for the next task. Must only be accessed while the
  // |tasks_lock_| is held.
  size_t next_task_ordinal_ = 0;

  mutable Lock tasks_lock_;
  ConditionVariable tasks_lock_cv_;

  const scoped_refptr<NonOwningProxyTaskRunner> proxy_task_runner_;
  std::unique_ptr<SingleThreadTaskRunner::CurrentDefaultHandle>
      thread_task_runner_handle_;

  // Set to true in RunLoop::Delegate::Quit() to signal the topmost
  // RunLoop::Delegate::Run() instance to stop, reset to false when it does.
  bool quit_run_loop_ = false;

  mutable MockClock mock_clock_;
};

}  // namespace base

#endif  // BASE_TEST_TEST_MOCK_TIME_TASK_RUNNER_H_
