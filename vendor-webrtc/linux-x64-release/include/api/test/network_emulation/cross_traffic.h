/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef API_TEST_NETWORK_EMULATION_CROSS_TRAFFIC_H_
#define API_TEST_NETWORK_EMULATION_CROSS_TRAFFIC_H_

#include <cstddef>
#include <functional>

#include "api/units/data_rate.h"
#include "api/units/data_size.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"

namespace webrtc {

// This API is still in development and can be changed without prior notice.

// Represents the endpoint for cross traffic that is going through the network.
// It can be used to emulate unexpected network load.
class CrossTrafficRoute {
 public:
  virtual ~CrossTrafficRoute() = default;

  // Triggers sending of dummy packets with size `packet_size` bytes.
  virtual void TriggerPacketBurst(size_t num_packets, size_t packet_size) = 0;
  // Sends a packet over the nodes. The content of the packet is unspecified;
  // only the size metter for the emulation purposes.
  virtual void SendPacket(size_t packet_size) = 0;
  // Sends a packet over the nodes and runs `action` when it has been delivered.
  virtual void NetworkDelayedAction(size_t packet_size,
                                    std::function<void()> action) = 0;
};

// Describes a way of generating cross traffic on some route. Used by
// NetworkEmulationManager to produce cross traffic during some period of time.
class CrossTrafficGenerator {
 public:
  virtual ~CrossTrafficGenerator() = default;

  // Time between Process calls.
  virtual TimeDelta GetProcessInterval() const = 0;

  // Called periodically by NetworkEmulationManager. Generates traffic on the
  // route.
  virtual void Process(Timestamp at_time) = 0;
};

// Config of a cross traffic generator. Generated traffic rises and falls
// randomly.
struct RandomWalkConfig {
  int random_seed = 1;
  DataRate peak_rate = DataRate::KilobitsPerSec(100);
  DataSize min_packet_size = DataSize::Bytes(200);
  TimeDelta min_packet_interval = TimeDelta::Millis(1);
  TimeDelta update_interval = TimeDelta::Millis(200);
  double variance = 0.6;
  double bias = -0.1;
};

// Config of a cross traffic generator. Generated traffic has form of periodic
// peaks alternating with periods of silence.
struct PulsedPeaksConfig {
  DataRate peak_rate = DataRate::KilobitsPerSec(100);
  DataSize min_packet_size = DataSize::Bytes(200);
  TimeDelta min_packet_interval = TimeDelta::Millis(1);
  TimeDelta send_duration = TimeDelta::Millis(100);
  TimeDelta hold_duration = TimeDelta::Millis(2000);
};

struct FakeTcpConfig {
  DataSize packet_size = DataSize::Bytes(1200);
  DataSize send_limit = DataSize::PlusInfinity();
  TimeDelta process_interval = TimeDelta::Millis(200);
  TimeDelta packet_timeout = TimeDelta::Seconds(1);
};

}  // namespace webrtc

#endif  // API_TEST_NETWORK_EMULATION_CROSS_TRAFFIC_H_
