// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_THREAD_GROUP_H_
#define BASE_TASK_THREAD_POOL_THREAD_GROUP_H_

#include <stddef.h>

#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <vector>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/dcheck_is_on.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/memory/stack_allocated.h"
#include "base/task/common/checked_lock.h"
#include "base/task/thread_pool/priority_queue.h"
#include "base/task/thread_pool/task.h"
#include "base/task/thread_pool/task_source.h"
#include "base/task/thread_pool/tracked_ref.h"
#include "base/task/thread_pool/worker_thread.h"
#include "build/build_config.h"
#include "third_party/abseil-cpp/absl/container/inlined_vector.h"

#if BUILDFLAG(IS_WIN)
#include "base/win/scoped_windows_thread_environment.h"
#endif

namespace base {

class WorkerThreadObserver;

namespace internal {

class TaskTracker;

// Interface and base implementation for a thread group. A thread group is a
// subset of the threads in the thread pool (see GetThreadGroupForTraits() for
// thread group selection logic when posting tasks and creating task runners).
//
// This class is thread-safe.
class BASE_EXPORT ThreadGroup {
 public:
  // Delegate interface for ThreadGroup.
  class BASE_EXPORT Delegate {
   public:
    virtual ~Delegate() = default;

    // Invoked when a TaskSource with |traits| is non-empty after the
    // ThreadGroup has run a task from it. The implementation must return the
    // thread group in which the TaskSource should be reenqueued.
    virtual ThreadGroup* GetThreadGroupForTraits(const TaskTraits& traits) = 0;
  };

  enum class WorkerEnvironment {
    // No special worker environment required.
    NONE,
#if BUILDFLAG(IS_WIN)
    // Initialize a COM MTA on the worker.
    COM_MTA,
#endif  // BUILDFLAG(IS_WIN)
  };

  ThreadGroup(const ThreadGroup&) = delete;
  ThreadGroup& operator=(const ThreadGroup&) = delete;
  virtual ~ThreadGroup();

  // Creates threads, allowing existing and future tasks to run. The thread
  // group runs at most `max_tasks` / `max_best_effort_tasks` unblocked task
  // with any / BEST_EFFORT priority concurrently. It reclaims unused threads
  // after `suggested_reclaim_time`. It uses `service_thread_task_runner` to
  // monitor for blocked tasks, `service_thread_task_runner` is used to setup
  // FileDescriptorWatcher on worker threads. It must refer to a Thread with
  // MessagePumpType::IO. If specified, it notifies `worker_thread_observer`
  // when a worker enters and exits its main function (the observer must not be
  // destroyed before JoinForTesting() has returned). `worker_environment`
  // specifies the environment in which tasks are executed.
  // `may_block_threshold` is the timeout after which a task in a MAY_BLOCK
  // ScopedBlockingCall is considered blocked (the thread group will choose an
  // appropriate value if none is specified).
  // `synchronous_thread_start_for_testing` is true if this ThreadGroup should
  // synchronously wait for OnMainEntry() after starting each worker. Can only
  // be called once. CHECKs on failure.
  virtual void Start(
      size_t max_tasks,
      size_t max_best_effort_tasks,
      TimeDelta suggested_reclaim_time,
      scoped_refptr<SingleThreadTaskRunner> service_thread_task_runner,
      WorkerThreadObserver* worker_thread_observer,
      WorkerEnvironment worker_environment,
      bool synchronous_thread_start_for_testing,
      std::optional<TimeDelta> may_block_threshold) = 0;

  // Registers the thread group in TLS.
  void BindToCurrentThread();

  // Resets the thread group in TLS.
  void UnbindFromCurrentThread();

  // Returns true if the thread group is registered in TLS.
  bool IsBoundToCurrentThread() const;

  // Sets a new maximum number of concurrent tasks, subject to adjustments for
  // blocking tasks.
  void SetMaxTasks(size_t max_tasks);

