/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_NETWORK_NETWORK_EMULATION_H_
#define TEST_NETWORK_NETWORK_EMULATION_H_

#include <cstdint>
#include <cstring>
#include <deque>
#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/base/nullability.h"
#include "api/numerics/samples_stats_counter.h"
#include "api/sequence_checker.h"
#include "api/task_queue/task_queue_base.h"
#include "api/test/network_emulation/network_emulation_interfaces.h"
#include "api/test/network_emulation_manager.h"
#include "api/test/simulated_network.h"
#include "api/transport/ecn_marking.h"
#include "api/units/data_size.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "rtc_base/checks.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "rtc_base/ip_address.h"
#include "rtc_base/network.h"
#include "rtc_base/network_constants.h"
#include "rtc_base/socket_address.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/task_utils/repeating_task.h"
#include "rtc_base/thread_annotations.h"
#include "system_wrappers/include/clock.h"

namespace webrtc {

// All methods of EmulatedNetworkOutgoingStatsBuilder have to be used on a
// single thread. It may be created on another thread.
class EmulatedNetworkOutgoingStatsBuilder {
 public:
  explicit EmulatedNetworkOutgoingStatsBuilder(
      EmulatedNetworkStatsGatheringMode stats_gathering_mode);

  void OnPacketSent(Timestamp sent_time, const EmulatedIpPacket& packet);

  void AddOutgoingStats(const EmulatedNetworkOutgoingStats& stats);

  EmulatedNetworkOutgoingStats Build() const;

 private:
  const EmulatedNetworkStatsGatheringMode stats_gathering_mode_;

  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequence_checker_;
  EmulatedNetworkOutgoingStats stats_ RTC_GUARDED_BY(sequence_checker_);
};

// All methods of EmulatedNetworkIncomingStatsBuilder have to be used on a
// single thread. It may be created on another thread.
class EmulatedNetworkIncomingStatsBuilder {
 public:
  explicit EmulatedNetworkIncomingStatsBuilder(
      EmulatedNetworkStatsGatheringMode stats_gathering_mode);

  void OnPacketDropped(DataSize packet_size);

  void OnPacketReceived(Timestamp received_time,
                        const EmulatedIpPacket& packet);

  // Adds stats collected from another endpoints to the builder.
  void AddIncomingStats(const EmulatedNetworkIncomingStats& stats);

  EmulatedNetworkIncomingStats Build() const;

 private:
  const EmulatedNetworkStatsGatheringMode stats_gathering_mode_;

  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequence_checker_;
  EmulatedNetworkIncomingStats stats_ RTC_GUARDED_BY(sequence_checker_);
};

// All methods of EmulatedNetworkStatsBuilder have to be used on a single
// thread. It may be created on another thread.
class EmulatedNetworkStatsBuilder {
 public:
  explicit EmulatedNetworkStatsBuilder(
      EmulatedNetworkStatsGatheringMode stats_gathering_mode);
  explicit EmulatedNetworkStatsBuilder(
      IPAddress local_ip,
      EmulatedNetworkStatsGatheringMode stats_gathering_mode);

  void OnPacketSent(Timestamp send_time, const EmulatedIpPacket& packet);

  void OnPacketDropped(IPAddress source_ip, DataSize packet_size);

  void OnPacketReceived(Timestamp received_time,
                        const EmulatedIpPacket& packet);

  void AddEmulatedNetworkStats(const EmulatedNetworkStats& stats);

  EmulatedNetworkStats Build() const;

 private:
  const EmulatedNetworkStatsGatheringMode stats_gathering_mode_;

  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequence_checker_;
  std::vector<IPAddress> local_addresses_ RTC_GUARDED_BY(sequence_checker_);
  SamplesStatsCounter sent_packets_queue_wait_time_us_;
  std::map<IPAddress, std::unique_ptr<EmulatedNetworkOutgoingStatsBuilder>>
      outgoing_stats_per_destination_ RTC_GUARDED_BY(sequence_checker_);
  std::map<IPAddress, std::unique_ptr<EmulatedNetworkIncomingStatsBuilder>>
      incoming_stats_per_source_ RTC_GUARDED_BY(sequence_checker_);
};

// All methods of EmulatedNetworkNodeStatsBuilder have to be used on a
// single thread. It may be created on another thread.
class EmulatedNetworkNodeStatsBuilder {
 public:
  explicit EmulatedNetworkNodeStatsBuilder(
      EmulatedNetworkStatsGatheringMode stats_gathering_mode);

  void AddPacketTransportTime(TimeDelta time, size_t packet_size);

