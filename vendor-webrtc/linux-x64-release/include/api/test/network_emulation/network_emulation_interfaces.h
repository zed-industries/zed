/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef API_TEST_NETWORK_EMULATION_NETWORK_EMULATION_INTERFACES_H_
#define API_TEST_NETWORK_EMULATION_NETWORK_EMULATION_INTERFACES_H_

#include <cstddef>
#include <cstdint>
#include <map>
#include <optional>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "api/numerics/samples_stats_counter.h"
#include "api/test/network_emulation/ecn_marking_counter.h"
#include "api/transport/ecn_marking.h"
#include "api/units/data_rate.h"
#include "api/units/data_size.h"
#include "api/units/timestamp.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "rtc_base/ip_address.h"
#include "rtc_base/socket_address.h"

namespace webrtc {

struct EmulatedIpPacket {
 public:
  EmulatedIpPacket(const SocketAddress& from,
                   const SocketAddress& to,
                   CopyOnWriteBuffer data,
                   Timestamp arrival_time,
                   uint16_t application_overhead = 0,
                   EcnMarking ecn = EcnMarking::kNotEct);
  ~EmulatedIpPacket() = default;
  // This object is not copyable or assignable.
  EmulatedIpPacket(const EmulatedIpPacket&) = delete;
  EmulatedIpPacket& operator=(const EmulatedIpPacket&) = delete;
  // This object is only moveable.
  EmulatedIpPacket(EmulatedIpPacket&&) = default;
  EmulatedIpPacket& operator=(EmulatedIpPacket&&) = default;

  size_t size() const { return data.size(); }
  const uint8_t* cdata() const { return data.cdata(); }

  size_t ip_packet_size() const { return size() + headers_size; }
  SocketAddress from;
  SocketAddress to;
  // Holds the UDP payload.
  CopyOnWriteBuffer data;
  uint16_t headers_size;
  Timestamp arrival_time;
  EcnMarking ecn;
};

// Interface for handling IP packets from an emulated network. This is used with
// EmulatedEndpoint to receive packets on a specific port.
class EmulatedNetworkReceiverInterface {
 public:
  virtual ~EmulatedNetworkReceiverInterface() = default;

  virtual void OnPacketReceived(EmulatedIpPacket packet) = 0;
};

struct EmulatedNetworkOutgoingStats {
  int64_t packets_sent = 0;

  DataSize bytes_sent = DataSize::Zero();

  // Sizes of all sent packets.
  // Collected iff EmulatedNetworkStatsGatheringMode::kDebug is enabled.
  SamplesStatsCounter sent_packets_size;

  DataSize first_sent_packet_size = DataSize::Zero();

  // Time of the first packet sent or infinite value if no packets were sent.
  Timestamp first_packet_sent_time = Timestamp::PlusInfinity();

  // Time of the last packet sent or infinite value if no packets were sent.
  Timestamp last_packet_sent_time = Timestamp::MinusInfinity();

  EcnMarkingCounter ecn_count;

  // Returns average send rate. Requires that at least 2 packets were sent.
  DataRate AverageSendRate() const;
};

struct EmulatedNetworkIncomingStats {
  // Total amount of packets received with or without destination.
  int64_t packets_received = 0;

  // Total amount of bytes in received packets.
  DataSize bytes_received = DataSize::Zero();

  // Sizes of all received packets.
  // Collected iff EmulatedNetworkStatsGatheringMode::kDebug is enabled.
  SamplesStatsCounter received_packets_size;

  // Total amount of packets that were received, but no destination was found.
  int64_t packets_discarded_no_receiver = 0;

  // Total amount of bytes in discarded packets.
  DataSize bytes_discarded_no_receiver = DataSize::Zero();

  // Sizes of all packets that were received, but no destination was found.
  // Collected iff EmulatedNetworkStatsGatheringMode::kDebug is enabled.
  SamplesStatsCounter packets_discarded_no_receiver_size;

  DataSize first_received_packet_size = DataSize::Zero();