  // Resets the maximum number of concurrent tasks to the default provided
  // in constructor, subject to adjustments for blocking tasks.
  void ResetMaxTasks();

  // Removes |task_source| from |priority_queue_|. Returns a
  // RegisteredTaskSource that evaluats to true if successful, or false if
  // |task_source| is not currently in |priority_queue_|, such as when a worker
  // is running a task from it.
  RegisteredTaskSource RemoveTaskSource(const TaskSource& task_source);

  // Updates the position of the TaskSource in |transaction| in this
  // ThreadGroup's PriorityQueue based on the TaskSource's current traits.
  //
  // Implementations should instantiate a concrete ScopedCommandsExecutor and
  // invoke UpdateSortKeyImpl().
  virtual void UpdateSortKey(TaskSource::Transaction transaction) = 0;

  // Pushes the TaskSource in |transaction_with_task_source| into this
  // ThreadGroup's PriorityQueue and wakes up workers as appropriate.
  //
  // Implementations should instantiate a concrete ScopedCommandsExecutor and
  // invoke PushTaskSourceAndWakeUpWorkersImpl().
  virtual void PushTaskSourceAndWakeUpWorkers(
      RegisteredTaskSourceAndTransaction transaction_with_task_source) = 0;

  // Move all task sources from this ThreadGroup's PriorityQueue to the
  // |destination_thread_group|'s.
  void HandoffAllTaskSourcesToOtherThreadGroup(
      ThreadGroup* destination_thread_group);
  // Move all task sources except the ones with TaskPriority::USER_BLOCKING,
  // from this ThreadGroup's PriorityQueue to the |destination_thread_group|'s.
  void HandoffNonUserBlockingTaskSourcesToOtherThreadGroup(
      ThreadGroup* destination_thread_group);

  // Returns true if a task with |sort_key| running in this thread group should
  // return ASAP, either because its priority is not allowed to run or because
  // work of higher priority is pending. Thread-safe but may return an outdated
  // result (if a task unnecessarily yields due to this, it will simply be
  // re-scheduled).
  bool ShouldYield(TaskSourceSortKey sort_key);

  // Prevents new tasks from starting to run and waits for currently running
  // tasks to complete their execution. It is guaranteed that no thread will do
  // work on behalf of this ThreadGroup after this returns. It is
  // invalid to post a task once this is called. TaskTracker::Flush() can be
  // called before this to complete existing tasks, which might otherwise post a
  // task during JoinForTesting(). This can only be called once.
  virtual void JoinForTesting() = 0;

  // Returns the maximum number of non-blocked tasks that can run concurrently
  // in this ThreadGroup.
  //
  // TODO(fdoray): Remove this method. https://crbug.com/687264
  virtual size_t GetMaxConcurrentNonBlockedTasksDeprecated() const;

  // Wakes up workers as appropriate for the new CanRunPolicy policy. Must be
  // called after an update to CanRunPolicy in TaskTracker.
  virtual void DidUpdateCanRunPolicy() = 0;

  virtual void OnShutdownStarted() = 0;

  // Returns true if a thread group is registered in TLS. Used by diagnostic
  // code to check whether it's inside a ThreadPool task.
  static bool CurrentThreadHasGroup();

  // Returns |max_tasks_|/|max_best_effort_tasks_|.
  size_t GetMaxTasksForTesting() const;
  size_t GetMaxBestEffortTasksForTesting() const;

  // Waits until at least |n| workers are idle. Note that while workers are
  // disallowed from cleaning up during this call: tests using a custom
  // |suggested_reclaim_time_| need to be careful to invoke this swiftly after
  // unblocking the waited upon workers as: if a worker is already detached by
  // the time this is invoked, it will never make it onto the idle set and
  // this call will hang.
  void WaitForWorkersIdleForTesting(size_t n);

  // Waits until at least |n| workers are idle.
  void WaitForWorkersIdleLockRequiredForTesting(size_t n)
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Waits until all workers are idle.
  void WaitForAllWorkersIdleForTesting();

