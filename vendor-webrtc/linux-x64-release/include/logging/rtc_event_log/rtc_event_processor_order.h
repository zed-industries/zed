/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_RTC_EVENT_PROCESSOR_ORDER_H_
#define LOGGING_RTC_EVENT_LOG_RTC_EVENT_PROCESSOR_ORDER_H_

#include <stdint.h>

#include <optional>

#include "logging/rtc_event_log/events/logged_rtp_rtcp.h"
#include "logging/rtc_event_log/events/rtc_event_alr_state.h"
#include "logging/rtc_event_log/events/rtc_event_audio_network_adaptation.h"
#include "logging/rtc_event_log/events/rtc_event_audio_playout.h"
#include "logging/rtc_event_log/events/rtc_event_audio_receive_stream_config.h"
#include "logging/rtc_event_log/events/rtc_event_audio_send_stream_config.h"
#include "logging/rtc_event_log/events/rtc_event_begin_log.h"
#include "logging/rtc_event_log/events/rtc_event_bwe_update_delay_based.h"
#include "logging/rtc_event_log/events/rtc_event_bwe_update_loss_based.h"
#include "logging/rtc_event_log/events/rtc_event_dtls_transport_state.h"
#include "logging/rtc_event_log/events/rtc_event_dtls_writable_state.h"
#include "logging/rtc_event_log/events/rtc_event_end_log.h"
#include "logging/rtc_event_log/events/rtc_event_frame_decoded.h"
#include "logging/rtc_event_log/events/rtc_event_generic_ack_received.h"
#include "logging/rtc_event_log/events/rtc_event_generic_packet_received.h"
#include "logging/rtc_event_log/events/rtc_event_generic_packet_sent.h"
#include "logging/rtc_event_log/events/rtc_event_ice_candidate_pair.h"
#include "logging/rtc_event_log/events/rtc_event_ice_candidate_pair_config.h"
#include "logging/rtc_event_log/events/rtc_event_neteq_set_minimum_delay.h"
#include "logging/rtc_event_log/events/rtc_event_probe_cluster_created.h"
#include "logging/rtc_event_log/events/rtc_event_probe_result_failure.h"
#include "logging/rtc_event_log/events/rtc_event_probe_result_success.h"
#include "logging/rtc_event_log/events/rtc_event_remote_estimate.h"
#include "logging/rtc_event_log/events/rtc_event_route_change.h"
#include "logging/rtc_event_log/events/rtc_event_video_receive_stream_config.h"
#include "logging/rtc_event_log/events/rtc_event_video_send_stream_config.h"
#include "logging/rtc_event_log/rtc_event_log_parser.h"

