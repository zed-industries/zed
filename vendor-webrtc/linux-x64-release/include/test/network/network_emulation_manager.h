/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_NETWORK_NETWORK_EMULATION_MANAGER_H_
#define TEST_NETWORK_NETWORK_EMULATION_MANAGER_H_

#include <cstdint>
#include <functional>
#include <list>
#include <map>
#include <memory>
#include <optional>
#include <set>
#include <utility>
#include <vector>

#include "absl/base/nullability.h"
#include "api/array_view.h"
#include "api/test/network_emulation/cross_traffic.h"
#include "api/test/network_emulation/network_emulation_interfaces.h"
#include "api/test/network_emulation_manager.h"
#include "api/test/simulated_network.h"
#include "api/test/time_controller.h"
#include "api/units/timestamp.h"
#include "rtc_base/ip_address.h"
#include "rtc_base/task_queue_for_test.h"
#include "rtc_base/task_utils/repeating_task.h"
#include "system_wrappers/include/clock.h"
#include "test/network/cross_traffic.h"
#include "test/network/emulated_network_manager.h"
#include "test/network/emulated_turn_server.h"
#include "test/network/network_emulation.h"

namespace webrtc {
namespace test {

class NetworkEmulationManagerImpl : public NetworkEmulationManager {
 public:
  explicit NetworkEmulationManagerImpl(NetworkEmulationManagerConfig config);
  ~NetworkEmulationManagerImpl();

  EmulatedNetworkNode* CreateEmulatedNode(BuiltInNetworkBehaviorConfig config,
                                          uint64_t random_seed = 1) override;
  EmulatedNetworkNode* CreateEmulatedNode(
      std::unique_ptr<NetworkBehaviorInterface> network_behavior) override;

  SimulatedNetworkNode::Builder NodeBuilder() override;

  EmulatedEndpointImpl* CreateEndpoint(EmulatedEndpointConfig config) override;
  void EnableEndpoint(EmulatedEndpoint* endpoint) override;
  void DisableEndpoint(EmulatedEndpoint* endpoint) override;

  EmulatedRoute* CreateRoute(EmulatedEndpoint* from,
                             const std::vector<EmulatedNetworkNode*>& via_nodes,
                             EmulatedEndpoint* to) override;

  EmulatedRoute* CreateRoute(
      const std::vector<EmulatedNetworkNode*>& via_nodes) override;

  EmulatedRoute* CreateDefaultRoute(
      EmulatedEndpoint* from,
      const std::vector<EmulatedNetworkNode*>& via_nodes,
      EmulatedEndpoint* to) override;

  void ClearRoute(EmulatedRoute* route) override;

  TcpMessageRoute* CreateTcpRoute(EmulatedRoute* send_route,
                                  EmulatedRoute* ret_route) override;

  CrossTrafficRoute* CreateCrossTrafficRoute(
      const std::vector<EmulatedNetworkNode*>& via_nodes) override;

  CrossTrafficGenerator* StartCrossTraffic(
      std::unique_ptr<CrossTrafficGenerator> generator) override;
  void StopCrossTraffic(CrossTrafficGenerator* generator) override;

  EmulatedNetworkManagerInterface* absl_nonnull
  CreateEmulatedNetworkManagerInterface(
      const std::vector<EmulatedEndpoint*>& endpoints) override;

  void GetStats(
      ArrayView<EmulatedEndpoint* const> endpoints,
      std::function<void(EmulatedNetworkStats)> stats_callback) override;

  void GetStats(
      ArrayView<EmulatedNetworkNode* const> nodes,
      std::function<void(EmulatedNetworkNodeStats)> stats_callback) override;

  TimeController* time_controller() override { return time_controller_.get(); }

  TimeMode time_mode() const override { return time_mode_; }

  Timestamp Now() const;

  EmulatedTURNServerInterface* CreateTURNServer(
      EmulatedTURNServerConfig config) override;

 private:
  using CrossTrafficSource =
      std::pair<std::unique_ptr<CrossTrafficGenerator>, RepeatingTaskHandle>;

  std::optional<IPAddress> GetNextIPv4Address();

  const TimeMode time_mode_;
  const EmulatedNetworkStatsGatheringMode stats_gathering_mode_;
  const std::unique_ptr<TimeController> time_controller_;
  Clock* const clock_;
  const bool fake_dtls_handshake_sizes_;
  int next_node_id_;

  RepeatingTaskHandle process_task_handle_;

  uint32_t next_ip4_address_;
  std::set<IPAddress> used_ip_addresses_;

  // All objects can be added to the manager only when it is idle.
  std::vector<std::unique_ptr<EmulatedEndpoint>> endpoints_;
  std::vector<std::unique_ptr<EmulatedNetworkNode>> network_nodes_;
  std::vector<std::unique_ptr<EmulatedRoute>> routes_;
  std::vector<std::unique_ptr<CrossTrafficRoute>> traffic_routes_;
  std::vector<CrossTrafficSource> cross_traffics_;
  std::list<std::unique_ptr<TcpMessageRouteImpl>> tcp_message_routes_;
  std::vector<std::unique_ptr<EndpointsContainer>> endpoints_containers_;
  std::vector<std::unique_ptr<EmulatedNetworkManager>> network_managers_;
  std::vector<std::unique_ptr<EmulatedTURNServer>> turn_servers_;

  std::map<EmulatedEndpoint*, EmulatedNetworkManager*>
      endpoint_to_network_manager_;

  // Must be the last field, so it will be deleted first, because tasks
  // in the TaskQueue can access other fields of the instance of this class.
  TaskQueueForTest task_queue_;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_NETWORK_NETWORK_EMULATION_MANAGER_H_