  // Time of the first packet received or infinite value if no packets were
  // received.
  Timestamp first_packet_received_time = Timestamp::PlusInfinity();

  // Time of the last packet received or infinite value if no packets were
  // received.
  Timestamp last_packet_received_time = Timestamp::MinusInfinity();

  EcnMarkingCounter ecn_count;

  DataRate AverageReceiveRate() const;
};

struct EmulatedNetworkStats {
  int64_t PacketsSent() const { return overall_outgoing_stats.packets_sent; }

  DataSize BytesSent() const { return overall_outgoing_stats.bytes_sent; }

  // Returns the timestamped sizes of all sent packets.
  // Returned reference is valid until the next call to a non-const method.
  // Collected iff EmulatedNetworkStatsGatheringMode::kDebug is enabled.
  const SamplesStatsCounter& SentPacketsSizeCounter() const {
    return overall_outgoing_stats.sent_packets_size;
  }

  DataSize FirstSentPacketSize() const {
    return overall_outgoing_stats.first_sent_packet_size;
  }

  // Returns time of the first packet sent or infinite value if no packets were
  // sent.
  Timestamp FirstPacketSentTime() const {
    return overall_outgoing_stats.first_packet_sent_time;
  }

  // Returns time of the last packet sent or infinite value if no packets were
  // sent.
  Timestamp LastPacketSentTime() const {
    return overall_outgoing_stats.last_packet_sent_time;
  }

  DataRate AverageSendRate() const {
    return overall_outgoing_stats.AverageSendRate();
  }

  // Total amount of packets received regardless of the destination address.
  int64_t PacketsReceived() const {
    return overall_incoming_stats.packets_received;
  }

  // Total amount of bytes in received packets.
  DataSize BytesReceived() const {
    return overall_incoming_stats.bytes_received;
  }

  // Returns the timestamped sizes of all received packets.
  // Returned reference is valid until the next call to a non-const method.
  // Collected iff EmulatedNetworkStatsGatheringMode::kDebug is enabled.
  const SamplesStatsCounter& ReceivedPacketsSizeCounter() const {
    return overall_incoming_stats.received_packets_size;
  }

  // Total amount of packets that were received, but no destination was found.
  int64_t PacketsDiscardedNoReceiver() const {
    return overall_incoming_stats.packets_discarded_no_receiver;
  }

  // Total amount of bytes in dropped packets.
  DataSize BytesDiscardedNoReceiver() const {
    return overall_incoming_stats.bytes_discarded_no_receiver;
  }

  // Returns counter with timestamped sizes of all packets that were received,
  // but no destination was found.
  // Returned reference is valid until the next call to a non-const method.
  // Collected iff EmulatedNetworkStatsGatheringMode::kDebug is enabled.
  const SamplesStatsCounter& PacketsDiscardedNoReceiverSizeCounter() const {
    return overall_incoming_stats.packets_discarded_no_receiver_size;
  }

  DataSize FirstReceivedPacketSize() const {
    return overall_incoming_stats.first_received_packet_size;
  }

  // Returns time of the first packet received or infinite value if no packets
  // were received.
  Timestamp FirstPacketReceivedTime() const {
    return overall_incoming_stats.first_packet_received_time;
  }

  // Returns time of the last packet received or infinite value if no packets
  // were received.
  Timestamp LastPacketReceivedTime() const {
    return overall_incoming_stats.last_packet_received_time;
  }

  DataRate AverageReceiveRate() const {
    return overall_incoming_stats.AverageReceiveRate();
  }

  // List of IP addresses that were used to send data considered in this stats
  // object.
  std::vector<IPAddress> local_addresses;

  // Overall outgoing stats for all IP addresses which were requested.
  EmulatedNetworkOutgoingStats overall_outgoing_stats;

  // Overall incoming stats for all IP addresses from which data was received
  // on requested interfaces.
  EmulatedNetworkIncomingStats overall_incoming_stats;

  std::map<IPAddress, EmulatedNetworkOutgoingStats>
      outgoing_stats_per_destination;
  std::map<IPAddress, EmulatedNetworkIncomingStats> incoming_stats_per_source;

