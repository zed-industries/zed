/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_NETWORK_EMULATION_MANAGER_H_
#define API_TEST_NETWORK_EMULATION_MANAGER_H_

#include <cstdint>
#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/base/nullability.h"
#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/field_trials_view.h"
#include "api/test/network_emulation/cross_traffic.h"
#include "api/test/network_emulation/network_emulation_interfaces.h"
#include "api/test/peer_network_dependencies.h"
#include "api/test/simulated_network.h"
#include "api/test/time_controller.h"
#include "api/units/data_rate.h"
#include "rtc_base/ip_address.h"
#include "rtc_base/network_constants.h"
#include "rtc_base/socket_address.h"

namespace webrtc {

// This API is still in development and can be changed without prior notice.

// These classes are forward declared here, because they used as handles, to
// make it possible for client code to operate with these abstractions and build
// required network configuration. With forward declaration here implementation
// is more readable, than with interfaces approach and cause user needn't any
// API methods on these abstractions it is acceptable here.

// EmulatedNetworkNode is an abstraction for some network in the real world,
// like 3G network between peers, or Wi-Fi for one peer and LTE for another.
// Multiple networks can be joined into chain emulating a network path from
// one peer to another.
class EmulatedNetworkNode;

// EmulatedRoute is handle for single route from one network interface on one
// peer device to another network interface on another peer device.
class EmulatedRoute;

enum class EmulatedNetworkStatsGatheringMode {
  // Gather main network stats counters. See more details on which particular
  // metrics are collected in the `EmulatedNetworkStats` and
  // `EmulatedNetworkNodeStats` documentation.
  kDefault,
  // kDefault + also gather per packet statistics. In this mode more memory
  // will be used.
  kDebug
};

struct EmulatedEndpointConfig {
  enum class IpAddressFamily { kIpv4, kIpv6 };

  // If specified will be used to name endpoint for logging purposes.
  std::optional<std::string> name = std::nullopt;
  IpAddressFamily generated_ip_family = IpAddressFamily::kIpv4;
  // If specified will be used as IP address for endpoint node. Must be unique
  // among all created nodes.
  std::optional<IPAddress> ip;
  // Should endpoint be enabled or not, when it will be created.
  // Enabled endpoints will be available for webrtc to send packets.
  bool start_as_enabled = true;
  // Network type which will be used to represent endpoint to WebRTC.
  AdapterType type = AdapterType::ADAPTER_TYPE_UNKNOWN;
  // Allow endpoint to send packets specifying source IP address different to
  // the current endpoint IP address. If false endpoint will crash if attempt
  // to send such packet will be done.
  bool allow_send_packet_with_different_source_ip = false;
  // Allow endpoint to receive packet with destination IP address different to
  // the current endpoint IP address. If false endpoint will crash if such
  // packet will arrive.
  bool allow_receive_packets_with_different_dest_ip = false;
};

struct EmulatedTURNServerConfig {
  EmulatedEndpointConfig client_config;
  EmulatedEndpointConfig peer_config;
  bool enable_permission_checks = true;
};

// EmulatedTURNServer is an abstraction for a TURN server.
class EmulatedTURNServerInterface {
 public:
  struct IceServerConfig {
    std::string username;
    std::string password;
    std::string url;
  };

  virtual ~EmulatedTURNServerInterface() {}

  // Get an IceServer configuration suitable to add to a PeerConnection.
  virtual IceServerConfig GetIceServerConfig() const = 0;

  // Get non-null client endpoint, an endpoint that accepts TURN allocations.
  // This shall typically be connected to one or more webrtc endpoint.
  virtual EmulatedEndpoint* GetClientEndpoint() const = 0;

  // Returns socket address, which client should use to connect to TURN server
  // and do TURN allocation.
  virtual SocketAddress GetClientEndpointAddress() const = 0;