  // Waits until |n| workers have cleaned up (went through
  // WorkerThread::Delegate::OnMainExit()) since the last call to
  // WaitForWorkersCleanedUpForTesting() (or Start() if that wasn't called yet).
  void WaitForWorkersCleanedUpForTesting(size_t n);

  // Returns the number of workers in this thread group.
  size_t NumberOfWorkersForTesting() const;
  // Returns the number of workers that are idle (i.e. not running tasks).
  size_t NumberOfIdleWorkersForTesting() const;
  // Returns the number of workers that are idle (i.e. not running tasks).
  virtual size_t NumberOfIdleWorkersLockRequiredForTesting() const
      EXCLUSIVE_LOCKS_REQUIRED(lock_) = 0;

  class ThreadGroupWorkerDelegate;

 protected:
  static constexpr size_t kMaxNumberOfWorkers = 256;

  ThreadGroup(std::string_view histogram_label,
              std::string_view thread_group_label,
              ThreadType thread_type_hint,
              TrackedRef<TaskTracker> task_tracker,
              TrackedRef<Delegate> delegate);

  void StartImplLockRequired(
      size_t max_tasks,
      size_t max_best_effort_tasks,
      TimeDelta suggested_reclaim_time,
      scoped_refptr<SingleThreadTaskRunner> service_thread_task_runner,
      WorkerThreadObserver* worker_thread_observer,
      WorkerEnvironment worker_environment,
      bool synchronous_thread_start_for_testing = false,
      std::optional<TimeDelta> may_block_threshold = std::optional<TimeDelta>())
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Derived classes must implement a ScopedCommandsExecutor that derives from
  // this to perform operations at the end of a scope, when all locks have been
  // released.
  class BaseScopedCommandsExecutor {
   public:
    BaseScopedCommandsExecutor(const BaseScopedCommandsExecutor&) = delete;
    BaseScopedCommandsExecutor& operator=(const BaseScopedCommandsExecutor&) =
        delete;
    virtual ~BaseScopedCommandsExecutor();

    void ScheduleStart(scoped_refptr<WorkerThread> worker);
    void ScheduleAdjustMaxTasks();
    void ScheduleReleaseTaskSource(RegisteredTaskSource task_source);

   protected:
    explicit BaseScopedCommandsExecutor(ThreadGroup* outer);

    // RAW_PTR_EXCLUSION: Performance: visible in sampling profiler and stack
    // scoped, also a back-pointer to the owning object.
    RAW_PTR_EXCLUSION ThreadGroup* outer_ = nullptr;

   protected:
    // Performs BaseScopedCommandsExecutor-related tasks, must be called in this
    // class' destructor.
    void Flush();

    std::vector<RegisteredTaskSource> task_sources_to_release_;
    absl::InlinedVector<scoped_refptr<WorkerThread>, 2> workers_to_start_;
    bool must_schedule_adjust_max_tasks_ = false;
  };

  // Allows a task source to be pushed to a ThreadGroup's PriorityQueue at the
  // end of a scope, when all locks have been released.
  class ScopedReenqueueExecutor {
    STACK_ALLOCATED();

   public:
    ScopedReenqueueExecutor();
    ScopedReenqueueExecutor(const ScopedReenqueueExecutor&) = delete;
    ScopedReenqueueExecutor& operator=(const ScopedReenqueueExecutor&) = delete;
    ~ScopedReenqueueExecutor();

    // A RegisteredTaskSourceAndTransaction and the ThreadGroup in which it
    // should be enqueued.
    void SchedulePushTaskSourceAndWakeUpWorkers(
        RegisteredTaskSourceAndTransaction transaction_with_task_source,
        ThreadGroup* destination_thread_group);

   private:
    // A RegisteredTaskSourceAndTransaction and the thread group in which it
    // should be enqueued.
    std::optional<RegisteredTaskSourceAndTransaction>
        transaction_with_task_source_;
    ThreadGroup* destination_thread_group_ = nullptr;
  };

