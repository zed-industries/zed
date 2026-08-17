// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_TASK_TRACKER_H_
#define BASE_TASK_THREAD_POOL_TASK_TRACKER_H_

#include <atomic>
#include <functional>
#include <limits>
#include <memory>
#include <optional>
#include <queue>
#include <string>

#include "base/atomic_sequence_num.h"
#include "base/base_export.h"
#include "base/containers/circular_deque.h"
#include "base/functional/callback_forward.h"
#include "base/sequence_checker.h"
#include "base/synchronization/waitable_event.h"
#include "base/task/common/checked_lock.h"
#include "base/task/common/task_annotator.h"
#include "base/task/task_traits.h"
#include "base/task/thread_pool/task.h"
#include "base/task/thread_pool/task_source.h"
#include "base/task/thread_pool/tracked_ref.h"
#include "base/thread_annotations.h"
#include "base/threading/thread_local.h"

namespace base {

class ConditionVariable;

namespace internal {

class JobTaskSource;

// Determines which tasks are allowed to run.
enum class CanRunPolicy {
  // All tasks are allowed to run.
  kAll,
  // Only USER_VISIBLE and USER_BLOCKING tasks are allowed to run.
  kForegroundOnly,
  // No tasks can run.
  kNone,
};

// TaskTracker enforces policies that determines whether:
// - A task can be pushed to a task source (WillPostTask).
// - A task source can be queued (WillQueueTaskSource).
// - Tasks for a given priority can run (CanRunPriority).
// - The next task in a queued task source can run (RunAndPopNextTask).
// TaskTracker also sets up the environment to run a task (RunAndPopNextTask)
// and records metrics and trace events. This class is thread-safe.
class BASE_EXPORT TaskTracker {
 public:
  TaskTracker();
  TaskTracker(const TaskTracker&) = delete;
  TaskTracker& operator=(const TaskTracker&) = delete;
  virtual ~TaskTracker();

  // Initiates shutdown. Once this is called, only BLOCK_SHUTDOWN tasks will
  // start running (doesn't affect tasks that are already running). This can
  // only be called once.
  void StartShutdown();

  // Synchronously completes shutdown. StartShutdown() must be called first.
  // Returns when:
  // - All SKIP_ON_SHUTDOWN tasks that were already running have completed their
  //   execution.
  // - All posted BLOCK_SHUTDOWN tasks have completed their execution.
  // CONTINUE_ON_SHUTDOWN tasks still may be running after Shutdown returns.
  // This can only be called once.
  void CompleteShutdown();

  // Waits until there are no incomplete task sources. May be called in tests
  // to validate that a condition is met after all task sources have run.
  //
  // Does not wait for delayed tasks. Waits for task sources posted from
  // other threads during the call. Returns immediately when shutdown completes.
  void FlushForTesting();

  // Returns and calls |flush_callback| when there are no incomplete undelayed
  // tasks. |flush_callback| may be called back on any thread and should not
  // perform a lot of work. May be used when additional work on the current
  // thread needs to be performed during a flush. Only one
  // FlushAsyncForTesting() may be pending at any given time.
  void FlushAsyncForTesting(OnceClosure flush_callback);

  // Sets the new CanRunPolicy policy, possibly affecting result of
  // CanRunPriority(). The caller must wake up worker as appropriate so that
  // tasks that are allowed to run by the new policy can be scheduled.
  void SetCanRunPolicy(CanRunPolicy can_run_policy);

  // Informs this TaskTracker that |task| with |shutdown_behavior| is about to
  // be pushed to a task source (if non-delayed) or be added to the
  // DelayedTaskManager (if delayed). Returns true if this operation is allowed
  // (the operation should be performed if-and-only-if it is). This method may
  // also modify metadata on |task| if desired.
  // If this returns false, `task` must be leaked by the caller if deleting it
  // on the current sequence may invoke sequence-affine code that belongs to
  // another sequence.
  bool WillPostTask(Task* task, TaskShutdownBehavior shutdown_behavior);

