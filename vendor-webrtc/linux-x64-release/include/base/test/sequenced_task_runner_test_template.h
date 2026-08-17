// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// SequencedTaskRunnerTest defines tests that implementations of
// SequencedTaskRunner should pass in order to be conformant.
// See task_runner_test_template.h for a description of how to use the
// constructs in this file; these work the same.

#ifndef BASE_TEST_SEQUENCED_TASK_RUNNER_TEST_TEMPLATE_H_
#define BASE_TEST_SEQUENCED_TASK_RUNNER_TEST_TEMPLATE_H_

#include <cstddef>
#include <iosfwd>
#include <vector>

#include "base/functional/bind.h"
#include "base/functional/callback.h"
#include "base/memory/ref_counted.h"
#include "base/synchronization/condition_variable.h"
#include "base/synchronization/lock.h"
#include "base/task/sequenced_task_runner.h"
#include "base/threading/platform_thread.h"
#include "base/time/time.h"
#include "testing/gtest/include/gtest/gtest.h"

namespace base {

namespace internal {

struct TaskEvent {
  enum Type { POST, START, END };
  TaskEvent(int i, Type type);
  int i;
  Type type;
};

// Utility class used in the tests below.
class SequencedTaskTracker : public RefCountedThreadSafe<SequencedTaskTracker> {
 public:
  SequencedTaskTracker();

  SequencedTaskTracker(const SequencedTaskTracker&) = delete;
  SequencedTaskTracker& operator=(const SequencedTaskTracker&) = delete;

  // Posts the non-nestable task |task|, and records its post event.
  void PostWrappedNonNestableTask(SequencedTaskRunner* task_runner,
                                  OnceClosure task);

  // Posts the nestable task |task|, and records its post event.
  void PostWrappedNestableTask(SequencedTaskRunner* task_runner,
                               OnceClosure task);

  // Posts the delayed non-nestable task |task|, and records its post event.
  void PostWrappedDelayedNonNestableTask(SequencedTaskRunner* task_runner,
                                         OnceClosure task,
                                         TimeDelta delay);

  // Posts |task_count| non-nestable tasks.
  void PostNonNestableTasks(SequencedTaskRunner* task_runner, int task_count);

  const std::vector<TaskEvent>& GetTaskEvents() const;

  // Returns after the tracker observes a total of |count| task completions.
  void WaitForCompletedTasks(int count);

 private:
  friend class RefCountedThreadSafe<SequencedTaskTracker>;

  ~SequencedTaskTracker();

  // A task which runs |task|, recording the start and end events.
  void RunTask(OnceClosure task, int task_i);

  // Records a post event for task |i|. The owner is expected to be holding
  // |lock_| (unlike |TaskStarted| and |TaskEnded|).
  void TaskPosted(int i);

  // Records a start event for task |i|.
  void TaskStarted(int i);

  // Records a end event for task |i|.
  void TaskEnded(int i);

  // Protects events_, next_post_i_, task_end_count_ and task_end_cv_.
  Lock lock_;

  // The events as they occurred for each task (protected by lock_).
  std::vector<TaskEvent> events_;

  // The ordinal to be used for the next task-posting task (protected by
  // lock_).
  int next_post_i_;

  // The number of task end events we've received.
  int task_end_count_;
  ConditionVariable task_end_cv_;
};

void PrintTo(const TaskEvent& event, std::ostream* os);

// Checks the non-nestable task invariants for all tasks in |events|.
//
// The invariants are:
// 1) Events started and ended in the same order that they were posted.
// 2) Events for an individual tasks occur in the order {POST, START, END},
//    and there is only one instance of each event type for a task.
// 3) The only events between a task's START and END events are the POSTs of
//    other tasks. I.e. tasks were run sequentially, not interleaved.
::testing::AssertionResult CheckNonNestableInvariants(
    const std::vector<TaskEvent>& events,
    int task_count);

}  // namespace internal

template <typename TaskRunnerTestDelegate>
class SequencedTaskRunnerTest : public testing::Test {
 protected:
  SequencedTaskRunnerTest()
      : task_tracker_(new internal::SequencedTaskTracker()) {}

