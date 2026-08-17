/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_SCENARIO_NETWORK_NODE_H_
#define TEST_SCENARIO_NETWORK_NODE_H_

#include <cstdint>
#include <functional>
#include <memory>

#include "api/array_view.h"
#include "api/call/transport.h"
#include "api/sequence_checker.h"
#include "api/test/network_emulation/network_emulation_interfaces.h"
#include "api/units/data_size.h"
#include "api/units/timestamp.h"
#include "call/call.h"
#include "rtc_base/network_route.h"
#include "rtc_base/socket_address.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"
#include "system_wrappers/include/clock.h"
#include "test/network/network_emulation.h"
#include "test/network/simulated_network.h"
#include "test/scenario/column_printer.h"
#include "test/scenario/scenario_config.h"

namespace webrtc {
namespace test {

class SimulationNode {
 public:
  SimulationNode(NetworkSimulationConfig config,
                 SimulatedNetwork* behavior,
                 EmulatedNetworkNode* network_node);
  static std::unique_ptr<SimulatedNetwork> CreateBehavior(
      NetworkSimulationConfig config);

  void UpdateConfig(std::function<void(NetworkSimulationConfig*)> modifier);
  void PauseTransmissionUntil(Timestamp until);
  ColumnPrinter ConfigPrinter() const;
  EmulatedNetworkNode* node() { return network_node_; }

 private:
  NetworkSimulationConfig config_;
  SimulatedNetwork* const simulation_;
  EmulatedNetworkNode* const network_node_;
};

class NetworkNodeTransport : public Transport {
 public:
  NetworkNodeTransport(Clock* sender_clock, Call* sender_call);
  ~NetworkNodeTransport() override;

  void UpdateAdapterId(int adapter_id);

  bool SendRtp(ArrayView<const uint8_t> packet,
               const PacketOptions& options) override;
  bool SendRtcp(ArrayView<const uint8_t> packet) override;

  void Connect(EmulatedEndpoint* endpoint,
               const SocketAddress& receiver_address,
               DataSize packet_overhead);
  void Disconnect();

  DataSize packet_overhead() {
    MutexLock lock(&mutex_);
    return packet_overhead_;
  }

 private:
  SequenceChecker sequence_checker_;
  int adapter_id_ RTC_GUARDED_BY(sequence_checker_) = 0;

  Mutex mutex_;
  Clock* const sender_clock_;
  Call* const sender_call_;
  EmulatedEndpoint* endpoint_ RTC_GUARDED_BY(mutex_) = nullptr;
  SocketAddress local_address_ RTC_GUARDED_BY(mutex_);
  SocketAddress remote_address_ RTC_GUARDED_BY(mutex_);
  DataSize packet_overhead_ RTC_GUARDED_BY(mutex_) = DataSize::Zero();
  NetworkRoute current_network_route_ RTC_GUARDED_BY(mutex_);
};
}  // namespace test
}  // namespace webrtc
#endif  // TEST_SCENARIO_NETWORK_NODE_H_
