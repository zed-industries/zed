/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_NETWORK_SCHEDULABLE_NETWORK_BEHAVIOR_H_
#define TEST_NETWORK_SCHEDULABLE_NETWORK_BEHAVIOR_H_

#include <cstdint>

#include "absl/functional/any_invocable.h"
#include "api/sequence_checker.h"
#include "api/test/network_emulation/network_config_schedule.pb.h"
#include "api/test/simulated_network.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "rtc_base/task_utils/repeating_task.h"
#include "rtc_base/thread_annotations.h"
#include "system_wrappers/include/clock.h"
#include "test/network/simulated_network.h"

namespace webrtc {

// Network behaviour implementation where parameters change over time as
// specified with a schedule proto.
class SchedulableNetworkBehavior : public SimulatedNetwork {
 public:
  SchedulableNetworkBehavior(
      network_behaviour::NetworkConfigSchedule schedule,
      uint64_t random_seed,
      Clock& clock,
      absl::AnyInvocable<bool(webrtc::Timestamp)> start_condition =
          [](webrtc::Timestamp) { return true; });

  bool EnqueuePacket(PacketInFlightInfo packet_info) override;

 private:
  TimeDelta UpdateConfigAndReschedule();

  SequenceChecker sequence_checker_;
  const network_behaviour::NetworkConfigSchedule schedule_;
  absl::AnyInvocable<bool(webrtc::Timestamp)> start_condition_
      RTC_GUARDED_BY(&sequence_checker_);
  // Send time of the first packet enqueued after `start_condition_` return
  // true.
  Timestamp first_send_time_ RTC_GUARDED_BY(&sequence_checker_) =
      Timestamp::MinusInfinity();

  Clock& clock_ RTC_GUARDED_BY(&sequence_checker_);
  BuiltInNetworkBehaviorConfig config_ RTC_GUARDED_BY(&sequence_checker_);
  // Index of the next schedule item to apply.
  int next_schedule_index_ RTC_GUARDED_BY(&sequence_checker_) = 0;
  // Total time from the first sent packet, until the last time the schedule
  // repeat.
  TimeDelta wrap_time_delta_ RTC_GUARDED_BY(&sequence_checker_) =
      TimeDelta::Zero();
  RepeatingTaskHandle schedule_task_ RTC_GUARDED_BY(&sequence_checker_);
};

}  // namespace webrtc

#endif  // TEST_NETWORK_SCHEDULABLE_NETWORK_BEHAVIOR_H_
