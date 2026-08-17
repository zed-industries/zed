// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FUZZER_SEQUENCE_MANAGER_FUZZER_PROCESSOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FUZZER_SEQUENCE_MANAGER_FUZZER_PROCESSOR_H_

#include <memory>

#include "base/time/time.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/test/fuzzer/proto/sequence_manager_test_description.pb.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace base {
namespace sequence_manager {

class ThreadManager;
class ThreadPoolManager;

// Provides functionality to parse the fuzzer's test description and run the
// relevant APIs.
//
// Warning: For unit testing purposes, the thread manager of the threads managed
// by the |thread_pool_manager_| should live for the scope of the main thread
// entry function i.e RunTest.
class PLATFORM_EXPORT SequenceManagerFuzzerProcessor {
  USING_FAST_MALLOC(SequenceManagerFuzzerProcessor);

 public:
  // Public interface used to parse the fuzzer's test description and
  // run the relevant APIs.
  static void ParseAndRun(const SequenceManagerTestDescription& description);

  ThreadPoolManager* thread_pool_manager() const;

 protected:
  struct TaskForTest {
    TaskForTest(uint64_t id, uint64_t start_time_ms, uint64_t end_time_ms);
    bool operator==(const TaskForTest& rhs) const;

    uint64_t task_id;
    uint64_t start_time_ms;
    uint64_t end_time_ms;
  };

  struct ActionForTest {
    enum class ActionType {
      kCreateTaskQueue,
      kPostDelayedTask,
      kSetQueuePriority,
      kSetQueueEnabled,
      kCreateQueueVoter,
      kCancelTask,
      kShutdownTaskQueue,
      kInsertFence,
      kRemoveFence,
      kCreateThread,
      kCrossThreadPostDelayedTask
    };

    ActionForTest(uint64_t id, ActionType type, uint64_t start_time_ms);

    bool operator==(const ActionForTest& rhs) const;

    uint64_t action_id;
    ActionType action_type;
    uint64_t start_time_ms;
  };

  SequenceManagerFuzzerProcessor();

  explicit SequenceManagerFuzzerProcessor(bool log_for_testing);

  ~SequenceManagerFuzzerProcessor();

  void RunTest(const SequenceManagerTestDescription& description);

  // Returns an ordered list of tasks executed on each thread. Note that the
  // ordering of the threads isn't deterministic since it follows the order in
  // which the threads were constructed. Furthermore, given that
  // ThreadPoolManager::CreateThread is used to construct these threads and
  // given that it can be called from multiple threads, the order of
  // construction isn't deterministic.
  const Vector<Vector<TaskForTest>>& ordered_tasks() const;

  // Returns an ordered list of actions executed on each thread. Note that the
  // ordering of the threads isn't deterministic. For more details, check the
  // comment above on ordered_tasks().
  const Vector<Vector<ActionForTest>>& ordered_actions() const;

 private:
  friend class ThreadManager;

  // Logs the task defined by the parameters passed to |ordered_tasks| if
  // |log_for_testing_| is enabled.
  void LogTaskForTesting(Vector<TaskForTest>* ordered_tasks,
                         uint64_t task_id,
                         base::TimeTicks start_time,
                         base::TimeTicks end_time);

  // Logs the action defined by the parameters passed to |ordered_actions| if
  // |log_for_testing_| is enabled.
  void LogActionForTesting(Vector<ActionForTest>* ordered_actions,
                           uint64_t action_id,
                           ActionForTest::ActionType type,
                           base::TimeTicks start_time);

  const bool log_for_testing_;

  const base::TimeTicks initial_time_;

  const std::unique_ptr<ThreadPoolManager> thread_pool_manager_;

  // The clock of the main thread manager task runner is initialized to
  // |initial_time_| and never advanced, since it can only execute actions at
  // the start of the program.
  const std::unique_ptr<ThreadManager> main_thread_manager_;

  // For Testing. Each entry contains the ordered list of tasks for one of the
  // created threads. The first entry is reserved for the main thread (which is
  // always empty since no tasks are executed on the main thread).
  Vector<Vector<TaskForTest>> ordered_tasks_;

  // For Testing. Each entry contains the ordered list of actions for one of the
  // created threads. The first entry is reserved for the main thread (which
  // can only contain ActionType::kCreateThread actions).
  Vector<Vector<ActionForTest>> ordered_actions_;
};

}  // namespace sequence_manager
}  // namespace base

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FUZZER_SEQUENCE_MANAGER_FUZZER_PROCESSOR_H_
