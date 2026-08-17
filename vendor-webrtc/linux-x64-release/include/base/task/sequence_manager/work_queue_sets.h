// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_WORK_QUEUE_SETS_H_
#define BASE_TASK_SEQUENCE_MANAGER_WORK_QUEUE_SETS_H_

#include <functional>
#include <optional>
#include <vector>

#include "base/base_export.h"
#include "base/containers/intrusive_heap.h"
#include "base/dcheck_is_on.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/memory/stack_allocated.h"
#include "base/task/sequence_manager/sequence_manager.h"
#include "base/task/sequence_manager/task_order.h"
#include "base/task/sequence_manager/task_queue_impl.h"
#include "base/task/sequence_manager/work_queue.h"

namespace base {
namespace sequence_manager {
namespace internal {

struct WorkQueueAndTaskOrder {
  STACK_ALLOCATED();

 public:
  WorkQueueAndTaskOrder(WorkQueue& work_queue, const TaskOrder& task_order)
      : queue(&work_queue), order(task_order) {}

  WorkQueue* queue = nullptr;
  TaskOrder order;
};

// There is a min-heap for each scheduler priority which keeps track of which
// queue in the set has the oldest task (i.e. the one that should be run next if
// the TaskQueueSelector chooses to run a task a given priority).
class BASE_EXPORT WorkQueueSets {
 public:
  class Observer {
   public:
    virtual ~Observer() = default;

    virtual void WorkQueueSetBecameEmpty(size_t set_index) = 0;

    virtual void WorkQueueSetBecameNonEmpty(size_t set_index) = 0;
  };

  WorkQueueSets(const char* name,
                Observer* observer,
                const SequenceManager::Settings& settings);
  WorkQueueSets(const WorkQueueSets&) = delete;
  WorkQueueSets& operator=(const WorkQueueSets&) = delete;
  ~WorkQueueSets();

  // O(log num queues)
  void AddQueue(WorkQueue* queue, size_t set_index);

  // O(log num queues)
  void RemoveQueue(WorkQueue* work_queue);

  // O(log num queues)
  void ChangeSetIndex(WorkQueue* queue, size_t set_index);

  // O(log num queues)
  void OnQueuesFrontTaskChanged(WorkQueue* queue);

  // O(log num queues)
  void OnTaskPushedToEmptyQueue(WorkQueue* work_queue);

  // If empty it's O(1) amortized, otherwise it's O(log num queues). Slightly
  // faster on average than OnQueuesFrontTaskChanged.
  // Assumes |work_queue| contains the lowest enqueue order in the set.
  void OnPopMinQueueInSet(WorkQueue* work_queue);

  // O(log num queues)
  void OnQueueBlocked(WorkQueue* work_queue);

  // O(1)
  std::optional<WorkQueueAndTaskOrder> GetOldestQueueAndTaskOrderInSet(
      size_t set_index) const;

#if DCHECK_IS_ON()
  // O(1)
  std::optional<WorkQueueAndTaskOrder> GetRandomQueueAndTaskOrderInSet(
      size_t set_index) const;
#endif

  // O(1)
  bool IsSetEmpty(size_t set_index) const;

#if DCHECK_IS_ON() || !defined(NDEBUG)
  // Note this iterates over everything in |work_queue_heaps_|.
  // It's intended for use with DCHECKS and for testing
  bool ContainsWorkQueueForTest(const WorkQueue* queue) const;
#endif

  const char* GetName() const { return name_; }

  // Collects ready tasks which where skipped over when |selected_work_queue|
  // was selected. Note this is somewhat expensive.
  void CollectSkippedOverLowerPriorityTasks(
      const internal::WorkQueue* selected_work_queue,
      std::vector<const Task*>* result) const;

 private:
  struct OldestTaskOrder {
    TaskOrder key;
    // RAW_PTR_EXCLUSION: Performance: visible in sampling profiler stacks.
    RAW_PTR_EXCLUSION WorkQueue* value = nullptr;

    // Used for a min-heap.
    bool operator>(const OldestTaskOrder& other) const {
      return key > other.key;
    }

    void SetHeapHandle(HeapHandle handle) { value->set_heap_handle(handle); }

    void ClearHeapHandle() { value->set_heap_handle(HeapHandle()); }

    HeapHandle GetHeapHandle() const { return value->heap_handle(); }
  };

  const char* const name_;

  // For each set |work_queue_heaps_| has a queue of WorkQueue ordered by the
  // oldest task in each WorkQueue.
  std::vector<IntrusiveHeap<OldestTaskOrder, std::greater<>>> work_queue_heaps_;

#if DCHECK_IS_ON()
  static inline uint64_t MurmurHash3(uint64_t value) {
    value ^= value >> 33;
    value *= uint64_t{0xFF51AFD7ED558CCD};
    value ^= value >> 33;
    value *= uint64_t{0xC4CEB9FE1A85EC53};
    value ^= value >> 33;
    return value;
  }

  // This is for a debugging feature which lets us randomize task selection. Its
  // not for production use.
  // TODO(crbug.com/40234060): Use a seedable PRNG from ::base if one is added.
  uint64_t Random() const {
    last_rand_ = MurmurHash3(last_rand_);
    return last_rand_;
  }

  mutable uint64_t last_rand_;
#endif

  const raw_ptr<Observer> observer_;
};

}  // namespace internal
}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_WORK_QUEUE_SETS_H_
