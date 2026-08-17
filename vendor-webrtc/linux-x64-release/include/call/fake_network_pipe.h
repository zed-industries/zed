/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_FAKE_NETWORK_PIPE_H_
#define CALL_FAKE_NETWORK_PIPE_H_

#include <cstddef>
#include <cstdint>
#include <deque>
#include <map>
#include <memory>
#include <optional>

#include "api/array_view.h"
#include "api/call/transport.h"
#include "api/test/simulated_network.h"
#include "call/simulated_packet_receiver.h"
#include "modules/rtp_rtcp/source/rtp_packet_received.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class Clock;
class PacketReceiver;
enum class MediaType;

class NetworkPacket {
 public:
  NetworkPacket(CopyOnWriteBuffer packet,
                int64_t send_time,
                int64_t arrival_time,
                std::optional<PacketOptions> packet_options,
                bool is_rtcp,
                MediaType media_type,
                std::optional<int64_t> packet_time_us,
                Transport* transport);

  NetworkPacket(RtpPacketReceived packet,
                MediaType media_type,
                int64_t send_time,
                int64_t arrival_time);

  // Disallow copy constructor and copy assignment (no deep copies of `data_`).
  NetworkPacket(const NetworkPacket&) = delete;
  ~NetworkPacket();
  NetworkPacket& operator=(const NetworkPacket&) = delete;
  // Allow move constructor/assignment, so that we can use in stl containers.
  NetworkPacket(NetworkPacket&&);
  NetworkPacket& operator=(NetworkPacket&&);

  const uint8_t* data() const { return packet_.data(); }
  size_t data_length() const { return packet_.size(); }
  CopyOnWriteBuffer* raw_packet() { return &packet_; }
  int64_t send_time() const { return send_time_; }
  int64_t arrival_time() const { return arrival_time_; }
  void IncrementArrivalTime(int64_t extra_delay) {
    arrival_time_ += extra_delay;
  }
  PacketOptions packet_options() const {
    return packet_options_.value_or(PacketOptions());
  }
  bool is_rtcp() const { return is_rtcp_; }
  MediaType media_type() const { return media_type_; }
  std::optional<int64_t> packet_time_us() const { return packet_time_us_; }
  RtpPacketReceived* packet_received() {
    return packet_received_ ? &packet_received_.value() : nullptr;
  }
  std::optional<RtpPacketReceived> packet_received() const {
    return packet_received_;
  }
  Transport* transport() const { return transport_; }

 private:
  CopyOnWriteBuffer packet_;
  // The time the packet was sent out on the network.
  int64_t send_time_;
  // The time the packet should arrive at the receiver.
  int64_t arrival_time_;
  // If using a Transport for outgoing degradation, populate with
  // PacketOptions (transport-wide sequence number) for RTP.
  std::optional<PacketOptions> packet_options_;
  bool is_rtcp_;
  // If using a PacketReceiver for incoming degradation, populate with
  // appropriate MediaType and packet time. This type/timing will be kept and
  // forwarded. The packet time might be altered to reflect time spent in fake
  // network pipe.
  MediaType media_type_;
  std::optional<int64_t> packet_time_us_;
  std::optional<RtpPacketReceived> packet_received_;
  Transport* transport_;
};

// Class faking a network link, internally is uses an implementation of a
// SimulatedNetworkInterface to simulate network behavior.
class FakeNetworkPipe : public SimulatedPacketReceiverInterface {
 public:
  // Will keep `network_behavior` alive while pipe is alive itself.
  FakeNetworkPipe(Clock* clock,
                  std::unique_ptr<NetworkBehaviorInterface> network_behavior);
  FakeNetworkPipe(Clock* clock,
                  std::unique_ptr<NetworkBehaviorInterface> network_behavior,
                  PacketReceiver* receiver);
  FakeNetworkPipe(Clock* clock,
                  std::unique_ptr<NetworkBehaviorInterface> network_behavior,
                  PacketReceiver* receiver,
                  uint64_t seed);

  ~FakeNetworkPipe() override;

  FakeNetworkPipe(const FakeNetworkPipe&) = delete;
  FakeNetworkPipe& operator=(const FakeNetworkPipe&) = delete;

  void SetClockOffset(int64_t offset_ms);

  // Must not be called in parallel with DeliverPacket or Process.
  void SetReceiver(PacketReceiver* receiver) override;