  void AddEmulatedNetworkNodeStats(const EmulatedNetworkNodeStats& stats);

  EmulatedNetworkNodeStats Build() const;

 private:
  const EmulatedNetworkStatsGatheringMode stats_gathering_mode_;

  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequence_checker_;
  EmulatedNetworkNodeStats stats_ RTC_GUARDED_BY(sequence_checker_);
};

class LinkEmulation : public EmulatedNetworkReceiverInterface {
 public:
  LinkEmulation(Clock* clock,
                TaskQueueBase* absl_nonnull task_queue,
                std::unique_ptr<NetworkBehaviorInterface> network_behavior,
                EmulatedNetworkReceiverInterface* receiver,
                EmulatedNetworkStatsGatheringMode stats_gathering_mode,
                bool fake_dtls_handshake_sizes);
  void OnPacketReceived(EmulatedIpPacket packet) override;

  EmulatedNetworkNodeStats stats() const;

 private:
  struct StoredPacket {
    uint64_t id;
    Timestamp sent_time;
    EmulatedIpPacket packet;
    bool removed;
  };
  void UpdateProcessSchedule() RTC_RUN_ON(task_queue_);
  void Process(Timestamp at_time) RTC_RUN_ON(task_queue_);
  size_t GetPacketSizeForEmulation(const EmulatedIpPacket& packet) const;

  Clock* const clock_;
  TaskQueueBase* absl_nonnull const task_queue_;
  const std::unique_ptr<NetworkBehaviorInterface> network_behavior_
      RTC_GUARDED_BY(task_queue_);
  EmulatedNetworkReceiverInterface* const receiver_;
  const bool fake_dtls_handshake_sizes_;

  RepeatingTaskHandle process_task_ RTC_GUARDED_BY(task_queue_);
  std::deque<StoredPacket> packets_ RTC_GUARDED_BY(task_queue_);
  uint64_t next_packet_id_ RTC_GUARDED_BY(task_queue_) = 1;

  EmulatedNetworkNodeStatsBuilder stats_builder_ RTC_GUARDED_BY(task_queue_);
};

// Represents a component responsible for routing packets based on their IP
// address. All possible routes have to be set explicitly before packet for
// desired destination will be seen for the first time. If route is unknown
// the packet will be silently dropped.
class NetworkRouterNode : public EmulatedNetworkReceiverInterface {
 public:
  explicit NetworkRouterNode(TaskQueueBase* absl_nonnull task_queue);

  void OnPacketReceived(EmulatedIpPacket packet) override;
  void SetReceiver(const IPAddress& dest_ip,
                   EmulatedNetworkReceiverInterface* receiver);
  void RemoveReceiver(const IPAddress& dest_ip);
  // Sets a default receive that will be used for all incoming packets for which
  // there is no specific receiver binded to their destination port.
  void SetDefaultReceiver(EmulatedNetworkReceiverInterface* receiver);
  void RemoveDefaultReceiver();
  void SetWatcher(std::function<void(const EmulatedIpPacket&)> watcher);
  void SetFilter(std::function<bool(const EmulatedIpPacket&)> filter);

 private:
  TaskQueueBase* absl_nonnull const task_queue_;
  std::optional<EmulatedNetworkReceiverInterface*> default_receiver_
      RTC_GUARDED_BY(task_queue_);
  std::map<IPAddress, EmulatedNetworkReceiverInterface*> routing_
      RTC_GUARDED_BY(task_queue_);
  std::function<void(const EmulatedIpPacket&)> watcher_
      RTC_GUARDED_BY(task_queue_);
  std::function<bool(const EmulatedIpPacket&)> filter_
      RTC_GUARDED_BY(task_queue_);
};

// Represents node in the emulated network. Nodes can be connected with each
// other to form different networks with different behavior. The behavior of
// the node itself is determined by a concrete implementation of
// NetworkBehaviorInterface that is provided on construction.
class EmulatedNetworkNode : public EmulatedNetworkReceiverInterface {
 public:
  // Creates node based on `network_behavior`. The specified `packet_overhead`
  // is added to the size of each packet in the information provided to
  // `network_behavior`.
  // `task_queue` is used to process packets and to forward the packets when
  // they are ready.
  EmulatedNetworkNode(
      Clock* clock,
      TaskQueueBase* absl_nonnull task_queue,
      std::unique_ptr<NetworkBehaviorInterface> network_behavior,
      EmulatedNetworkStatsGatheringMode stats_gathering_mode,
      bool fake_dtls_handshake_sizes);
  ~EmulatedNetworkNode() override;

  EmulatedNetworkNode(const EmulatedNetworkNode&) = delete;
  EmulatedNetworkNode& operator=(const EmulatedNetworkNode&) = delete;

