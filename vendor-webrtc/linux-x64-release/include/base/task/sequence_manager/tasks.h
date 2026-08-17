// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_TASKS_H_
#define BASE_TASK_SEQUENCE_MANAGER_TASKS_H_

#include <optional>
#include <variant>

#include "base/base_export.h"
#include "base/check.h"
#include "base/containers/intrusive_heap.h"
#include "base/dcheck_is_on.h"
#include "base/pending_task.h"
#include "base/task/delay_policy.h"
#include "base/task/sequence_manager/delayed_task_handle_delegate.h"
#include "base/task/sequence_manager/enqueue_order.h"
#include "base/task/sequenced_task_runner.h"

namespace base {
namespace sequence_manager {

using TaskType = uint8_t;
constexpr TaskType kTaskTypeNone = 0;

class TaskOrder;

namespace internal {

// Wrapper around PostTask method arguments and the assigned task type.
// Eventually it becomes a PendingTask once accepted by a TaskQueueImpl.
struct BASE_EXPORT PostedTask {
  explicit PostedTask(scoped_refptr<SequencedTaskRunner> task_runner,
                      OnceClosure callback,
                      Location location,
                      TimeDelta delay = base::TimeDelta(),
                      Nestable nestable = Nestable::kNestable,
                      TaskType task_type = kTaskTypeNone,
                      WeakPtr<DelayedTaskHandleDelegate>
                          delayed_task_handle_delegate = nullptr);
  explicit PostedTask(scoped_refptr<SequencedTaskRunner> task_runner,
                      OnceClosure callback,
                      Location location,
                      TimeTicks delayed_run_time,
                      subtle::DelayPolicy delay_policy,
                      Nestable nestable = Nestable::kNestable,
                      TaskType task_type = kTaskTypeNone,
                      WeakPtr<DelayedTaskHandleDelegate>
                          delayed_task_handle_delegate = nullptr);
  PostedTask(PostedTask&& move_from) noexcept;
  PostedTask(const PostedTask&) = delete;
  PostedTask& operator=(const PostedTask&) = delete;
  ~PostedTask();

  bool is_delayed() const {
    return std::holds_alternative<TimeTicks>(delay_or_delayed_run_time)
               ? !std::get<TimeTicks>(delay_or_delayed_run_time).is_null()
               : !std::get<TimeDelta>(delay_or_delayed_run_time).is_zero();
  }

  OnceClosure callback;
  Location location;
  Nestable nestable = Nestable::kNestable;
  TaskType task_type = kTaskTypeNone;
  std::variant<TimeDelta, TimeTicks> delay_or_delayed_run_time;
  subtle::DelayPolicy delay_policy = subtle::DelayPolicy::kFlexibleNoSooner;
  // The task runner this task is running on. Can be used by task runners that
  // support posting back to the "current sequence".
  scoped_refptr<SequencedTaskRunner> task_runner;
  // The delegate for the DelayedTaskHandle, if this task was posted through
  // PostCancelableDelayedTask(), nullptr otherwise.
  WeakPtr<DelayedTaskHandleDelegate> delayed_task_handle_delegate;
};

}  // namespace internal

// Represents a time at which a task wants to run.
struct WakeUp {
  // is_null() for immediate wake up.
  TimeTicks time;
  // These are meaningless if is_immediate().
  TimeDelta leeway;
  subtle::DelayPolicy delay_policy = subtle::DelayPolicy::kFlexibleNoSooner;

  bool operator!=(const WakeUp& other) const {
    return time != other.time || leeway != other.leeway ||
           delay_policy != other.delay_policy;
  }

  bool operator==(const WakeUp& other) const { return !(*this != other); }

  bool is_immediate() const { return time.is_null(); }

  TimeTicks earliest_time() const;
  TimeTicks latest_time() const;
};

// PendingTask with extra metadata for SequenceManager.
struct BASE_EXPORT Task : public PendingTask {
  Task(internal::PostedTask posted_task,
       EnqueueOrder sequence_order,
       EnqueueOrder enqueue_order = EnqueueOrder(),
       TimeTicks queue_time = TimeTicks(),
       TimeDelta leeway = TimeDelta());
  Task(Task&& move_from);
  ~Task();
  Task& operator=(Task&& other);

  // SequenceManager is particularly sensitive to enqueue order,
  // so we have accessors for safety.
  EnqueueOrder enqueue_order() const {
    DCHECK(enqueue_order_);
    return enqueue_order_;
  }

  void set_enqueue_order(EnqueueOrder enqueue_order) {
    DCHECK(!enqueue_order_);
    enqueue_order_ = enqueue_order;
  }

  bool enqueue_order_set() const { return enqueue_order_; }

  TaskOrder task_order() const;

  // OK to dispatch from a nested loop.
  Nestable nestable = Nestable::kNonNestable;

  TaskType task_type;

  // The task runner this task is running on. Can be used by task runners that
  // support posting back to the "current sequence".
  scoped_refptr<SequencedTaskRunner> task_runner;

#if DCHECK_IS_ON()
  bool cross_thread_;
#endif

  // Implement the intrusive heap contract.
  void SetHeapHandle(HeapHandle heap_handle);
  void ClearHeapHandle();
  HeapHandle GetHeapHandle() const;

  // Returns true if this task was canceled, either through weak pointer
  // invalidation or through |delayed_task_handle_delegate_|.
  bool IsCanceled() const;

  // Must be invoked before running the task. Returns true if the task must run
  // (any delayed task handle will have been invalidated by this method), false
  // if it mustn't run (e.g. delayed task handle was invalidated prior to
  // calling this method).
  bool WillRunTask();

 private:
  // `enqueue_order_` is the primary component used to order tasks (see
  // `TaskOrder`). For immediate tasks, `enqueue_order` is set when posted, but
  // for delayed tasks it's not defined until they are enqueued. This is because
  // otherwise delayed tasks could run before an immediate task posted after the
  // delayed task.
  EnqueueOrder enqueue_order_;

  // The delegate for the DelayedTaskHandle, if this task was posted through
  // `PostCancelableDelayedTask()`, not set otherwise. The task is canceled if
  // `WeakPtr::WasInvalidated` is true. Note: if the task was not posted via
  // `PostCancelableDelayedTask()`. the weak pointer won't be valid, but
  // `WeakPtr::WasInvalidated` will be false.
  WeakPtr<internal::DelayedTaskHandleDelegate> delayed_task_handle_delegate_;
};

}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_TASKS_H_
