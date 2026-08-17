/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_RTC_EVENT_LOG_UNITTEST_HELPER_H_
#define LOGGING_RTC_EVENT_LOG_RTC_EVENT_LOG_UNITTEST_HELPER_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <vector>

#include "api/rtc_event_log/rtc_event_log.h"
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
#include "logging/rtc_event_log/events/rtc_event_rtcp_packet_incoming.h"
#include "logging/rtc_event_log/events/rtc_event_rtcp_packet_outgoing.h"
#include "logging/rtc_event_log/events/rtc_event_rtp_packet_incoming.h"
#include "logging/rtc_event_log/events/rtc_event_rtp_packet_outgoing.h"
#include "logging/rtc_event_log/events/rtc_event_video_receive_stream_config.h"
#include "logging/rtc_event_log/events/rtc_event_video_send_stream_config.h"
#include "modules/rtp_rtcp/include/rtp_header_extension_map.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "modules/rtp_rtcp/source/rtcp_packet/bye.h"
#include "modules/rtp_rtcp/source/rtcp_packet/extended_reports.h"
#include "modules/rtp_rtcp/source/rtcp_packet/fir.h"
#include "modules/rtp_rtcp/source/rtcp_packet/loss_notification.h"
#include "modules/rtp_rtcp/source/rtcp_packet/nack.h"
#include "modules/rtp_rtcp/source/rtcp_packet/pli.h"
#include "modules/rtp_rtcp/source/rtcp_packet/receiver_report.h"
#include "modules/rtp_rtcp/source/rtcp_packet/remb.h"
#include "modules/rtp_rtcp/source/rtcp_packet/report_block.h"
#include "modules/rtp_rtcp/source/rtcp_packet/sender_report.h"
#include "modules/rtp_rtcp/source/rtcp_packet/transport_feedback.h"
#include "modules/rtp_rtcp/source/rtp_packet.h"
#include "rtc_base/random.h"

namespace webrtc {

namespace test {

class EventGenerator {
 public:
  explicit EventGenerator(uint64_t seed) : prng_(seed) {}

  std::unique_ptr<RtcEventAlrState> NewAlrState();
  std::unique_ptr<RtcEventAudioNetworkAdaptation> NewAudioNetworkAdaptation();
  std::unique_ptr<RtcEventAudioPlayout> NewAudioPlayout(uint32_t ssrc);
  std::unique_ptr<RtcEventBweUpdateDelayBased> NewBweUpdateDelayBased();
  std::unique_ptr<RtcEventBweUpdateLossBased> NewBweUpdateLossBased();
  std::unique_ptr<RtcEventDtlsTransportState> NewDtlsTransportState();
  std::unique_ptr<RtcEventDtlsWritableState> NewDtlsWritableState();
  std::unique_ptr<RtcEventFrameDecoded> NewFrameDecodedEvent(uint32_t ssrc);
  std::unique_ptr<RtcEventGenericAckReceived> NewGenericAckReceived();
  std::unique_ptr<RtcEventGenericPacketReceived> NewGenericPacketReceived();
  std::unique_ptr<RtcEventGenericPacketSent> NewGenericPacketSent();
  std::unique_ptr<RtcEventIceCandidatePair> NewIceCandidatePair();
  std::unique_ptr<RtcEventIceCandidatePairConfig> NewIceCandidatePairConfig();
  std::unique_ptr<RtcEventNetEqSetMinimumDelay> NewNetEqSetMinimumDelay(
      uint32_t ssrc);
  std::unique_ptr<RtcEventProbeClusterCreated> NewProbeClusterCreated();
  std::unique_ptr<RtcEventProbeResultFailure> NewProbeResultFailure();
  std::unique_ptr<RtcEventProbeResultSuccess> NewProbeResultSuccess();
  std::unique_ptr<RtcEventRouteChange> NewRouteChange();
  std::unique_ptr<RtcEventRemoteEstimate> NewRemoteEstimate();
  std::unique_ptr<RtcEventRtcpPacketIncoming> NewRtcpPacketIncoming();
  std::unique_ptr<RtcEventRtcpPacketOutgoing> NewRtcpPacketOutgoing();

