// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FUZZER_THREAD_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FUZZER_THREAD_MANAGER_H_

#include <memory>

#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/synchronization/lock.h"
#include "base/task/sequence_manager/test/sequence_manager_for_test.h"
#include "base/task/single_thread_task_runner.h"
#include "base/test/test_mock_time_task_runner.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/test/fuzzer/proto/sequence_manager_test_description.pb.h"
#include "third_party/blink/renderer/platform/scheduler/test/fuzzer/sequence_manager_fuzzer_processor.h"
#include "third_party/blink/renderer/platform/scheduler/test/fuzzer/task_queue_with_voters.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace base {
namespace sequence_manager {

// Used by the SequenceManagerFuzzerProcessor to execute actions on a thread.
class PLATFORM_EXPORT ThreadManager {
  USING_FAST_MALLOC(ThreadManager);

 public:
  // |initial_time| is the time in which |this| was instantiated.
  ThreadManager(base::TimeTicks initial_time,
                SequenceManagerFuzzerProcessor* processor);

  ~ThreadManager();

  // Returns the time of the underlying task runner.
  base::TimeTicks NowTicks();

  // Returns the delay of the oldest pending task on the thread |this| is bound
  // to.
  base::TimeDelta NextPendingTaskDelay();

  // Advances the clock of the underlying task runner by |delta|.
  void AdvanceMockTickClock(base::TimeDelta delta);

  // Used to create a thread and register it with the thread pool manager owned
  // by |processor_|.
  void ExecuteCreateThreadAction(
      uint64_t action_id,
      const SequenceManagerTestDescription::CreateThreadAction& action);

  // Used by the thread pool manager owned by |processor_| to execute the
  // thread.
  void ExecuteThread(
      const google::protobuf::RepeatedPtrField<
          SequenceManagerTestDescription::Action>& initial_thread_actions);

  const Vector<SequenceManagerFuzzerProcessor::TaskForTest>& ordered_tasks()
      const;

  const Vector<SequenceManagerFuzzerProcessor::ActionForTest>& ordered_actions()
      const;

 protected:
  class Task {
    USING_FAST_MALLOC(Task);

   public:
    explicit Task(ThreadManager* thread_manager);
    ~Task() = default;

    void Execute(const SequenceManagerTestDescription::Task& task);

    bool is_running_;

    // Should outlive |this|.
    raw_ptr<ThreadManager> thread_manager_;
    base::WeakPtrFactory<Task> weak_ptr_factory_{this};
  };

  void RunAction(const SequenceManagerTestDescription::Action& action);

  void ExecuteCreateTaskQueueAction(
      uint64_t action_id,
      const SequenceManagerTestDescription::CreateTaskQueueAction& action);
  void ExecutePostDelayedTaskAction(
      uint64_t action_id,
      const SequenceManagerTestDescription::PostDelayedTaskAction& action);
  void ExecuteCrossThreadPostDelayedTaskAction(
      uint64_t action_id,
      const SequenceManagerTestDescription::CrossThreadPostDelayedTaskAction&
          action);
  void ExecuteSetQueuePriorityAction(
      uint64_t action_id,
      const SequenceManagerTestDescription::SetQueuePriorityAction& action);
  void ExecuteSetQueueEnabledAction(
      uint64_t action_id,
      const SequenceManagerTestDescription::SetQueueEnabledAction& action);
  void ExecuteCreateQueueVoterAction(
      uint64_t action_id,
      const SequenceManagerTestDescription::CreateQueueVoterAction& action);
  void ExecuteShutdownTaskQueueAction(
      uint64_t action_id,
      const SequenceManagerTestDescription::ShutdownTaskQueueAction& action);
  void ExecuteCancelTaskAction(
      uint64_t action_id,
      const SequenceManagerTestDescription::CancelTaskAction& action);
  void ExecuteInsertFenceAction(
      uint64_t action_id,
      const SequenceManagerTestDescription::InsertFenceAction& action);
  void ExecuteRemoveFenceAction(
      uint64_t action_id,
      const SequenceManagerTestDescription::RemoveFenceAction& action);

 private:
  using ActionForTest = SequenceManagerFuzzerProcessor::ActionForTest;
  using TaskForTest = SequenceManagerFuzzerProcessor::TaskForTest;

  void PostDelayedTask(uint64_t task_queue_id,
                       uint32_t delay_ms,
                       const SequenceManagerTestDescription::Task& task);

  void ExecuteTask(const SequenceManagerTestDescription::Task& task);

  // Used to delete |task| from |pending_tasks_|.
  void DeleteTask(Task* task);

  scoped_refptr<TaskQueueWithVoters> GetTaskQueueFor(uint64_t task_queue_id);

  scoped_refptr<SingleThreadTaskRunner> GetTaskRunnerFor(
      uint64_t task_queue_id);

  // Used to protect |task_queues_| and |pending_tasks_|.
  Lock lock_;

  // Bound to the thread in which this object was instantiated. Used to
  // control the clock of the sequence manager.
  scoped_refptr<TestMockTimeTaskRunner> test_task_runner_;

  std::unique_ptr<SequenceManagerForTest> manager_;

  // For testing purposes, this should follow the order in which queues
  // were created on the thread in which |this| was instantiated.
  Vector<scoped_refptr<TaskQueueWithVoters>> task_queues_;

  // Used to be able to cancel pending tasks from the sequence manager. For
  // testing purposes, this should follow the order in which the tasks were
  // posted to the thread in which |this| was instantiated.
  Vector<std::unique_ptr<Task>> pending_tasks_;

  // For Testing. Used to log tasks in their order of execution on the
  // thread in which |this| was instantiated.
  Vector<SequenceManagerFuzzerProcessor::TaskForTest> ordered_tasks_;

  // For Testing. Used to log actions in their order of execution on the
  // thread in which |this| was instantiated.
  Vector<SequenceManagerFuzzerProcessor::ActionForTest> ordered_actions_;

  // Outlives this class. |processor_| owns a thread pool manager that creates
  // threads.
  const raw_ptr<SequenceManagerFuzzerProcessor> processor_;

  THREAD_CHECKER(thread_checker_);
};

}  // namespace sequence_manager
}  // namespace base

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FUZZER_THREAD_MANAGER_H_
