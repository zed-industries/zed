// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_WEBRTC_OVERRIDES_COALESCED_TASKS_H_
#define THIRD_PARTY_WEBRTC_OVERRIDES_COALESCED_TASKS_H_

#include <cstdint>
#include <map>
#include <optional>
#include <set>

#include "base/functional/callback.h"
#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "base/time/time.h"
#include "third_party/abseil-cpp/absl/functional/any_invocable.h"
#include "third_party/webrtc/rtc_base/system/rtc_export.h"

namespace blink {

// A thread-safe class for storing queued tasks until they are scheduled to run.
// This is useful for implementing metronome-like task queues where tasks are
// coalesced onto metronome ticks but the tasks still need to execute in order.
class RTC_EXPORT CoalescedTasks {
 public:
  typedef base::RepeatingCallback<std::optional<base::TimeTicks>()>
      PrepareRunTaskCallback;
  typedef base::RepeatingCallback<void(std::optional<base::TimeTicks>)>
      FinalizeRunTaskCallback;

  ~CoalescedTasks();

  // Queue a delayed tasks that can later be run with RunScheduledTasks().
  // `task_time` is the original run time of `task` and `scheduled_time` is the
  // `task_time` but snapped to the next time that scheduled tasks may run.
  // If true is returned, this is the first task that has been queued to run on
  // `scheduled_time`. In this case, the caller is responsible for scheduling a
  // call to RunScheduledTasks() at `scheduled_time`.
  bool QueueDelayedTask(base::TimeTicks task_time,
                        absl::AnyInvocable<void() &&> task,
                        base::TimeTicks scheduled_time);
  // Run all queued tasks up to and including `scheduled_time`. If multiple
  // tasks were queued onto the same `scheduled_time` they will execute in order
  // of their `task_time`. Optionally, the prepare/finalize callbacks can be
  // used to sample task run times by being called before/after each task.
  void RunScheduledTasks(base::TimeTicks scheduled_time,
                         PrepareRunTaskCallback prepare_run_task_callback =
                             PrepareRunTaskCallback(),
                         FinalizeRunTaskCallback finalize_run_task_callback =
                             FinalizeRunTaskCallback());
  // Clear the queue, deleting all tasks on the calling sequence without running
  // them.
  void Clear();

  // Returns true if there are no stored tasks.
  bool Empty() const {
    base::AutoLock lock(lock_);
    return delayed_tasks_.empty();
  }

 private:
  // The (time_ticks, unique_id) pair allows multiple tasks to be scheduled on
  // the same `time_ticks`.
  struct RTC_EXPORT UniqueTimeTicks {
    UniqueTimeTicks(base::TimeTicks time_ticks, uint64_t unique_id);

    // Used for std::map<> ordering.
    bool operator<(const UniqueTimeTicks& other) const;

    base::TimeTicks time_ticks;
    uint64_t unique_id;
  };

  mutable base::Lock lock_;
  std::set<base::TimeTicks> scheduled_ticks_ GUARDED_BY(lock_);
  uint64_t next_unique_id_ GUARDED_BY(lock_) = 0;
  std::map<UniqueTimeTicks, absl::AnyInvocable<void() &&>> delayed_tasks_
      GUARDED_BY(lock_);
};

}  // namespace blink

#endif  // THIRD_PARTY_WEBRTC_OVERRIDES_COALESCED_TASKS_H_
