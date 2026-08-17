// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_SEQUENCE_H_
#define BASE_TASK_THREAD_POOL_SEQUENCE_H_

#include <stddef.h>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/containers/intrusive_heap.h"
#include "base/containers/queue.h"
#include "base/sequence_token.h"
#include "base/task/task_traits.h"
#include "base/task/thread_pool/pooled_parallel_task_runner.h"
#include "base/task/thread_pool/task.h"
#include "base/task/thread_pool/task_source.h"
#include "base/task/thread_pool/task_source_sort_key.h"
#include "base/thread_annotations.h"
#include "base/threading/sequence_local_storage_map.h"

namespace base {
namespace internal {

// A Sequence is intended to hold delayed tasks and immediate tasks.
// Delayed tasks are held in a prority_queue until they are ripe and
// immediate tasks in a simple fifo queue.
// When Sequence::TakeTask is called, we select the next appropriate task
// from both queues and return it.
// Each queue holds slots each containing up to a single Task that must be
// executed in posting/runtime order.
//
// In comments below, an "empty Sequence" is a Sequence with no slot.
//
// Note: there is a known refcounted-ownership cycle in the Scheduler
// architecture: Sequence -> Task -> TaskRunner -> Sequence -> ...
// This is okay so long as the other owners of Sequence (PriorityQueue and
// WorkerThread in alternation and
// ThreadGroup::WorkerThreadDelegateImpl::GetWork()
// temporarily) keep running it (and taking Tasks from it as a result). A
// dangling reference cycle would only occur should they release their reference
// to it while it's not empty. In other words, it is only correct for them to
// release it after PopTask() returns false to indicate it was made empty by
// that call (in which case the next PushImmediateTask() will return true to
// indicate to the caller that the Sequence should be re-enqueued for
// execution). This class is thread-safe.
class BASE_EXPORT Sequence : public TaskSource {
 public:
  // A Transaction can perform multiple operations atomically on a
  // Sequence. While a Transaction is alive, it is guaranteed that nothing
  // else will access the Sequence; the Sequence's lock is held for the
  // lifetime of the Transaction.
  class BASE_EXPORT Transaction : public TaskSource::Transaction {
   public:
    Transaction(Transaction&& other);
    Transaction(const Transaction&) = delete;
    Transaction& operator=(const Transaction&) = delete;
    ~Transaction();

    // Returns true if the sequence must be added to the immediate queue after
    // receiving a new immediate Task in order to be scheduled. If the caller
    // doesn't want the sequence to be scheduled, it may not add the sequence to
    // the immediate queue even if this returns true.
    bool WillPushImmediateTask();

    // Adds immediate |task| to the end of this sequence.
    void PushImmediateTask(Task task);

    // Adds a delayed |task| in this sequence, and returns true if the sequence
    // needs to be re-enqueued in the delayed queue as a result of this
    // sequence's delayed sort key changing.
    bool PushDelayedTask(Task task);

    Sequence* sequence() const { return static_cast<Sequence*>(task_source()); }

   private:
    friend class Sequence;

    explicit Transaction(Sequence* sequence);
  };

  // |traits| is metadata that applies to all Tasks in the Sequence.
  // |task_runner| is a reference to the TaskRunner feeding this TaskSource.
  // |task_runner| can be nullptr only for tasks with no TaskRunner, in which
  // case |execution_mode| must be kParallel. Otherwise, |execution_mode| is the
  // execution mode of |task_runner|.
  Sequence(const TaskTraits& traits,
           SequencedTaskRunner* task_runner,
           TaskSourceExecutionMode execution_mode);
  Sequence(const Sequence&) = delete;
  Sequence& operator=(const Sequence&) = delete;

  // Begins a Transaction. This method cannot be called on a thread which has an
  // active Sequence::Transaction.
  [[nodiscard]] Transaction BeginTransaction();

  // TaskSource:
  ExecutionEnvironment GetExecutionEnvironment() override;
  size_t GetRemainingConcurrency() const override;
  TaskSourceSortKey GetSortKey() const override;
  TimeTicks GetDelayedSortKey() const override;

  // Returns a token that uniquely identifies this Sequence.
  const SequenceToken& token() const LIFETIME_BOUND { return token_; }

