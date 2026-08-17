// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_THREAD_GROUP_IMPL_H_
#define BASE_TASK_THREAD_POOL_THREAD_GROUP_IMPL_H_

#include <optional>
#include <string_view>
#include <vector>

#include "base/base_export.h"
#include "base/gtest_prod_util.h"
#include "base/profiler/thread_group_profiler.h"
#include "base/synchronization/condition_variable.h"
#include "base/synchronization/waitable_event.h"
#include "base/task/thread_pool/task_source.h"
#include "base/task/thread_pool/thread_group.h"
#include "base/task/thread_pool/tracked_ref.h"
#include "base/task/thread_pool/worker_thread.h"
#include "base/task/thread_pool/worker_thread_set.h"
#include "base/time/time.h"

namespace base {

class WorkerThreadObserver;
class ThreadGroupProfiler;

namespace internal {

class TaskTracker;

// A group of |WorkerThread|s that run |Task|s.
//
// The thread group doesn't create threads until Start() is called. Tasks can be
// posted at any time but will not run until after Start() is called.
//
// This class is thread-safe.
class BASE_EXPORT ThreadGroupImpl : public ThreadGroup {
 public:
  // Constructs a group without workers.
  //
  // |histogram_label| is used to label the thread group's histograms as
  // "ThreadPool." + histogram_name + "." + |histogram_label| + extra suffixes.
  // It must not be empty. |thread group_label| is used to label the thread
  // group's threads, it must not be empty. |thread_type_hint| is the preferred
  // thread type; the actual thread type depends on shutdown state and platform
  // capabilities. |thread_group_type| is used for thread group profiler to tag
  // the profiles collected on this group. |task_tracker| keeps track of tasks.
  ThreadGroupImpl(std::string_view histogram_label,
                  std::string_view thread_group_label,
                  ThreadType thread_type_hint,
                  int64_t thread_group_type,
                  TrackedRef<TaskTracker> task_tracker,
                  TrackedRef<Delegate> delegate);

  ThreadGroupImpl(const ThreadGroupImpl&) = delete;
  ThreadGroupImpl& operator=(const ThreadGroupImpl&) = delete;
  // Destroying a ThreadGroupImpl returned by Create() is not allowed
  // in production; it is always leaked. In tests, it can only be destroyed
  // after JoinForTesting() has returned.
  ~ThreadGroupImpl() override;

  // ThreadGroup:
  void Start(size_t max_tasks,
             size_t max_best_effort_tasks,
             TimeDelta suggested_reclaim_time,
             scoped_refptr<SingleThreadTaskRunner> service_thread_task_runner,
             WorkerThreadObserver* worker_thread_observer,
             WorkerEnvironment worker_environment,
             bool synchronous_thread_start_for_testing,
             std::optional<TimeDelta> may_block_threshold) override;
  void Start(size_t max_tasks,
             size_t max_best_effort_tasks,
             TimeDelta suggested_reclaim_time,
             scoped_refptr<SingleThreadTaskRunner> service_thread_task_runner,
             WorkerThreadObserver* worker_thread_observer,
             WorkerEnvironment worker_environment,
             bool synchronous_thread_start_for_testing = false) {
    Start(max_tasks, max_best_effort_tasks, suggested_reclaim_time,
          service_thread_task_runner, worker_thread_observer,
          worker_environment, synchronous_thread_start_for_testing, {});
  }
  void JoinForTesting() override;
  void DidUpdateCanRunPolicy() override;
  void OnShutdownStarted() override;
  // Returns the number of workers that are idle (i.e. not running tasks).
  size_t NumberOfIdleWorkersLockRequiredForTesting() const
      EXCLUSIVE_LOCKS_REQUIRED(lock_) override;

 private:
  class ScopedCommandsExecutor;
  class WorkerDelegate;
  friend class WorkerDelegate;

  // friend tests so that they can access |blocked_workers_poll_period| and
  // may_block_threshold(), both in ThreadGroup.
  friend class ThreadGroupImplBlockingTest;
  friend class ThreadGroupImplMayBlockTest;
  FRIEND_TEST_ALL_PREFIXES(ThreadGroupImplBlockingTest,
                           ThreadBlockUnblockPremature);
  FRIEND_TEST_ALL_PREFIXES(ThreadGroupImplBlockingTest,
                           ThreadBlockUnblockPrematureBestEffort);

  // ThreadGroup:
  void UpdateSortKey(TaskSource::Transaction transaction) override;
  void PushTaskSourceAndWakeUpWorkers(
      RegisteredTaskSourceAndTransaction transaction_with_task_source) override;
  void EnsureEnoughWorkersLockRequired(BaseScopedCommandsExecutor* executor)
      override EXCLUSIVE_LOCKS_REQUIRED(lock_);
  void ScheduleAdjustMaxTasks() override;
  void AdjustMaxTasks() override;

  // Creates a worker and schedules its start, if needed, to maintain one idle
  // worker, |max_tasks_| permitting.
  void MaintainAtLeastOneIdleWorkerLockRequired(
      ScopedCommandsExecutor* executor) EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Creates a worker, adds it to the thread group, schedules its start and
  // returns it. Cannot be called before Start().
  scoped_refptr<WorkerThread> CreateAndRegisterWorkerLockRequired(
      ScopedCommandsExecutor* executor) EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Returns the number of workers that are awake (i.e. not on the idle set).
  size_t GetNumAwakeWorkersLockRequired() const EXCLUSIVE_LOCKS_REQUIRED(lock_);

  bool IsOnIdleSetLockRequired(WorkerThread* worker) const
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  size_t worker_sequence_num_ GUARDED_BY(lock_) = 0;

  // Ordered set of idle workers; the order uses pointer comparison, this is
  // arbitrary but stable. Initially, all workers are on this set. A worker is
  // removed from the set before its WakeUp() function is called and when it
  // receives work from GetWork() (a worker calls GetWork() when its sleep
  // timeout expires, even if its WakeUp() method hasn't been called). A worker
  // is inserted on this set when it receives nullptr from GetWork().
  WorkerThreadSet idle_workers_set_ GUARDED_BY(lock_);

  // This is used in ThreadGroupProfiler to tag as metadata on profiles
  // collected for worker threads within this thread group.
  const int64_t thread_group_type_;

  // This is set in Start() if profiling is enabled, before any worker thread is
  // created. If profiling is not enabled, this will remain std::nullopt. If
  // created the ThreadGroupProfiler instance will exist until ThreadGroupImpl
  // destruction.
  std::optional<ThreadGroupProfiler> thread_group_profiler_;

  // Ensures recently cleaned up workers (ref.
  // WorkerDelegate::CleanupLockRequired()) had time to exit as
  // they have a raw reference to |this| (and to TaskTracker) which can
  // otherwise result in racy use-after-frees per no longer being part of
  // |workers_| and hence not being explicitly joined in JoinForTesting():
  // https://crbug.com/810464. Uses AtomicRefCount to make its only public
  // method thread-safe.
  TrackedRefFactory<ThreadGroupImpl> tracked_ref_factory_;
};

}  // namespace internal
}  // namespace base

#endif  // BASE_TASK_THREAD_POOL_THREAD_GROUP_IMPL_H_
