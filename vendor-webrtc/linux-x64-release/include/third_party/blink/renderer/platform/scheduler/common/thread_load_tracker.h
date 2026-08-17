// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THREAD_LOAD_TRACKER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THREAD_LOAD_TRACKER_H_

#include "base/functional/callback.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {
namespace scheduler {

// This class tracks thread load level, i.e. percentage of wall time spent
// running tasks.
// In order to avoid bias it reports load level at regular intervals.
// Every |reporting_interval_| time units, it reports the average thread load
// level computed using a sliding window of width |reporting_interval_|.
class PLATFORM_EXPORT ThreadLoadTracker {
  DISALLOW_NEW();

 public:
  // Callback is called with (current_time, load_level) parameters.
  using Callback = base::RepeatingCallback<void(base::TimeTicks, double)>;

  ThreadLoadTracker(base::TimeTicks now,
                    const Callback& callback,
                    base::TimeDelta reporting_interval);
  ~ThreadLoadTracker();

  void Pause(base::TimeTicks now);
  void Resume(base::TimeTicks now);

  // Note: this does not change |thread_state_|.
  void Reset(base::TimeTicks now);

  void RecordTaskTime(base::TimeTicks start_time, base::TimeTicks end_time);

  void RecordIdle(base::TimeTicks now);

  // TODO(altimin): Count wake-ups.

 private:
  enum class ThreadState { kActive, kPaused };

  enum class TaskState { kTaskRunning, kIdle };

  // This function advances |time_| to |now|, calling |callback_|
  // in the process (multiple times if needed).
  void Advance(base::TimeTicks now, TaskState task_state);

  double Load();

  // |time_| is the last timestamp LoadTracker knows about.
  base::TimeTicks time_;
  base::TimeTicks next_reporting_time_;

  ThreadState thread_state_;
  base::TimeTicks last_state_change_time_;

  base::TimeDelta reporting_interval_;

  // Recorded run time in window
  // [next_reporting_time - reporting_interval, next_reporting_time].
  base::TimeDelta run_time_inside_window_;

  Callback callback_;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THREAD_LOAD_TRACKER_H_