  SequenceLocalStorageMap* sequence_local_storage() {
    return &sequence_local_storage_;
  }

  bool OnBecomeReady() override;

  bool has_worker_for_testing() const NO_THREAD_SAFETY_ANALYSIS {
    return has_worker_;
  }
  bool is_immediate_for_testing() const { return is_immediate_; }
  bool IsEmptyForTesting() const NO_THREAD_SAFETY_ANALYSIS { return IsEmpty(); }

  // A reference to TaskRunner is only retained between
  // PushImmediateTask()/PushDelayedTask() and when DidProcessTask() returns
  // false, guaranteeing it is safe to dereference this pointer. Otherwise, the
  // caller should guarantee such TaskRunner still exists before dereferencing.
  SequencedTaskRunner* task_runner() const { return task_runner_; }

 private:
  ~Sequence() override;

  struct DelayedTaskGreater {
    bool operator()(const Task& lhs, const Task& rhs) const;
  };

  // TaskSource:
  RunStatus WillRunTask() override;
  Task TakeTask(TaskSource::Transaction* transaction) override;
  std::optional<Task> Clear(TaskSource::Transaction* transaction) override;
  bool DidProcessTask(TaskSource::Transaction* transaction) override;
  bool WillReEnqueue(TimeTicks now,
                     TaskSource::Transaction* transaction) override;

  // Returns true if the delayed task to be posted will cause the delayed sort
  // key to change.
  bool DelayedSortKeyWillChange(const Task& delayed_task) const
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Selects the earliest task to run, either from immediate or
  // delayed queue and return it.
  // Expects this sequence to have at least one task that can run
  // immediately.
  Task TakeEarliestTask() EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Get and return next task from immediate queue
  Task TakeNextImmediateTask() EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Update the next earliest/latest ready time.
  void UpdateReadyTimes() EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Returns true if there are immediate tasks
  bool HasImmediateTasks() const EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Returns true if tasks ready to be executed
  bool HasReadyTasks(TimeTicks now) const override;

  bool IsEmpty() const EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Releases reference to TaskRunner.
  void ReleaseTaskRunner();

  const SequenceToken token_ = SequenceToken::Create();

  // A pointer to the TaskRunner that posts to this TaskSource, if any. The
  // derived class is responsible for calling AddRef() when a TaskSource from
  // which no Task is executing becomes non-empty and Release() when
  // it becomes empty again (e.g. when DidProcessTask() returns false).
  //
  // In practise, this pointer is going to become dangling. See task_runner()
  // comment.
  raw_ptr<SequencedTaskRunner, DisableDanglingPtrDetection> task_runner_;

  // Queues of tasks to execute.
  base::queue<Task> queue_ GUARDED_BY(lock_);
  base::IntrusiveHeap<Task, DelayedTaskGreater> delayed_queue_
      GUARDED_BY(lock_);

  // Caches the latest/earliest ready time for atomic access. Writes are
  // protected by |lock_|, but allows atomic reads outside of |lock_|. If this
  // sequence is empty, these are in an unknown state and shouldn't be read.

  // Minimum of latest_delayed_run_time() of next delayed task if any, and
  // |queue_time| of next immediate task if any.
  std::atomic<TimeTicks> latest_ready_time_ GUARDED_BY(lock_){TimeTicks()};
  // is_null() if there is an immediate task, or earliest_delayed_run_time() of
  // next delayed task otherwise.
  std::atomic<TimeTicks> earliest_ready_time_ GUARDED_BY(lock_){TimeTicks()};

  // True if a worker is currently associated with a Task from this Sequence.
  bool has_worker_ = false;

  // True if the sequence has ready tasks and requested to be queued as such
  // through WillPushImmediateTask() or OnBecomeReady(). Reset to false once all
  // ready tasks are done being processed and either DidProcessTask() or
  // WillReEnqueue() returned false. Normally, |is_immediate_| is protected by
  // |lock_|, except in OnBecomeReady() hence the use of atomics.
  std::atomic_bool is_immediate_{false};

  // Holds data stored through the SequenceLocalStorageSlot API.
  SequenceLocalStorageMap sequence_local_storage_;
};

}  // namespace internal
}  // namespace base

#endif  // BASE_TASK_THREAD_POOL_SEQUENCE_H_
