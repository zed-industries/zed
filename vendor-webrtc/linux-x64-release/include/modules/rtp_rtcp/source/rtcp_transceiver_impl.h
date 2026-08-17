/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_RTCP_TRANSCEIVER_IMPL_H_
#define MODULES_RTP_RTCP_SOURCE_RTCP_TRANSCEIVER_IMPL_H_

#include <cstddef>
#include <cstdint>
#include <functional>
#include <list>
#include <memory>
#include <optional>
#include <vector>

#include "api/array_view.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "modules/rtp_rtcp/include/report_block_data.h"
#include "modules/rtp_rtcp/source/rtcp_packet.h"
#include "modules/rtp_rtcp/source/rtcp_packet/common_header.h"
#include "modules/rtp_rtcp/source/rtcp_packet/dlrr.h"
#include "modules/rtp_rtcp/source/rtcp_packet/remb.h"
#include "modules/rtp_rtcp/source/rtcp_packet/report_block.h"
#include "modules/rtp_rtcp/source/rtcp_packet/target_bitrate.h"
#include "modules/rtp_rtcp/source/rtcp_transceiver_config.h"
#include "rtc_base/containers/flat_map.h"
#include "rtc_base/task_utils/repeating_task.h"

namespace webrtc {
//
// Manage incoming and outgoing rtcp messages for multiple BUNDLED streams.
//
// This class is not thread-safe.
class RtcpTransceiverImpl {
 public:
  explicit RtcpTransceiverImpl(const RtcpTransceiverConfig& config);
  RtcpTransceiverImpl(const RtcpTransceiverImpl&) = delete;
  RtcpTransceiverImpl& operator=(const RtcpTransceiverImpl&) = delete;
  ~RtcpTransceiverImpl();

  void StopPeriodicTask() { periodic_task_handle_.Stop(); }

  void AddMediaReceiverRtcpObserver(uint32_t remote_ssrc,
                                    MediaReceiverRtcpObserver* observer);
  void RemoveMediaReceiverRtcpObserver(uint32_t remote_ssrc,
                                       MediaReceiverRtcpObserver* observer);

  // Returns false on failure, e.g. when there is already an handler for the
  // `local_ssrc`.
  bool AddMediaSender(uint32_t local_ssrc, RtpStreamRtcpHandler* handler);
  bool RemoveMediaSender(uint32_t local_ssrc);

  void SetReadyToSend(bool ready);

  void ReceivePacket(ArrayView<const uint8_t> packet, Timestamp now);

  void SendCompoundPacket();

  void SetRemb(int64_t bitrate_bps, std::vector<uint32_t> ssrcs);
  void UnsetRemb();

  void SendNack(uint32_t ssrc, std::vector<uint16_t> sequence_numbers);

  void SendPictureLossIndication(uint32_t ssrc);
  // If new_request is true then requested sequence no. will increase for each
  // requested ssrc.
  void SendFullIntraRequest(ArrayView<const uint32_t> ssrcs, bool new_request);

  // SendCombinedRtcpPacket ignores rtcp mode and does not send a compound
  // message. https://tools.ietf.org/html/rfc4585#section-3.1
  void SendCombinedRtcpPacket(
      std::vector<std::unique_ptr<rtcp::RtcpPacket>> rtcp_packets);

 private:
  class PacketSender;
  struct RemoteSenderState;
  struct LocalSenderState;
  struct RrtrTimes {
    // Received remote NTP timestamp in compact representation.
    uint32_t received_remote_mid_ntp_time;

    // Local NTP time when the report was received in compact representation.
    uint32_t local_receive_mid_ntp_time;
  };

