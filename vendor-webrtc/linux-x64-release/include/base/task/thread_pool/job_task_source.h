// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_JOB_TASK_SOURCE_H_
#define BASE_TASK_THREAD_POOL_JOB_TASK_SOURCE_H_

#include <stddef.h>

#include <atomic>
#include <limits>
#include <optional>
#include <utility>

#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/synchronization/condition_variable.h"
#include "base/task/common/checked_lock.h"
#include "base/task/common/task_annotator.h"
#include "base/task/post_job.h"
#include "base/task/task_traits.h"
#include "base/task/thread_pool/task.h"
#include "base/task/thread_pool/task_source.h"
#include "base/task/thread_pool/task_source_sort_key.h"

namespace base {
namespace internal {

class PooledTaskRunnerDelegate;

// A JobTaskSource generates many Tasks from a single RepeatingClosure.
//
// Derived classes control the intended concurrency with GetMaxConcurrency().
class BASE_EXPORT JobTaskSource : public TaskSource {
 public:
  JobTaskSource(const Location& from_here,
                const TaskTraits& traits,
                RepeatingCallback<void(JobDelegate*)> worker_task,
                MaxConcurrencyCallback max_concurrency_callback,
                PooledTaskRunnerDelegate* delegate);
  JobTaskSource(const JobTaskSource&) = delete;
  JobTaskSource& operator=(const JobTaskSource&) = delete;

  static JobHandle CreateJobHandle(
      scoped_refptr<internal::JobTaskSource> task_source) {
    return JobHandle(std::move(task_source));
  }

  // Called before the task source is enqueued to initialize task metadata.
  void WillEnqueue(int sequence_num, TaskAnnotator& annotator);

  // Notifies this task source that max concurrency was increased, and the
  // number of worker should be adjusted.
  void NotifyConcurrencyIncrease();

  // Informs this JobTaskSource that the current thread would like to join and
  // contribute to running |worker_task|. Returns true if the joining thread can
  // contribute (RunJoinTask() can be called), or false if joining was completed
  // and all other workers returned because either there's no work remaining or
  // Job was cancelled.
  bool WillJoin();

  // Contributes to running |worker_task| and returns true if the joining thread
  // can contribute again (RunJoinTask() can be called again), or false if
  // joining was completed and all other workers returned because either there's
  // no work remaining or Job was cancelled. This should be called only after
  // WillJoin() or RunJoinTask() previously returned true.
  bool RunJoinTask();

  // Cancels this JobTaskSource, causing all workers to yield and WillRunTask()
  // to return RunStatus::kDisallowed.
  void Cancel(TaskSource::Transaction* transaction = nullptr);

  // TaskSource:
  ExecutionEnvironment GetExecutionEnvironment() override;
  size_t GetRemainingConcurrency() const override;
  TaskSourceSortKey GetSortKey() const override;
  TimeTicks GetDelayedSortKey() const override;
  bool HasReadyTasks(TimeTicks now) const override;

  bool IsActive() const;
  size_t GetWorkerCount() const;

  // Returns the maximum number of tasks from this TaskSource that can run
  // concurrently.
  size_t GetMaxConcurrency() const;

  uint8_t AcquireTaskId();
  void ReleaseTaskId(uint8_t task_id);

  // Returns true if a worker should return from the worker task on the current
  // thread ASAP.
  bool ShouldYield();

  PooledTaskRunnerDelegate* delegate() const { return delegate_; }

 private:
  // Atomic internal state to track the number of workers running a task from
  // this JobTaskSource and whether this JobTaskSource is canceled. All
  // operations are performed with std::memory_order_relaxed as State is only
  // ever modified under a lock or read atomically (optimistic read).
  class State {
   public:
    static constexpr uint32_t kCanceledMask = 1;
    static constexpr int kWorkerCountBitOffset = 1;
    static constexpr uint32_t kWorkerCountIncrement = 1
                                                      << kWorkerCountBitOffset;

    struct Value {
      uint8_t worker_count() const {
        return static_cast<uint8_t>(value >> kWorkerCountBitOffset);
      }
      // Returns true if canceled.
      bool is_canceled() const { return value & kCanceledMask; }

      uint32_t value;
    };

    State();
    ~State();

    // Sets as canceled. Returns the state
    // before the operation.
    Value Cancel();

