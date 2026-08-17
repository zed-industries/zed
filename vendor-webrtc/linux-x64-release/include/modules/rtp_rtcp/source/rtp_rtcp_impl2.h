/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_RTP_RTCP_IMPL2_H_
#define MODULES_RTP_RTCP_SOURCE_RTP_RTCP_IMPL2_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <optional>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/environment/environment.h"
#include "api/rtp_headers.h"
#include "api/sequence_checker.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "api/task_queue/task_queue_base.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "api/video/video_bitrate_allocation.h"
#include "modules/include/module_fec_types.h"
#include "modules/rtp_rtcp/include/report_block_data.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"  // RTCPPacketType
#include "modules/rtp_rtcp/source/packet_sequencer.h"
#include "modules/rtp_rtcp/source/rtcp_packet.h"
#include "modules/rtp_rtcp/source/rtcp_packet/tmmb_item.h"
#include "modules/rtp_rtcp/source/rtcp_receiver.h"
#include "modules/rtp_rtcp/source/rtcp_sender.h"
#include "modules/rtp_rtcp/source/rtp_packet_history.h"
#include "modules/rtp_rtcp/source/rtp_packet_to_send.h"
#include "modules/rtp_rtcp/source/rtp_rtcp_interface.h"
#include "modules/rtp_rtcp/source/rtp_sender.h"
#include "modules/rtp_rtcp/source/rtp_sender_egress.h"
#include "modules/rtp_rtcp/source/rtp_sequence_number_map.h"
#include "rtc_base/gtest_prod_util.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/task_utils/repeating_task.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

struct PacedPacketInfo;
struct RTPVideoHeader;

