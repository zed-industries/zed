// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_DEFERRED_SEQUENCED_TASK_RUNNER_H_
#define BASE_TASK_DEFERRED_SEQUENCED_TASK_RUNNER_H_

#include <vector>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/functional/callback.h"
#include "base/synchronization/lock.h"
#include "base/task/sequenced_task_runner.h"
#include "base/threading/platform_thread.h"
#include "base/time/time.h"

namespace base {

// A DeferredSequencedTaskRunner is a subclass of SequencedTaskRunner that
// queues up all requests until the first call to Start() is issued.
// DeferredSequencedTaskRunner may be created in two ways:
// . with an explicit SequencedTaskRunner that the events are flushed to
// . without a SequencedTaskRunner. In this configuration the
//   SequencedTaskRunner is supplied in StartWithTaskRunner().
class BASE_EXPORT DeferredSequencedTaskRunner : public SequencedTaskRunner {
 public:
  explicit DeferredSequencedTaskRunner(
      scoped_refptr<SequencedTaskRunner> target_runner);

  // Use this constructor when you don't have the target SequencedTaskRunner.
  // When using this call StartWithTaskRunner().
  DeferredSequencedTaskRunner();
  DeferredSequencedTaskRunner(const DeferredSequencedTaskRunner&) = delete;
  DeferredSequencedTaskRunner& operator=(const DeferredSequencedTaskRunner&) =
      delete;

  // TaskRunner implementation
  bool PostDelayedTask(const Location& from_here,
                       OnceClosure task,
                       TimeDelta delay) override;

  // SequencedTaskRunner implementation
  bool RunsTasksInCurrentSequence() const override;
  bool PostNonNestableDelayedTask(const Location& from_here,
                                  OnceClosure task,
                                  TimeDelta delay) override;

  // Start the execution - posts all queued tasks to the target executor. The
  // deferred tasks are posted with their initial delay, meaning that the task
  // execution delay is actually measured from Start.
  // Fails when called a second time.
  void Start();

  // Same as Start(), but must be used with the no-arg constructor.
  void StartWithTaskRunner(
      scoped_refptr<SequencedTaskRunner> target_task_runner);

  // Returns true if task execution has been started.
  bool Started() const;

 private:
  struct DeferredTask {
    DeferredTask();
    DeferredTask(DeferredTask&& other);
    ~DeferredTask();
    DeferredTask& operator=(DeferredTask&& other);

    Location posted_from;
    OnceClosure task;
    // The delay this task was initially posted with.
    TimeDelta delay;
    bool is_non_nestable;
  };

  ~DeferredSequencedTaskRunner() override;

  // Both variants of Start() call into this.
  void StartImpl();

  // Creates a |Task| object and adds it to |deferred_tasks_queue_|.
  void QueueDeferredTask(const Location& from_here,
                         OnceClosure task,
                         TimeDelta delay,
                         bool is_non_nestable);

  mutable Lock lock_;

  const PlatformThreadId created_thread_id_;

  // An atomic pointer that allows to call task_runner methods without lock.
  // It's possible because the pointer starts as null, is set to a non-null
  // value only once, and is never changed again.
  // This is used to implement a lock-free RunsTasksInCurrentSequence method.
  std::atomic<SequencedTaskRunner*> task_runner_atomic_ptr_{nullptr};
  bool started_ GUARDED_BY(lock_) = false;
  scoped_refptr<SequencedTaskRunner> target_task_runner_ GUARDED_BY(lock_);
  std::vector<DeferredTask> deferred_tasks_queue_ GUARDED_BY(lock_);
};

}  // namespace base

#endif  // BASE_TASK_DEFERRED_SEQUENCED_TASK_RUNNER_H_