  ThreadGroup(TrackedRef<TaskTracker> task_tracker,
              TrackedRef<Delegate> delegate);

#if BUILDFLAG(IS_WIN)
  static std::unique_ptr<win::ScopedWindowsThreadEnvironment>
  GetScopedWindowsThreadEnvironment(WorkerEnvironment environment);
#endif

  const TrackedRef<TaskTracker> task_tracker_;
  const TrackedRef<Delegate> delegate_;

  // Returns the number of workers required of workers to run all queued
  // BEST_EFFORT task sources allowed to run by the current CanRunPolicy.
  size_t GetNumAdditionalWorkersForBestEffortTaskSourcesLockRequired() const
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Returns the number of workers required to run all queued
  // USER_VISIBLE/USER_BLOCKING task sources allowed to run by the current
  // CanRunPolicy.
  size_t GetNumAdditionalWorkersForForegroundTaskSourcesLockRequired() const
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Ensures that there are enough workers to run queued task sources.
  // |executor| is forwarded from the one received in
  // PushTaskSourceAndWakeUpWorkersImpl()
  virtual void EnsureEnoughWorkersLockRequired(
      BaseScopedCommandsExecutor* executor) EXCLUSIVE_LOCKS_REQUIRED(lock_) = 0;

  // Reenqueues a |transaction_with_task_source| from which a Task just ran in
  // the current ThreadGroup into the appropriate ThreadGroup.
  void ReEnqueueTaskSourceLockRequired(
      BaseScopedCommandsExecutor* workers_executor,
      ScopedReenqueueExecutor* reenqueue_executor,
      RegisteredTaskSourceAndTransaction transaction_with_task_source)
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Returns the next task source from |priority_queue_| if permitted to run and
  // pops |priority_queue_| if the task source returned no longer needs to be
  // queued (reached its maximum concurrency). Otherwise returns nullptr and
  // pops |priority_queue_| so this can be called again.
  RegisteredTaskSource TakeRegisteredTaskSource(
      BaseScopedCommandsExecutor* executor) EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Must be invoked by implementations of the corresponding non-Impl() methods.
  void UpdateSortKeyImpl(BaseScopedCommandsExecutor* executor,
                         TaskSource::Transaction transaction);
  void PushTaskSourceAndWakeUpWorkersImpl(
      BaseScopedCommandsExecutor* executor,
      RegisteredTaskSourceAndTransaction transaction_with_task_source);

  // Returns the desired number of awake workers, given current workload and
  // concurrency limits.
  size_t GetDesiredNumAwakeWorkersLockRequired() const
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Enqueues all task sources from `new_priority_queue` into this thread group.
  void EnqueueAllTaskSources(PriorityQueue* new_priority_queue);

  // Returns the threshold after which the max tasks is increased to compensate
  // for a worker that is within a MAY_BLOCK ScopedBlockingCall.
  TimeDelta may_block_threshold_for_testing() const {
    return after_start().may_block_threshold;
  }

  // Interval at which the service thread checks for workers in this thread
  // group that have been in a MAY_BLOCK ScopedBlockingCall for more than
  // may_block_threshold().
  TimeDelta blocked_workers_poll_period_for_testing() const {
    return after_start().blocked_workers_poll_period;
  }

  // Schedules AdjustMaxTasks() if required.
  void MaybeScheduleAdjustMaxTasksLockRequired(
      BaseScopedCommandsExecutor* executor) EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Starts calling AdjustMaxTasks() periodically on
  // |service_thread_task_runner_|.
  virtual void ScheduleAdjustMaxTasks() = 0;

  // Examines the list of WorkerThreads and increments |max_tasks_| for each
  // worker that has been within the scope of a MAY_BLOCK ScopedBlockingCall for
  // more than BlockedThreshold(). Reschedules a call if necessary.
  virtual void AdjustMaxTasks() = 0;

