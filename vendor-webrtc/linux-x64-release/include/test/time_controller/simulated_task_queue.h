/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_TIME_CONTROLLER_SIMULATED_TASK_QUEUE_H_
#define TEST_TIME_CONTROLLER_SIMULATED_TASK_QUEUE_H_

#include <deque>
#include <map>
#include <memory>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "api/units/time_delta.h"
#include "rtc_base/synchronization/mutex.h"
#include "test/time_controller/simulated_time_controller.h"

namespace webrtc {

class SimulatedTaskQueue : public TaskQueueBase,
                           public sim_time_impl::SimulatedSequenceRunner {
 public:
  SimulatedTaskQueue(sim_time_impl::SimulatedTimeControllerImpl* handler,
                     absl::string_view name);

  ~SimulatedTaskQueue();

  void RunReady(Timestamp at_time) override;

  Timestamp GetNextRunTime() const override {
    MutexLock lock(&lock_);
    return next_run_time_;
  }
  TaskQueueBase* GetAsTaskQueue() override { return this; }

  // TaskQueueBase interface
  void Delete() override;
  void PostTaskImpl(absl::AnyInvocable<void() &&> task,
                    const PostTaskTraits& traits,
                    const Location& location) override;
  void PostDelayedTaskImpl(absl::AnyInvocable<void() &&> task,
                           TimeDelta delay,
                           const PostDelayedTaskTraits& traits,
                           const Location& location) override;

 private:
  sim_time_impl::SimulatedTimeControllerImpl* const handler_;
  // Using char* to be debugger friendly.
  char* name_;

  mutable Mutex lock_;

  std::deque<absl::AnyInvocable<void() &&>> ready_tasks_ RTC_GUARDED_BY(lock_);
  std::map<Timestamp, std::vector<absl::AnyInvocable<void() &&>>> delayed_tasks_
      RTC_GUARDED_BY(lock_);

  Timestamp next_run_time_ RTC_GUARDED_BY(lock_) = Timestamp::PlusInfinity();
};

}  // namespace webrtc

#endif  // TEST_TIME_CONTROLLER_SIMULATED_TASK_QUEUE_H_
