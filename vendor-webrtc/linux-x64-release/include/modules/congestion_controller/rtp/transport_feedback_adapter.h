/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_CONGESTION_CONTROLLER_RTP_TRANSPORT_FEEDBACK_ADAPTER_H_
#define MODULES_CONGESTION_CONTROLLER_RTP_TRANSPORT_FEEDBACK_ADAPTER_H_

#include <cstddef>
#include <cstdint>
#include <map>
#include <optional>
#include <tuple>
#include <vector>

#include "api/transport/network_types.h"
#include "api/units/data_size.h"
#include "api/units/timestamp.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "modules/rtp_rtcp/source/rtcp_packet/congestion_control_feedback.h"
#include "modules/rtp_rtcp/source/rtp_packet_to_send.h"
#include "rtc_base/network/sent_packet.h"
#include "rtc_base/network_route.h"
#include "rtc_base/numerics/sequence_number_unwrapper.h"

namespace webrtc {

struct PacketFeedback {
  PacketFeedback() = default;
  // Time corresponding to when this object was created.
  Timestamp creation_time = Timestamp::MinusInfinity();
  SentPacket sent;
  // Time corresponding to when the packet was received. Timestamped with the
  // receiver's clock. For unreceived packet, Timestamp::PlusInfinity() is
  // used.
  Timestamp receive_time = Timestamp::PlusInfinity();

  // The network route that this packet is associated with.
  NetworkRoute network_route;

  uint32_t ssrc = 0;
  uint16_t rtp_sequence_number = 0;
};

class InFlightBytesTracker {
 public:
  void AddInFlightPacketBytes(const PacketFeedback& packet);
  void RemoveInFlightPacketBytes(const PacketFeedback& packet);
  DataSize GetOutstandingData(const NetworkRoute& network_route) const;

 private:
  struct NetworkRouteComparator {
    bool operator()(const NetworkRoute& a, const NetworkRoute& b) const;
  };
  std::map<NetworkRoute, DataSize, NetworkRouteComparator> in_flight_data_;
};

// TransportFeedbackAdapter converts RTCP feedback packets to RTCP agnostic per
// packet send/receive information.
// It supports rtcp::CongestionControlFeedback according to RFC 8888 and
// rtcp::TransportFeedback according to
// https://datatracker.ietf.org/doc/html/draft-holmer-rmcat-transport-wide-cc-extensions-01
class TransportFeedbackAdapter {
 public:
  TransportFeedbackAdapter();

  void AddPacket(const RtpPacketToSend& packet,
                 const PacedPacketInfo& pacing_info,
                 size_t overhead_bytes,
                 Timestamp creation_time);

  std::optional<SentPacket> ProcessSentPacket(
      const SentPacketInfo& sent_packet);

  std::optional<TransportPacketsFeedback> ProcessTransportFeedback(
      const rtcp::TransportFeedback& feedback,
      Timestamp feedback_receive_time);

  std::optional<TransportPacketsFeedback> ProcessCongestionControlFeedback(
      const rtcp::CongestionControlFeedback& feedback,
      Timestamp feedback_receive_time);

  void SetNetworkRoute(const NetworkRoute& network_route);

  DataSize GetOutstandingData() const;

 private:
  enum class SendTimeHistoryStatus { kNotAdded, kOk, kDuplicate };

  struct SsrcAndRtpSequencenumber {
    uint32_t ssrc;
    uint16_t rtp_sequence_number;

    bool operator<(const SsrcAndRtpSequencenumber& other) const {
      return std::tie(ssrc, rtp_sequence_number) <
             std::tie(other.ssrc, other.rtp_sequence_number);
    }
  };

  std::optional<PacketFeedback> RetrievePacketFeedback(
      int64_t transport_seq_num,
      bool received);
  std::optional<PacketFeedback> RetrievePacketFeedback(
      const SsrcAndRtpSequencenumber& key,
      bool received);
  std::optional<TransportPacketsFeedback> ToTransportFeedback(
      std::vector<PacketResult> packet_results,
      Timestamp feedback_receive_time,
      bool supports_ecn);

  DataSize pending_untracked_size_ = DataSize::Zero();
  Timestamp last_send_time_ = Timestamp::MinusInfinity();
  Timestamp last_untracked_send_time_ = Timestamp::MinusInfinity();
  RtpSequenceNumberUnwrapper seq_num_unwrapper_;

  // Sequence numbers are never negative, using -1 as it always < a real
  // sequence number.
  int64_t last_ack_seq_num_ = -1;
  InFlightBytesTracker in_flight_;
  NetworkRoute network_route_;

  Timestamp current_offset_ = Timestamp::MinusInfinity();

  // `last_transport_feedback_base_time` is only used for transport feedback to
  // track base time.
  Timestamp last_transport_feedback_base_time_ = Timestamp::MinusInfinity();
  // Used by RFC 8888 congestion control feedback to track base time.
  std::optional<uint32_t> last_feedback_compact_ntp_time_;

  // Map SSRC and RTP sequence number to transport sequence number.
  std::map<SsrcAndRtpSequencenumber, int64_t /*transport_sequence_number*/>
      rtp_to_transport_sequence_number_;
  std::map<int64_t, PacketFeedback> history_;
};

}  // namespace webrtc

#endif  // MODULES_CONGESTION_CONTROLLER_RTP_TRANSPORT_FEEDBACK_ADAPTER_H_
