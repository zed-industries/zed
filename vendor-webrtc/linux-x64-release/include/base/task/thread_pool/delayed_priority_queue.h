// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_DELAYED_PRIORITY_QUEUE_H_
#define BASE_TASK_THREAD_POOL_DELAYED_PRIORITY_QUEUE_H_

#include "base/base_export.h"
#include "base/containers/intrusive_heap.h"
#include "base/stl_util.h"
#include "base/task/thread_pool/task_source.h"
#include "base/time/time.h"

namespace base::internal {

// A DelayedPriorityQueue holds TaskSources not ready to run yet. TaskSources
// are ordered by delayed runtime. This class is not thread-safe (requires
// external synchronization).
class BASE_EXPORT DelayedPriorityQueue {
 public:
  DelayedPriorityQueue();
  DelayedPriorityQueue(const DelayedPriorityQueue&) = delete;
  DelayedPriorityQueue& operator=(const DelayedPriorityQueue&) = delete;
  ~DelayedPriorityQueue();

  DelayedPriorityQueue& operator=(DelayedPriorityQueue&& other);

  // Inserts |task_source| in the DelayedPriorityQueue with |delayed_sort_key|.
  void Push(scoped_refptr<TaskSource> task_source, TimeTicks delayed_sort_key);

  // Returns a delayed sort key representing the priority of the highest pending
  // task in this DelayedPriorityQueue. Cannot be called on an empty
  // DelayedPriorityQueue.
  const TimeTicks PeekDelayedSortKey() const;

  // Returns a pointer to the earliest-to-run TaskSource in this
  // DelayedPriorityQueue. Cannot be called on an empty DelayedPriorityQueue.
  scoped_refptr<TaskSource> PeekTaskSource() const;

  // Removes and returns the highest priority TaskSource in this
  // DelayedPriorityQueue. Cannot be called on an empty DelayedPriorityQueue.
  scoped_refptr<TaskSource> PopTaskSource();

  // Removes |task_source| from the DelayedPriorityQueue. Returns a TaskSource
  // which evaluates to true if successful, or false if |task_source| is not
  // currently in the DelayedPriorityQueue or the DelayedPriorityQueue is empty.
  scoped_refptr<TaskSource> RemoveTaskSource(
      scoped_refptr<TaskSource> task_source);

  // Updates the delayed sort key of |task_source| to |delayed_sort_key|,
  // reordering |task_source| in the queue if necessary. No-ops if the
  // TaskSource is not in the DelayedPriorityQueue or the DelayedPriorityQueue
  // is empty.
  void UpdateDelayedSortKey(scoped_refptr<TaskSource> task_source);

  // Returns true if the DelayedPriorityQueue is empty.
  bool IsEmpty() const;

  // Returns the number of TaskSources in the DelayedPriorityQueue.
  size_t Size() const;

  // Set the DelayedPriorityQueue to empty all its TaskSources of Tasks when it
  // is destroyed; needed to prevent memory leaks caused by a reference cycle
  // (TaskSource -> Task -> TaskRunner -> TaskSource...) during test teardown.
  void EnableFlushTaskSourcesOnDestroyForTesting();

 private:
  // A class combining a TaskSource and the delayed_sort_key that determines
  // its position in a PriorityQueue.
  class TaskSourceAndDelayedSortKey;

  struct DelayedPriorityQueueComparisonOperator {
    bool operator()(const TaskSourceAndDelayedSortKey& lhs,
                    const TaskSourceAndDelayedSortKey& rhs) const;
  };

  using ContainerType = IntrusiveHeap<TaskSourceAndDelayedSortKey,
                                      DelayedPriorityQueueComparisonOperator>;

  ContainerType container_;

  // Should only be enabled by EnableFlushTaskSourcesOnDestroyForTesting().
  bool is_flush_task_sources_on_destroy_enabled_ = false;
};

}  // namespace base::internal

#endif  // BASE_TASK_THREAD_POOL_DELAYED_PRIORITY_QUEUE_H_