  // Get non-null peer endpoint, that is "connected to the internet".
  // This shall typically be connected to another TURN server.
  virtual EmulatedEndpoint* GetPeerEndpoint() const = 0;
};

// Provide interface to obtain all required objects to inject network emulation
// layer into PeerConnection. Also contains information about network interfaces
// accessible by PeerConnection.
class EmulatedNetworkManagerInterface
    : public webrtc_pc_e2e::PeerNetworkDependencies {
 public:
  ~EmulatedNetworkManagerInterface() override = default;

  // Returns list of endpoints that are associated with this instance. Pointers
  // are guaranteed to be non-null and are owned by NetworkEmulationManager.
  virtual std::vector<EmulatedEndpoint*> endpoints() const = 0;

  // Passes summarized network stats for endpoints for this manager into
  // specified `stats_callback`. Callback will be executed on network emulation
  // internal task queue.
  virtual void GetStats(
      std::function<void(EmulatedNetworkStats)> stats_callback) const = 0;
};

enum class TimeMode { kRealTime, kSimulated };

// Called implicitly when parsing an ABSL_FLAG of type TimeMode.
// from the command line flag value `text`.
// Returns `true` and sets `*mode` on success;
// returns `false` and sets `*error` on failure.
bool AbslParseFlag(absl::string_view text, TimeMode* mode, std::string* error);

// AbslUnparseFlag returns a textual flag value corresponding to the TimeMode
// `mode`.
std::string AbslUnparseFlag(TimeMode mode);

// The construction-time configuration options for NetworkEmulationManager.
struct NetworkEmulationManagerConfig {
  // The mode of the underlying time controller.
  TimeMode time_mode = TimeMode::kRealTime;
  // The mode that determines the set of metrics to collect into
  // `EmulatedNetworkStats` and `EmulatedNetworkNodeStats`.
  EmulatedNetworkStatsGatheringMode stats_gathering_mode =
      EmulatedNetworkStatsGatheringMode::kDefault;
  // Field trials that can alter the behavior of NetworkEmulationManager.
  const FieldTrialsView* field_trials = nullptr;
  // If this flag is set, NetworkEmulationManager ignores the sizes of peers'
  // DTLS handshake packets when determining when to let the packets through
  // a constrained emulated network. Actual hanshake's packet size is ignored
  // and a hardcoded fake size is used to compute packet's use of link capacity.
  // This is useful for tests that require deterministic packets scheduling
  // timing-wise even when the sizes of DTLS hadshake packets are not
  // deterministic. This mode make sense only together with the simulated time.
  bool fake_dtls_handshake_sizes = false;
};

// Provides an API for creating and configuring emulated network layer.
// All objects returned by this API are owned by NetworkEmulationManager itself
// and will be deleted when manager will be deleted.
class NetworkEmulationManager {
 public:
  // Helper struct to simplify creation of simulated network behaviors. Contains
  // non-owning pointers as the underlying instances are owned by the manager.
  struct SimulatedNetworkNode {
    SimulatedNetworkInterface* simulation;
    EmulatedNetworkNode* node;

    class Builder {
     public:
      explicit Builder(NetworkEmulationManager* net) : net_(net) {}
      Builder() : net_(nullptr) {}
      Builder(const Builder&) = default;
      // Sets the config state, note that this will replace any previously set
      // values.
      Builder& config(BuiltInNetworkBehaviorConfig config);
      Builder& delay_ms(int queue_delay_ms);
      Builder& capacity(DataRate link_capacity);
      Builder& capacity_kbps(int link_capacity_kbps);
      Builder& capacity_Mbps(int link_capacity_Mbps);
      Builder& loss(double loss_rate);
      Builder& packet_queue_length(int max_queue_length_in_packets);
      Builder& delay_standard_deviation_ms(int delay_standard_deviation_ms);
      Builder& allow_reordering();
      Builder& avg_burst_loss_length(int avg_burst_loss_length);
      Builder& packet_overhead(int packet_overhead);
      SimulatedNetworkNode Build(uint64_t random_seed = 1) const;
      SimulatedNetworkNode Build(NetworkEmulationManager* net,
                                 uint64_t random_seed = 1) const;

     private:
      NetworkEmulationManager* const net_;
      BuiltInNetworkBehaviorConfig config_;
    };
  };
  virtual ~NetworkEmulationManager() = default;