class ModuleRtpRtcpImpl2 final : public RtpRtcpInterface,
                                 public RTCPReceiver::ModuleRtpRtcp {
 public:
  ModuleRtpRtcpImpl2(const Environment& env,
                     const RtpRtcpInterface::Configuration& configuration);
  ~ModuleRtpRtcpImpl2() override;

  // Receiver part.

  // Called when we receive an RTCP packet.
  void IncomingRtcpPacket(ArrayView<const uint8_t> incoming_packet) override;

  void SetRemoteSSRC(uint32_t ssrc) override;

  void SetLocalSsrc(uint32_t local_ssrc) override;

  // Sender part.
  void RegisterSendPayloadFrequency(int payload_type,
                                    int payload_frequency) override;

  int32_t DeRegisterSendPayload(int8_t payload_type) override;

  void SetExtmapAllowMixed(bool extmap_allow_mixed) override;

  void RegisterRtpHeaderExtension(absl::string_view uri, int id) override;
  void DeregisterSendRtpHeaderExtension(absl::string_view uri) override;

  bool SupportsPadding() const override;
  bool SupportsRtxPayloadPadding() const override;

  // Get start timestamp.
  uint32_t StartTimestamp() const override;

  // Configure start timestamp, default is a random number.
  void SetStartTimestamp(uint32_t timestamp) override;

  uint16_t SequenceNumber() const override;

  // Set SequenceNumber, default is a random number.
  void SetSequenceNumber(uint16_t seq) override;

  void SetRtpState(const RtpState& rtp_state) override;
  void SetRtxState(const RtpState& rtp_state) override;
  RtpState GetRtpState() const override;
  RtpState GetRtxState() const override;

  void SetNonSenderRttMeasurement(bool enabled) override;

  uint32_t SSRC() const override { return rtcp_sender_.SSRC(); }

  // Semantically identical to `SSRC()` but must be called on the packet
  // delivery thread/tq and returns the ssrc that maps to
  // RtpRtcpInterface::Configuration::local_media_ssrc.
  uint32_t local_media_ssrc() const;

  void SetMid(absl::string_view mid) override;

  RTCPSender::FeedbackState GetFeedbackState();

  void SetRtxSendStatus(int mode) override;
  int RtxSendStatus() const override;
  std::optional<uint32_t> RtxSsrc() const override;

  void SetRtxSendPayloadType(int payload_type,
                             int associated_payload_type) override;

  std::optional<uint32_t> FlexfecSsrc() const override;

  // Sends kRtcpByeCode when going from true to false.
  int32_t SetSendingStatus(bool sending) override;

  bool Sending() const override;

  // Drops or relays media packets.
  void SetSendingMediaStatus(bool sending) override;

  bool SendingMedia() const override;

  bool IsAudioConfigured() const override;

  void SetAsPartOfAllocation(bool part_of_allocation) override;

  bool OnSendingRtpFrame(uint32_t timestamp,
                         int64_t capture_time_ms,
                         int payload_type,
                         bool force_sender_report) override;

  bool CanSendPacket(const RtpPacketToSend& packet) const override;

  void AssignSequenceNumber(RtpPacketToSend& packet) override;

  void SendPacket(std::unique_ptr<RtpPacketToSend> packet,
                  const PacedPacketInfo& pacing_info) override;

  bool TrySendPacket(std::unique_ptr<RtpPacketToSend> packet,
                     const PacedPacketInfo& pacing_info) override;
  void OnBatchComplete() override;

  void SetFecProtectionParams(const FecProtectionParams& delta_params,
                              const FecProtectionParams& key_params) override;

  std::vector<std::unique_ptr<RtpPacketToSend>> FetchFecPackets() override;

  void OnAbortedRetransmissions(
      ArrayView<const uint16_t> sequence_numbers) override;

  void OnPacketsAcknowledged(
      ArrayView<const uint16_t> sequence_numbers) override;

  std::vector<std::unique_ptr<RtpPacketToSend>> GeneratePadding(
      size_t target_size_bytes) override;

  std::vector<RtpSequenceNumberMap::Info> GetSentRtpPacketInfos(
      ArrayView<const uint16_t> sequence_numbers) const override;

  size_t ExpectedPerPacketOverhead() const override;

  void OnPacketSendingThreadSwitched() override;

  // RTCP part.

  // Get RTCP status.
  RtcpMode RTCP() const override;

  // Configure RTCP status i.e on/off.
  void SetRTCPStatus(RtcpMode method) override;

  // Set RTCP CName.
  int32_t SetCNAME(absl::string_view c_name) override;

  // Get RoundTripTime.
  std::optional<TimeDelta> LastRtt() const override;

  TimeDelta ExpectedRetransmissionTime() const override;

  // Force a send of an RTCP packet.
  // Normal SR and RR are triggered via the task queue that's current when this
  // object is created.
  int32_t SendRTCP(RTCPPacketType rtcpPacketType) override;

  void GetSendStreamDataCounters(
      StreamDataCounters* rtp_counters,
      StreamDataCounters* rtx_counters) const override;

  // A snapshot of the most recent Report Block with additional data of
  // interest to statistics. Used to implement RTCRemoteInboundRtpStreamStats.
  // Within this list, the `ReportBlockData::source_ssrc()`, which is the SSRC
  // of the corresponding outbound RTP stream, is unique.
  std::vector<ReportBlockData> GetLatestReportBlockData() const override;
  std::optional<SenderReportStats> GetSenderReportStats() const override;
  std::optional<NonSenderRttStats> GetNonSenderRttStats() const override;

  // (REMB) Receiver Estimated Max Bitrate.
  void SetRemb(int64_t bitrate_bps, std::vector<uint32_t> ssrcs) override;
  void UnsetRemb() override;

  void SetTmmbn(std::vector<rtcp::TmmbItem> bounding_set) override;

  size_t MaxRtpPacketSize() const override;

  void SetMaxRtpPacketSize(size_t max_packet_size) override;

  // (NACK) Negative acknowledgment part.

  // Send a Negative acknowledgment packet.
  // TODO(philipel): Deprecate SendNACK and use SendNack instead.
  int32_t SendNACK(const uint16_t* nack_list, uint16_t size) override;

  void SendNack(const std::vector<uint16_t>& sequence_numbers) override;

  // Store the sent packets, needed to answer to a negative acknowledgment
  // requests.
  void SetStorePacketsStatus(bool enable, uint16_t number_to_store) override;

  void SendCombinedRtcpPacket(
      std::vector<std::unique_ptr<rtcp::RtcpPacket>> rtcp_packets) override;

  // Video part.
  int32_t SendLossNotification(uint16_t last_decoded_seq_num,
                               uint16_t last_received_seq_num,
                               bool decodability_flag,
                               bool buffering_allowed) override;

  RtpSendRates GetSendRates() const override;

  void OnReceivedNack(
      const std::vector<uint16_t>& nack_sequence_numbers) override;
  void OnReceivedRtcpReportBlocks(
      ArrayView<const ReportBlockData> report_blocks) override;
  void OnRequestSendReport() override;

  void SetVideoBitrateAllocation(
      const VideoBitrateAllocation& bitrate) override;

  RTPSender* RtpSender() override;
  const RTPSender* RtpSender() const override;

 private:
  FRIEND_TEST_ALL_PREFIXES(RtpRtcpImpl2Test, Rtt);
  FRIEND_TEST_ALL_PREFIXES(RtpRtcpImpl2Test, RttForReceiverOnly);

  struct RtpSenderContext {
    explicit RtpSenderContext(const Environment& env,
                              TaskQueueBase& worker_queue,
                              const RtpRtcpInterface::Configuration& config);
    // Storage of packets, for retransmissions and padding, if applicable.
    RtpPacketHistory packet_history;
    SequenceChecker sequencing_checker;
    // Handles sequence number assignment and padding timestamp generation.
    PacketSequencer sequencer RTC_GUARDED_BY(sequencing_checker);
    // Handles final time timestamping/stats/etc and handover to Transport.
    RtpSenderEgress packet_sender;
    // If no paced sender configured, this class will be used to pass packets
    // from `packet_generator_` to `packet_sender_`.
    RtpSenderEgress::NonPacedPacketSender non_paced_sender;
    // Handles creation of RTP packets to be sent.
    RTPSender packet_generator;
  };

  void set_rtt_ms(int64_t rtt_ms);
  int64_t rtt_ms() const;

  bool TimeToSendFullNackList(int64_t now) const;

  // Called on a timer, once a second, on the worker_queue_, to update the RTT,
  // check if we need to send RTCP report, send TMMBR updates and fire events.
  void PeriodicUpdate();

  // Returns true if the module is configured to store packets.
  bool StorePackets() const;

  // Used from RtcpSenderMediator to maybe send rtcp.
  void MaybeSendRtcp() RTC_RUN_ON(worker_queue_);

  // Called when `rtcp_sender_` informs of the next RTCP instant. The method may
  // be called on various sequences, and is called under a RTCPSenderLock.
  void ScheduleRtcpSendEvaluation(TimeDelta duration);

  // Helper method combating too early delayed calls from task queues.
  // TODO(bugs.webrtc.org/12889): Consider removing this function when the issue
  // is resolved.
  void MaybeSendRtcpAtOrAfterTimestamp(Timestamp execution_time)
      RTC_RUN_ON(worker_queue_);

  // Schedules a call to MaybeSendRtcpAtOrAfterTimestamp delayed by `duration`.
  void ScheduleMaybeSendRtcpAtOrAfterTimestamp(Timestamp execution_time,
                                               TimeDelta duration);

  const Environment env_;
  TaskQueueBase* const worker_queue_;
  RTC_NO_UNIQUE_ADDRESS SequenceChecker rtcp_thread_checker_;

  std::unique_ptr<RtpSenderContext> rtp_sender_;
  RTCPSender rtcp_sender_;
  RTCPReceiver rtcp_receiver_;

  uint16_t packet_overhead_;

  // Send side
  int64_t nack_last_time_sent_full_ms_;
  uint16_t nack_last_seq_number_sent_;

  RtcpRttStats* const rtt_stats_;
  RepeatingTaskHandle rtt_update_task_ RTC_GUARDED_BY(worker_queue_);

  // The processed RTT from RtcpRttStats.
  mutable Mutex mutex_rtt_;
  int64_t rtt_ms_ RTC_GUARDED_BY(mutex_rtt_);

  RTC_NO_UNIQUE_ADDRESS ScopedTaskSafety task_safety_;
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_RTP_RTCP_IMPL2_H_
