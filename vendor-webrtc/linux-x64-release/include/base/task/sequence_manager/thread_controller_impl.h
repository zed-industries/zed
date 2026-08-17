// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_THREAD_CONTROLLER_IMPL_H_
#define BASE_TASK_SEQUENCE_MANAGER_THREAD_CONTROLLER_IMPL_H_

#include <memory>

#include "base/base_export.h"
#include "base/cancelable_callback.h"
#include "base/compiler_specific.h"
#include "base/dcheck_is_on.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/run_loop.h"
#include "base/sequence_checker.h"
#include "base/task/common/task_annotator.h"
#include "base/task/sequence_manager/thread_controller.h"
#include "base/task/sequence_manager/work_deduplicator.h"
#include "base/task/single_thread_task_runner.h"
#include "build/build_config.h"

namespace base {
namespace sequence_manager {
namespace internal {
class SequenceManagerImpl;

// This is the interface between a SequenceManager which sits on top of an
// underlying SequenceManagerImpl or SingleThreadTaskRunner. Currently it's only
// used for workers in blink although we'd intend to migrate those to
// ThreadControllerWithMessagePumpImpl (https://crbug.com/948051). Long term we
// intend to use this for sequence funneling.
class BASE_EXPORT ThreadControllerImpl : public ThreadController,
                                         public RunLoop::NestingObserver {
 public:
  ThreadControllerImpl(const ThreadControllerImpl&) = delete;
  ThreadControllerImpl& operator=(const ThreadControllerImpl&) = delete;
  ~ThreadControllerImpl() override;

  // TODO(crbug.com/40620995): replace |funneled_sequence_manager| with
  // |funneled_task_runner| when we sort out the workers
  static std::unique_ptr<ThreadControllerImpl> Create(
      SequenceManagerImpl* funneled_sequence_manager,
      const TickClock* time_source);

  // ThreadController:
  void SetWorkBatchSize(int work_batch_size) override;
  void WillQueueTask(PendingTask* pending_task) override;
  void ScheduleWork() override;
  void BindToCurrentThread(std::unique_ptr<MessagePump> message_pump) override;
  void SetNextDelayedDoWork(LazyNow* lazy_now,
                            std::optional<WakeUp> wake_up) override;
  void SetSequencedTaskSource(SequencedTaskSource* sequence) override;
  bool RunsTasksInCurrentSequence() override;
  void SetDefaultTaskRunner(scoped_refptr<SingleThreadTaskRunner>) override;
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
  ThreadControllerImpl(SequenceManagerImpl* sequence_manager,
                       scoped_refptr<SingleThreadTaskRunner> task_runner,
                       const TickClock* time_source);

  const raw_ptr<SequenceManagerImpl> funneled_sequence_manager_;
  const scoped_refptr<SingleThreadTaskRunner> task_runner_;

  raw_ptr<RunLoop::NestingObserver> nesting_observer_ = nullptr;

 private:
  enum class WorkType { kImmediate, kDelayed };

  void DoWork(WorkType work_type);

  // TODO(scheduler-dev): Maybe fold this into the main class and use
  // thread annotations.
  struct MainSequenceOnly {
    MainSequenceOnly();
    ~MainSequenceOnly();

    int work_batch_size_ = 1;

    TimeTicks next_delayed_do_work = TimeTicks::Max();
  };

  MainSequenceOnly main_sequence_only_;
  MainSequenceOnly& main_sequence_only() LIFETIME_BOUND {
    DCHECK_CALLED_ON_VALID_SEQUENCE(associated_thread_->sequence_checker);
    return main_sequence_only_;
  }
  const MainSequenceOnly& main_sequence_only() const LIFETIME_BOUND {
    DCHECK_CALLED_ON_VALID_SEQUENCE(associated_thread_->sequence_checker);
    return main_sequence_only_;
  }

  scoped_refptr<SingleThreadTaskRunner> message_loop_task_runner_;
  RepeatingClosure immediate_do_work_closure_;
  RepeatingClosure delayed_do_work_closure_;
  CancelableRepeatingClosure cancelable_delayed_do_work_closure_;
  raw_ptr<SequencedTaskSource> sequence_ = nullptr;  // Not owned.
  TaskAnnotator task_annotator_;
  WorkDeduplicator work_deduplicator_;

#if DCHECK_IS_ON()
  bool default_task_runner_set_ = false;
#endif

  WeakPtrFactory<ThreadControllerImpl> weak_factory_{this};
};

}  // namespace internal
}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_THREAD_CONTROLLER_IMPL_H_
