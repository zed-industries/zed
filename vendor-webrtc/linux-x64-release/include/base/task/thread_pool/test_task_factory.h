// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_TEST_TASK_FACTORY_H_
#define BASE_TASK_THREAD_POOL_TEST_TASK_FACTORY_H_

#include <stddef.h>

#include <unordered_set>

#include "base/functional/callback_forward.h"
#include "base/memory/ref_counted.h"
#include "base/synchronization/condition_variable.h"
#include "base/synchronization/lock.h"
#include "base/task/task_runner.h"
#include "base/task/task_traits.h"
#include "base/task/thread_pool/test_utils.h"
#include "base/threading/thread_checker_impl.h"

namespace base {
namespace internal {
namespace test {

// A TestTaskFactory posts tasks to a TaskRunner and verifies that they run as
// expected. Generates a test failure when:
// - The RunsTasksInCurrentSequence() method of the SequencedTaskRunner
//   (kSequenced or kSingleThread modes) returns false on a thread on which a
//   Task is run.
// - The task runner CurrentDefaultHandles set in the context of the task don't
//   match what's expected for the tested TaskSourceExecutionMode.
// - The TaskSourceExecutionMode of the TaskRunner is kSequenced or
//   kSingleThread and Tasks don't run in posting order.
// - The TaskSourceExecutionMode of the TaskRunner is kSingleThread and Tasks
//   don't run on the same thread.
// - A Task runs more than once.
class TestTaskFactory {
 public:
  enum class PostNestedTask {
    YES,
    NO,
  };

  // Constructs a TestTaskFactory that posts tasks to |task_runner|.
  // |execution_mode| is the TaskSourceExecutionMode of |task_runner|.
  TestTaskFactory(scoped_refptr<TaskRunner> task_runner,
                  TaskSourceExecutionMode execution_mode);

  TestTaskFactory(const TestTaskFactory&) = delete;
  TestTaskFactory& operator=(const TestTaskFactory&) = delete;
  ~TestTaskFactory();

  // Posts a task. The posted task will:
  // - Post a new task if |post_nested_task| is YES. The nested task won't run
  //   |after_task_closure|.
  // - Verify conditions in which the task runs (see potential failures above).
  // - Run |after_task_closure| if it is not null.
  bool PostTask(PostNestedTask post_nested_task,
                OnceClosure after_task_closure);

  // Waits for all tasks posted by PostTask() to start running. It is not
  // guaranteed that the tasks have completed their execution when this returns.
  void WaitForAllTasksToRun() const;

  const TaskRunner* task_runner() const { return task_runner_.get(); }

 private:
  void RunTaskCallback(size_t task_index,
                       PostNestedTask post_nested_task,
                       OnceClosure after_task_closure);

  // Synchronizes access to all members.
  mutable Lock lock_;

  // Condition variable signaled when a task runs.
  mutable ConditionVariable cv_;

  // Task runner through which this factory posts tasks.
  const scoped_refptr<TaskRunner> task_runner_;

  // Execution mode of |task_runner_|.
  const TaskSourceExecutionMode execution_mode_;

  // Number of tasks posted by PostTask().
  size_t num_posted_tasks_ = 0;

  // Indexes of tasks that ran.
  std::unordered_set<size_t> ran_tasks_;

  // Used to verify that all tasks run on the same thread when |execution_mode_|
  // is SINGLE_THREADED.
  ThreadCheckerImpl thread_checker_;
};

}  // namespace test
}  // namespace internal
}  // namespace base

#endif  // BASE_TASK_THREAD_POOL_TEST_TASK_FACTORY_H_
