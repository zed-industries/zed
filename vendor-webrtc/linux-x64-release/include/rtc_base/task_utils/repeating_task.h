/*
 *  Copyright 2019 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_TASK_UTILS_REPEATING_TASK_H_
#define RTC_BASE_TASK_UTILS_REPEATING_TASK_H_

#include <memory>
#include <type_traits>
#include <utility>

#include "absl/functional/any_invocable.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "api/task_queue/task_queue_base.h"
#include "api/units/time_delta.h"
#include "system_wrappers/include/clock.h"

namespace webrtc {

namespace webrtc_repeating_task_impl {

// Methods simplifying external tracing of RepeatingTaskHandle operations.
void RepeatingTaskHandleDTraceProbeStart();
void RepeatingTaskHandleDTraceProbeDelayedStart();
void RepeatingTaskImplDTraceProbeRun();

}  // namespace webrtc_repeating_task_impl

// Allows starting tasks that repeat themselves on a TaskQueue indefinately
// until they are stopped or the TaskQueue is destroyed. It allows starting and
// stopping multiple times, but you must stop one task before starting another
// and it can only be stopped when in the running state. The public interface is
// not thread safe.
class RepeatingTaskHandle {
 public:
  RepeatingTaskHandle() = default;
  ~RepeatingTaskHandle() = default;
  RepeatingTaskHandle(RepeatingTaskHandle&& other) = default;
  RepeatingTaskHandle& operator=(RepeatingTaskHandle&& other) = default;
  RepeatingTaskHandle(const RepeatingTaskHandle&) = delete;
  RepeatingTaskHandle& operator=(const RepeatingTaskHandle&) = delete;

  // Start can be used to start a task that will be reposted with a delay
  // determined by the return value of the provided closure. The actual task is
  // owned by the TaskQueue and will live until it has been stopped or the
  // TaskQueue deletes it. It's perfectly fine to destroy the handle while the
  // task is running, since the repeated task is owned by the TaskQueue.
  // The tasks are scheduled onto the task queue using the specified precision.
  static RepeatingTaskHandle Start(
      TaskQueueBase* task_queue,
      absl::AnyInvocable<TimeDelta()> closure,
      TaskQueueBase::DelayPrecision precision =
          TaskQueueBase::DelayPrecision::kLow,
      Clock* clock = Clock::GetRealTimeClock(),
      const Location& location = Location::Current());

  // DelayedStart is equivalent to Start except that the first invocation of the
  // closure will be delayed by the given amount.
  static RepeatingTaskHandle DelayedStart(
      TaskQueueBase* task_queue,
      TimeDelta first_delay,
      absl::AnyInvocable<TimeDelta()> closure,
      TaskQueueBase::DelayPrecision precision =
          TaskQueueBase::DelayPrecision::kLow,
      Clock* clock = Clock::GetRealTimeClock(),
      const Location& location = Location::Current());

  // Stops future invocations of the repeating task closure. Can only be called
  // from the TaskQueue where the task is running. The closure is guaranteed to
  // not be running after Stop() returns unless Stop() is called from the
  // closure itself.
  void Stop();

  // Returns true until Stop() was called.
  // Can only be called from the TaskQueue where the task is running.
  bool Running() const;

 private:
  explicit RepeatingTaskHandle(scoped_refptr<PendingTaskSafetyFlag> alive_flag)
      : repeating_task_(std::move(alive_flag)) {}
  scoped_refptr<PendingTaskSafetyFlag> repeating_task_;
};

}  // namespace webrtc
#endif  // RTC_BASE_TASK_UTILS_REPEATING_TASK_H_
