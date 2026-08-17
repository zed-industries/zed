// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_POOLED_SINGLE_THREAD_TASK_RUNNER_MANAGER_H_
#define BASE_TASK_THREAD_POOL_POOLED_SINGLE_THREAD_TASK_RUNNER_MANAGER_H_

#include <memory>
#include <string>
#include <vector>

#include "base/base_export.h"
#include "base/memory/raw_ptr.h"
#include "base/task/common/checked_lock.h"
#include "base/task/single_thread_task_runner_thread_mode.h"
#include "base/task/thread_pool/environment_config.h"
#include "base/task/thread_pool/tracked_ref.h"
#include "base/thread_annotations.h"
#include "base/threading/platform_thread.h"
#include "build/build_config.h"

namespace base {

class TaskTraits;
class WorkerThreadObserver;
class SingleThreadTaskRunner;

namespace internal {

class DelayedTaskManager;
class WorkerThread;
class TaskTracker;

namespace {

class WorkerThreadDelegate;

}  // namespace

// Manages a group of threads which are each associated with one or more
// SingleThreadTaskRunners.
//
// SingleThreadTaskRunners using SingleThreadTaskRunnerThreadMode::SHARED are
// backed by shared WorkerThreads for each COM+task environment combination.
// These workers are lazily instantiated and then only reclaimed during
// JoinForTesting()
//
// No threads are created (and hence no tasks can run) before Start() is called.
//
// This class is thread-safe.
class BASE_EXPORT PooledSingleThreadTaskRunnerManager final {
 public:
  PooledSingleThreadTaskRunnerManager(TrackedRef<TaskTracker> task_tracker,
                                      DelayedTaskManager* delayed_task_manager);
  PooledSingleThreadTaskRunnerManager(
      const PooledSingleThreadTaskRunnerManager&) = delete;
  PooledSingleThreadTaskRunnerManager& operator=(
      const PooledSingleThreadTaskRunnerManager&) = delete;
  ~PooledSingleThreadTaskRunnerManager();

  // Starts threads for existing SingleThreadTaskRunners and allows threads to
  // be started when SingleThreadTaskRunners are created in the future.
  // `io_thread_task_runner` is used to setup FileDescriptorWatcher on worker
  // threads. `io_thread_task_runner` must refer to a Thread with
  // MessgaePumpType::IO. If specified, |worker_thread_observer| will be
  // notified when a worker enters and exits its main function. It must not be
  // destroyed before JoinForTesting() has returned (must never be destroyed in
  // production).
  void Start(scoped_refptr<SingleThreadTaskRunner> io_thread_task_runner,
             WorkerThreadObserver* worker_thread_observer = nullptr);

  // Wakes up workers as appropriate for the new CanRunPolicy policy. Must be
  // called after an update to CanRunPolicy in TaskTracker.
  void DidUpdateCanRunPolicy();

  // Creates a SingleThreadTaskRunner which runs tasks with |traits| on a thread
  // named "ThreadPoolSingleThread[Shared]" +
  // kEnvironmentParams[GetEnvironmentIndexForTraits(traits)].name_suffix +
  // index.
  scoped_refptr<SingleThreadTaskRunner> CreateSingleThreadTaskRunner(
      const TaskTraits& traits,
      SingleThreadTaskRunnerThreadMode thread_mode);

#if BUILDFLAG(IS_WIN)
  // Creates a SingleThreadTaskRunner which runs tasks with |traits| on a COM
  // STA thread named "ThreadPoolSingleThreadCOMSTA[Shared]" +
  // kEnvironmentParams[GetEnvironmentIndexForTraits(traits)].name_suffix +
  // index.
  scoped_refptr<SingleThreadTaskRunner> CreateCOMSTATaskRunner(
      const TaskTraits& traits,
      SingleThreadTaskRunnerThreadMode thread_mode);
#endif  // BUILDFLAG(IS_WIN)

  void JoinForTesting();

 private:
  class PooledSingleThreadTaskRunner;

  enum ContinueOnShutdown {
    IS_CONTINUE_ON_SHUTDOWN,
    IS_NOT_CONTINUE_ON_SHUTDOWN,
    CONTINUE_ON_SHUTDOWN_COUNT,
  };

  static ContinueOnShutdown TraitsToContinueOnShutdown(
      const TaskTraits& traits);

  template <typename DelegateType>
  scoped_refptr<PooledSingleThreadTaskRunner> CreateTaskRunnerImpl(
      const TaskTraits& traits,
      SingleThreadTaskRunnerThreadMode thread_mode);

  template <typename DelegateType>
  std::unique_ptr<WorkerThreadDelegate> CreateWorkerThreadDelegate(
      const std::string& name,
      int id,
      SingleThreadTaskRunnerThreadMode thread_mode);

  template <typename DelegateType>
  WorkerThread* CreateAndRegisterWorkerThread(
      const std::string& name,
      SingleThreadTaskRunnerThreadMode thread_mode,
      ThreadType thread_type_hint) EXCLUSIVE_LOCKS_REQUIRED(lock_);

  template <typename DelegateType>
  WorkerThread*& GetSharedWorkerThreadForTraits(const TaskTraits& traits);

  void UnregisterWorkerThread(WorkerThread* worker);

  void ReleaseSharedWorkerThreads();

  const TrackedRef<TaskTracker> task_tracker_;
  const raw_ptr<DelayedTaskManager> delayed_task_manager_;

  scoped_refptr<SingleThreadTaskRunner> io_thread_task_runner_;

  // Optional observer notified when a worker enters and exits its main
  // function. Set in Start() and never modified afterwards.
  raw_ptr<WorkerThreadObserver> worker_thread_observer_ = nullptr;

  CheckedLock lock_;
  std::vector<scoped_refptr<WorkerThread>> workers_ GUARDED_BY(lock_);
  int next_worker_id_ GUARDED_BY(lock_) = 0;

  // Workers for SingleThreadTaskRunnerThreadMode::SHARED tasks. It is
  // important to have separate threads for CONTINUE_ON_SHUTDOWN and non-
  // CONTINUE_ON_SHUTDOWN to avoid being in a situation where a
  // CONTINUE_ON_SHUTDOWN task effectively blocks shutdown by preventing a
  // BLOCK_SHUTDOWN task to be scheduled. https://crbug.com/829786
  WorkerThread* shared_worker_threads_[ENVIRONMENT_COUNT]
                                      [CONTINUE_ON_SHUTDOWN_COUNT] GUARDED_BY(
                                          lock_) = {};
#if BUILDFLAG(IS_WIN)
  WorkerThread* shared_com_worker_threads_
      [ENVIRONMENT_COUNT][CONTINUE_ON_SHUTDOWN_COUNT] GUARDED_BY(lock_) = {};
#endif  // BUILDFLAG(IS_WIN)

  // Set to true when Start() is called.
  bool started_ GUARDED_BY(lock_) = false;
};

}  // namespace internal
}  // namespace base

#endif  // BASE_TASK_THREAD_POOL_POOLED_SINGLE_THREAD_TASK_RUNNER_MANAGER_H_
