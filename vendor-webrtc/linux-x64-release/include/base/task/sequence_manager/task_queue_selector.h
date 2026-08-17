// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_TASK_QUEUE_SELECTOR_H_
#define BASE_TASK_SEQUENCE_MANAGER_TASK_QUEUE_SELECTOR_H_

#include <stddef.h>

#include <atomic>
#include <optional>
#include <vector>

#include "base/base_export.h"
#include "base/dcheck_is_on.h"
#include "base/memory/raw_ptr.h"
#include "base/pending_task.h"
#include "base/task/sequence_manager/sequence_manager.h"
#include "base/task/sequence_manager/sequenced_task_source.h"
#include "base/task/sequence_manager/task_order.h"
#include "base/task/sequence_manager/work_queue_sets.h"
#include "base/values.h"

namespace base {
namespace sequence_manager {
namespace internal {

class AssociatedThreadId;

// TaskQueueSelector is used by the SchedulerHelper to enable prioritization
// of particular task queues.
class BASE_EXPORT TaskQueueSelector : public WorkQueueSets::Observer {
 public:
  using SelectTaskOption = SequencedTaskSource::SelectTaskOption;

  TaskQueueSelector(scoped_refptr<const AssociatedThreadId> associated_thread,
                    const SequenceManager::Settings& settings);

  TaskQueueSelector(const TaskQueueSelector&) = delete;
  TaskQueueSelector& operator=(const TaskQueueSelector&) = delete;
  ~TaskQueueSelector() override;

  // Called to register a queue that can be selected. This function is called
  // on the main thread.
  void AddQueue(internal::TaskQueueImpl* queue,
                TaskQueue::QueuePriority priority);

  // The specified work will no longer be considered for selection. This
  // function is called on the main thread.
  void RemoveQueue(internal::TaskQueueImpl* queue);

  // Make |queue| eligible for selection. This function is called on the main
  // thread. Must only be called if |queue| is disabled.
  void EnableQueue(internal::TaskQueueImpl* queue);

  // Disable selection from |queue|. Must only be called if |queue| is enabled.
  void DisableQueue(internal::TaskQueueImpl* queue);

  // Called get or set the priority of |queue|.
  void SetQueuePriority(internal::TaskQueueImpl* queue,
                        TaskQueue::QueuePriority priority);

  // Called to choose the work queue from which the next task should be taken
  // and run. Return the queue to service if there is one or null otherwise.
  // This function is called on the main thread.
  WorkQueue* SelectWorkQueueToService(
      SelectTaskOption option = SelectTaskOption::kDefault);

  // Serialize the selector state for tracing/debugging.
  Value::Dict AsValue() const;

  class BASE_EXPORT Observer {
   public:
    virtual ~Observer() = default;

    // Called when |queue| transitions from disabled to enabled.
    virtual void OnTaskQueueEnabled(internal::TaskQueueImpl* queue) = 0;

    // Called when work becomes available.
    virtual void OnWorkAvailable() = 0;
  };

  // Called once to set the Observer. This function is called
  // on the main thread. If |observer| is null, then no callbacks will occur.
  void SetTaskQueueSelectorObserver(Observer* observer);

  // Returns the priority of the most important pending task if one exists.
  // O(1).
  std::optional<TaskQueue::QueuePriority> GetHighestPendingPriority(
      SelectTaskOption option = SelectTaskOption::kDefault) const;

  // WorkQueueSets::Observer implementation:
  void WorkQueueSetBecameEmpty(size_t set_index) override;
  void WorkQueueSetBecameNonEmpty(size_t set_index) override;

  // Populates |result| with tasks with lower priority than the first task from
  // |selected_work_queue| which could otherwise run now.
  void CollectSkippedOverLowerPriorityTasks(
      const internal::WorkQueue* selected_work_queue,
      std::vector<const Task*>* result) const;

 protected:
  WorkQueueSets* delayed_work_queue_sets() { return &delayed_work_queue_sets_; }

  WorkQueueSets* immediate_work_queue_sets() {
    return &immediate_work_queue_sets_;
  }

  // This method will force select an immediate task if those are being
  // starved by delayed tasks.
  void SetImmediateStarvationCountForTest(int immediate_starvation_count);

  // Tracks which priorities are currently active, meaning there are pending
  // runnable tasks with that priority. Because there are only a handful of
  // priorities, and because we always run tasks in order from highest to lowest
  // priority, we can use a single integer to represent enabled priorities,
  // using a bit per priority.
  class BASE_EXPORT ActivePriorityTracker {
   public:
    ActivePriorityTracker();

    bool HasActivePriority() const { return active_priorities_ != 0; }

    bool IsActive(TaskQueue::QueuePriority priority) const {
      return active_priorities_ & (size_t{1} << static_cast<size_t>(priority));
    }

    void SetActive(TaskQueue::QueuePriority priority, bool is_active);

    TaskQueue::QueuePriority HighestActivePriority() const;

