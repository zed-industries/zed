/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_TIME_CONTROLLER_SIMULATED_THREAD_H_
#define TEST_TIME_CONTROLLER_SIMULATED_THREAD_H_

#include <memory>

#include "rtc_base/synchronization/mutex.h"
#include "test/time_controller/simulated_time_controller.h"

namespace webrtc {

class SimulatedThread : public Thread,
                        public sim_time_impl::SimulatedSequenceRunner {
 public:
  using CurrentThreadSetter = CurrentThreadSetter;
  SimulatedThread(sim_time_impl::SimulatedTimeControllerImpl* handler,
                  absl::string_view name,
                  std::unique_ptr<SocketServer> socket_server);
  ~SimulatedThread() override;

  void RunReady(Timestamp at_time) override;

  Timestamp GetNextRunTime() const override {
    MutexLock lock(&lock_);
    return next_run_time_;
  }

  TaskQueueBase* GetAsTaskQueue() override { return this; }

  // Thread interface
  void BlockingCallImpl(FunctionView<void()> functor,
                        const Location& location) override;
  void PostTaskImpl(absl::AnyInvocable<void() &&> task,
                    const PostTaskTraits& traits,
                    const Location& location) override;
  void PostDelayedTaskImpl(absl::AnyInvocable<void() &&> task,
                           TimeDelta delay,
                           const PostDelayedTaskTraits& traits,
                           const Location& location) override;

  void Stop() override;

 private:
  sim_time_impl::SimulatedTimeControllerImpl* const handler_;
  // Using char* to be debugger friendly.
  char* name_;
  mutable Mutex lock_;
  Timestamp next_run_time_ RTC_GUARDED_BY(lock_) = Timestamp::PlusInfinity();
};

class SimulatedMainThread : public SimulatedThread {
 public:
  explicit SimulatedMainThread(
      sim_time_impl::SimulatedTimeControllerImpl* handler);
  ~SimulatedMainThread();

 private:
  CurrentThreadSetter current_setter_;
};
}  // namespace webrtc
#endif  // TEST_TIME_CONTROLLER_SIMULATED_THREAD_H_