  virtual TimeController* time_controller() = 0;
  // Returns a mode in which underlying time controller operates.
  virtual TimeMode time_mode() const = 0;

  // Creates an emulated network node, which represents ideal network with
  // unlimited capacity, no delay and no packet loss.
  EmulatedNetworkNode* CreateUnconstrainedEmulatedNode() {
    return CreateEmulatedNode(BuiltInNetworkBehaviorConfig());
  }
  // Creates an emulated network node, which represents single network in
  // the emulated network layer. Uses default implementation on network behavior
  // which can be configured with `config`. `random_seed` can be provided to
  // alter randomization behavior.
  virtual EmulatedNetworkNode* CreateEmulatedNode(
      BuiltInNetworkBehaviorConfig config,
      uint64_t random_seed = 1) = 0;
  // Creates an emulated network node, which represents single network in
  // the emulated network layer. `network_behavior` determines how created node
  // will forward incoming packets to the next receiver.
  virtual EmulatedNetworkNode* CreateEmulatedNode(
      std::unique_ptr<NetworkBehaviorInterface> network_behavior) = 0;

  virtual SimulatedNetworkNode::Builder NodeBuilder() = 0;

  // Creates an emulated endpoint, which represents single network interface on
  // the peer's device.
  virtual EmulatedEndpoint* CreateEndpoint(EmulatedEndpointConfig config) = 0;
  // Enable emulated endpoint to make it available for webrtc.
  // Caller mustn't enable currently enabled endpoint.
  virtual void EnableEndpoint(EmulatedEndpoint* endpoint) = 0;
  // Disable emulated endpoint to make it unavailable for webrtc.
  // Caller mustn't disable currently disabled endpoint.
  virtual void DisableEndpoint(EmulatedEndpoint* endpoint) = 0;

  // Creates a route between endpoints going through specified network nodes.
  // This route is single direction only and describe how traffic that was
  // sent by network interface `from` have to be delivered to the network
  // interface `to`. Return object can be used to remove created route. The
  // route must contains at least one network node inside it.
  //
  // Assume that E{0-9} are endpoints and N{0-9} are network nodes, then
  // creation of the route have to follow these rules:
  //   1. A route consists of a source endpoint, an ordered list of one or
  //      more network nodes, and a destination endpoint.
  //   2. If (E1, ..., E2) is a route, then E1 != E2.
  //      In other words, the source and the destination may not be the same.
  //   3. Given two simultaneously existing routes (E1, ..., E2) and
  //      (E3, ..., E4), either E1 != E3 or E2 != E4.
  //      In other words, there may be at most one route from any given source
  //      endpoint to any given destination endpoint.
  //   4. Given two simultaneously existing routes (E1, ..., N1, ..., E2)
  //      and (E3, ..., N2, ..., E4), either N1 != N2 or E2 != E4.
  //      In other words, a network node may not belong to two routes that lead
  //      to the same destination endpoint.
  virtual EmulatedRoute* CreateRoute(
      EmulatedEndpoint* from,
      const std::vector<EmulatedNetworkNode*>& via_nodes,
      EmulatedEndpoint* to) = 0;

  // Creates a route over the given `via_nodes` creating the required endpoints
  // in the process. The returned EmulatedRoute pointer can be used in other
  // calls as a transport route for message or cross traffic.
  virtual EmulatedRoute* CreateRoute(
      const std::vector<EmulatedNetworkNode*>& via_nodes) = 0;