  // Informs this TaskTracker that |task| that is about to be pushed to a task
  // source with |priority|. Returns true if this operation is allowed (the
  // operation should be performed if-and-only-if it is).
  [[nodiscard]] bool WillPostTaskNow(const Task& task,
                                     TaskPriority priority) const;

  // Informs this TaskTracker that |task_source| is about to be queued. Returns
  // a RegisteredTaskSource that should be queued if-and-only-if it evaluates to
  // true.
  RegisteredTaskSource RegisterTaskSource(
      scoped_refptr<TaskSource> task_source);

  // Informs this TaskTracker that |task_source| is about to be queued.
  void WillEnqueueJob(JobTaskSource* task_source);

  // Returns true if a task with |priority| can run under to the current policy.
  bool CanRunPriority(TaskPriority priority) const;

  // Runs the next task in |task_source| unless the current shutdown state
  // prevents that. Then, pops the task from |task_source| (even if it didn't
  // run). Returns |task_source| if non-empty after popping a task from it
  // (which indicates that it should be reenqueued). WillPostTask() must have
  // allowed the task in front of |task_source| to be posted before this is
  // called.
  RegisteredTaskSource RunAndPopNextTask(RegisteredTaskSource task_source);

  // Returns true once shutdown has started (StartShutdown() was called).
  // Note: sequential consistency with the thread calling StartShutdown() isn't
  // guaranteed by this call.
  bool HasShutdownStarted() const;

  // Returns true if shutdown has completed (StartShutdown() was called and
  // no tasks are blocking shutdown).
  bool IsShutdownComplete() const;

  TrackedRef<TaskTracker> GetTrackedRef() {
    return tracked_ref_factory_.GetTrackedRef();
  }

  void BeginFizzlingBlockShutdownTasks();
  void EndFizzlingBlockShutdownTasks();

  // Returns true if there are task sources that haven't completed their
  // execution (still queued or in progress). If it returns false: the side-
  // effects of all completed tasks are guaranteed to be visible to the caller.
  bool HasIncompleteTaskSourcesForTesting() const;

 protected:
  // Runs and deletes |task|. |task| is deleted in the environment where it
  // runs. |task_source| is the task source from which |task| was extracted.
  // |traits| are the traits of |task_source|. An override is expected to call
  // its parent's implementation but is free to perform extra work before and
  // after doing so.
  virtual void RunTask(Task task,
                       TaskSource* task_source,
                       const TaskTraits& traits);

  // Allow a subclass to wait more interactively for any running shutdown tasks
  // before blocking the thread.
  virtual void BeginCompleteShutdown(base::WaitableEvent& shutdown_event);

  // Asserts that FlushForTesting() is allowed to be called. Overridden in tests
  // in situations where it is not.
  virtual void AssertFlushForTestingAllowed() {}

 private:
  friend class RegisteredTaskSource;
  class State;

  void PerformShutdown();

  // Called before WillPostTask() informs the tracing system that a task has
  // been posted. Updates |num_items_blocking_shutdown_| if necessary and
  // returns true if the current shutdown state allows the task to be posted.
  bool BeforeQueueTaskSource(TaskShutdownBehavior shutdown_behavior);

  // Called before a task with |effective_shutdown_behavior| is run by
  // RunTask(). Updates |num_items_blocking_shutdown_| if necessary and returns
  // true if the current shutdown state allows the task to be run.
  bool BeforeRunTask(TaskShutdownBehavior shutdown_behavior);

  // Called after a task with |effective_shutdown_behavior| has been run by
  // RunTask(). Updates |num_items_blocking_shutdown_| if necessary.
  void AfterRunTask(TaskShutdownBehavior shutdown_behavior);

  // Informs this TaskTracker that |task_source| won't be reenqueued and returns
  // the underlying TaskSource. This is called before destroying a valid
  // RegisteredTaskSource. Updates |num_items_blocking_shutdown_| if necessary.
  scoped_refptr<TaskSource> UnregisterTaskSource(
      scoped_refptr<TaskSource> task_source);

  // Called when an item blocking shutdown finishes after shutdown has started.
  void DecrementNumItemsBlockingShutdown();

