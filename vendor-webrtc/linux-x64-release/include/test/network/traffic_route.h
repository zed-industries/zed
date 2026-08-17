/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_NETWORK_TRAFFIC_ROUTE_H_
#define TEST_NETWORK_TRAFFIC_ROUTE_H_

#include <cstddef>
#include <cstdint>
#include <functional>
#include <memory>
#include <vector>

#include "api/test/network_emulation/cross_traffic.h"
#include "api/test/network_emulation/network_emulation_interfaces.h"
#include "system_wrappers/include/clock.h"
#include "test/network/network_emulation.h"

namespace webrtc {
namespace test {

// Represents the endpoint for cross traffic that is going through the network.
// It can be used to emulate unexpected network load.
class CrossTrafficRouteImpl final : public CrossTrafficRoute {
 public:
  CrossTrafficRouteImpl(Clock* clock,
                        EmulatedNetworkReceiverInterface* receiver,
                        EmulatedEndpointImpl* endpoint);
  ~CrossTrafficRouteImpl();

  // Triggers sending of dummy packets with size `packet_size` bytes.
  void TriggerPacketBurst(size_t num_packets, size_t packet_size) override;
  // Sends a packet over the nodes and runs `action` when it has been delivered.
  void NetworkDelayedAction(size_t packet_size,
                            std::function<void()> action) override;

  void SendPacket(size_t packet_size) override;

 private:
  void SendPacket(size_t packet_size, uint16_t dest_port);

  Clock* const clock_;
  EmulatedNetworkReceiverInterface* const receiver_;
  EmulatedEndpointImpl* const endpoint_;

  uint16_t null_receiver_port_;
  std::unique_ptr<EmulatedNetworkReceiverInterface> null_receiver_;
  std::vector<std::unique_ptr<EmulatedNetworkReceiverInterface>> actions_;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_NETWORK_TRAFFIC_ROUTE_H_
