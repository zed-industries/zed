/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_PACING_PACKET_ROUTER_H_
#define MODULES_PACING_PACKET_ROUTER_H_

#include <stddef.h>
#include <stdint.h>

#include <cstdint>
#include <list>
#include <memory>
#include <optional>
#include <set>
#include <unordered_map>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "api/array_view.h"
#include "api/sequence_checker.h"
#include "api/transport/network_types.h"
#include "api/units/data_size.h"
#include "modules/pacing/pacing_controller.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "modules/rtp_rtcp/source/rtcp_packet.h"
#include "modules/rtp_rtcp/source/rtp_packet_to_send.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class RtpRtcpInterface;

// PacketRouter keeps track of rtp send modules to support the pacer.
// In addition, it handles feedback messages, which are sent on a send
// module if possible (sender report), otherwise on receive module
// (receiver report). For the latter case, we also keep track of the
// receive modules.
class PacketRouter : public PacingController::PacketSender {
 public:
  PacketRouter();
  ~PacketRouter() override;

  PacketRouter(const PacketRouter&) = delete;
  PacketRouter& operator=(const PacketRouter&) = delete;

  // Callback is invoked after pacing, before a packet is forwarded to the
  // sending rtp module.
  void RegisterNotifyBweCallback(
      absl::AnyInvocable<void(const RtpPacketToSend& packet,
                              const PacedPacketInfo& pacing_info)> callback);

  // Ensures that PacketRouter generates transport sequence numbers for all RTP
  // packets. If `send_rtp_packets_as_ect1` is true, packets will be requested
  // to be sent as ect1.
  void ConfigureForRfc8888Feedback(bool send_rtp_packets_as_ect1);

  void AddSendRtpModule(RtpRtcpInterface* rtp_module, bool remb_candidate);
  void RemoveSendRtpModule(RtpRtcpInterface* rtp_module);

  bool SupportsRtxPayloadPadding() const;

  void AddReceiveRtpModule(RtcpFeedbackSenderInterface* rtcp_sender,
                           bool remb_candidate);
  void RemoveReceiveRtpModule(RtcpFeedbackSenderInterface* rtcp_sender);

  void SendPacket(std::unique_ptr<RtpPacketToSend> packet,
                  const PacedPacketInfo& cluster_info) override;
  std::vector<std::unique_ptr<RtpPacketToSend>> FetchFec() override;
  std::vector<std::unique_ptr<RtpPacketToSend>> GeneratePadding(
      DataSize size) override;
  void OnAbortedRetransmissions(
      uint32_t ssrc,
      ArrayView<const uint16_t> sequence_numbers) override;
  std::optional<uint32_t> GetRtxSsrcForMedia(uint32_t ssrc) const override;
  void OnBatchComplete() override;

  // Send REMB feedback.
  void SendRemb(int64_t bitrate_bps, std::vector<uint32_t> ssrcs);

  // Sends `packets` in one or more IP packets.
  void SendCombinedRtcpPacket(
      std::vector<std::unique_ptr<rtcp::RtcpPacket>> packets);

 private:
  void AddRembModuleCandidate(RtcpFeedbackSenderInterface* candidate_module,
                              bool media_sender);
  void MaybeRemoveRembModuleCandidate(
      RtcpFeedbackSenderInterface* candidate_module,
      bool media_sender);
  void UnsetActiveRembModule();
  void DetermineActiveRembModule();
  void AddSendRtpModuleToMap(RtpRtcpInterface* rtp_module, uint32_t ssrc);
  void RemoveSendRtpModuleFromMap(uint32_t ssrc);

  SequenceChecker thread_checker_;
  // Ssrc to RtpRtcpInterface module;
  std::unordered_map<uint32_t, RtpRtcpInterface*> send_modules_map_
      RTC_GUARDED_BY(thread_checker_);
  std::list<RtpRtcpInterface*> send_modules_list_
      RTC_GUARDED_BY(thread_checker_);
  // The last module used to send media.
  RtpRtcpInterface* last_send_module_ RTC_GUARDED_BY(thread_checker_);
  // Rtcp modules of the rtp receivers.
  std::vector<RtcpFeedbackSenderInterface*> rtcp_feedback_senders_
      RTC_GUARDED_BY(thread_checker_);

  // Candidates for the REMB module can be RTP sender/receiver modules, with
  // the sender modules taking precedence.
  std::vector<RtcpFeedbackSenderInterface*> sender_remb_candidates_
      RTC_GUARDED_BY(thread_checker_);
  std::vector<RtcpFeedbackSenderInterface*> receiver_remb_candidates_
      RTC_GUARDED_BY(thread_checker_);
  RtcpFeedbackSenderInterface* active_remb_module_
      RTC_GUARDED_BY(thread_checker_);

  uint64_t transport_seq_ RTC_GUARDED_BY(thread_checker_);
  bool use_cc_feedback_according_to_rfc8888_ RTC_GUARDED_BY(thread_checker_) =
      false;
  bool send_rtp_packets_as_ect1_ RTC_GUARDED_BY(thread_checker_) = false;
  absl::AnyInvocable<void(RtpPacketToSend& packet,
                          const PacedPacketInfo& pacing_info)>
      notify_bwe_callback_ RTC_GUARDED_BY(thread_checker_) = nullptr;

  std::vector<std::unique_ptr<RtpPacketToSend>> pending_fec_packets_
      RTC_GUARDED_BY(thread_checker_);
  std::set<RtpRtcpInterface*> modules_used_in_current_batch_
      RTC_GUARDED_BY(thread_checker_);
};
}  // namespace webrtc
#endif  // MODULES_PACING_PACKET_ROUTER_H_
