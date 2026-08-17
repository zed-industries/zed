// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_THREAD_POOL_IMPL_H_
#define BASE_TASK_THREAD_POOL_THREAD_POOL_IMPL_H_

#include <memory>
#include <optional>
#include <string_view>

#include "base/base_export.h"
#include "base/containers/flat_map.h"
#include "base/dcheck_is_on.h"
#include "base/functional/callback.h"
#include "base/memory/ptr_util.h"
#include "base/sequence_checker.h"
#include "base/synchronization/atomic_flag.h"
#include "base/task/single_thread_task_runner_thread_mode.h"
#include "base/task/task_traits.h"
#include "base/task/thread_pool/delayed_task_manager.h"
#include "base/task/thread_pool/environment_config.h"
#include "base/task/thread_pool/pooled_sequenced_task_runner.h"
#include "base/task/thread_pool/pooled_single_thread_task_runner_manager.h"
#include "base/task/thread_pool/pooled_task_runner_delegate.h"
#include "base/task/thread_pool/service_thread.h"
#include "base/task/thread_pool/task_source.h"
#include "base/task/thread_pool/task_tracker.h"
#include "base/task/thread_pool/thread_group.h"
#include "base/task/thread_pool/thread_pool_instance.h"
#include "base/task/updateable_sequenced_task_runner.h"
#include "base/thread_annotations.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_WIN)
#include "base/win/com_init_check_hook.h"
#endif