   private:
    static_assert(SequenceManager::PrioritySettings::kMaxPriorities <
                      sizeof(size_t) * 8,
                  "The number of priorities must be strictly less than the "
                  "number of bits of |active_priorities_|!");
    size_t active_priorities_ = 0;
  };

  /*
   * SetOperation is used to configure ChooseWithPriority() and must have:
   *
   * static std::optional<WorkQueueAndTaskOrder>
   * GetWithPriority(const WorkQueueSets& sets,
   *                 TaskQueue::QueuePriority priority);
   */

  // The default
  struct SetOperationOldest {
    static std::optional<WorkQueueAndTaskOrder> GetWithPriority(
        const WorkQueueSets& sets,
        TaskQueue::QueuePriority priority) {
      return sets.GetOldestQueueAndTaskOrderInSet(priority);
    }
  };

#if DCHECK_IS_ON()
  struct SetOperationRandom {
    static std::optional<WorkQueueAndTaskOrder> GetWithPriority(
        const WorkQueueSets& sets,
        TaskQueue::QueuePriority priority) {
      return sets.GetRandomQueueAndTaskOrderInSet(priority);
    }
  };
#endif  // DCHECK_IS_ON()

  template <typename SetOperation>
  WorkQueue* ChooseWithPriority(TaskQueue::QueuePriority priority) const {
    // Maximum number of delayed tasks tasks which can be run while there's a
    // waiting non-delayed task.
    static const int kMaxDelayedStarvationTasks = 3;

    // Select an immediate work queue if we are starving immediate tasks.
    if (immediate_starvation_count_ >= kMaxDelayedStarvationTasks) {
      WorkQueue* queue =
          ChooseImmediateOnlyWithPriority<SetOperation>(priority);
      if (queue) {
        return queue;
      }
      return ChooseDelayedOnlyWithPriority<SetOperation>(priority);
    }
    return ChooseImmediateOrDelayedTaskWithPriority<SetOperation>(priority);
  }

  template <typename SetOperation>
  WorkQueue* ChooseImmediateOnlyWithPriority(
      TaskQueue::QueuePriority priority) const {
    if (auto queue_and_order = SetOperation::GetWithPriority(
            immediate_work_queue_sets_, priority)) {
      return queue_and_order->queue;
    }
    return nullptr;
  }

  template <typename SetOperation>
  WorkQueue* ChooseDelayedOnlyWithPriority(
      TaskQueue::QueuePriority priority) const {
    if (auto queue_and_order =
            SetOperation::GetWithPriority(delayed_work_queue_sets_, priority)) {
      return queue_and_order->queue;
    }
    return nullptr;
  }

 private:
  size_t priority_count() const { return non_empty_set_counts_.size(); }

  void ChangeSetIndex(internal::TaskQueueImpl* queue,
                      TaskQueue::QueuePriority priority);
  void AddQueueImpl(internal::TaskQueueImpl* queue,
                    TaskQueue::QueuePriority priority);
  void RemoveQueueImpl(internal::TaskQueueImpl* queue);

#if DCHECK_IS_ON() || !defined(NDEBUG)
  bool CheckContainsQueueForTest(const internal::TaskQueueImpl* queue) const;
#endif

  template <typename SetOperation>
  WorkQueue* ChooseImmediateOrDelayedTaskWithPriority(
      TaskQueue::QueuePriority priority) const {
    if (auto immediate_queue_and_order = SetOperation::GetWithPriority(
            immediate_work_queue_sets_, priority)) {
      if (auto delayed_queue_and_order = SetOperation::GetWithPriority(
              delayed_work_queue_sets_, priority)) {
        return immediate_queue_and_order->order < delayed_queue_and_order->order
                   ? immediate_queue_and_order->queue
                   : delayed_queue_and_order->queue;
      }
      return immediate_queue_and_order->queue;
    }
    return ChooseDelayedOnlyWithPriority<SetOperation>(priority);
  }

  // Returns true if there are pending tasks with priority |priority|.
  bool HasTasksWithPriority(TaskQueue::QueuePriority priority) const;

  const scoped_refptr<const AssociatedThreadId> associated_thread_;

#if DCHECK_IS_ON()
  const bool random_task_selection_ = false;
#endif

  // Count of the number of sets (delayed or immediate) for each priority.
  // Should only contain 0, 1 or 2.
  std::vector<int> non_empty_set_counts_;

  static constexpr const int kMaxNonEmptySetCount = 2;
  // An atomic is used here because InitializeFeatures() can race with
  // SequenceManager reading this.
  static std::atomic_int g_max_delayed_starvation_tasks;

  // List of active priorities, which is used to work out which priority to run
  // next.
  ActivePriorityTracker active_priority_tracker_;

  WorkQueueSets delayed_work_queue_sets_;
  WorkQueueSets immediate_work_queue_sets_;
  int immediate_starvation_count_ = 0;

  raw_ptr<Observer> task_queue_selector_observer_ = nullptr;  // Not owned.
};

}  // namespace internal
}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_TASK_QUEUE_SELECTOR_H_