  rtcp::SenderReport NewSenderReport();
  rtcp::ReceiverReport NewReceiverReport();
  rtcp::ExtendedReports NewExtendedReports();
  rtcp::Nack NewNack();
  rtcp::Remb NewRemb();
  rtcp::Fir NewFir();
  rtcp::Pli NewPli();
  rtcp::Bye NewBye();
  rtcp::TransportFeedback NewTransportFeedback();
  rtcp::LossNotification NewLossNotification();

  // `all_configured_exts` determines whether the RTP packet exhibits all
  // configured extensions, or a random subset thereof.
  void RandomizeRtpPacket(size_t payload_size,
                          size_t padding_size,
                          uint32_t ssrc,
                          const RtpHeaderExtensionMap& extension_map,
                          RtpPacket* rtp_packet,
                          bool all_configured_exts);

  // `all_configured_exts` determines whether the RTP packet exhibits all
  // configured extensions, or a random subset thereof.
  std::unique_ptr<RtcEventRtpPacketIncoming> NewRtpPacketIncoming(
      uint32_t ssrc,
      const RtpHeaderExtensionMap& extension_map,
      bool all_configured_exts = true);

  // `all_configured_exts` determines whether the RTP packet exhibits all
  // configured extensions, or a random subset thereof.
  std::unique_ptr<RtcEventRtpPacketOutgoing> NewRtpPacketOutgoing(
      uint32_t ssrc,
      const RtpHeaderExtensionMap& extension_map,
      bool all_configured_exts = true);

  // `configure_all` determines whether all supported extensions are configured,
  // or a random subset. Extensions in `excluded_extensions` will always be
  // excluded.
  RtpHeaderExtensionMap NewRtpHeaderExtensionMap(
      bool configure_all = false,
      const std::vector<RTPExtensionType>& excluded_extensions = {});

  std::unique_ptr<RtcEventAudioReceiveStreamConfig> NewAudioReceiveStreamConfig(
      uint32_t ssrc,
      const RtpHeaderExtensionMap& extensions);

  std::unique_ptr<RtcEventAudioSendStreamConfig> NewAudioSendStreamConfig(
      uint32_t ssrc,
      const RtpHeaderExtensionMap& extensions);

  std::unique_ptr<RtcEventVideoReceiveStreamConfig> NewVideoReceiveStreamConfig(
      uint32_t ssrc,
      const RtpHeaderExtensionMap& extensions);

  std::unique_ptr<RtcEventVideoSendStreamConfig> NewVideoSendStreamConfig(
      uint32_t ssrc,
      const RtpHeaderExtensionMap& extensions);

 private:
  rtcp::ReportBlock NewReportBlock();
  int sent_packet_number_ = 0;
  int received_packet_number_ = 0;

  Random prng_;
};

class EventVerifier {
 public:
  explicit EventVerifier(RtcEventLog::EncodingType encoding_type)
      : encoding_type_(encoding_type) {}

  void ExpectDependencyDescriptorExtensionIsSet(bool value) {
    expect_dependency_descriptor_rtp_header_extension_is_set_ = value;
  }

  void VerifyLoggedAlrStateEvent(const RtcEventAlrState& original_event,
                                 const LoggedAlrStateEvent& logged_event) const;

  void VerifyLoggedAudioPlayoutEvent(
      const RtcEventAudioPlayout& original_event,
      const LoggedAudioPlayoutEvent& logged_event) const;

  void VerifyLoggedAudioNetworkAdaptationEvent(
      const RtcEventAudioNetworkAdaptation& original_event,
      const LoggedAudioNetworkAdaptationEvent& logged_event) const;

