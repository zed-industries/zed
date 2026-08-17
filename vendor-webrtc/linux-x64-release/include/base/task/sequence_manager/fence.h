// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_FENCE_H_
#define BASE_TASK_SEQUENCE_MANAGER_FENCE_H_

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/task/sequence_manager/enqueue_order.h"
#include "base/task/sequence_manager/task_order.h"

namespace base {

class TimeTicks;

namespace sequence_manager {
namespace internal {

class TaskQueueImpl;

// `Fence`s are used to prevent the execution of tasks starting with a
// particular `TaskOrder`, such that for a `Task` and a `Fence`, if
// task.task_order() >= fence.task_order(), then the task is blocked from
// running. Blocking fences are a special kind of fence that have a `TaskOrder`
// less than that of any `Task`.
class BASE_EXPORT Fence {
 public:
  // Creates a `Fence` with the same `TaskOrder` as `task_order`, which is
  // useful for creating a fence relative to a particular `Task`.
  // `task_order.enqueue_order()` must be "set", i.e. it cannot be
  // `EnqueueOrder::none()`.
  explicit Fence(const TaskOrder& task_order);
  Fence(const Fence& other);
  Fence& operator=(const Fence& other);
  ~Fence();

  // Creates a blocking fence which has a `TaskOrder` that is less than that of
  // all tasks.
  static Fence BlockingFence();

  const TaskOrder& task_order() const LIFETIME_BOUND { return task_order_; }

  // Returns true iff this is a blocking fence.
  bool IsBlockingFence() const {
    return task_order_.enqueue_order() == EnqueueOrder::blocking_fence();
  }

 private:
  friend class TaskQueueImpl;  // For `CreateWithEnqueueOrder()`.

  Fence(EnqueueOrder enqueue_order,
        TimeTicks delayed_run_time,
        int sequence_num);

  // Creates a `Fence` with `enqueue_order` and a null delayed run time.
  // `enqueue_order` cannot be EnqueueOrder::none().
  static Fence CreateWithEnqueueOrder(EnqueueOrder enqueue_order);

  TaskOrder task_order_;
};

}  // namespace internal
}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_FENCE_H_