namespace webrtc {

// The RTC event log only uses millisecond precision timestamps
// and doesn't preserve order between events in different batches.
// This is a heuristic to order events in a way that preserves
// "typical" dependencies, e.g. we receive packets before we
// send feedback about them, and RTP packets sent or received
// during the same millisecond are in sequence number order.

enum class TypeOrder {
  Start,
  // Connectivity and stream configurations before incoming packets
  StreamConfig,
  IceCondidateConfig,
  IceCandidateEvent,
  DtlsTransportState,
  DtlsWritable,
  RouteChange,
  // Incoming packets
  RtpIn,
  RtcpIn,
  GenericPacketIn,
  GenericAckIn,
  // BWE depends on incoming feedback (send side estimation)
  // or incoming media packets (receive side estimation).
  // Delay-based BWE depends on probe results.
  // Loss-based BWE depends on delay-based BWE.
  // Loss-based BWE may trigger new probes.
  BweRemoteEstimate,
  BweProbeFailure,
  BweProbeSuccess,
  BweDelayBased,
  BweLossBased,
  BweProbeCreated,
  // General processing events. No obvious order.
  AudioNetworkAdaptation,
  NetEqSetMinDelay,
  AudioPlayout,
  FrameDecoded,
  // Outgoing packets and feedback depends on BWE and might depend on
  // processing.
  RtpOut,
  RtcpOut,
  GenericPacketOut,
  // Alr is updated after a packet is sent.
  AlrState,
  Stop,
};

template <typename T>
class TieBreaker {
  static_assert(sizeof(T) != sizeof(T),
                "Specialize TieBreaker to define an order for the event type.");
};

template <>
class TieBreaker<LoggedStartEvent> {
 public:
  static constexpr int type_order = static_cast<int>(TypeOrder::Start);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedStartEvent&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedStopEvent> {
 public:
  static constexpr int type_order = static_cast<int>(TypeOrder::Stop);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedStopEvent&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedAudioRecvConfig> {
 public:
  static constexpr int type_order = static_cast<int>(TypeOrder::StreamConfig);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedAudioRecvConfig&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedAudioSendConfig> {
 public:
  static constexpr int type_order = static_cast<int>(TypeOrder::StreamConfig);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedAudioSendConfig&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedVideoRecvConfig> {
 public:
  static constexpr int type_order = static_cast<int>(TypeOrder::StreamConfig);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedVideoRecvConfig&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedVideoSendConfig> {
 public:
  static constexpr int type_order = static_cast<int>(TypeOrder::StreamConfig);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedVideoSendConfig&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedIceCandidatePairConfig> {
 public:
  static constexpr int type_order =
      static_cast<int>(TypeOrder::IceCondidateConfig);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedIceCandidatePairConfig&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedIceCandidatePairEvent> {
 public:
  static constexpr int type_order =
      static_cast<int>(TypeOrder::IceCandidateEvent);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedIceCandidatePairEvent&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedDtlsTransportState> {
 public:
  static constexpr int type_order =
      static_cast<int>(TypeOrder::DtlsTransportState);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedDtlsTransportState&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedDtlsWritableState> {
 public:
  static constexpr int type_order = static_cast<int>(TypeOrder::DtlsWritable);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedDtlsWritableState&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedRouteChangeEvent> {
 public:
  static constexpr int type_order = static_cast<int>(TypeOrder::RouteChange);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedRouteChangeEvent&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedRemoteEstimateEvent> {
 public:
  static constexpr int type_order =
      static_cast<int>(TypeOrder::BweRemoteEstimate);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedRemoteEstimateEvent&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedBweProbeFailureEvent> {
 public:
  static constexpr int type_order =
      static_cast<int>(TypeOrder::BweProbeFailure);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedBweProbeFailureEvent&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedBweProbeSuccessEvent> {
 public:
  static constexpr int type_order =
      static_cast<int>(TypeOrder::BweProbeSuccess);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedBweProbeSuccessEvent&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedBweDelayBasedUpdate> {
 public:
  static constexpr int type_order = static_cast<int>(TypeOrder::BweDelayBased);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedBweDelayBasedUpdate&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedBweLossBasedUpdate> {
 public:
  static constexpr int type_order = static_cast<int>(TypeOrder::BweLossBased);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedBweLossBasedUpdate&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedBweProbeClusterCreatedEvent> {
 public:
  static constexpr int type_order =
      static_cast<int>(TypeOrder::BweProbeCreated);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedBweProbeClusterCreatedEvent&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedAudioNetworkAdaptationEvent> {
 public:
  static constexpr int type_order =
      static_cast<int>(TypeOrder::AudioNetworkAdaptation);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedAudioNetworkAdaptationEvent&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedNetEqSetMinimumDelayEvent> {
 public:
  static constexpr int type_order =
      static_cast<int>(TypeOrder::NetEqSetMinDelay);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedNetEqSetMinimumDelayEvent&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedAudioPlayoutEvent> {
 public:
  static constexpr int type_order = static_cast<int>(TypeOrder::AudioPlayout);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedAudioPlayoutEvent&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedFrameDecoded> {
 public:
  static constexpr int type_order = static_cast<int>(TypeOrder::FrameDecoded);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedFrameDecoded&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedGenericPacketReceived> {
 public:
  static constexpr int type_order =
      static_cast<int>(TypeOrder::GenericPacketIn);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedGenericPacketReceived&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedGenericAckReceived> {
 public:
  static constexpr int type_order = static_cast<int>(TypeOrder::GenericAckIn);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedGenericAckReceived&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedGenericPacketSent> {
 public:
  static constexpr int type_order =
      static_cast<int>(TypeOrder::GenericPacketOut);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedGenericPacketSent&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedRtpPacket> {
 public:
  static constexpr int type_order(PacketDirection direction) {
    return static_cast<int>(direction == PacketDirection::kIncomingPacket
                                ? TypeOrder::RtpIn
                                : TypeOrder::RtpOut);
  }
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedRtpPacket& p) {
    return p.header.extension.hasTransportSequenceNumber
               ? p.header.extension.transportSequenceNumber
               : std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedPacketInfo> {
 public:
  static constexpr int type_order(PacketDirection direction) {
    return static_cast<int>(direction == PacketDirection::kIncomingPacket
                                ? TypeOrder::RtpIn
                                : TypeOrder::RtpOut);
  }
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedPacketInfo& p) {
    return p.has_transport_seq_no ? p.transport_seq_no
                                  : std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedRtpPacketIncoming> {
 public:
  static constexpr int type_order = static_cast<int>(TypeOrder::RtpIn);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedRtpPacketIncoming& p) {
    return p.rtp.header.extension.hasTransportSequenceNumber
               ? p.rtp.header.extension.transportSequenceNumber
               : std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedRtpPacketOutgoing> {
 public:
  static constexpr int type_order = static_cast<int>(TypeOrder::RtpOut);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedRtpPacketOutgoing& p) {
    return p.rtp.header.extension.hasTransportSequenceNumber
               ? p.rtp.header.extension.transportSequenceNumber
               : std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedRtcpPacketIncoming> {
 public:
  static constexpr int type_order = static_cast<int>(TypeOrder::RtcpIn);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedRtcpPacketIncoming&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedRtcpPacketOutgoing> {
 public:
  static constexpr int type_order = static_cast<int>(TypeOrder::RtcpOut);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedRtcpPacketOutgoing&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedRtcpPacketTransportFeedback> {
 public:
  static constexpr int type_order(PacketDirection direction) {
    return static_cast<int>(direction == PacketDirection::kIncomingPacket
                                ? TypeOrder::RtcpIn
                                : TypeOrder::RtcpOut);
  }
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedRtcpPacketTransportFeedback&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedRtcpPacketSenderReport> {
 public:
  static constexpr int type_order(PacketDirection direction) {
    return static_cast<int>(direction == PacketDirection::kIncomingPacket
                                ? TypeOrder::RtcpIn
                                : TypeOrder::RtcpOut);
  }
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedRtcpPacketSenderReport&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedRtcpPacketReceiverReport> {
 public:
  static constexpr int type_order(PacketDirection direction) {
    return static_cast<int>(direction == PacketDirection::kIncomingPacket
                                ? TypeOrder::RtcpIn
                                : TypeOrder::RtcpOut);
  }
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedRtcpPacketReceiverReport&) {
    return std::optional<uint16_t>();
  }
};

template <>
class TieBreaker<LoggedAlrStateEvent> {
 public:
  static constexpr int type_order = static_cast<int>(TypeOrder::AlrState);
  static std::optional<uint16_t> transport_seq_num_accessor(
      const LoggedAlrStateEvent&) {
    return std::optional<uint16_t>();
  }
};

}  // namespace webrtc

#endif  // LOGGING_RTC_EVENT_LOG_RTC_EVENT_PROCESSOR_ORDER_H_