namespace base {

namespace internal {

// Default ThreadPoolInstance implementation. This class is thread-safe, except
// for methods noted otherwise in thread_pool_instance.h.
class BASE_EXPORT ThreadPoolImpl : public ThreadPoolInstance,
                                   public ThreadGroup::Delegate,
                                   public PooledTaskRunnerDelegate {
 public:
  using TaskTrackerImpl = TaskTracker;

  // Creates a ThreadPoolImpl with a production TaskTracker. |histogram_label|
  // is used to label histograms. No histograms are recorded if it is empty.
  explicit ThreadPoolImpl(std::string_view histogram_label);

  // For testing only. Creates a ThreadPoolImpl with a custom TaskTracker.
  // If |!use_background_threads|, background threads will run with default
  // priority.
  ThreadPoolImpl(std::string_view histogram_label,
                 std::unique_ptr<TaskTrackerImpl> task_tracker,
                 bool use_background_threads = true);

  ThreadPoolImpl(const ThreadPoolImpl&) = delete;
  ThreadPoolImpl& operator=(const ThreadPoolImpl&) = delete;
  ~ThreadPoolImpl() override;

  // ThreadPoolInstance:
  void Start(const ThreadPoolInstance::InitParams& init_params,
             WorkerThreadObserver* worker_thread_observer) override;
  bool WasStarted() const final;
  bool WasStartedUnsafe() const final;
  size_t GetMaxConcurrentNonBlockedTasksWithTraitsDeprecated(
      const TaskTraits& traits) const override;
  void Shutdown() override;
  void FlushForTesting() override;
  void FlushAsyncForTesting(OnceClosure flush_callback) override;
  void JoinForTesting() override;
  void BeginFence() override;
  void EndFence() override;
  void BeginBestEffortFence() override;
  void EndBestEffortFence() override;
  void BeginRestrictedTasks() override;
  void EndRestrictedTasks() override;
  void BeginFizzlingBlockShutdownTasks() override;
  void EndFizzlingBlockShutdownTasks() override;

  // PooledTaskRunnerDelegate:
  bool EnqueueJobTaskSource(scoped_refptr<JobTaskSource> task_source) override;
  void RemoveJobTaskSource(scoped_refptr<JobTaskSource> task_source) override;
  void UpdatePriority(scoped_refptr<TaskSource> task_source,
                      TaskPriority priority) override;
  void UpdateJobPriority(scoped_refptr<TaskSource> task_source,
                         TaskPriority priority) override;

  // Returns the TimeTicks of the next task scheduled on ThreadPool (Now() if
  // immediate, nullopt if none). This is thread-safe, i.e., it's safe if tasks
  // are being posted in parallel with this call but such a situation obviously
  // results in a race as to whether this call will see the new tasks in time.
  std::optional<TimeTicks> NextScheduledRunTimeForTesting() const;

  // Forces ripe delayed tasks to be posted (e.g. when time is mocked and
  // advances faster than the real-time delay on ServiceThread).
  void ProcessRipeDelayedTasksForTesting();

  // Requests that all threads started by future ThreadPoolImpls in this process
  // have a synchronous start (if |enabled|; cancels this behavior otherwise).
  // Must be called while no ThreadPoolImpls are alive in this process. This is
  // exposed here on this internal API rather than as a ThreadPoolInstance
  // configuration param because only one internal test truly needs this.
  static void SetSynchronousThreadStartForTesting(bool enabled);

  // Posts |task| with a |delay| and specific |traits|. |delay| can be zero. For
  // one off tasks that don't require a TaskRunner. Returns false if the task
  // definitely won't run because of current shutdown state.
  bool PostDelayedTask(const Location& from_here,
                       const TaskTraits& traits,
                       OnceClosure task,
                       TimeDelta delay);

  // Returns a TaskRunner whose PostTask invocations result in scheduling tasks
  // using |traits|. Tasks may run in any order and in parallel.
  scoped_refptr<TaskRunner> CreateTaskRunner(const TaskTraits& traits);

  // Returns a SequencedTaskRunner whose PostTask invocations result in
  // scheduling tasks using |traits|. Tasks run one at a time in posting order.
  scoped_refptr<SequencedTaskRunner> CreateSequencedTaskRunner(
      const TaskTraits& traits);

  // Returns a SingleThreadTaskRunner whose PostTask invocations result in
  // scheduling tasks using |traits|. Tasks run on a single thread in posting
  // order. If |traits| identifies an existing thread,
  // SingleThreadTaskRunnerThreadMode::SHARED must be used.
  scoped_refptr<SingleThreadTaskRunner> CreateSingleThreadTaskRunner(
      const TaskTraits& traits,
      SingleThreadTaskRunnerThreadMode thread_mode);

#if BUILDFLAG(IS_WIN)
  // Returns a SingleThreadTaskRunner whose PostTask invocations result in
  // scheduling tasks using |traits| in a COM Single-Threaded Apartment. Tasks
  // run in the same Single-Threaded Apartment in posting order for the returned
  // SingleThreadTaskRunner. If |traits| identifies an existing thread,
  // SingleThreadTaskRunnerThreadMode::SHARED must be used.
  scoped_refptr<SingleThreadTaskRunner> CreateCOMSTATaskRunner(
      const TaskTraits& traits,
      SingleThreadTaskRunnerThreadMode thread_mode);
#endif  // BUILDFLAG(IS_WIN)

  // Returns a task runner whose PostTask invocations result in scheduling tasks
  // using |traits|. The priority in |traits| can be updated at any time via
  // UpdateableSequencedTaskRunner::UpdatePriority(). An update affects all
  // tasks posted to the task runner that aren't running yet. Tasks run one at a
  // time in posting order.
  //
  // |traits| requirements:
  // - base::ThreadPolicy must be specified if the priority of the task runner
  //   will ever be increased from BEST_EFFORT.
  scoped_refptr<UpdateableSequencedTaskRunner>
  CreateUpdateableSequencedTaskRunner(const TaskTraits& traits);

  // Returns a SequencedTaskRunner whose PostTask invocations result in
  // scheduling tasks using |traits|. Tasks run one at a time in posting order.
  // Returns the existing `SequenceTaskRunner` for 'path', or creates it.
  // Ensures tasks accessing the same `path` are sequenced, even if posted from
  // `SequencedTaskRunner`s obtained in different contexts. The same `traits`
  // must be provided to all calls with the same `path`.
  scoped_refptr<SequencedTaskRunner> CreateSequencedTaskRunnerForResource(
      const TaskTraits& traits,
      const base::FilePath& path);

 private:
  // Invoked after |num_fences_| or |num_best_effort_fences_| is updated. Sets
  // the CanRunPolicy in TaskTracker and wakes up workers as appropriate.
  void UpdateCanRunPolicy();

  const ThreadGroup* GetThreadGroupForTraits(const TaskTraits& traits) const;

  // ThreadGroup::Delegate:
  ThreadGroup* GetThreadGroupForTraits(const TaskTraits& traits) override;

  // Posts |task| to be executed by the appropriate thread group as part of
  // |sequence|. This must only be called after |task| has gone through
  // TaskTracker::WillPostTask() and after |task|'s delayed run time.
  bool PostTaskWithSequenceNow(Task task, scoped_refptr<Sequence> sequence);

  // PooledTaskRunnerDelegate:
  bool PostTaskWithSequence(Task task,
                            scoped_refptr<Sequence> sequence) override;
  bool ShouldYield(const TaskSource* task_source) override;

  const std::string histogram_label_;
  const std::unique_ptr<TaskTrackerImpl> task_tracker_;
  ServiceThread service_thread_;
  DelayedTaskManager delayed_task_manager_;
  PooledSingleThreadTaskRunnerManager single_thread_task_runner_manager_;

  std::unique_ptr<ThreadGroup> foreground_thread_group_;
  std::unique_ptr<ThreadGroup> utility_thread_group_;
  std::unique_ptr<ThreadGroup> background_thread_group_;

  // Whether this TaskScheduler was started.
  bool started_ GUARDED_BY_CONTEXT(sequence_checker_) = false;

  // Whether the --disable-best-effort-tasks switch is preventing execution of
  // BEST_EFFORT tasks until shutdown.
  const bool has_disable_best_effort_switch_;

  // Number of fences preventing execution of tasks of any/BEST_EFFORT priority.
  int num_fences_ GUARDED_BY_CONTEXT(sequence_checker_) = 0;
  int num_best_effort_fences_ GUARDED_BY_CONTEXT(sequence_checker_) = 0;

#if DCHECK_IS_ON()
  // Set once JoinForTesting() has returned.
  AtomicFlag join_for_testing_returned_;
#endif

#if BUILDFLAG(IS_WIN) && defined(COM_INIT_CHECK_HOOK_ENABLED)
  // Provides COM initialization verification for supported builds.
  base::win::ComInitCheckHook com_init_check_hook_;
#endif

  base::Lock sequences_for_resources_lock_;
  base::flat_map<base::FilePath, scoped_refptr<PooledSequencedTaskRunner>>
      sequences_for_resources_ GUARDED_BY(sequences_for_resources_lock_);

  // Asserts that operations occur in sequence with Start().
  SEQUENCE_CHECKER(sequence_checker_);

  TrackedRefFactory<ThreadGroup::Delegate> tracked_ref_factory_;
};

}  // namespace internal
}  // namespace base

#endif  // BASE_TASK_THREAD_POOL_THREAD_POOL_IMPL_H_