  // Creates a default route between endpoints going through specified network
  // nodes. Default route is used for packet when there is no known route for
  // packet's destination IP.
  //
  // This route is single direction only and describe how traffic that was
  // sent by network interface `from` have to be delivered in case if routing
  // was unspecified. Return object can be used to remove created route. The
  // route must contains at least one network node inside it.
  //
  // Assume that E{0-9} are endpoints and N{0-9} are network nodes, then
  // creation of the route have to follow these rules:
  //   1. A route consists of a source endpoint, an ordered list of one or
  //      more network nodes, and a destination endpoint.
  //   2. If (E1, ..., E2) is a route, then E1 != E2.
  //      In other words, the source and the destination may not be the same.
  //   3. Given two simultaneously existing routes (E1, ..., E2) and
  //      (E3, ..., E4), either E1 != E3 or E2 != E4.
  //      In other words, there may be at most one route from any given source
  //      endpoint to any given destination endpoint.
  //   4. Given two simultaneously existing routes (E1, ..., N1, ..., E2)
  //      and (E3, ..., N2, ..., E4), either N1 != N2 or E2 != E4.
  //      In other words, a network node may not belong to two routes that lead
  //      to the same destination endpoint.
  //   5. Any node N can belong to only one default route.
  virtual EmulatedRoute* CreateDefaultRoute(
      EmulatedEndpoint* from,
      const std::vector<EmulatedNetworkNode*>& via_nodes,
      EmulatedEndpoint* to) = 0;

  // Removes route previously created by CreateRoute(...).
  // Caller mustn't call this function with route, that have been already
  // removed earlier. Removing a route that is currently in use will lead to
  // packets being dropped.
  virtual void ClearRoute(EmulatedRoute* route) = 0;

  // Creates a simulated TCP connection using `send_route` for traffic and
  // `ret_route` for feedback. This can be used to emulate HTTP cross traffic
  // and to implement realistic reliable signaling over lossy networks.
  // TODO(srte): Handle clearing of the routes involved.
  virtual TcpMessageRoute* CreateTcpRoute(EmulatedRoute* send_route,
                                          EmulatedRoute* ret_route) = 0;

  // Creates a route over the given `via_nodes`. Returns an object that can be
  // used to emulate network load with cross traffic over the created route.
  virtual CrossTrafficRoute* CreateCrossTrafficRoute(
      const std::vector<EmulatedNetworkNode*>& via_nodes) = 0;

  // Starts generating cross traffic using given `generator`. Takes ownership
  // over the generator.
  virtual CrossTrafficGenerator* StartCrossTraffic(
      std::unique_ptr<CrossTrafficGenerator> generator) = 0;

  // Stops generating cross traffic that was started using given `generator`.
  // The `generator` shouldn't be used after and the reference may be invalid.
  virtual void StopCrossTraffic(CrossTrafficGenerator* generator) = 0;

  // Creates EmulatedNetworkManagerInterface which can be used then to inject
  // network emulation layer into PeerConnectionFactory. `endpoints` are
  // available network interfaces for PeerConnection. If endpoint is enabled, it
  // will be immediately available for PeerConnection, otherwise user will be
  // able to enable endpoint later to make it available for PeerConnection.
  virtual EmulatedNetworkManagerInterface* absl_nonnull
  CreateEmulatedNetworkManagerInterface(
      const std::vector<EmulatedEndpoint*>& endpoints) = 0;

  // Passes combined network stats for all specified `endpoints` into specified
  // `stats_callback`. Callback will be executed on network emulation
  // internal task queue.
  virtual void GetStats(
      ArrayView<EmulatedEndpoint* const> endpoints,
      std::function<void(EmulatedNetworkStats)> stats_callback) = 0;

  // Passes combined network stats for all specified `nodes` into specified
  // `stats_callback`. Callback will be executed on network emulation
  // internal task queue.
  virtual void GetStats(
      ArrayView<EmulatedNetworkNode* const> nodes,
      std::function<void(EmulatedNetworkNodeStats)> stats_callback) = 0;

  // Create a EmulatedTURNServer.
  // The TURN server has 2 endpoints that need to be connected with routes,
  // - GetClientEndpoint() - the endpoint that accepts TURN allocations.
  // - GetPeerEndpoint() - the endpoint that is "connected to the internet".
  virtual EmulatedTURNServerInterface* CreateTURNServer(
      EmulatedTURNServerConfig config) = 0;

  // Create a pair of EmulatedNetworkManagerInterfaces connected to each other.
  std::pair<EmulatedNetworkManagerInterface*, EmulatedNetworkManagerInterface*>
  CreateEndpointPairWithTwoWayRoutes(
      const BuiltInNetworkBehaviorConfig& config);
};

}  // namespace webrtc

#endif  // API_TEST_NETWORK_EMULATION_MANAGER_H_