  const scoped_refptr<internal::SequencedTaskTracker> task_tracker_;
  TaskRunnerTestDelegate delegate_;
};

TYPED_TEST_SUITE_P(SequencedTaskRunnerTest);

// This test posts N non-nestable tasks in sequence, and expects them to run
// in FIFO order, with no part of any two tasks' execution
// overlapping. I.e. that each task starts only after the previously-posted
// one has finished.
TYPED_TEST_P(SequencedTaskRunnerTest, SequentialNonNestable) {
  const int kTaskCount = 1000;

  this->delegate_.StartTaskRunner();
  const scoped_refptr<SequencedTaskRunner> task_runner =
      this->delegate_.GetTaskRunner();

  this->task_tracker_->PostWrappedNonNestableTask(
      task_runner.get(), BindOnce(&PlatformThread::Sleep, Seconds(1)));
  for (int i = 1; i < kTaskCount; ++i) {
    this->task_tracker_->PostWrappedNonNestableTask(task_runner.get(),
                                                    OnceClosure());
  }

  this->delegate_.StopTaskRunner();

  EXPECT_TRUE(CheckNonNestableInvariants(this->task_tracker_->GetTaskEvents(),
                                         kTaskCount));
}

// This test posts N nestable tasks in sequence. It has the same expectations
// as SequentialNonNestable because even though the tasks are nestable, they
// will not be run nestedly in this case.
TYPED_TEST_P(SequencedTaskRunnerTest, SequentialNestable) {
  const int kTaskCount = 1000;

  this->delegate_.StartTaskRunner();
  const scoped_refptr<SequencedTaskRunner> task_runner =
      this->delegate_.GetTaskRunner();

  this->task_tracker_->PostWrappedNestableTask(
      task_runner.get(), BindOnce(&PlatformThread::Sleep, Seconds(1)));
  for (int i = 1; i < kTaskCount; ++i) {
    this->task_tracker_->PostWrappedNestableTask(task_runner.get(),
                                                 OnceClosure());
  }

  this->delegate_.StopTaskRunner();

  EXPECT_TRUE(CheckNonNestableInvariants(this->task_tracker_->GetTaskEvents(),
                                         kTaskCount));
}

// This test posts non-nestable tasks in order of increasing delay, and checks
// that that the tasks are run in FIFO order and that there is no execution
// overlap whatsoever between any two tasks.
TYPED_TEST_P(SequencedTaskRunnerTest, SequentialDelayedNonNestable) {
  const int kTaskCount = 20;
  const int kDelayIncrementMs = 50;

  this->delegate_.StartTaskRunner();
  const scoped_refptr<SequencedTaskRunner> task_runner =
      this->delegate_.GetTaskRunner();

  for (int i = 0; i < kTaskCount; ++i) {
    this->task_tracker_->PostWrappedDelayedNonNestableTask(
        task_runner.get(), OnceClosure(), Milliseconds(kDelayIncrementMs * i));
  }

  this->task_tracker_->WaitForCompletedTasks(kTaskCount);
  this->delegate_.StopTaskRunner();

  EXPECT_TRUE(CheckNonNestableInvariants(this->task_tracker_->GetTaskEvents(),
                                         kTaskCount));
}

// This test posts a fast, non-nestable task from within each of a number of
// slow, non-nestable tasks and checks that they all run in the sequence they
// were posted in and that there is no execution overlap whatsoever.
TYPED_TEST_P(SequencedTaskRunnerTest, NonNestablePostFromNonNestableTask) {
  const int kParentCount = 10;
  const int kChildrenPerParent = 10;

  this->delegate_.StartTaskRunner();
  const scoped_refptr<SequencedTaskRunner> task_runner =
      this->delegate_.GetTaskRunner();

  for (int i = 0; i < kParentCount; ++i) {
    auto task = BindOnce(&internal::SequencedTaskTracker::PostNonNestableTasks,
                         this->task_tracker_, RetainedRef(task_runner),
                         kChildrenPerParent);
    this->task_tracker_->PostWrappedNonNestableTask(task_runner.get(),
                                                    std::move(task));
  }

  this->delegate_.StopTaskRunner();

  EXPECT_TRUE(
      CheckNonNestableInvariants(this->task_tracker_->GetTaskEvents(),
                                 kParentCount * (kChildrenPerParent + 1)));
}

// This test posts two tasks with the same delay, and checks that the tasks are
// run in the order in which they were posted.
//
// NOTE: This is actually an approximate test since the API only takes a
// "delay" parameter, so we are not exactly simulating two tasks that get
// posted at the exact same time. It would be nice if the API allowed us to
// specify the desired run time.
TYPED_TEST_P(SequencedTaskRunnerTest, DelayedTasksSameDelay) {
  const int kTaskCount = 2;
  const TimeDelta kDelay = Milliseconds(100);

  this->delegate_.StartTaskRunner();
  const scoped_refptr<SequencedTaskRunner> task_runner =
      this->delegate_.GetTaskRunner();

  this->task_tracker_->PostWrappedDelayedNonNestableTask(task_runner.get(),
                                                         OnceClosure(), kDelay);
  this->task_tracker_->PostWrappedDelayedNonNestableTask(task_runner.get(),
                                                         OnceClosure(), kDelay);
  this->task_tracker_->WaitForCompletedTasks(kTaskCount);
  this->delegate_.StopTaskRunner();

  EXPECT_TRUE(CheckNonNestableInvariants(this->task_tracker_->GetTaskEvents(),
                                         kTaskCount));
}

// This test posts a normal task and a delayed task, and checks that the
// delayed task runs after the normal task even if the normal task takes
// a long time to run.
TYPED_TEST_P(SequencedTaskRunnerTest, DelayedTaskAfterLongTask) {
  const int kTaskCount = 2;

  this->delegate_.StartTaskRunner();
  const scoped_refptr<SequencedTaskRunner> task_runner =
      this->delegate_.GetTaskRunner();

  this->task_tracker_->PostWrappedNonNestableTask(
      task_runner.get(),
      base::BindOnce(&PlatformThread::Sleep, Milliseconds(50)));
  this->task_tracker_->PostWrappedDelayedNonNestableTask(
      task_runner.get(), OnceClosure(), Milliseconds(10));
  this->task_tracker_->WaitForCompletedTasks(kTaskCount);
  this->delegate_.StopTaskRunner();

  EXPECT_TRUE(CheckNonNestableInvariants(this->task_tracker_->GetTaskEvents(),
                                         kTaskCount));
}

// Test that a pile of normal tasks and a delayed task run in the
// time-to-run order.
TYPED_TEST_P(SequencedTaskRunnerTest, DelayedTaskAfterManyLongTasks) {
  const int kTaskCount = 11;

  this->delegate_.StartTaskRunner();
  const scoped_refptr<SequencedTaskRunner> task_runner =
      this->delegate_.GetTaskRunner();

  for (int i = 0; i < kTaskCount - 1; i++) {
    this->task_tracker_->PostWrappedNonNestableTask(
        task_runner.get(),
        base::BindOnce(&PlatformThread::Sleep, Milliseconds(50)));
  }
  this->task_tracker_->PostWrappedDelayedNonNestableTask(
      task_runner.get(), OnceClosure(), Milliseconds(10));
  this->task_tracker_->WaitForCompletedTasks(kTaskCount);
  this->delegate_.StopTaskRunner();

  EXPECT_TRUE(CheckNonNestableInvariants(this->task_tracker_->GetTaskEvents(),
                                         kTaskCount));
}

// TODO(francoisk777@gmail.com) Add a test, similiar to the above, which runs
// some tasked nestedly (which should be implemented in the test
// delegate). Also add, to the test delegate, a predicate which checks
// whether the implementation supports nested tasks.
//

// The SequencedTaskRunnerTest test case verifies behaviour that is expected
// from a sequenced task runner in order to be conformant.
REGISTER_TYPED_TEST_SUITE_P(SequencedTaskRunnerTest,
                            SequentialNonNestable,
                            SequentialNestable,
                            SequentialDelayedNonNestable,
                            NonNestablePostFromNonNestableTask,
                            DelayedTasksSameDelay,
                            DelayedTaskAfterLongTask,
                            DelayedTaskAfterManyLongTasks);

}  // namespace base

#endif  // BASE_TEST_SEQUENCED_TASK_RUNNER_TEST_TEMPLATE_H_
