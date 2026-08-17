// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_TASK_SOURCE_SORT_KEY_H_
#define BASE_TASK_THREAD_POOL_TASK_SOURCE_SORT_KEY_H_

#include "base/base_export.h"
#include "base/task/task_traits.h"
#include "base/time/time.h"

namespace base {
namespace internal {

// An immutable but assignable representation of the priority of a Sequence.
class BASE_EXPORT TaskSourceSortKey final {
 public:
  constexpr TaskSourceSortKey() = default;
  constexpr TaskSourceSortKey(TaskPriority priority,
                              TimeTicks ready_time,
                              uint8_t worker_count = 0)
      : priority_(priority),
        worker_count_(worker_count),
        ready_time_(ready_time) {}

  TaskPriority priority() const { return priority_; }
  uint8_t worker_count() const { return worker_count_; }
  TimeTicks ready_time() const { return ready_time_; }

  // Used for a max-heap.
  bool operator<(const TaskSourceSortKey& other) const;

  bool operator==(const TaskSourceSortKey& other) const {
    return priority_ == other.priority_ &&
           worker_count_ == other.worker_count_ &&
           ready_time_ == other.ready_time_;
  }
  bool operator!=(const TaskSourceSortKey& other) const {
    return !(other == *this);
  }

 private:
  // The private section allows this class to keep its immutable property while
  // being copy-assignable (i.e. instead of making its members const).

  // Highest task priority in the sequence at the time this sort key was
  // created.
  TaskPriority priority_;

  // Number of workers running the task source, used as secondary sort key
  // prioritizing task sources with fewer workers.
  uint8_t worker_count_;

  // Time since the task source has been ready to run upcoming work, used as
  // secondary sort key after |worker_count| prioritizing older task sources.
  TimeTicks ready_time_;
};

}  // namespace internal
}  // namespace base

#endif  // BASE_TASK_THREAD_POOL_TASK_SOURCE_SORT_KEY_H_