    // Increments the worker count by 1. Returns the state before the operation.
    Value IncrementWorkerCount();

    // Decrements the worker count by 1. Returns the state before the operation.
    Value DecrementWorkerCount();

    // Loads and returns the state.
    Value Load() const;

   private:
    std::atomic<uint32_t> value_{0};
  };

  // Atomic flag that indicates if the joining thread is currently waiting on
  // another worker to yield or to signal.
  class JoinFlag {
   public:
    static constexpr uint32_t kNotWaiting = 0;
    static constexpr uint32_t kWaitingForWorkerToSignal = 1;
    static constexpr uint32_t kWaitingForWorkerToYield = 3;
    // kWaitingForWorkerToYield is 3 because the impl relies on the following
    // property.
    static_assert((kWaitingForWorkerToYield & kWaitingForWorkerToSignal) ==
                      kWaitingForWorkerToSignal,
                  "");

    JoinFlag();
    ~JoinFlag();

    // Returns true if the status is not kNotWaiting, using
    // std::memory_order_relaxed.
    bool IsWaiting() {
      return value_.load(std::memory_order_relaxed) != kNotWaiting;
    }

    // Resets the status as kNotWaiting  using std::memory_order_relaxed.
    void Reset();

    // Sets the status as kWaitingForWorkerToYield using
    // std::memory_order_relaxed.
    void SetWaiting();

    // If the flag is kWaitingForWorkerToYield, returns true indicating that the
    // worker should yield, and atomically updates to kWaitingForWorkerToSignal
    // (using std::memory_order_relaxed) to ensure that a single worker yields
    // in response to SetWaiting().
    bool ShouldWorkerYield();

    // If the flag is kWaiting*, returns true indicating that the worker should
    // signal, and atomically updates to kNotWaiting (using
    // std::memory_order_relaxed) to ensure that a single worker signals in
    // response to SetWaiting().
    bool ShouldWorkerSignal();

   private:
    std::atomic<uint32_t> value_{kNotWaiting};
  };

  ~JobTaskSource() override;

  // Called from the joining thread. Waits for the worker count to be below or
  // equal to max concurrency (will happen when a worker calls
  // DidProcessTask()). Returns true if the joining thread should run a task, or
  // false if joining was completed and all other workers returned because
  // either there's no work remaining or Job was cancelled.
  bool WaitForParticipationOpportunity() EXCLUSIVE_LOCKS_REQUIRED(worker_lock_);

  size_t GetMaxConcurrency(size_t worker_count) const;

  // TaskSource:
  RunStatus WillRunTask() override;
  Task TakeTask(TaskSource::Transaction* transaction) override;
  std::optional<Task> Clear(TaskSource::Transaction* transaction) override;
  bool DidProcessTask(TaskSource::Transaction* transaction) override;
  bool WillReEnqueue(TimeTicks now,
                     TaskSource::Transaction* transaction) override;
  bool OnBecomeReady() override;

  // Synchronizes access to workers state.
  mutable CheckedLock worker_lock_{UniversalSuccessor()};

  // Current atomic state (atomic despite the lock to allow optimistic reads
  // and cancellation without the lock).
  State state_ GUARDED_BY(worker_lock_);
  // Normally, |join_flag_| is protected by |lock_|, except in ShouldYield()
  // hence the use of atomics.
  JoinFlag join_flag_ GUARDED_BY(worker_lock_);
  // Signaled when |join_flag_| is kWaiting* and a worker returns.
  std::optional<ConditionVariable> worker_released_condition_
      GUARDED_BY(worker_lock_);

  std::atomic<uint32_t> assigned_task_ids_{0};

  RepeatingCallback<size_t(size_t)> max_concurrency_callback_;

  // Worker task set by the job owner.
  RepeatingCallback<void(JobDelegate*)> worker_task_;
  // Task returned from TakeTask(), that calls |worker_task_| internally.
  RepeatingClosure primary_task_;

  TaskMetadata task_metadata_;

  const TimeTicks ready_time_;
  raw_ptr<PooledTaskRunnerDelegate, LeakedDanglingUntriaged> delegate_;
};

}  // namespace internal
}  // namespace base

#endif  // BASE_TASK_THREAD_POOL_JOB_TASK_SOURCE_H_