  void VerifyLoggedBweDelayBasedUpdate(
      const RtcEventBweUpdateDelayBased& original_event,
      const LoggedBweDelayBasedUpdate& logged_event) const;

  void VerifyLoggedBweLossBasedUpdate(
      const RtcEventBweUpdateLossBased& original_event,
      const LoggedBweLossBasedUpdate& logged_event) const;

  void VerifyLoggedBweProbeClusterCreatedEvent(
      const RtcEventProbeClusterCreated& original_event,
      const LoggedBweProbeClusterCreatedEvent& logged_event) const;

  void VerifyLoggedBweProbeFailureEvent(
      const RtcEventProbeResultFailure& original_event,
      const LoggedBweProbeFailureEvent& logged_event) const;

  void VerifyLoggedBweProbeSuccessEvent(
      const RtcEventProbeResultSuccess& original_event,
      const LoggedBweProbeSuccessEvent& logged_event) const;

  void VerifyLoggedDtlsTransportState(
      const RtcEventDtlsTransportState& original_event,
      const LoggedDtlsTransportState& logged_event) const;

  void VerifyLoggedDtlsWritableState(
      const RtcEventDtlsWritableState& original_event,
      const LoggedDtlsWritableState& logged_event) const;

  void VerifyLoggedFrameDecoded(const RtcEventFrameDecoded& original_event,
                                const LoggedFrameDecoded& logged_event) const;

  void VerifyLoggedIceCandidatePairConfig(
      const RtcEventIceCandidatePairConfig& original_event,
      const LoggedIceCandidatePairConfig& logged_event) const;

  void VerifyLoggedIceCandidatePairEvent(
      const RtcEventIceCandidatePair& original_event,
      const LoggedIceCandidatePairEvent& logged_event) const;

  void VerifyLoggedRouteChangeEvent(
      const RtcEventRouteChange& original_event,
      const LoggedRouteChangeEvent& logged_event) const;

  void VerifyLoggedRemoteEstimateEvent(
      const RtcEventRemoteEstimate& original_event,
      const LoggedRemoteEstimateEvent& logged_event) const;

  void VerifyLoggedRtpPacketIncoming(
      const RtcEventRtpPacketIncoming& original_event,
      const LoggedRtpPacketIncoming& logged_event) const;

  void VerifyLoggedRtpPacketOutgoing(
      const RtcEventRtpPacketOutgoing& original_event,
      const LoggedRtpPacketOutgoing& logged_event) const;

  void VerifyLoggedGenericPacketSent(
      const RtcEventGenericPacketSent& original_event,
      const LoggedGenericPacketSent& logged_event) const;

  void VerifyLoggedGenericPacketReceived(
      const RtcEventGenericPacketReceived& original_event,
      const LoggedGenericPacketReceived& logged_event) const;

  void VerifyLoggedGenericAckReceived(
      const RtcEventGenericAckReceived& original_event,
      const LoggedGenericAckReceived& logged_event) const;

  template <typename EventType, typename ParsedType>
  void VerifyLoggedRtpPacket(const EventType& /* original_event */,
                             const ParsedType& /* logged_event */) {
    static_assert(sizeof(ParsedType) == 0,
                  "You have to use one of the two defined template "
                  "specializations of VerifyLoggedRtpPacket");
  }

  template <>
  void VerifyLoggedRtpPacket(const RtcEventRtpPacketIncoming& original_event,
                             const LoggedRtpPacketIncoming& logged_event) {
    VerifyLoggedRtpPacketIncoming(original_event, logged_event);
  }

  template <>
  void VerifyLoggedRtpPacket(const RtcEventRtpPacketOutgoing& original_event,
                             const LoggedRtpPacketOutgoing& logged_event) {
    VerifyLoggedRtpPacketOutgoing(original_event, logged_event);
  }

  void VerifyLoggedRtcpPacketIncoming(
      const RtcEventRtcpPacketIncoming& original_event,
      const LoggedRtcpPacketIncoming& logged_event) const;

