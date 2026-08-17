// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_THREAD_CONTROLLER_WITH_MESSAGE_PUMP_IMPL_H_
#define BASE_TASK_SEQUENCE_MANAGER_THREAD_CONTROLLER_WITH_MESSAGE_PUMP_IMPL_H_

#include <memory>
#include <optional>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/memory/raw_ptr.h"
#include "base/message_loop/message_pump.h"
#include "base/message_loop/work_id_provider.h"
#include "base/run_loop.h"
#include "base/task/common/checked_lock.h"
#include "base/task/common/task_annotator.h"
#include "base/task/sequence_manager/sequence_manager_impl.h"
#include "base/task/sequence_manager/sequenced_task_source.h"
#include "base/task/sequence_manager/thread_controller.h"
#include "base/task/sequence_manager/thread_controller_power_monitor.h"
#include "base/task/sequence_manager/work_deduplicator.h"
#include "base/thread_annotations.h"
#include "base/threading/hang_watcher.h"
#include "base/threading/platform_thread.h"
#include "base/threading/sequence_local_storage_map.h"
#include "build/build_config.h"

namespace base {
namespace sequence_manager {
namespace internal {

// This is the interface between the SequenceManager and the MessagePump.
class BASE_EXPORT ThreadControllerWithMessagePumpImpl
    : public ThreadController,
      public MessagePump::Delegate,
      public RunLoop::Delegate,
      public RunLoop::NestingObserver {
 public:
  static void InitializeFeatures();
  static void ResetFeatures();

  ThreadControllerWithMessagePumpImpl(
      std::unique_ptr<MessagePump> message_pump,
      const SequenceManager::Settings& settings);
  ThreadControllerWithMessagePumpImpl(
      const ThreadControllerWithMessagePumpImpl&) = delete;
  ThreadControllerWithMessagePumpImpl& operator=(
      const ThreadControllerWithMessagePumpImpl&) = delete;
  ~ThreadControllerWithMessagePumpImpl() override;

  using ShouldScheduleWork = WorkDeduplicator::ShouldScheduleWork;

  static std::unique_ptr<ThreadControllerWithMessagePumpImpl> CreateUnbound(
      const SequenceManager::Settings& settings);

  // ThreadController implementation:
  void SetSequencedTaskSource(SequencedTaskSource* task_source) override;
  void BindToCurrentThread(std::unique_ptr<MessagePump> message_pump) override;
  void SetWorkBatchSize(int work_batch_size) override;
  void WillQueueTask(PendingTask* pending_task) override;
  void ScheduleWork() override;
  void SetNextDelayedDoWork(LazyNow* lazy_now,
                            std::optional<WakeUp> wake_up) override;
  bool RunsTasksInCurrentSequence() override;
  void SetDefaultTaskRunner(
      scoped_refptr<SingleThreadTaskRunner> task_runner) override;
  scoped_refptr<SingleThreadTaskRunner> GetDefaultTaskRunner() override;
  void RestoreDefaultTaskRunner() override;
  void AddNestingObserver(RunLoop::NestingObserver* observer) override;
  void RemoveNestingObserver(RunLoop::NestingObserver* observer) override;
  void SetTaskExecutionAllowedInNativeNestedLoop(bool allowed) override;
  bool IsTaskExecutionAllowed() const override;
  MessagePump* GetBoundMessagePump() const override;
#if BUILDFLAG(IS_IOS) || BUILDFLAG(IS_ANDROID)
  void AttachToMessagePump() override;
#endif
#if BUILDFLAG(IS_IOS)
  void DetachFromMessagePump() override;
#endif
  bool ShouldQuitRunLoopWhenIdle() override;

  // RunLoop::NestingObserver:
  void OnBeginNestedRunLoop() override;
  void OnExitNestedRunLoop() override;

 protected:
  explicit ThreadControllerWithMessagePumpImpl(
      const SequenceManager::Settings& settings);

  // MessagePump::Delegate implementation.
  void OnBeginWorkItem() override;
  void OnEndWorkItem(int run_level_depth) override;
  void BeforeWait() override;
  void BeginNativeWorkBeforeDoWork() override;
  MessagePump::Delegate::NextWorkInfo DoWork() override;
  void DoIdleWork() override;
  int RunDepth() override;

  void OnBeginWorkItemImpl(LazyNow& lazy_now);
  void OnEndWorkItemImpl(LazyNow& lazy_now, int run_level_depth);

  // RunLoop::Delegate implementation.
  void Run(bool application_tasks_allowed, TimeDelta timeout) override;
  void Quit() override;
  void EnsureWorkScheduled() override;

  struct MainThreadOnly {
    MainThreadOnly();
    ~MainThreadOnly();

    raw_ptr<SequencedTaskSource> task_source = nullptr;            // Not owned.
    raw_ptr<RunLoop::NestingObserver> nesting_observer = nullptr;  // Not owned.
    std::unique_ptr<SingleThreadTaskRunner::CurrentDefaultHandle>
        thread_task_runner_handle;

    // Indicates that we should yield DoWork between each task to let a possibly
    // nested RunLoop exit.
    bool quit_pending = false;

    // Whether high resolution timing is enabled or not.
    bool in_high_res_mode = false;

    // Number of tasks processed in a single DoWork invocation.
    int work_batch_size = 1;

    bool can_change_batch_size = true;

    // The time after which the runloop should quit.
    TimeTicks quit_runloop_after = TimeTicks::Max();

    bool task_execution_allowed = true;
  };

  const MainThreadOnly& MainThreadOnlyForTesting() const {
    return main_thread_only_;
  }

  ThreadControllerPowerMonitor* ThreadControllerPowerMonitorForTesting() {
    return &power_monitor_;
  }

 private:
  friend class DoWorkScope;
  friend class RunScope;

  // Returns a WakeUp for the next pending task, is_immediate() if the next task
  // can run immediately, or nullopt if there are no more immediate or delayed
  // tasks.
  std::optional<WakeUp> DoWorkImpl(LazyNow* continuation_lazy_now);

  bool RunsTasksByBatches() const;

  void InitializeSingleThreadTaskRunnerCurrentDefaultHandle()
      EXCLUSIVE_LOCKS_REQUIRED(task_runner_lock_);

  MainThreadOnly& main_thread_only() LIFETIME_BOUND {
    DCHECK_CALLED_ON_VALID_THREAD(associated_thread_->thread_checker);
    return main_thread_only_;
  }

  const MainThreadOnly& main_thread_only() const LIFETIME_BOUND {
    DCHECK_CALLED_ON_VALID_THREAD(associated_thread_->thread_checker);
    return main_thread_only_;
  }

  MainThreadOnly main_thread_only_;

  mutable base::internal::CheckedLock task_runner_lock_;
  scoped_refptr<SingleThreadTaskRunner> task_runner_
      GUARDED_BY(task_runner_lock_);

  WorkDeduplicator work_deduplicator_;
  bool do_work_needed_before_wait_ = false;
  bool task_execution_allowed_in_native_nested_loop_ = false;

  ThreadControllerPowerMonitor power_monitor_;

  TaskAnnotator task_annotator_;

  // Non-null provider of id state for identifying distinct work items executed
  // by the message loop (task, event, etc.). Cached on the class to avoid TLS
  // lookups on task execution.
  raw_ptr<WorkIdProvider> work_id_provider_ = nullptr;

  // Required to register the current thread as a sequence. Must be declared
  // after |main_thread_only_| so that the destructors of state stored in the
  // map run while the main thread state is still valid (crbug.com/1221382)
  base::internal::SequenceLocalStorageMap sequence_local_storage_map_;
  std::unique_ptr<
      base::internal::ScopedSetSequenceLocalStorageMapForCurrentThread>
      scoped_set_sequence_local_storage_map_for_current_thread_;

  // Whether tasks can run by batches (i.e. multiple tasks run between each
  // check for native work). Tasks will only run by batches if this is true and
  // the "RunTasksByBatches" feature is enabled.
  bool can_run_tasks_by_batches_ = false;

  // Reset at the start & end of each unit of work to cover the work itself and
  // the overhead between each work item (no-op if HangWatcher is not enabled
  // on this thread). Cleared when going to sleep and at the end of a Run()
  // (i.e. when Quit()). Nested runs override their parent.
  std::optional<WatchHangsInScope> hang_watch_scope_;

  // Can only be set once (just before calling
  // work_deduplicator_.BindToCurrentThread()). After that only read access is
  // allowed.
  // NOTE: |pump_| accesses other members but other members should not access
  // |pump_|. This means that it should be destroyed first. This member cannot
  // be moved up.
  std::unique_ptr<MessagePump> pump_;
};

}  // namespace internal
}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_THREAD_CONTROLLER_WITH_MESSAGE_PUMP_IMPL_H_
