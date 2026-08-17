/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_RTP_PACKET_OUTGOING_H_
#define LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_RTP_PACKET_OUTGOING_H_

#include <cstddef>
#include <cstdint>
#include <map>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/rtc_event_log/rtc_event.h"
#include "logging/rtc_event_log/events/logged_rtp_rtcp.h"
#include "logging/rtc_event_log/events/rtc_event_log_parse_status.h"
#include "modules/rtp_rtcp/source/rtp_packet.h"

namespace webrtc {

class RtpPacketToSend;

class RtcEventRtpPacketOutgoing final : public RtcEvent {
 public:
  static constexpr Type kType = Type::RtpPacketOutgoing;

  RtcEventRtpPacketOutgoing(const RtpPacketToSend& packet,
                            int probe_cluster_id);
  ~RtcEventRtpPacketOutgoing() override;

  Type GetType() const override { return kType; }
  bool IsConfigEvent() const override { return false; }

  std::unique_ptr<RtcEventRtpPacketOutgoing> Copy() const;

  size_t packet_length() const { return packet_.size(); }

  ArrayView<const uint8_t> RawHeader() const {
    return MakeArrayView(packet_.data(), header_length());
  }
  uint32_t Ssrc() const { return packet_.Ssrc(); }
  uint32_t Timestamp() const { return packet_.Timestamp(); }
  uint16_t SequenceNumber() const { return packet_.SequenceNumber(); }
  uint8_t PayloadType() const { return packet_.PayloadType(); }
  bool Marker() const { return packet_.Marker(); }
  template <typename ExtensionTrait, typename... Args>
  bool GetExtension(Args&&... args) const {
    return packet_.GetExtension<ExtensionTrait>(std::forward<Args>(args)...);
  }
  template <typename ExtensionTrait>
  ArrayView<const uint8_t> GetRawExtension() const {
    return packet_.GetRawExtension<ExtensionTrait>();
  }
  template <typename ExtensionTrait>
  bool HasExtension() const {
    return packet_.HasExtension<ExtensionTrait>();
  }

  size_t payload_length() const { return packet_.payload_size(); }
  size_t header_length() const { return packet_.headers_size(); }
  size_t padding_length() const { return packet_.padding_size(); }
  int probe_cluster_id() const { return probe_cluster_id_; }

  static std::string Encode(ArrayView<const RtcEvent*> /* batch */) {
    // TODO(terelius): Implement
    return "";
  }

  static RtcEventLogParseStatus Parse(
      absl::string_view /* encoded_bytes */,
      bool /* batched */,
      std::map<uint32_t, std::vector<LoggedRtpPacketOutgoing>>& /* output */) {
    // TODO(terelius): Implement
    return RtcEventLogParseStatus::Error("Not Implemented", __FILE__, __LINE__);
  }

 private:
  RtcEventRtpPacketOutgoing(const RtcEventRtpPacketOutgoing& other);

  const RtpPacket packet_;
  // TODO(eladalon): Delete `probe_cluster_id_` along with legacy encoding.
  const int probe_cluster_id_;
};

}  // namespace webrtc

#endif  // LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_RTP_PACKET_OUTGOING_H_
