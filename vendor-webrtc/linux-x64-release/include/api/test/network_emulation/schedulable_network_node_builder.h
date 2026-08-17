/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef API_TEST_NETWORK_EMULATION_SCHEDULABLE_NETWORK_NODE_BUILDER_H_
#define API_TEST_NETWORK_EMULATION_SCHEDULABLE_NETWORK_NODE_BUILDER_H_

#include <cstdint>
#include <optional>

#include "absl/functional/any_invocable.h"
#include "api/test/network_emulation/network_config_schedule.pb.h"
#include "api/test/network_emulation_manager.h"
#include "api/units/timestamp.h"

namespace webrtc {

class SchedulableNetworkNodeBuilder {
 public:
  SchedulableNetworkNodeBuilder(
      webrtc::NetworkEmulationManager& net,
      network_behaviour::NetworkConfigSchedule schedule);
  // set_start_condition allows a test to control when the schedule start.
  // `start_condition` is invoked every time a packet is enqueued on the network
  // until the first time `start_condition` returns true. Until then, the first
  // NetworkConfigScheduleItem is used. There is no guarantee on which
  // thread/task queue that will be used.
  void set_start_condition(
      absl::AnyInvocable<bool(webrtc::Timestamp)> start_condition);

  // If no random seed is provided, one will be created.
  // The random seed is required for loss rate and to delay standard deviation.
  webrtc::EmulatedNetworkNode* Build(
      std::optional<uint64_t> random_seed = std::nullopt);

 private:
  webrtc::NetworkEmulationManager& net_;
  network_behaviour::NetworkConfigSchedule schedule_;
  absl::AnyInvocable<bool(webrtc::Timestamp)> start_condition_;
};

}  // namespace webrtc

#endif  // API_TEST_NETWORK_EMULATION_SCHEDULABLE_NETWORK_NODE_BUILDER_H_
