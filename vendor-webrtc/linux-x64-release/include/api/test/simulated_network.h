/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_SIMULATED_NETWORK_H_
#define API_TEST_SIMULATED_NETWORK_H_

#include <stddef.h>
#include <stdint.h>

#include <functional>
#include <optional>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "api/transport/ecn_marking.h"
#include "api/units/data_rate.h"

namespace webrtc {

struct PacketInFlightInfo {
  PacketInFlightInfo(size_t size,
                     int64_t send_time_us,
                     uint64_t packet_id,
                     webrtc::EcnMarking ecn)
      : size(size),
        send_time_us(send_time_us),
        packet_id(packet_id),
        ecn(ecn) {}

  PacketInFlightInfo(size_t size, int64_t send_time_us, uint64_t packet_id)
      : PacketInFlightInfo(size,
                           send_time_us,
                           packet_id,
                           webrtc::EcnMarking::kNotEct) {}

  size_t size;
  int64_t send_time_us;
  // Unique identifier for the packet in relation to other packets in flight.
  uint64_t packet_id;
  webrtc::EcnMarking ecn;
};

struct PacketDeliveryInfo {
  static constexpr int kNotReceived = -1;
  PacketDeliveryInfo(PacketInFlightInfo source, int64_t receive_time_us)
      : receive_time_us(receive_time_us),
        packet_id(source.packet_id),
        ecn(source.ecn) {}

  bool operator==(const PacketDeliveryInfo& other) const {
    return receive_time_us == other.receive_time_us &&
           packet_id == other.packet_id;
  }

  int64_t receive_time_us;
  uint64_t packet_id;
  webrtc::EcnMarking ecn;
};

// BuiltInNetworkBehaviorConfig is a built-in network behavior configuration
// for built-in network behavior that will be used by WebRTC if no custom
// NetworkBehaviorInterface is provided.
struct BuiltInNetworkBehaviorConfig {
  //  Queue length in number of packets.
  size_t queue_length_packets = 0;
  // Delay in addition to capacity induced delay.
  int queue_delay_ms = 0;
  // Standard deviation of the extra delay.
  int delay_standard_deviation_ms = 0;
  // Link capacity.
  DataRate link_capacity = DataRate::Infinity();
  // Random packet loss, range 0 to 100.
  double loss_percent = 0.;
  // If packets are allowed to be reordered.
  bool allow_reordering = false;
  // The average length of a burst of lost packets.
  int avg_burst_loss_length = -1;
  // Additional bytes to add to packet size.
  int packet_overhead = 0;
};

// Interface that represents a Network behaviour.
//
// It is clients of this interface responsibility to enqueue and dequeue
// packets (based on the estimated delivery time expressed by
// NextDeliveryTimeUs).
//
// To enqueue packets, call EnqueuePacket:
// EXPECT_TRUE(network.EnqueuePacket(
//     PacketInFlightInfo(/*size=*/1, /*send_time_us=*/0, /*packet_id=*/1)));
//
// To know when to call DequeueDeliverablePackets to pull packets out of the
// network, call NextDeliveryTimeUs and schedule a task to invoke
// DequeueDeliverablePackets (if not already scheduled).
//
// DequeueDeliverablePackets will return a vector of delivered packets, but this
// vector can be empty in case of extra delay. In such case, make sure to invoke
// NextDeliveryTimeUs and schedule a task to call DequeueDeliverablePackets for
// the next estimated delivery of packets.
//
// std::vector<PacketDeliveryInfo> delivered_packets =
//     network.DequeueDeliverablePackets(/*receive_time_us=*/1000000);
class NetworkBehaviorInterface {
 public:
  // Enqueues a packet in the network and returns true if the action was
  // successful, false otherwise (for example, because the network capacity has
  // been saturated). If the return value is false, the packet should be
  // considered as dropped and it will not be returned by future calls
  // to DequeueDeliverablePackets.
  // Packets enqueued will exit the network when DequeueDeliverablePackets is
  // called and enough time has passed (see NextDeliveryTimeUs).
  virtual bool EnqueuePacket(PacketInFlightInfo packet_info) = 0;
  // Retrieves all packets that should be delivered by the given receive time.
  // Not all the packets in the returned std::vector are actually delivered.
  // In order to know the state of each packet it is necessary to check the
  // `receive_time_us` field of each packet. If that is set to
  // PacketDeliveryInfo::kNotReceived then the packet is considered lost in the
  // network.
  virtual std::vector<PacketDeliveryInfo> DequeueDeliverablePackets(
      int64_t receive_time_us) = 0;
  // Returns time in microseconds when caller should call
  // DequeueDeliverablePackets to get the next set of delivered packets. It is
  // possible that no packet will be delivered by that time (e.g. in case of
  // random extra delay), in such case this method should be called again to get
  // the updated estimated delivery time.
  virtual std::optional<int64_t> NextDeliveryTimeUs() const = 0;
  // Registers a callback that should be triggered by an implementation if the
  // next NextDeliveryTimeUs() has changed between a call to NextDeliveryTimeUs
  // and DequeueDeliverablePackets.
  // The intended usage is to invoke NextDeliveryTimeUs and reschedule the
  // DequeueDeliverablePackets call when network parameters (such as link
  // capacity) changes.
  virtual void RegisterDeliveryTimeChangedCallback(
      absl::AnyInvocable<void()> /* callback */) {}
  virtual ~NetworkBehaviorInterface() = default;
};

// Class simulating a network link. This is a simple and naive solution just
// faking capacity and adding an extra transport delay in addition to the
// capacity introduced delay.
class SimulatedNetworkInterface : public NetworkBehaviorInterface {
 public:
  // Sets a new configuration.
  virtual void SetConfig(const BuiltInNetworkBehaviorConfig& config) = 0;
  virtual void UpdateConfig(
      std::function<void(BuiltInNetworkBehaviorConfig*)> config_modifier) = 0;
  // Pauses the network until `until_us`. This affects both delivery (calling
  // DequeueDeliverablePackets before `until_us` results in an empty std::vector
  // of packets) and capacity (the network is paused, so packets are not
  // flowing and they will restart flowing at `until_us`).
  virtual void PauseTransmissionUntil(int64_t until_us) = 0;
};

}  // namespace webrtc

#endif  // API_TEST_SIMULATED_NETWORK_H_