  // Returns true if AdjustMaxTasks() should periodically be called on
  // |service_thread_task_runner_|.
  bool ShouldPeriodicallyAdjustMaxTasksLockRequired()
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Updates the minimum priority allowed to run below which tasks should yield.
  // This should be called whenever |num_running_tasks_| or |max_tasks| changes,
  // or when a new task is added to |priority_queue_|.
  void UpdateMinAllowedPriorityLockRequired() EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Increments/decrements the number of tasks of |priority| that are currently
  // running in this thread group. Must be invoked before/after running a task.
  void DecrementTasksRunningLockRequired(TaskPriority priority)
      EXCLUSIVE_LOCKS_REQUIRED(lock_);
  void IncrementTasksRunningLockRequired(TaskPriority priority)
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Increments/decrements the number of [best effort] tasks that can run in
  // this thread group.
  void DecrementMaxTasksLockRequired() EXCLUSIVE_LOCKS_REQUIRED(lock_);
  void IncrementMaxTasksLockRequired() EXCLUSIVE_LOCKS_REQUIRED(lock_);
  void DecrementMaxBestEffortTasksLockRequired()
      EXCLUSIVE_LOCKS_REQUIRED(lock_);
  void IncrementMaxBestEffortTasksLockRequired()
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Values set at Start() and never modified afterwards.
  struct InitializedInStart {
    InitializedInStart();
    ~InitializedInStart();

#if DCHECK_IS_ON()
    // Set after all members of this struct are set to ensure
    // `InitializedInStart` is read-only after initialization.
    bool initialized = false;
    // Set to ensure Start() is only called once and that `ThreadGroup`
    // operations only occur after it is called.
    bool start_called = false;
#endif

    // Initial value of |max_tasks_|.
    size_t initial_max_tasks = 0;

    // Suggested reclaim time for workers.
    TimeDelta suggested_reclaim_time;

    // Environment to be initialized per worker.
    WorkerEnvironment worker_environment = WorkerEnvironment::NONE;

    scoped_refptr<SingleThreadTaskRunner> service_thread_task_runner;

    // Optional observer notified when a worker enters and exits its main.
    raw_ptr<WorkerThreadObserver> worker_thread_observer = nullptr;

    // Threshold after which the max tasks is increased to compensate for a
    // worker that is within a MAY_BLOCK ScopedBlockingCall.
    TimeDelta may_block_threshold;

    // The period between calls to AdjustMaxTasks() when the thread group is at
    // capacity.
    TimeDelta blocked_workers_poll_period;
  } initialized_in_start_;

  InitializedInStart& in_start() LIFETIME_BOUND {
#if DCHECK_IS_ON()
    DCHECK(!initialized_in_start_.initialized);
#endif
    return initialized_in_start_;
  }
  const InitializedInStart& after_start() const LIFETIME_BOUND {
#if DCHECK_IS_ON()
    DCHECK(initialized_in_start_.initialized);
#endif
    return initialized_in_start_;
  }

  // Synchronizes accesses to all members of this class which are neither const,
  // atomic, nor immutable after start. Since this lock is a bottleneck to post
  // and schedule work, only simple data structure manipulations are allowed
  // within its scope (no thread creation or wake up).
  mutable CheckedLock lock_{};

  bool disable_fair_scheduling_ GUARDED_BY(lock_){false};

  // PriorityQueue from which all threads of this ThreadGroup get work.
  PriorityQueue priority_queue_ GUARDED_BY(lock_);

  struct YieldSortKey {
    TaskPriority priority;
    uint8_t worker_count;
  };
  // Sort key which compares greater than or equal to any other sort key.
  static constexpr YieldSortKey kMaxYieldSortKey = {TaskPriority::BEST_EFFORT,
                                                    0U};