  void OnPacketReceived(EmulatedIpPacket packet) override;

  LinkEmulation* link() { return &link_; }
  NetworkRouterNode* router() { return &router_; }
  EmulatedNetworkNodeStats stats() const;

  // Creates a route for the given receiver_ip over all the given nodes to the
  // given receiver.
  static void CreateRoute(const IPAddress& receiver_ip,
                          std::vector<EmulatedNetworkNode*> nodes,
                          EmulatedNetworkReceiverInterface* receiver);
  static void ClearRoute(const IPAddress& receiver_ip,
                         std::vector<EmulatedNetworkNode*> nodes);

 private:
  NetworkRouterNode router_;
  LinkEmulation link_;
};

// Represents single network interface on the device.
// It will be used as sender from socket side to send data to the network and
// will act as packet receiver from emulated network side to receive packets
// from other EmulatedNetworkNodes.
class EmulatedEndpointImpl : public EmulatedEndpoint {
 public:
  struct Options {
    Options(uint64_t id,
            const IPAddress& ip,
            const EmulatedEndpointConfig& config,
            EmulatedNetworkStatsGatheringMode stats_gathering_mode);

    // TODO(titovartem) check if we can remove id.
    uint64_t id;
    // Endpoint local IP address.
    IPAddress ip;
    EmulatedNetworkStatsGatheringMode stats_gathering_mode;
    AdapterType type;
    // Allow endpoint to send packets specifying source IP address different to
    // the current endpoint IP address. If false endpoint will crash if attempt
    // to send such packet will be done.
    bool allow_send_packet_with_different_source_ip;
    // Allow endpoint to receive packet with destination IP address different to
    // the current endpoint IP address. If false endpoint will crash if such
    // packet will arrive.
    bool allow_receive_packets_with_different_dest_ip;
    // Name of the endpoint used for logging purposes.
    std::string log_name;
  };

  EmulatedEndpointImpl(const Options& options,
                       bool is_enabled,
                       TaskQueueBase* absl_nonnull task_queue,
                       Clock* clock);
  ~EmulatedEndpointImpl() override;

  uint64_t GetId() const;

  NetworkRouterNode* router() { return &router_; }

  void SendPacket(const SocketAddress& from,
                  const SocketAddress& to,
                  CopyOnWriteBuffer packet_data,
                  uint16_t application_overhead = 0,
                  EcnMarking ecn = EcnMarking::kNotEct) override;

  std::optional<uint16_t> BindReceiver(
      uint16_t desired_port,
      EmulatedNetworkReceiverInterface* receiver) override;
  // Binds a receiver, and automatically removes the binding after first call to
  // OnPacketReceived.
  std::optional<uint16_t> BindOneShotReceiver(
      uint16_t desired_port,
      EmulatedNetworkReceiverInterface* receiver);
  void UnbindReceiver(uint16_t port) override;
  void BindDefaultReceiver(EmulatedNetworkReceiverInterface* receiver) override;
  void UnbindDefaultReceiver() override;

  IPAddress GetPeerLocalAddress() const override;

  // Will be called to deliver packet into endpoint from network node.
  void OnPacketReceived(EmulatedIpPacket packet) override;

  void Enable();
  void Disable();
  bool Enabled() const;

  const Network& network() const { return *network_.get(); }

  EmulatedNetworkStats stats() const;

 private:
  struct ReceiverBinding {
    EmulatedNetworkReceiverInterface* receiver;
    bool is_one_shot;
  };

  std::optional<uint16_t> BindReceiverInternal(
      uint16_t desired_port,
      EmulatedNetworkReceiverInterface* receiver,
      bool is_one_shot);

  static constexpr uint16_t kFirstEphemeralPort = 49152;
  uint16_t NextPort() RTC_EXCLUSIVE_LOCKS_REQUIRED(receiver_lock_);

  Mutex receiver_lock_;
  mutable Mutex enable_state_mutex_;

  const Options options_;
  bool is_enabled_ RTC_GUARDED_BY(enable_state_mutex_);
  Clock* const clock_;
  TaskQueueBase* absl_nonnull const task_queue_;
  std::unique_ptr<Network> network_;
  NetworkRouterNode router_;

  uint16_t next_port_ RTC_GUARDED_BY(receiver_lock_);
  std::optional<EmulatedNetworkReceiverInterface*> default_receiver_
      RTC_GUARDED_BY(receiver_lock_);
  std::map<uint16_t, ReceiverBinding> port_to_receiver_
      RTC_GUARDED_BY(receiver_lock_);

