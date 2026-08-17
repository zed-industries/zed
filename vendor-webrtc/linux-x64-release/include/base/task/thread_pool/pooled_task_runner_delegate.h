// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_POOLED_TASK_RUNNER_DELEGATE_H_
#define BASE_TASK_THREAD_POOL_POOLED_TASK_RUNNER_DELEGATE_H_

#include "base/base_export.h"
#include "base/task/task_traits.h"
#include "base/task/thread_pool/job_task_source.h"
#include "base/task/thread_pool/sequence.h"
#include "base/task/thread_pool/task.h"
#include "base/task/thread_pool/task_source.h"

namespace base {
namespace internal {

// Delegate interface for PooledParallelTaskRunner and
// PooledSequencedTaskRunner.
class BASE_EXPORT PooledTaskRunnerDelegate {
 public:
  PooledTaskRunnerDelegate();
  virtual ~PooledTaskRunnerDelegate();

  // Returns true if the current PooledTaskRunnerDelegate instance in the
  // process matches `delegate`. This is needed in case of unit tests wherein
  // a TaskRunner outlives the ThreadPoolInstance that created it, in which case
  // the current delegate would be null (and not match) or even have been
  // re-initialized to a new delegate in a following test.
  static bool MatchesCurrentDelegate(PooledTaskRunnerDelegate* delegate);

  // Returns true if |task_source| currently running *must* return ASAP.
  // Thread-safe but may return an outdated result (if a task unnecessarily
  // yields due to this, it will simply be re-scheduled).
  virtual bool ShouldYield(const TaskSource* task_source) = 0;

  // Invoked when a |task| is posted to the PooledParallelTaskRunner or
  // PooledSequencedTaskRunner. The implementation must post |task| to
  // |sequence| within the appropriate priority queue, depending on |sequence|
  // traits. Returns true if task was successfully posted.
  virtual bool PostTaskWithSequence(Task task,
                                    scoped_refptr<Sequence> sequence) = 0;

  // Invoked when a task is posted as a Job. The implementation must add
  // |task_source| to the appropriate priority queue, depending on |task_source|
  // traits, if it's not there already. Returns true if task source was
  // successfully enqueued or was already enqueued.
  virtual bool EnqueueJobTaskSource(
      scoped_refptr<JobTaskSource> task_source) = 0;

  // Removes |task_source| from the priority queue.
  virtual void RemoveJobTaskSource(
      scoped_refptr<JobTaskSource> task_source) = 0;

  // Invoked when the priority of |sequence|'s TaskRunner is updated. The
  // implementation must update |sequence|'s priority to |priority|, then place
  // |sequence| in the correct priority-queue position within the appropriate
  // thread group.
  virtual void UpdatePriority(scoped_refptr<TaskSource> task_source,
                              TaskPriority priority) = 0;
  virtual void UpdateJobPriority(scoped_refptr<TaskSource> task_source,
                                 TaskPriority priority) = 0;
};

}  // namespace internal
}  // namespace base

#endif  // BASE_TASK_THREAD_POOL_POOLED_TASK_RUNNER_DELEGATE_H_