  // When the thread group is at or above capacity and has pending work, this is
  // set to contain the priority and worker count of the next TaskSource to
  // schedule, or kMaxYieldSortKey otherwise. This is used to decide whether a
  // TaskSource should yield. Once ShouldYield() returns true, it is reset to
  // kMaxYieldSortKey to prevent additional from unnecessary yielding. This is
  // expected to be always kept up-to-date by derived classes when |lock_| is
  // released. It is annotated as GUARDED_BY(lock_) because it is always updated
  // under the lock (to avoid races with other state during the update) but it
  // is nonetheless always safe to read it without the lock (since it's atomic).
  std::atomic<YieldSortKey> max_allowed_sort_key_ GUARDED_BY(lock_){
      kMaxYieldSortKey};

  const std::string histogram_label_;
  const std::string thread_group_label_;
  const ThreadType thread_type_hint_;

  // All workers owned by this thread group.
  size_t worker_sequence_num_ GUARDED_BY(lock_) = 0;

  bool shutdown_started_ GUARDED_BY(lock_) = false;

  // Maximum number of tasks of any priority / BEST_EFFORT priority that can run
  // concurrently in this thread group currently, excluding adjustment for
  // blocking tasks.
  size_t baseline_max_tasks_ GUARDED_BY(lock_) = 0;
  // Same as `baseline_max_tasks_`, but including adjustment for blocking tasks.
  size_t max_tasks_ GUARDED_BY(lock_) = 0;
  size_t max_best_effort_tasks_ GUARDED_BY(lock_) = 0;

  // Number of tasks of any priority / BEST_EFFORT priority that are currently
  // running in this thread group.
  size_t num_running_tasks_ GUARDED_BY(lock_) = 0;
  size_t num_running_best_effort_tasks_ GUARDED_BY(lock_) = 0;

  // Number of workers running a task of any priority / BEST_EFFORT priority
  // that are within the scope of a MAY_BLOCK ScopedBlockingCall but haven't
  // caused a max tasks increase yet.
  int num_unresolved_may_block_ GUARDED_BY(lock_) = 0;
  int num_unresolved_best_effort_may_block_ GUARDED_BY(lock_) = 0;

  // Signaled when a worker is added to the idle workers set.
  ConditionVariable idle_workers_set_cv_for_testing_ GUARDED_BY(lock_);

  // Whether an AdjustMaxTasks() task was posted to the service thread.
  bool adjust_max_tasks_posted_ GUARDED_BY(lock_) = false;

  // Indicates to the delegates that workers are not permitted to cleanup.
  bool worker_cleanup_disallowed_for_testing_ GUARDED_BY(lock_) = false;

  // Counts the number of workers cleaned up (went through
  // WorkerThreadDelegateImpl::OnMainExit()) since the last call to
  // WaitForWorkersCleanedUpForTesting() (or Start() if that wasn't called yet).
  // |some_workers_cleaned_up_for_testing_| is true if this was ever
  // incremented. Tests with a custom |suggested_reclaim_time_| can wait on a
  // specific number of workers being cleaned up via
  // WaitForWorkersCleanedUpForTesting().
  size_t num_workers_cleaned_up_for_testing_ GUARDED_BY(lock_) = 0;
#if DCHECK_IS_ON()
  bool some_workers_cleaned_up_for_testing_ GUARDED_BY(lock_) = false;
#endif

  // Signaled, if non-null, when |num_workers_cleaned_up_for_testing_| is
  // incremented.
  std::optional<ConditionVariable> num_workers_cleaned_up_for_testing_cv_
      GUARDED_BY(lock_);

  // All workers owned by this thread group.
  std::vector<scoped_refptr<WorkerThread>> workers_ GUARDED_BY(lock_);

  // Null-opt unless |synchronous_thread_start_for_testing| was true at
  // construction. In that case, it's signaled each time
  // WorkerThreadDelegateImpl::OnMainEntry() completes.
  std::optional<WaitableEvent> worker_started_for_testing_;

  // Set at the start of JoinForTesting().
  bool join_for_testing_started_ GUARDED_BY(lock_) = false;
};

}  // namespace internal
}  // namespace base

#endif  // BASE_TASK_THREAD_POOL_THREAD_GROUP_H_