  // Decrements the number of incomplete task sources and signals |flush_cv_|
  // if it reaches zero.
  void DecrementNumIncompleteTaskSources();

  // Invokes all |flush_callbacks_for_testing_| if any in a lock-safe manner.
  void InvokeFlushCallbacksForTesting();

  // Adds ThreadPool related trace event metadata to the event `ctx`. Notably,
  // records sequence information, as well as priority/execution mode.
  void EmitThreadPoolTraceEventMetadata(perfetto::EventContext& ctx,
                                        const TaskTraits& traits,
                                        TaskSource* task_source,
                                        const SequenceToken& token);

  // Dummy frames to allow identification of shutdown behavior in a stack trace.
  void RunContinueOnShutdown(Task& task,
                             const TaskTraits& traits,
                             TaskSource* task_source,
                             const SequenceToken& token);
  void RunSkipOnShutdown(Task& task,
                         const TaskTraits& traits,
                         TaskSource* task_source,
                         const SequenceToken& token);
  void RunBlockShutdown(Task& task,
                        const TaskTraits& traits,
                        TaskSource* task_source,
                        const SequenceToken& token);
  void RunTaskWithShutdownBehavior(Task& task,
                                   const TaskTraits& traits,
                                   TaskSource* task_source,
                                   const SequenceToken& token);

  NOT_TAIL_CALLED void RunTaskImpl(Task& task,
                                   const TaskTraits& traits,
                                   TaskSource* task_source,
                                   const SequenceToken& token);

  TaskAnnotator task_annotator_;

  // Indicates whether logging information about TaskPriority::BEST_EFFORT tasks
  // was enabled with a command line switch.
  const bool has_log_best_effort_tasks_switch_;

  // Number of tasks blocking shutdown and boolean indicating whether shutdown
  // has started. |shutdown_lock_| should be held to access |shutdown_event_|
  // when this indicates that shutdown has started because State doesn't provide
  // memory barriers. It intentionally trades having to use a Lock on shutdown
  // with not needing memory barriers at runtime.
  const std::unique_ptr<State> state_;

  // Number of task sources that haven't completed their execution. Is
  // decremented with a memory barrier after the last task of a task source
  // runs. Is accessed with an acquire memory barrier in FlushForTesting(). The
  // memory barriers ensure that the memory written by flushed task sources is
  // visible when FlushForTesting() returns.
  std::atomic_int num_incomplete_task_sources_{0};

  // Global policy the determines result of CanRunPriority().
  std::atomic<CanRunPolicy> can_run_policy_;

  // Lock associated with |flush_cv_|. Partially synchronizes access to
  // |num_incomplete_task_sources_|. Full synchronization isn't needed
  // because it's atomic, but synchronization is needed to coordinate waking and
  // sleeping at the right time. Fully synchronizes access to
  // |flush_callbacks_for_testing_|.
  mutable CheckedLock flush_lock_;

  // Signaled when |num_incomplete_task_sources_| is or reaches zero or when
  // shutdown completes.
  ConditionVariable flush_cv_;

  // All invoked, if any, when |num_incomplete_task_sources_| is zero or when
  // shutdown completes.
  base::circular_deque<OnceClosure> flush_callbacks_for_testing_
      GUARDED_BY(flush_lock_);

  // Synchronizes access to shutdown related members below.
  mutable CheckedLock shutdown_lock_;

  // Event instantiated when shutdown starts and signaled when shutdown
  // completes.
  std::optional<WaitableEvent> shutdown_event_ GUARDED_BY(shutdown_lock_);

  // Used to generate unique |PendingTask::sequence_num| when posting tasks.
  AtomicSequenceNumber sequence_nums_;

  // Ensures all state (e.g. dangling cleaned up workers) is coalesced before
  // destroying the TaskTracker (e.g. in test environments).
  // Ref. https://crbug.com/827615.
  TrackedRefFactory<TaskTracker> tracked_ref_factory_;
};

}  // namespace internal
}  // namespace base

#endif  // BASE_TASK_THREAD_POOL_TASK_TRACKER_H_
