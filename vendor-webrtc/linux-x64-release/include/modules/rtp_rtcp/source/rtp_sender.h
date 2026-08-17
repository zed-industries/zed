/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_RTP_SENDER_H_
#define MODULES_RTP_RTCP_SOURCE_RTP_SENDER_H_

#include <cstddef>
#include <cstdint>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/environment/environment.h"
#include "api/rtp_packet_sender.h"
#include "modules/rtp_rtcp/include/flexfec_sender.h"
#include "modules/rtp_rtcp/include/rtp_header_extension_map.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "modules/rtp_rtcp/source/rtp_header_extension_size.h"
#include "modules/rtp_rtcp/source/rtp_packet_history.h"
#include "modules/rtp_rtcp/source/rtp_rtcp_interface.h"
#include "rtc_base/random.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class FrameEncryptorInterface;
class RateLimiter;
class RtpPacketToSend;

// Maximum amount of padding in RFC 3550 is 255 bytes.
constexpr size_t kMaxPaddingLength = 255;

class RTPSender {
 public:
  RTPSender(const Environment& env,
            const RtpRtcpInterface::Configuration& config,
            RtpPacketHistory* packet_history,
            RtpPacketSender* packet_sender);

  RTPSender(const RTPSender&) = delete;
  RTPSender& operator=(const RTPSender&) = delete;

  ~RTPSender();

  void SetSendingMediaStatus(bool enabled) RTC_LOCKS_EXCLUDED(send_mutex_);
  bool SendingMedia() const RTC_LOCKS_EXCLUDED(send_mutex_);
  bool IsAudioConfigured() const RTC_LOCKS_EXCLUDED(send_mutex_);

  uint32_t TimestampOffset() const RTC_LOCKS_EXCLUDED(send_mutex_);
  void SetTimestampOffset(uint32_t timestamp) RTC_LOCKS_EXCLUDED(send_mutex_);

  void SetMid(absl::string_view mid) RTC_LOCKS_EXCLUDED(send_mutex_);

  uint16_t SequenceNumber() const RTC_LOCKS_EXCLUDED(send_mutex_);
  void SetSequenceNumber(uint16_t seq) RTC_LOCKS_EXCLUDED(send_mutex_);

  void SetMaxRtpPacketSize(size_t max_packet_size)
      RTC_LOCKS_EXCLUDED(send_mutex_);

  void SetExtmapAllowMixed(bool extmap_allow_mixed)
      RTC_LOCKS_EXCLUDED(send_mutex_);

  // RTP header extension
  bool RegisterRtpHeaderExtension(absl::string_view uri, int id)
      RTC_LOCKS_EXCLUDED(send_mutex_);
  bool IsRtpHeaderExtensionRegistered(RTPExtensionType type) const
      RTC_LOCKS_EXCLUDED(send_mutex_);
  void DeregisterRtpHeaderExtension(absl::string_view uri)
      RTC_LOCKS_EXCLUDED(send_mutex_);

  bool SupportsPadding() const RTC_LOCKS_EXCLUDED(send_mutex_);
  bool SupportsRtxPayloadPadding() const RTC_LOCKS_EXCLUDED(send_mutex_);

  std::vector<std::unique_ptr<RtpPacketToSend>> GeneratePadding(
      size_t target_size_bytes,
      bool media_has_been_sent,
      bool can_send_padding_on_media_ssrc) RTC_LOCKS_EXCLUDED(send_mutex_);

  // NACK.
  void OnReceivedNack(const std::vector<uint16_t>& nack_sequence_numbers,
                      int64_t avg_rtt) RTC_LOCKS_EXCLUDED(send_mutex_);

  int32_t ReSendPacket(uint16_t packet_id) RTC_LOCKS_EXCLUDED(send_mutex_);

  // ACK.
  void OnReceivedAckOnSsrc(int64_t extended_highest_sequence_number)
      RTC_LOCKS_EXCLUDED(send_mutex_);
  void OnReceivedAckOnRtxSsrc(int64_t extended_highest_sequence_number)
      RTC_LOCKS_EXCLUDED(send_mutex_);

  // RTX.
  void SetRtxStatus(int mode) RTC_LOCKS_EXCLUDED(send_mutex_);
  int RtxStatus() const RTC_LOCKS_EXCLUDED(send_mutex_);
  std::optional<uint32_t> RtxSsrc() const RTC_LOCKS_EXCLUDED(send_mutex_) {
    return rtx_ssrc_;
  }
  // Returns expected size difference between an RTX packet and media packet
  // that RTX packet is created from. Returns 0 if RTX is disabled.
  size_t RtxPacketOverhead() const;

  void SetRtxPayloadType(int payload_type, int associated_payload_type)
      RTC_LOCKS_EXCLUDED(send_mutex_);

