// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_PRIORITY_QUEUE_H_
#define BASE_TASK_THREAD_POOL_PRIORITY_QUEUE_H_

#include <functional>
#include <memory>

#include "base/base_export.h"
#include "base/containers/intrusive_heap.h"
#include "base/task/common/checked_lock.h"
#include "base/task/thread_pool/task_source.h"
#include "base/task/thread_pool/task_source_sort_key.h"
#include "base/types/cxx23_to_underlying.h"

namespace base {
namespace internal {

// A PriorityQueue holds TaskSources of Tasks. This class is not thread-safe
// (requires external synchronization).
class BASE_EXPORT PriorityQueue {
 public:
  PriorityQueue();
  PriorityQueue(const PriorityQueue&) = delete;
  PriorityQueue& operator=(const PriorityQueue&) = delete;
  ~PriorityQueue();

  PriorityQueue& operator=(PriorityQueue&& other);

  // Inserts |task_source| in the PriorityQueue with |task_source_sort_key|.
  void Push(RegisteredTaskSource task_source,
            TaskSourceSortKey task_source_sort_key);

  // Returns a reference to the TaskSourceSortKey representing the priority of
  // the highest pending task in this PriorityQueue. The reference becomes
  // invalid the next time that this PriorityQueue is modified.
  // Cannot be called on an empty PriorityQueue.
  const TaskSourceSortKey& PeekSortKey() const;

  // Returns a reference to the highest priority TaskSource in this
  // PriorityQueue. Cannot be called on an empty PriorityQueue. The returned
  // task source may be modified as long as its sort key isn't affected.
  RegisteredTaskSource& PeekTaskSource() const;

  // Removes and returns the highest priority TaskSource in this PriorityQueue.
  // Cannot be called on an empty PriorityQueue.
  RegisteredTaskSource PopTaskSource();

  // Removes |task_source| from the PriorityQueue. Returns a
  // RegisteredTaskSource which evaluates to true if successful, or false if
  // |task_source| is not currently in the PriorityQueue or the PriorityQueue is
  // empty.
  RegisteredTaskSource RemoveTaskSource(const TaskSource& task_source);

  // Updates the sort key of |task_source| to |sort_key|, reordering
  // |task_source| in the queue if necessary. No-ops if the TaskSource is not in
  // the PriorityQueue or the PriorityQueue is empty.
  void UpdateSortKey(const TaskSource& task_source, TaskSourceSortKey sort_key);

  // Returns true if the PriorityQueue is empty.
  bool IsEmpty() const;

  // Returns the number of TaskSources in the PriorityQueue.
  size_t Size() const;

  // Returns the number of TaskSources with |priority|.
  size_t GetNumTaskSourcesWithPriority(TaskPriority priority) const {
    return num_task_sources_per_priority_[base::to_underlying(priority)];
  }

  // Set the PriorityQueue to empty all its TaskSources of Tasks when it is
  // destroyed; needed to prevent memory leaks caused by a reference cycle
  // (TaskSource -> Task -> TaskRunner -> TaskSource...) during test teardown.
  void EnableFlushTaskSourcesOnDestroyForTesting();

  void swap(PriorityQueue& other);

 private:
  // A class combining a TaskSource and the TaskSourceSortKey that determines
  // its position in a PriorityQueue.
  class TaskSourceAndSortKey;

  using ContainerType = IntrusiveHeap<TaskSourceAndSortKey>;

  void DecrementNumTaskSourcesForPriority(TaskPriority priority);
  void IncrementNumTaskSourcesForPriority(TaskPriority priority);

  ContainerType container_;

  std::array<size_t, static_cast<int>(TaskPriority::HIGHEST) + 1>
      num_task_sources_per_priority_ = {};

  // Should only be enabled by EnableFlushTaskSourcesOnDestroyForTesting().
  bool is_flush_task_sources_on_destroy_enabled_ = false;
};

}  // namespace internal
}  // namespace base

#endif  // BASE_TASK_THREAD_POOL_PRIORITY_QUEUE_H_