  void VerifyLoggedRtcpPacketOutgoing(
      const RtcEventRtcpPacketOutgoing& original_event,
      const LoggedRtcpPacketOutgoing& logged_event) const;

  void VerifyLoggedSenderReport(int64_t log_time_ms,
                                const rtcp::SenderReport& original_sr,
                                const LoggedRtcpPacketSenderReport& logged_sr);
  void VerifyLoggedReceiverReport(
      int64_t log_time_ms,
      const rtcp::ReceiverReport& original_rr,
      const LoggedRtcpPacketReceiverReport& logged_rr);
  void VerifyLoggedExtendedReports(
      int64_t log_time_ms,
      const rtcp::ExtendedReports& original_xr,
      const LoggedRtcpPacketExtendedReports& logged_xr);
  void VerifyLoggedFir(int64_t log_time_ms,
                       const rtcp::Fir& original_fir,
                       const LoggedRtcpPacketFir& logged_fir);
  void VerifyLoggedPli(int64_t log_time_ms,
                       const rtcp::Pli& original_pli,
                       const LoggedRtcpPacketPli& logged_pli);
  void VerifyLoggedBye(int64_t log_time_ms,
                       const rtcp::Bye& original_bye,
                       const LoggedRtcpPacketBye& logged_bye);
  void VerifyLoggedNack(int64_t log_time_ms,
                        const rtcp::Nack& original_nack,
                        const LoggedRtcpPacketNack& logged_nack);
  void VerifyLoggedTransportFeedback(
      int64_t log_time_ms,
      const rtcp::TransportFeedback& original_transport_feedback,
      const LoggedRtcpPacketTransportFeedback& logged_transport_feedback);
  void VerifyLoggedRemb(int64_t log_time_ms,
                        const rtcp::Remb& original_remb,
                        const LoggedRtcpPacketRemb& logged_remb);
  void VerifyLoggedLossNotification(
      int64_t log_time_ms,
      const rtcp::LossNotification& original_loss_notification,
      const LoggedRtcpPacketLossNotification& logged_loss_notification);

  void VerifyLoggedStartEvent(int64_t start_time_us,
                              int64_t utc_start_time_us,
                              const LoggedStartEvent& logged_event) const;
  void VerifyLoggedStopEvent(int64_t stop_time_us,
                             const LoggedStopEvent& logged_event) const;

  void VerifyLoggedAudioRecvConfig(
      const RtcEventAudioReceiveStreamConfig& original_event,
      const LoggedAudioRecvConfig& logged_event) const;

  void VerifyLoggedAudioSendConfig(
      const RtcEventAudioSendStreamConfig& original_event,
      const LoggedAudioSendConfig& logged_event) const;

  void VerifyLoggedVideoRecvConfig(
      const RtcEventVideoReceiveStreamConfig& original_event,
      const LoggedVideoRecvConfig& logged_event) const;

  void VerifyLoggedVideoSendConfig(
      const RtcEventVideoSendStreamConfig& original_event,
      const LoggedVideoSendConfig& logged_event) const;

  void VerifyLoggedNetEqSetMinimumDelay(
      const RtcEventNetEqSetMinimumDelay& original_event,
      const LoggedNetEqSetMinimumDelayEvent& logged_event) const;

 private:
  void VerifyReportBlock(const rtcp::ReportBlock& original_report_block,
                         const rtcp::ReportBlock& logged_report_block);

  template <typename Event>
  void VerifyLoggedDependencyDescriptor(
      const Event& packet,
      const std::vector<uint8_t>& logged_dd) const;

  RtcEventLog::EncodingType encoding_type_;
  bool expect_dependency_descriptor_rtp_header_extension_is_set_ = true;
};

}  // namespace test
}  // namespace webrtc

#endif  // LOGGING_RTC_EVENT_LOG_RTC_EVENT_LOG_UNITTEST_HELPER_H_