  // Adds/subtracts references to Transport instances. If a Transport is
  // destroyed we cannot use to forward a potential delayed packet, these
  // methods are used to maintain a map of which instances are live.
  void AddActiveTransport(Transport* transport);
  void RemoveActiveTransport(Transport* transport);

  // Methods for use with Transport interface. When/if packets are delivered,
  // they will be passed to the instance specified by the `transport` parameter.
  // Note that that instance must be in the map of active transports.
  bool SendRtp(ArrayView<const uint8_t> packet,
               const PacketOptions& options,
               Transport* transport);
  bool SendRtcp(ArrayView<const uint8_t> packet, Transport* transport);

  // Implements the PacketReceiver interface. When/if packets are delivered,
  // they will be passed directly to the receiver instance given in
  // SetReceiver(). The receive time will be increased by the amount of time the
  // packet spent in the fake network pipe.
  void DeliverRtpPacket(
      MediaType media_type,
      RtpPacketReceived packet,
      OnUndemuxablePacketHandler undemuxable_packet_handler) override;
  void DeliverRtcpPacket(CopyOnWriteBuffer packet) override;

  // Processes the network queues and trigger PacketReceiver::IncomingPacket for
  // packets ready to be delivered.
  void Process() override;
  std::optional<int64_t> TimeUntilNextProcess() override;

  // Get statistics.
  float PercentageLoss();
  int AverageDelay() override;
  size_t DroppedPackets();
  size_t SentPackets();
  void ResetStats();

 protected:
  void DeliverPacketWithLock(NetworkPacket* packet);
  int64_t GetTimeInMicroseconds() const;
  bool ShouldProcess(int64_t time_now_us) const;
  void SetTimeToNextProcess(int64_t skip_us);

 private:
  struct StoredPacket {
    NetworkPacket packet;
    bool removed = false;
    explicit StoredPacket(NetworkPacket&& packet);
    StoredPacket(StoredPacket&&) = default;
    StoredPacket(const StoredPacket&) = delete;
    StoredPacket& operator=(const StoredPacket&) = delete;
    StoredPacket() = delete;
  };

  // Returns true if enqueued, or false if packet was dropped. Use this method
  // when enqueueing packets that should be received by PacketReceiver instance.
  bool EnqueuePacket(CopyOnWriteBuffer packet,
                     std::optional<PacketOptions> options,
                     bool is_rtcp,
                     MediaType media_type,
                     std::optional<int64_t> packet_time_us);

  // Returns true if enqueued, or false if packet was dropped. Use this method
  // when enqueueing packets that should be received by Transport instance.
  bool EnqueuePacket(CopyOnWriteBuffer packet,
                     std::optional<PacketOptions> options,
                     bool is_rtcp,
                     Transport* transport);

  bool EnqueuePacket(NetworkPacket&& net_packet)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(process_lock_);

  void DeliverNetworkPacket(NetworkPacket* packet)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(config_lock_);
  bool HasReceiver() const;

  Clock* const clock_;
  // `config_lock` guards the mostly constant things like the callbacks.
  mutable Mutex config_lock_;
  const std::unique_ptr<NetworkBehaviorInterface> network_behavior_;
  PacketReceiver* receiver_ RTC_GUARDED_BY(config_lock_);

  // `process_lock` guards the data structures involved in delay and loss
  // processes, such as the packet queues.
  Mutex process_lock_;
  // Packets  are added at the back of the deque, this makes the deque ordered
  // by increasing send time. The common case when removing packets from the
  // deque is removing early packets, which will be close to the front of the
  // deque. This makes finding the packets in the deque efficient in the common
  // case.
  std::deque<StoredPacket> packets_in_flight_ RTC_GUARDED_BY(process_lock_);

  int64_t clock_offset_ms_ RTC_GUARDED_BY(config_lock_);

  // Statistics.
  size_t dropped_packets_ RTC_GUARDED_BY(process_lock_);
  size_t sent_packets_ RTC_GUARDED_BY(process_lock_);
  int64_t total_packet_delay_us_ RTC_GUARDED_BY(process_lock_);
  int64_t last_log_time_us_;

  std::map<Transport*, size_t> active_transports_ RTC_GUARDED_BY(config_lock_);
};

}  // namespace webrtc

#endif  // CALL_FAKE_NETWORK_PIPE_H_
