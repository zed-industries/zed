// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_TASK_ORDER_H_
#define BASE_TASK_SEQUENCE_MANAGER_TASK_ORDER_H_

#include "base/base_export.h"
#include "base/task/sequence_manager/enqueue_order.h"
#include "base/time/time.h"

namespace base {
namespace sequence_manager {

struct Task;

namespace internal {
class Fence;
}  // namespace internal

// `TaskOrder` represents the order of a `Task` relative to other `Task`s. The <
// operator on the set of all `TaskOrder`s is a strict total ordering [1].
// `TaskOrder` consists of the following:
//  - `enqueue_order_`: The order the task was enqueued. It is assigned at
//    posting time for immediate tasks and enqueue time for delayed tasks, which
//    is the time at which a pending delayed task is moved to its `WorkQueue`
//    (after its delay has expired, during a wake-up). This is the primary
//    ordering for tasks. Delayed tasks that are enqueued during the same
//    wake-up have the same `enqueue_order_` and their order is decided by
//    `delayed_run_time_` and `sequence_num_`.
//
//  - `delayed_run_time_`: The latest time at which a delayed task should run;
//    only non-zero for delayed tasks. Before they become ripe, delayed tasks
//    are maintained in a heap ordered by `latest_delayed_run_time`.
//
//  - `sequence_num_`: a strictly increasing number assigned at posting time for
//    all tasks. This is used to order delayed tasks if their `enqueue_order_`
//    and `delayed_run_time_`s match.
//
// While `TaskOrder` can be used to order a set `Task`s, it is not necessarily
// the order that the associated tasks will run: tasks are executed in order of
// highest to lowest priority, tasks from disabled queues and queues blocked by
// fences are prevented from running, and sequence manager may choose immediate
// over delayed tasks to prevent starvation.
//
// [1] sequence_num is an int rollovers are possible, however it is extremely
// unlikely that two delayed tasks would have the same posting order and delayed
// run time.
class BASE_EXPORT TaskOrder {
 public:
  TaskOrder(const TaskOrder& other);
  TaskOrder& operator=(const TaskOrder& other);
  ~TaskOrder();

  EnqueueOrder enqueue_order() const { return enqueue_order_; }

  int sequence_num() const { return sequence_num_; }

  // TODO(crbug.com/40158967): Rename to latest_delayed_run_time() for clarity.
  TimeTicks delayed_run_time() const { return delayed_run_time_; }

  static TaskOrder CreateForTesting(EnqueueOrder enqueue_order,
                                    TimeTicks delayed_run_time,
                                    int sequence_num);
  static TaskOrder CreateForTesting(EnqueueOrder enqueue_order);

  bool operator>(const TaskOrder& other) const;
  bool operator<(const TaskOrder& other) const;
  bool operator<=(const TaskOrder& other) const;
  bool operator>=(const TaskOrder& other) const;
  bool operator==(const TaskOrder& other) const;
  bool operator!=(const TaskOrder& other) const;

 protected:
  TaskOrder(EnqueueOrder enqueue_order,
            TimeTicks delayed_run_time,
            int sequence_num);

 private:
  friend class internal::Fence;
  friend struct Task;

  EnqueueOrder enqueue_order_;
  TimeTicks delayed_run_time_;
  int sequence_num_;
};

}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_TASK_ORDER_H_