  EmulatedNetworkStatsBuilder stats_builder_ RTC_GUARDED_BY(task_queue_);
};

class EmulatedRoute {
 public:
  EmulatedRoute(EmulatedEndpointImpl* from,
                std::vector<EmulatedNetworkNode*> via_nodes,
                EmulatedEndpointImpl* to,
                bool is_default)
      : from(from),
        via_nodes(std::move(via_nodes)),
        to(to),
        active(true),
        is_default(is_default) {}

  EmulatedEndpointImpl* from;
  std::vector<EmulatedNetworkNode*> via_nodes;
  EmulatedEndpointImpl* to;
  bool active;
  bool is_default;
};

// This object is immutable and so thread safe.
class EndpointsContainer {
 public:
  EndpointsContainer(const std::vector<EmulatedEndpointImpl*>& endpoints,
                     EmulatedNetworkStatsGatheringMode stats_gathering_mode);

  EmulatedEndpointImpl* LookupByLocalAddress(const IPAddress& local_ip) const;
  bool HasEndpoint(EmulatedEndpointImpl* endpoint) const;
  // Returns list of networks for enabled endpoints. Caller takes ownership of
  // returned webrtc::Network objects.
  std::vector<std::unique_ptr<Network>> GetEnabledNetworks() const;
  std::vector<EmulatedEndpoint*> GetEndpoints() const;
  EmulatedNetworkStats GetStats() const;

 private:
  const std::vector<EmulatedEndpointImpl*> endpoints_;
  const EmulatedNetworkStatsGatheringMode stats_gathering_mode_;
};

template <typename FakePacketType>
class FakePacketRoute : public EmulatedNetworkReceiverInterface {
 public:
  FakePacketRoute(EmulatedRoute* route,
                  std::function<void(FakePacketType, Timestamp)> action)
      : route_(route),
        action_(std::move(action)),
        send_addr_(route_->from->GetPeerLocalAddress(), 0),
        recv_addr_(route_->to->GetPeerLocalAddress(),
                   *route_->to->BindReceiver(0, this)) {}

  ~FakePacketRoute() { route_->to->UnbindReceiver(recv_addr_.port()); }

  void SendPacket(size_t size, FakePacketType packet) {
    RTC_CHECK_GE(size, sizeof(int));
    sent_.emplace(next_packet_id_, packet);
    CopyOnWriteBuffer buf(size);
    memset(buf.MutableData(), 0, size);
    reinterpret_cast<int*>(buf.MutableData())[0] = next_packet_id_++;
    route_->from->SendPacket(send_addr_, recv_addr_, buf);
  }

  void OnPacketReceived(EmulatedIpPacket packet) override {
    int packet_id = reinterpret_cast<const int*>(packet.data.data())[0];
    action_(std::move(sent_[packet_id]), packet.arrival_time);
    sent_.erase(packet_id);
  }

 private:
  EmulatedRoute* const route_;
  const std::function<void(FakePacketType, Timestamp)> action_;
  const SocketAddress send_addr_;
  const SocketAddress recv_addr_;
  int next_packet_id_ = 0;
  std::map<int, FakePacketType> sent_;
};

template <typename RequestPacketType, typename ResponsePacketType>
class TwoWayFakeTrafficRoute {
 public:
  class TrafficHandlerInterface {
   public:
    virtual void OnRequest(RequestPacketType, Timestamp) = 0;
    virtual void OnResponse(ResponsePacketType, Timestamp) = 0;
    virtual ~TrafficHandlerInterface() = default;
  };
  TwoWayFakeTrafficRoute(TrafficHandlerInterface* handler,
                         EmulatedRoute* send_route,
                         EmulatedRoute* ret_route)
      : handler_(handler),
        request_handler_{send_route,
                         [&](RequestPacketType packet, Timestamp arrival_time) {
                           handler_->OnRequest(std::move(packet), arrival_time);
                         }},
        response_handler_{
            ret_route, [&](ResponsePacketType packet, Timestamp arrival_time) {
              handler_->OnResponse(std::move(packet), arrival_time);
            }} {}
  void SendRequest(size_t size, RequestPacketType packet) {
    request_handler_.SendPacket(size, std::move(packet));
  }
  void SendResponse(size_t size, ResponsePacketType packet) {
    response_handler_.SendPacket(size, std::move(packet));
  }

 private:
  TrafficHandlerInterface* handler_;
  FakePacketRoute<RequestPacketType> request_handler_;
  FakePacketRoute<ResponsePacketType> response_handler_;
};
}  // namespace webrtc

#endif  // TEST_NETWORK_NETWORK_EMULATION_H_
