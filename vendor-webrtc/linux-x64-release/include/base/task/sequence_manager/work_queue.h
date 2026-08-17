// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_WORK_QUEUE_H_
#define BASE_TASK_SEQUENCE_MANAGER_WORK_QUEUE_H_

#include <optional>

#include "base/base_export.h"
#include "base/containers/intrusive_heap.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/task/sequence_manager/fence.h"
#include "base/task/sequence_manager/sequenced_task_source.h"
#include "base/task/sequence_manager/task_queue_impl.h"
#include "base/values.h"

namespace base {
namespace sequence_manager {
class TaskOrder;

namespace internal {

class WorkQueueSets;

// This class keeps track of immediate and delayed tasks which are due to run
// now. It interfaces deeply with WorkQueueSets which keeps track of which queue
// (with a given priority) contains the oldest task.
//
// If a fence is inserted, WorkQueue behaves normally up until
// TakeTaskFromWorkQueue reaches or exceeds the fence.  At that point it the
// API subset used by WorkQueueSets pretends the WorkQueue is empty until the
// fence is removed.  This functionality is a primitive intended for use by
// throttling mechanisms.
class BASE_EXPORT WorkQueue {
 public:
  enum class RemoveCancelledTasksPolicy {
    // Removes cancelled tasks at the front of the queue. This is most efficient
    // as it doesn't traverse all tasks in the queue.
    kFront,
    // Removes all cancelled tasks. This requires traversing all the queue.
    kAll
  };

  using QueueType = internal::TaskQueueImpl::WorkQueueType;

  // Note |task_queue| can be null if queue_type is kNonNestable.
  WorkQueue(TaskQueueImpl* task_queue, const char* name, QueueType queue_type);
  WorkQueue(const WorkQueue&) = delete;
  WorkQueue& operator=(const WorkQueue&) = delete;
  ~WorkQueue();

  // Associates this work queue with the given work queue sets. This must be
  // called before any tasks can be inserted into this work queue.
  void AssignToWorkQueueSets(WorkQueueSets* work_queue_sets);

  // Assigns the current set index.
  void AssignSetIndex(size_t work_queue_set_index);

  Value::List AsValue(TimeTicks now) const;

  // Returns true if the |tasks_| is empty. This method ignores any fences.
  bool Empty() const { return tasks_.empty(); }

  // Returns the front task's TaskOrder if `tasks_` is non-empty and a fence
  // hasn't been reached, otherwise returns nullopt.
  std::optional<TaskOrder> GetFrontTaskOrder() const;

  // Returns the first task in this queue or null if the queue is empty. This
  // method ignores any fences.
  const Task* GetFrontTask() const;

  // Returns the last task in this queue or null if the queue is empty. This
  // method ignores any fences.
  const Task* GetBackTask() const;

  // Pushes the task onto the |tasks_| and if a fence hasn't been reached
  // it informs the WorkQueueSets if the head changed.
  void Push(Task task);

  // RAII helper that helps efficiently push N Tasks to a WorkQueue.
  class BASE_EXPORT TaskPusher {
   public:
    TaskPusher(const TaskPusher&) = delete;
    TaskPusher(TaskPusher&& other);
    ~TaskPusher();

    void Push(Task task);

   private:
    friend class WorkQueue;

    explicit TaskPusher(WorkQueue* work_queue);

    // RAW_PTR_EXCLUSION: Performance reasons (based on analysis of sampling
    // profiler data and tab_search:top100:2020).
    RAW_PTR_EXCLUSION WorkQueue* work_queue_ = nullptr;

    const bool was_empty_;
  };

  // Returns an RAII helper to efficiently push multiple tasks.
  TaskPusher CreateTaskPusher();

  // Pushes the task onto the front of the |tasks_| and if it's before any
  // fence it informs the WorkQueueSets the head changed. Use with caution this
  // API can easily lead to task starvation if misused.
  void PushNonNestableTaskToFront(Task task);

  // Reloads the empty |tasks_| with
  // |task_queue_->TakeImmediateIncomingQueue| and if a fence hasn't been
  // reached it informs the WorkQueueSets if the head changed.
  void TakeImmediateIncomingQueueTasks();

  size_t Size() const { return tasks_.size(); }

  // Pulls a task off the |tasks_| and informs the WorkQueueSets.  If the
  // task removed had an enqueue order >= the current fence then WorkQueue
  // pretends to be empty as far as the WorkQueueSets is concerned.
  Task TakeTaskFromWorkQueue();

  // Removes cancelled tasks from the queue. Returns true if any tasks were
  // removed.
  bool RemoveCancelledTasks(RemoveCancelledTasksPolicy policy);

  const char* name() const { return name_; }

  TaskQueueImpl* task_queue() const { return task_queue_; }

  WorkQueueSets* work_queue_sets() const { return work_queue_sets_; }

  size_t work_queue_set_index() const { return work_queue_set_index_; }

  HeapHandle heap_handle() const { return heap_handle_; }

  void set_heap_handle(HeapHandle handle) { heap_handle_ = handle; }

  QueueType queue_type() const { return queue_type_; }

  // Submit a fence. When TakeTaskFromWorkQueue encounters a task whose
  // enqueue_order is >= |fence| then the WorkQueue will start pretending to be.
  // empty.
  // Inserting a fence may supersede a previous one and unblock some tasks.
  // Returns true if any tasks where unblocked, returns false otherwise.
  bool InsertFence(Fence fence);

  // Submit a fence without triggering a WorkQueueSets notification.
  // Caller must ensure that WorkQueueSets are properly updated.
  // This method should not be called when a fence is already present.
  void InsertFenceSilently(Fence fence);

  // Removes any fences that where added and if WorkQueue was pretending to be
  // empty, then the real value is reported to WorkQueueSets. Returns true if
  // any tasks where unblocked.
  bool RemoveFence();

  // Returns true if any tasks are blocked by the fence. Returns true if the
  // queue is empty and fence has been set (i.e. future tasks would be blocked).
  // Otherwise returns false.
  bool BlockedByFence() const;

  // Test support function. This should not be used in production code.
  void PopTaskForTesting();

  // Iterates through |tasks_| adding any that are older than |reference| to
  // |result|.
  void CollectTasksOlderThan(TaskOrder reference,
                             std::vector<const Task*>* result) const;

  bool InsertFenceImpl(Fence fence);

  TaskQueueImpl::TaskDeque tasks_;
  // RAW_PTR_EXCLUSION: Performance reasons (based on analysis of speedometer3).
  RAW_PTR_EXCLUSION WorkQueueSets* work_queue_sets_ = nullptr;   // NOT OWNED.
  RAW_PTR_EXCLUSION TaskQueueImpl* const task_queue_ = nullptr;  // NOT OWNED.
  size_t work_queue_set_index_ = 0;

  // Iff the queue isn't empty (or appearing to be empty due to a fence) then
  // |heap_handle_| will be valid and correspond to this queue's location within
  // an IntrusiveHeap inside the WorkQueueSet.
  HeapHandle heap_handle_;
  const char* const name_;
  std::optional<Fence> fence_;
  const QueueType queue_type_;
};

}  // namespace internal
}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_WORK_QUEUE_H_