  // Duration between packet was received on network interface and was
  // dispatched to the network in microseconds.
  // Collected iff EmulatedNetworkStatsGatheringMode::kDebug is enabled.
  SamplesStatsCounter sent_packets_queue_wait_time_us;
};

struct EmulatedNetworkNodeStats {
  // Amount of time each packet spent in the emulated network node for which
  // stats were collected.
  //
  // Collected iff EmulatedNetworkStatsGatheringMode::kDebug is enabled.
  SamplesStatsCounter packet_transport_time;

  // For each packet contains its size divided on the amount of time which it
  // spent in the emulated network node for which stats were collected.
  //
  // Collected iff EmulatedNetworkStatsGatheringMode::kDebug is enabled.
  SamplesStatsCounter size_to_packet_transport_time;
};

// EmulatedEndpoint is an abstraction for network interface on device. Instances
// of this are created by NetworkEmulationManager::CreateEndpoint and
// thread safe.
class EmulatedEndpoint : public EmulatedNetworkReceiverInterface {
 public:
  // Send packet into network.
  // `from` will be used to set source address for the packet in destination
  // socket.
  // `to` will be used for routing verification and picking right socket by port
  // on destination endpoint.
  virtual void SendPacket(const SocketAddress& from,
                          const SocketAddress& to,
                          CopyOnWriteBuffer packet_data,
                          uint16_t application_overhead = 0,
                          EcnMarking ecn = EcnMarking::kNotEct) = 0;

  // Binds receiver to this endpoint to send and receive data.
  // `desired_port` is a port that should be used. If it is equal to 0,
  // endpoint will pick the first available port starting from
  // `kFirstEphemeralPort`.
  //
  // Returns the port, that should be used (it will be equals to desired, if
  // `desired_port` != 0 and is free or will be the one, selected by endpoint)
  // or std::nullopt if desired_port in used. Also fails if there are no more
  // free ports to bind to.
  //
  // The Bind- and Unbind-methods must not be called from within a bound
  // receiver's OnPacketReceived method.
  virtual std::optional<uint16_t> BindReceiver(
      uint16_t desired_port,
      EmulatedNetworkReceiverInterface* receiver) = 0;
  // Unbinds receiver from the specified port. Do nothing if no receiver was
  // bound before. After this method returns, no more packets can be delivered
  // to the receiver, and it is safe to destroy it.
  virtual void UnbindReceiver(uint16_t port) = 0;
  // Binds receiver that will accept all packets which arrived on any port
  // for which there are no bound receiver.
  virtual void BindDefaultReceiver(
      EmulatedNetworkReceiverInterface* receiver) = 0;
  // Unbinds default receiver. Do nothing if no default receiver was bound
  // before.
  virtual void UnbindDefaultReceiver() = 0;
  virtual IPAddress GetPeerLocalAddress() const = 0;

 private:
  // Ensure that there can be no other subclass than EmulatedEndpointImpl. This
  // means that it's always safe to downcast EmulatedEndpoint instances to
  // EmulatedEndpointImpl.
  friend class EmulatedEndpointImpl;
  EmulatedEndpoint() = default;
};

// Simulates a TCP connection, this roughly implements the Reno algorithm. In
// difference from TCP this only support sending messages with a fixed length,
// no streaming. This is useful to simulate signaling and cross traffic using
// message based protocols such as HTTP. It differs from UDP messages in that
// they are guranteed to be delivered eventually, even on lossy networks.
class TcpMessageRoute {
 public:
  // Sends a TCP message of the given `size` over the route, `on_received` is
  // called when the message has been delivered. Note that the connection
  // parameters are reset iff there's no currently pending message on the route.
  virtual void SendMessage(size_t size,
                           absl::AnyInvocable<void()> on_received) = 0;

 protected:
  ~TcpMessageRoute() = default;
};
}  // namespace webrtc

#endif  // API_TEST_NETWORK_EMULATION_NETWORK_EMULATION_INTERFACES_H_