  void HandleReceivedPacket(const rtcp::CommonHeader& rtcp_packet_header,
                            Timestamp now,
                            std::vector<ReportBlockData>& report_blocks);
  // Individual rtcp packet handlers.
  void HandleBye(const rtcp::CommonHeader& rtcp_packet_header);
  void HandleSenderReport(const rtcp::CommonHeader& rtcp_packet_header,
                          Timestamp now,
                          std::vector<ReportBlockData>& report_blocks);
  void HandleReceiverReport(const rtcp::CommonHeader& rtcp_packet_header,
                            Timestamp now,
                            std::vector<ReportBlockData>& report_blocks);
  void HandleReportBlocks(uint32_t sender_ssrc,
                          Timestamp now,
                          ArrayView<const rtcp::ReportBlock> rtcp_report_blocks,
                          std::vector<ReportBlockData>& report_blocks);
  void HandlePayloadSpecificFeedback(
      const rtcp::CommonHeader& rtcp_packet_header,
      Timestamp now);
  void HandleRtpFeedback(const rtcp::CommonHeader& rtcp_packet_header,
                         Timestamp now);
  void HandleFir(const rtcp::CommonHeader& rtcp_packet_header);
  void HandlePli(const rtcp::CommonHeader& rtcp_packet_header);
  void HandleRemb(const rtcp::CommonHeader& rtcp_packet_header, Timestamp now);
  void HandleNack(const rtcp::CommonHeader& rtcp_packet_header);
  void HandleTransportFeedback(const rtcp::CommonHeader& rtcp_packet_header,
                               Timestamp now);
  void HandleCongestionControlFeedback(
      const rtcp::CommonHeader& rtcp_packet_header,
      Timestamp now);
  void HandleExtendedReports(const rtcp::CommonHeader& rtcp_packet_header,
                             Timestamp now);
  // Extended Reports blocks handlers.
  void HandleDlrr(const rtcp::Dlrr& dlrr, Timestamp now);
  void HandleTargetBitrate(const rtcp::TargetBitrate& target_bitrate,
                           uint32_t remote_ssrc);
  void ProcessReportBlocks(Timestamp now,
                           ArrayView<const ReportBlockData> report_blocks);

  void ReschedulePeriodicCompoundPackets();
  void SchedulePeriodicCompoundPackets(TimeDelta delay);
  // Appends RTCP sender and receiver reports to the `sender`.
  // Both sender and receiver reports may have attached report blocks.
  // Uses up to `config_.max_packet_size - reserved_bytes.per_packet`
  // Returns list of sender ssrc in sender reports.
  struct ReservedBytes {
    size_t per_packet = 0;
    size_t per_sender = 0;
  };
  std::vector<uint32_t> FillReports(Timestamp now,
                                    ReservedBytes reserved_bytes,
                                    PacketSender& rtcp_sender);

  // Creates compound RTCP packet, as defined in
  // https://tools.ietf.org/html/rfc5506#section-2
  void CreateCompoundPacket(Timestamp now,
                            size_t reserved_bytes,
                            PacketSender& rtcp_sender);

  // Sends RTCP packets.
  void SendPeriodicCompoundPacket();
  void SendImmediateFeedback(const rtcp::RtcpPacket& rtcp_packet);
  // Generate Report Blocks to be send in Sender or Receiver Reports.
  std::vector<rtcp::ReportBlock> CreateReportBlocks(Timestamp now,
                                                    size_t num_max_blocks);

  const RtcpTransceiverConfig config_;
  std::function<void(ArrayView<const uint8_t>)> rtcp_transport_;

  bool ready_to_send_;
  std::optional<rtcp::Remb> remb_;
  // TODO(bugs.webrtc.org/8239): Remove entries from remote_senders_ that are no
  // longer needed.
  flat_map<uint32_t, RemoteSenderState> remote_senders_;
  std::list<LocalSenderState> local_senders_;
  flat_map<uint32_t, std::list<LocalSenderState>::iterator>
      local_senders_by_ssrc_;
  flat_map<uint32_t, RrtrTimes> received_rrtrs_;
  RepeatingTaskHandle periodic_task_handle_;
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_RTCP_TRANSCEIVER_IMPL_H_
