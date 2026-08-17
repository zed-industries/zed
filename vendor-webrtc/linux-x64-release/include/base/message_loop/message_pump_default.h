// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MESSAGE_LOOP_MESSAGE_PUMP_DEFAULT_H_
#define BASE_MESSAGE_LOOP_MESSAGE_PUMP_DEFAULT_H_

#include "base/base_export.h"
#include "base/message_loop/message_pump.h"
#include "base/synchronization/waitable_event.h"
#include "base/time/time.h"
#include "build/build_config.h"

namespace base {

class BASE_EXPORT MessagePumpDefault : public MessagePump {
 public:
  MessagePumpDefault();

  MessagePumpDefault(const MessagePumpDefault&) = delete;
  MessagePumpDefault& operator=(const MessagePumpDefault&) = delete;

  ~MessagePumpDefault() override;

  // MessagePump methods:
  void Run(Delegate* delegate) override;
  void Quit() override;
  void ScheduleWork() override;
  void ScheduleDelayedWork(
      const Delegate::NextWorkInfo& next_work_info) override;

  // Visible for testing.
  void RecordWaitTime(base::TimeDelta wait_time);
  bool ShouldBusyLoop() const;

 private:
  // Returns whether the event was signaled.
  bool BusyWaitOnEvent(base::TimeTicks before);

  // This flag is set to false when Run should return.
  bool keep_running_;

  // Used to sleep until there is more work to do.
  WaitableEvent event_;

  base::TimeDelta last_wait_time_;
  base::TimeDelta wait_time_exponential_moving_average_;
};

}  // namespace base

#endif  // BASE_MESSAGE_LOOP_MESSAGE_PUMP_DEFAULT_H_