  // Size info for header extensions used by FEC packets.
  static ArrayView<const RtpExtensionSize> FecExtensionSizes()
      RTC_LOCKS_EXCLUDED(send_mutex_);

  // Size info for header extensions used by video packets.
  static ArrayView<const RtpExtensionSize> VideoExtensionSizes()
      RTC_LOCKS_EXCLUDED(send_mutex_);

  // Size info for header extensions used by audio packets.
  static ArrayView<const RtpExtensionSize> AudioExtensionSizes()
      RTC_LOCKS_EXCLUDED(send_mutex_);

  // Create empty packet, fills ssrc, csrcs and reserve place for header
  // extensions RtpSender updates before sending.
  std::unique_ptr<RtpPacketToSend> AllocatePacket(
      ArrayView<const uint32_t> csrcs = {}) RTC_LOCKS_EXCLUDED(send_mutex_);

  // Maximum header overhead per fec/padding packet.
  size_t FecOrPaddingPacketMaxRtpHeaderLength() const
      RTC_LOCKS_EXCLUDED(send_mutex_);
  // Expected header overhead per media packet.
  size_t ExpectedPerPacketOverhead() const RTC_LOCKS_EXCLUDED(send_mutex_);
  // Including RTP headers.
  size_t MaxRtpPacketSize() const RTC_LOCKS_EXCLUDED(send_mutex_);

  uint32_t SSRC() const RTC_LOCKS_EXCLUDED(send_mutex_) { return ssrc_; }

  std::optional<uint32_t> FlexfecSsrc() const RTC_LOCKS_EXCLUDED(send_mutex_) {
    return flexfec_ssrc_;
  }

  // Pass a set of packets to RtpPacketSender instance, for paced or immediate
  // sending to the network.
  void EnqueuePackets(std::vector<std::unique_ptr<RtpPacketToSend>> packets)
      RTC_LOCKS_EXCLUDED(send_mutex_);

  void SetRtpState(const RtpState& rtp_state) RTC_LOCKS_EXCLUDED(send_mutex_);
  RtpState GetRtpState() const RTC_LOCKS_EXCLUDED(send_mutex_);
  void SetRtxRtpState(const RtpState& rtp_state)
      RTC_LOCKS_EXCLUDED(send_mutex_);
  RtpState GetRtxRtpState() const RTC_LOCKS_EXCLUDED(send_mutex_);

 private:
  std::unique_ptr<RtpPacketToSend> BuildRtxPacket(
      const RtpPacketToSend& packet);

  bool IsFecPacket(const RtpPacketToSend& packet) const;

  void UpdateHeaderSizes() RTC_EXCLUSIVE_LOCKS_REQUIRED(send_mutex_);

  void UpdateLastPacketState(const RtpPacketToSend& packet)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(send_mutex_);

  Clock* const clock_;
  Random random_ RTC_GUARDED_BY(send_mutex_);

  const bool audio_configured_;

  const uint32_t ssrc_;
  const std::optional<uint32_t> rtx_ssrc_;
  const std::optional<uint32_t> flexfec_ssrc_;

  RtpPacketHistory* const packet_history_;
  RtpPacketSender* const paced_sender_;

  mutable Mutex send_mutex_;

  bool sending_media_ RTC_GUARDED_BY(send_mutex_);
  size_t max_packet_size_;

  RtpHeaderExtensionMap rtp_header_extension_map_ RTC_GUARDED_BY(send_mutex_);
  size_t max_media_packet_header_ RTC_GUARDED_BY(send_mutex_);
  size_t max_padding_fec_packet_header_ RTC_GUARDED_BY(send_mutex_);

  // RTP variables
  uint32_t timestamp_offset_ RTC_GUARDED_BY(send_mutex_);
  // RID value to send in the RID or RepairedRID header extension.
  const std::string rid_;
  // MID value to send in the MID header extension.
  std::string mid_ RTC_GUARDED_BY(send_mutex_);
  // Should we send MID/RID even when ACKed? (see below).
  const bool always_send_mid_and_rid_;
  // Track if any ACK has been received on the SSRC and RTX SSRC to indicate
  // when to stop sending the MID and RID header extensions.
  bool ssrc_has_acked_ RTC_GUARDED_BY(send_mutex_);
  bool rtx_ssrc_has_acked_ RTC_GUARDED_BY(send_mutex_);
  // Maximum number of csrcs this sender is used with.
  size_t max_num_csrcs_ RTC_GUARDED_BY(send_mutex_) = 0;
  int rtx_ RTC_GUARDED_BY(send_mutex_);
  // Mapping rtx_payload_type_map_[associated] = rtx.
  std::map<int8_t, int8_t> rtx_payload_type_map_ RTC_GUARDED_BY(send_mutex_);
  bool supports_bwe_extension_ RTC_GUARDED_BY(send_mutex_);

  RateLimiter* const retransmission_rate_limiter_;
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_RTP_SENDER_H_
