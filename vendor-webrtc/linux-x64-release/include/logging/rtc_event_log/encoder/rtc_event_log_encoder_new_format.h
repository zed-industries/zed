/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_ENCODER_RTC_EVENT_LOG_ENCODER_NEW_FORMAT_H_
#define LOGGING_RTC_EVENT_LOG_ENCODER_RTC_EVENT_LOG_ENCODER_NEW_FORMAT_H_

#include <cstdint>
#include <deque>
#include <map>
#include <memory>
#include <string>
#include <vector>

#include "api/array_view.h"
#include "api/field_trials_view.h"
#include "api/rtc_event_log/rtc_event.h"
#include "logging/rtc_event_log/encoder/rtc_event_log_encoder.h"

namespace webrtc {

namespace rtclog2 {
class EventStream;  // Auto-generated from protobuf.
}  // namespace rtclog2

class RtcEventAlrState;
class RtcEventRouteChange;
class RtcEventRemoteEstimate;
class RtcEventAudioNetworkAdaptation;
class RtcEventAudioPlayout;
class RtcEventAudioReceiveStreamConfig;
class RtcEventAudioSendStreamConfig;
class RtcEventBweUpdateDelayBased;
class RtcEventBweUpdateLossBased;
class RtcEventDtlsTransportState;
class RtcEventDtlsWritableState;
class RtcEventLoggingStarted;
class RtcEventLoggingStopped;
class RtcEventNetEqSetMinimumDelay;
class RtcEventProbeClusterCreated;
class RtcEventProbeResultFailure;
class RtcEventProbeResultSuccess;
class RtcEventRtcpPacketIncoming;
class RtcEventRtcpPacketOutgoing;
class RtcEventRtpPacketIncoming;
class RtcEventRtpPacketOutgoing;
class RtcEventVideoReceiveStreamConfig;
class RtcEventVideoSendStreamConfig;
class RtcEventIceCandidatePairConfig;
class RtcEventIceCandidatePair;
class RtpPacket;
class RtcEventFrameDecoded;
class RtcEventGenericAckReceived;
class RtcEventGenericPacketReceived;
class RtcEventGenericPacketSent;

class RtcEventLogEncoderNewFormat final : public RtcEventLogEncoder {
 public:
  explicit RtcEventLogEncoderNewFormat(const FieldTrialsView& field_trials);
  ~RtcEventLogEncoderNewFormat() override = default;

  std::string EncodeBatch(
      std::deque<std::unique_ptr<RtcEvent>>::const_iterator begin,
      std::deque<std::unique_ptr<RtcEvent>>::const_iterator end) override;

  std::string EncodeLogStart(int64_t timestamp_us,
                             int64_t utc_time_us) override;
  std::string EncodeLogEnd(int64_t timestamp_us) override;

