// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_SCHEDULER_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_SCHEDULER_HELPER_H_

#include <stddef.h>

#include "base/logging.h"
#include "base/memory/raw_ptr.h"
#include "base/task/sequence_manager/sequence_manager.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "base/time/tick_clock.h"
#include "third_party/blink/public/platform/task_type.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace base {
class TaskObserver;
}

namespace blink {
namespace scheduler {

// Common scheduler functionality for default tasks.
// TODO(carlscab): This class is not really needed and should be removed
class PLATFORM_EXPORT SchedulerHelper
    : public base::sequence_manager::SequenceManager::Observer {
 public:
  // |sequence_manager| must remain valid until Shutdown() is called or the
  // object is destroyed.
  explicit SchedulerHelper(
      base::sequence_manager::SequenceManager* sequence_manager);
  SchedulerHelper(const SchedulerHelper&) = delete;
  SchedulerHelper& operator=(const SchedulerHelper&) = delete;
  ~SchedulerHelper() override;

  // Must be called before invoking AttachToCurrentThread().
  void InitDefaultTaskRunner(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  // Must be invoked before running any task from the scheduler, on the thread
  // that will run these tasks. Setups the ThreadChecker.
  void AttachToCurrentThread();

  // SequenceManager::Observer implementation:
  void OnBeginNestedRunLoop() override;
  void OnExitNestedRunLoop() override;

  const base::TickClock* GetClock() const;
  base::TimeTicks NowTicks() const;

  // Returns the task runner for the default task queue.
  const scoped_refptr<base::SingleThreadTaskRunner>& DefaultTaskRunner() {
    return default_task_runner_;
  }

  // Returns the task runner for the control task queue.  Tasks posted to this
  // queue are executed with the highest priority. Care must be taken to avoid
  // starvation of other task queues.
  virtual const scoped_refptr<base::SingleThreadTaskRunner>&
  ControlTaskRunner() = 0;

  // Adds or removes a task observer from the scheduler. The observer will be
  // notified before and after every executed task. These functions can only be
  // called on the thread this class was created on.
  void AddTaskObserver(base::TaskObserver* task_observer);
  void RemoveTaskObserver(base::TaskObserver* task_observer);

  void AddTaskTimeObserver(
      base::sequence_manager::TaskTimeObserver* task_time_observer);
  void RemoveTaskTimeObserver(
      base::sequence_manager::TaskTimeObserver* task_time_observer);

  // Shuts down the scheduler by dropping any remaining pending work in the work
  // queues. After this call any work posted to the task queue will be
  // silently dropped.
  void Shutdown();

  // Returns true if Shutdown() has been called. Otherwise returns false.
  bool IsShutdown() const { return !sequence_manager_; }

  inline void CheckOnValidThread() const {
    DCHECK_CALLED_ON_VALID_THREAD(thread_checker_);
  }

  class PLATFORM_EXPORT Observer {
   public:
    virtual ~Observer() = default;

    // Called when scheduler executes task with nested run loop.
    virtual void OnBeginNestedRunLoop() = 0;

    // Called when the scheduler spots we've exited a nested run loop.
    virtual void OnExitNestedRunLoop() = 0;
  };

  // Called once to set the Observer. This function is called on the main
  // thread. If |observer| is null, then no callbacks will occur.
  // Note |observer| is expected to outlive the SchedulerHelper.
  void SetObserver(Observer* observer);

  // Remove all canceled delayed tasks and consider shrinking to fit all
  // internal queues.
  void ReclaimMemory();

  // Accessor methods.
  std::optional<base::sequence_manager::WakeUp> GetNextWakeUp() const;
  void SetTimeDomain(base::sequence_manager::TimeDomain* time_domain);
  void ResetTimeDomain();
  bool GetAndClearSystemIsQuiescentBit();

  bool IsInNestedRunloop() const {
    CheckOnValidThread();
    return nested_runloop_depth_ > 0;
  }

  // Test helpers.
  void SetWorkBatchSizeForTesting(int work_batch_size);

 protected:
  virtual void ShutdownAllQueues() {}

  THREAD_CHECKER(thread_checker_);
  raw_ptr<base::sequence_manager::SequenceManager>
      sequence_manager_;  // NOT OWNED

 private:
  friend class SchedulerHelperTest;

  scoped_refptr<base::SingleThreadTaskRunner> default_task_runner_;

  raw_ptr<Observer> observer_;  // NOT OWNED

  // Depth of nested_runloop.
  int nested_runloop_depth_ = 0;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_SCHEDULER_HELPER_H_
