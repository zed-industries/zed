// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_WAKE_UP_QUEUE_H_
#define BASE_TASK_SEQUENCE_MANAGER_WAKE_UP_QUEUE_H_

#include <optional>

#include "base/base_export.h"
#include "base/check.h"
#include "base/containers/intrusive_heap.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/task/common/lazy_now.h"
#include "base/task/sequence_manager/task_queue_impl.h"
#include "base/time/time.h"
#include "base/values.h"

namespace base {
namespace sequence_manager {

class EnqueueOrder;

namespace internal {

class AssociatedThreadId;
class SequenceManagerImpl;
class TaskQueueImpl;

// WakeUpQueue is a queue of (wake_up, TaskQueueImpl*) pairs which
// aggregates wake-ups from multiple TaskQueueImpl into a single wake-up, and
// notifies TaskQueueImpls when wake-up times are reached.
class BASE_EXPORT WakeUpQueue {
 public:
  WakeUpQueue(const WakeUpQueue&) = delete;
  WakeUpQueue& operator=(const WakeUpQueue&) = delete;
  virtual ~WakeUpQueue();

  // Returns a wake-up for the next pending delayed task (pending delayed tasks
  // that are ripe may be ignored). If there are no such tasks (immediate tasks
  // don't count) or queues are disabled it returns nullopt.
  std::optional<WakeUp> GetNextDelayedWakeUp() const;

  // Debug info.
  Value::Dict AsValue(TimeTicks now) const;

  // Returns true if there are no pending delayed tasks.
  bool empty() const { return wake_up_queue_.empty(); }

  // Moves ready delayed tasks in TaskQueues to delayed WorkQueues, consuming
  // expired wake-ups in the process.
  void MoveReadyDelayedTasksToWorkQueues(LazyNow* lazy_now,
                                         EnqueueOrder enqueue_order);

  // Schedule `queue` to wake up at certain time. Repeating calls with the same
  // `queue` invalidate previous requests. Nullopt `wake_up` cancels a
  // previously set wake up for `queue`.
  void SetNextWakeUpForQueue(internal::TaskQueueImpl* queue,
                             LazyNow* lazy_now,
                             std::optional<WakeUp> wake_up);

  // Remove the TaskQueue from any internal data structures.
  virtual void UnregisterQueue(internal::TaskQueueImpl* queue) = 0;

  // Removes all canceled delayed tasks from the front of the queue. After
  // calling this, GetNextDelayedWakeUp() is guaranteed to return a wake up time
  // for a non-canceled task.
  void RemoveAllCanceledDelayedTasksFromFront(LazyNow* lazy_now);

 protected:
  explicit WakeUpQueue(
      scoped_refptr<const internal::AssociatedThreadId> associated_thread);

  // Called every time the next `next_wake_up` changes. std::nullopt is used to
  // cancel the next wake-up. Subclasses may use this to tell SequenceManager to
  // schedule the next wake-up at the given time.
  virtual void OnNextWakeUpChanged(LazyNow* lazy_now,
                                   std::optional<WakeUp> next_wake_up) = 0;

  virtual const char* GetName() const = 0;

 private:
  friend class MockWakeUpQueue;

  struct ScheduledWakeUp {
    WakeUp wake_up;
    // RAW_PTR_EXCLUSION: Performance reasons (based on analysis of
    // speedometer3).
    RAW_PTR_EXCLUSION internal::TaskQueueImpl* queue = nullptr;

    bool operator>(const ScheduledWakeUp& other) const {
      return wake_up.latest_time() > other.wake_up.latest_time();
    }

    void SetHeapHandle(HeapHandle handle) {
      DCHECK(handle.IsValid());
      queue->set_heap_handle(handle);
    }

    void ClearHeapHandle() {
      DCHECK(queue->heap_handle().IsValid());
      queue->set_heap_handle(HeapHandle());
    }

    HeapHandle GetHeapHandle() const { return queue->heap_handle(); }
  };

  IntrusiveHeap<ScheduledWakeUp, std::greater<>> wake_up_queue_;

  const scoped_refptr<const internal::AssociatedThreadId> associated_thread_;
};

// Default WakeUpQueue implementation that forwards wake-ups to
// `sequence_manager_`.
class BASE_EXPORT DefaultWakeUpQueue : public WakeUpQueue {
 public:
  DefaultWakeUpQueue(
      scoped_refptr<internal::AssociatedThreadId> associated_thread,
      internal::SequenceManagerImpl* sequence_manager);
  ~DefaultWakeUpQueue() override;

 private:
  // WakeUpQueue implementation:
  void OnNextWakeUpChanged(LazyNow* lazy_now,
                           std::optional<WakeUp> wake_up) override;
  const char* GetName() const override;
  void UnregisterQueue(internal::TaskQueueImpl* queue) override;

  raw_ptr<internal::SequenceManagerImpl> sequence_manager_;  // Not owned.
};

// WakeUpQueue implementation that doesn't sends wake-ups to
// any SequenceManager, such that task queues don't cause wake-ups.
class BASE_EXPORT NonWakingWakeUpQueue : public WakeUpQueue {
 public:
  explicit NonWakingWakeUpQueue(
      scoped_refptr<internal::AssociatedThreadId> associated_thread);
  ~NonWakingWakeUpQueue() override;

 private:
  // WakeUpQueue implementation:
  void OnNextWakeUpChanged(LazyNow* lazy_now,
                           std::optional<WakeUp> wake_up) override;
  const char* GetName() const override;
  void UnregisterQueue(internal::TaskQueueImpl* queue) override;
};

}  // namespace internal
}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_WAKE_UP_QUEUE_H_