 private:
  // Encoding entry-point for the various RtcEvent subclasses.
  void EncodeAlrState(ArrayView<const RtcEventAlrState*> batch,
                      rtclog2::EventStream* event_stream);
  void EncodeAudioNetworkAdaptation(
      ArrayView<const RtcEventAudioNetworkAdaptation*> batch,
      rtclog2::EventStream* event_stream);
  void EncodeAudioPlayout(ArrayView<const RtcEventAudioPlayout*> batch,
                          rtclog2::EventStream* event_stream);
  void EncodeAudioRecvStreamConfig(
      ArrayView<const RtcEventAudioReceiveStreamConfig*> batch,
      rtclog2::EventStream* event_stream);
  void EncodeAudioSendStreamConfig(
      ArrayView<const RtcEventAudioSendStreamConfig*> batch,
      rtclog2::EventStream* event_stream);
  void EncodeBweUpdateDelayBased(
      ArrayView<const RtcEventBweUpdateDelayBased*> batch,
      rtclog2::EventStream* event_stream);
  void EncodeBweUpdateLossBased(
      ArrayView<const RtcEventBweUpdateLossBased*> batch,
      rtclog2::EventStream* event_stream);
  void EncodeDtlsTransportState(
      ArrayView<const RtcEventDtlsTransportState*> batch,
      rtclog2::EventStream* event_stream);
  void EncodeDtlsWritableState(
      ArrayView<const RtcEventDtlsWritableState*> batch,
      rtclog2::EventStream* event_stream);
  void EncodeFramesDecoded(ArrayView<const RtcEventFrameDecoded* const> batch,
                           rtclog2::EventStream* event_stream);
  void EncodeGenericAcksReceived(
      ArrayView<const RtcEventGenericAckReceived*> batch,
      rtclog2::EventStream* event_stream);
  void EncodeGenericPacketsReceived(
      ArrayView<const RtcEventGenericPacketReceived*> batch,
      rtclog2::EventStream* event_stream);
  void EncodeGenericPacketsSent(
      ArrayView<const RtcEventGenericPacketSent*> batch,
      rtclog2::EventStream* event_stream);
  void EncodeIceCandidatePairConfig(
      ArrayView<const RtcEventIceCandidatePairConfig*> batch,
      rtclog2::EventStream* event_stream);
  void EncodeIceCandidatePairEvent(
      ArrayView<const RtcEventIceCandidatePair*> batch,
      rtclog2::EventStream* event_stream);
  void EncodeLoggingStarted(ArrayView<const RtcEventLoggingStarted*> batch,
                            rtclog2::EventStream* event_stream);
  void EncodeLoggingStopped(ArrayView<const RtcEventLoggingStopped*> batch,
                            rtclog2::EventStream* event_stream);
  void EncodeNetEqSetMinimumDelay(
      ArrayView<const RtcEventNetEqSetMinimumDelay*> batch,
      rtclog2::EventStream* event_stream);
  void EncodeProbeClusterCreated(
      ArrayView<const RtcEventProbeClusterCreated*> batch,
      rtclog2::EventStream* event_stream);
  void EncodeProbeResultFailure(
      ArrayView<const RtcEventProbeResultFailure*> batch,
      rtclog2::EventStream* event_stream);
  void EncodeProbeResultSuccess(
      ArrayView<const RtcEventProbeResultSuccess*> batch,
      rtclog2::EventStream* event_stream);
  void EncodeRouteChange(ArrayView<const RtcEventRouteChange*> batch,
                         rtclog2::EventStream* event_stream);
  void EncodeRemoteEstimate(ArrayView<const RtcEventRemoteEstimate*> batch,
                            rtclog2::EventStream* event_stream);
  void EncodeRtcpPacketIncoming(
      ArrayView<const RtcEventRtcpPacketIncoming*> batch,
      rtclog2::EventStream* event_stream);
  void EncodeRtcpPacketOutgoing(
      ArrayView<const RtcEventRtcpPacketOutgoing*> batch,
      rtclog2::EventStream* event_stream);
  void EncodeRtpPacketIncoming(
      const std::map<uint32_t, std::vector<const RtcEventRtpPacketIncoming*>>&
          batch,
      rtclog2::EventStream* event_stream);
  void EncodeRtpPacketOutgoing(
      const std::map<uint32_t, std::vector<const RtcEventRtpPacketOutgoing*>>&
          batch,
      rtclog2::EventStream* event_stream);
  void EncodeVideoRecvStreamConfig(
      ArrayView<const RtcEventVideoReceiveStreamConfig*> batch,
      rtclog2::EventStream* event_stream);
  void EncodeVideoSendStreamConfig(
      ArrayView<const RtcEventVideoSendStreamConfig*> batch,
      rtclog2::EventStream* event_stream);
  template <typename Batch, typename ProtoType>
  void EncodeRtpPacket(const Batch& batch, ProtoType* proto_batch);

  const bool encode_neteq_set_minimum_delay_kill_switch_;
  const bool encode_dependency_descriptor_;
};

}  // namespace webrtc

#endif  // LOGGING_RTC_EVENT_LOG_ENCODER_RTC_EVENT_LOG_ENCODER_NEW_FORMAT_H_
