// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file defines tests that implementations of TaskRunner should
// pass in order to be conformant, as well as test cases for optional behavior.
// Here's how you use it to test your implementation.
//
// Say your class is called MyTaskRunner.  Then you need to define a
// class called MyTaskRunnerTestDelegate in my_task_runner_unittest.cc
// like this:
//
//   class MyTaskRunnerTestDelegate {
//    public:
//     // Tasks posted to the task runner after this and before
//     // StopTaskRunner() is called is called should run successfully.
//     void StartTaskRunner() {
//       ...
//     }
//
//     // Should return the task runner implementation.  Only called
//     // after StartTaskRunner and before StopTaskRunner.
//     scoped_refptr<MyTaskRunner> GetTaskRunner() {
//       ...
//     }
//
//     // Stop the task runner and make sure all tasks posted before
//     // this is called are run. Caveat: delayed tasks are not run,
// they're simply deleted.
//     void StopTaskRunner() {
//       ...
//     }
//   };
//
// The TaskRunnerTest test harness will have a member variable of
// this delegate type and will call its functions in the various
// tests.
//
// Then you simply #include this file as well as gtest.h and add the
// following statement to my_task_runner_unittest.cc:
//
//   INSTANTIATE_TYPED_TEST_SUITE_P(
//       MyTaskRunner, TaskRunnerTest, MyTaskRunnerTestDelegate);
//
// Easy!

#ifndef BASE_TEST_TASK_RUNNER_TEST_TEMPLATE_H_
#define BASE_TEST_TASK_RUNNER_TEST_TEMPLATE_H_

#include <cstddef>
#include <map>

#include "base/functional/bind.h"
#include "base/functional/callback.h"
#include "base/location.h"
#include "base/memory/ref_counted.h"
#include "base/synchronization/condition_variable.h"
#include "base/synchronization/lock.h"
#include "base/task/single_thread_task_runner.h"
#include "base/task/task_runner.h"
#include "base/threading/thread.h"
#include "testing/gtest/include/gtest/gtest.h"

namespace base {

namespace test {

// Utility class that keeps track of how many times particular tasks
// are run.
class TaskTracker : public RefCountedThreadSafe<TaskTracker> {
 public:
  TaskTracker();

  TaskTracker(const TaskTracker&) = delete;
  TaskTracker& operator=(const TaskTracker&) = delete;

  // Returns a closure that runs the given task and increments the run
  // count of |i| by one.  |task| may be null.  It is guaranteed that
  // only one task wrapped by a given tracker will be run at a time.
  RepeatingClosure WrapTask(RepeatingClosure task, int i);

  std::map<int, int> GetTaskRunCounts() const;

  // Returns after the tracker observes a total of |count| task completions.
  void WaitForCompletedTasks(int count);

 private:
  friend class RefCountedThreadSafe<TaskTracker>;

  ~TaskTracker();

  void RunTask(RepeatingClosure task, int i);

  mutable Lock lock_;
  std::map<int, int> task_run_counts_;
  int task_runs_;
  ConditionVariable task_runs_cv_;
};

}  // namespace test

template <typename TaskRunnerTestDelegate>
class TaskRunnerTest : public testing::Test {
 protected:
  TaskRunnerTest() : task_tracker_(base::MakeRefCounted<test::TaskTracker>()) {}

  const scoped_refptr<test::TaskTracker> task_tracker_;
  TaskRunnerTestDelegate delegate_;
};

TYPED_TEST_SUITE_P(TaskRunnerTest);

// We can't really test much, since TaskRunner provides very few
// guarantees.

// Post a bunch of tasks to the task runner.  They should all
// complete.
TYPED_TEST_P(TaskRunnerTest, Basic) {
  std::map<int, int> expected_task_run_counts;

  this->delegate_.StartTaskRunner();
  scoped_refptr<TaskRunner> task_runner = this->delegate_.GetTaskRunner();
  // Post each ith task i+1 times.
  for (int i = 0; i < 20; ++i) {
    RepeatingClosure ith_task =
        this->task_tracker_->WrapTask(RepeatingClosure(), i);
    for (int j = 0; j < i + 1; ++j) {
      task_runner->PostTask(FROM_HERE, ith_task);
      ++expected_task_run_counts[i];
    }
  }
  this->delegate_.StopTaskRunner();

  EXPECT_EQ(expected_task_run_counts, this->task_tracker_->GetTaskRunCounts());
}

// Post a bunch of delayed tasks to the task runner.  They should all
// complete.
TYPED_TEST_P(TaskRunnerTest, Delayed) {
  std::map<int, int> expected_task_run_counts;
  int expected_total_tasks = 0;

  this->delegate_.StartTaskRunner();
  scoped_refptr<TaskRunner> task_runner = this->delegate_.GetTaskRunner();
  // Post each ith task i+1 times with delays from 0-i.
  for (int i = 0; i < 20; ++i) {
    RepeatingClosure ith_task =
        this->task_tracker_->WrapTask(RepeatingClosure(), i);
    for (int j = 0; j < i + 1; ++j) {
      task_runner->PostDelayedTask(FROM_HERE, ith_task, base::Milliseconds(j));
      ++expected_task_run_counts[i];
      ++expected_total_tasks;
    }
  }
  this->task_tracker_->WaitForCompletedTasks(expected_total_tasks);
  this->delegate_.StopTaskRunner();

  EXPECT_EQ(expected_task_run_counts, this->task_tracker_->GetTaskRunCounts());
}

// The TaskRunnerTest test case verifies behaviour that is expected from a
// task runner in order to be conformant.
REGISTER_TYPED_TEST_SUITE_P(TaskRunnerTest, Basic, Delayed);

}  // namespace base

#endif  // BASE_TEST_TASK_RUNNER_TEST_TEMPLATE_H_
