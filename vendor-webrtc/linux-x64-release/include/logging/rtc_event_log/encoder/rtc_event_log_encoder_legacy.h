/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_ENCODER_RTC_EVENT_LOG_ENCODER_LEGACY_H_
#define LOGGING_RTC_EVENT_LOG_ENCODER_RTC_EVENT_LOG_ENCODER_LEGACY_H_

#include <cstddef>
#include <cstdint>
#include <deque>
#include <memory>
#include <string>

#include "api/array_view.h"
#include "api/rtc_event_log/rtc_event.h"
#include "logging/rtc_event_log/encoder/rtc_event_log_encoder.h"
#include "rtc_base/buffer.h"

namespace webrtc {

namespace rtclog {
class Event;  // Auto-generated from protobuf.
}  // namespace rtclog

class RtcEventAlrState;
class RtcEventAudioNetworkAdaptation;
class RtcEventAudioPlayout;
class RtcEventAudioReceiveStreamConfig;
class RtcEventAudioSendStreamConfig;
class RtcEventBweUpdateDelayBased;
class RtcEventBweUpdateLossBased;
class RtcEventIceCandidatePairConfig;
class RtcEventIceCandidatePair;
class RtcEventLoggingStarted;
class RtcEventLoggingStopped;
class RtcEventProbeClusterCreated;
class RtcEventProbeResultFailure;
class RtcEventProbeResultSuccess;
class RtcEventRemoteEstimate;
class RtcEventRtcpPacketIncoming;
class RtcEventRtcpPacketOutgoing;
class RtcEventRtpPacketIncoming;
class RtcEventRtpPacketOutgoing;
class RtcEventVideoReceiveStreamConfig;
class RtcEventVideoSendStreamConfig;
class RtpPacket;

class RtcEventLogEncoderLegacy final : public RtcEventLogEncoder {
 public:
  ~RtcEventLogEncoderLegacy() override = default;

  std::string EncodeLogStart(int64_t timestamp_us,
                             int64_t utc_time_us) override;
  std::string EncodeLogEnd(int64_t timestamp_us) override;

  std::string EncodeBatch(
      std::deque<std::unique_ptr<RtcEvent>>::const_iterator begin,
      std::deque<std::unique_ptr<RtcEvent>>::const_iterator end) override;

 private:
  std::string Encode(const RtcEvent& event);
  // Encoding entry-point for the various RtcEvent subclasses.
  std::string EncodeAlrState(const RtcEventAlrState& event);
  std::string EncodeAudioNetworkAdaptation(
      const RtcEventAudioNetworkAdaptation& event);
  std::string EncodeAudioPlayout(const RtcEventAudioPlayout& event);
  std::string EncodeAudioReceiveStreamConfig(
      const RtcEventAudioReceiveStreamConfig& event);
  std::string EncodeAudioSendStreamConfig(
      const RtcEventAudioSendStreamConfig& event);
  std::string EncodeBweUpdateDelayBased(
      const RtcEventBweUpdateDelayBased& event);
  std::string EncodeBweUpdateLossBased(const RtcEventBweUpdateLossBased& event);
  std::string EncodeIceCandidatePairConfig(
      const RtcEventIceCandidatePairConfig& event);
  std::string EncodeIceCandidatePairEvent(
      const RtcEventIceCandidatePair& event);
  std::string EncodeProbeClusterCreated(
      const RtcEventProbeClusterCreated& event);
  std::string EncodeProbeResultFailure(const RtcEventProbeResultFailure& event);
  std::string EncodeProbeResultSuccess(const RtcEventProbeResultSuccess&);
  std::string EncodeRemoteEstimate(const RtcEventRemoteEstimate& event);
  std::string EncodeRtcpPacketIncoming(const RtcEventRtcpPacketIncoming& event);
  std::string EncodeRtcpPacketOutgoing(const RtcEventRtcpPacketOutgoing& event);
  std::string EncodeRtpPacketIncoming(const RtcEventRtpPacketIncoming& event);
  std::string EncodeRtpPacketOutgoing(const RtcEventRtpPacketOutgoing& event);
  std::string EncodeVideoReceiveStreamConfig(
      const RtcEventVideoReceiveStreamConfig& event);
  std::string EncodeVideoSendStreamConfig(
      const RtcEventVideoSendStreamConfig& event);

  // RTCP/RTP are handled similarly for incoming/outgoing.
  std::string EncodeRtcpPacket(int64_t timestamp_us,
                               const Buffer& packet,
                               bool is_incoming);
  std::string EncodeRtpPacket(int64_t timestamp_us,
                              ArrayView<const uint8_t> header,
                              size_t packet_length,
                              int probe_cluster_id,
                              bool is_incoming);

  std::string Serialize(rtclog::Event* event);
};

}  // namespace webrtc

#endif  // LOGGING_RTC_EVENT_LOG_ENCODER_RTC_EVENT_LOG_ENCODER_LEGACY_H_
