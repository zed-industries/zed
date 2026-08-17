/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_RTP_RTCP_SOURCE_RTP_PACKET_TO_SEND_H_
#define MODULES_RTP_RTCP_SOURCE_RTP_PACKET_TO_SEND_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>
#include <utility>

#include "api/ref_counted_base.h"
#include "api/scoped_refptr.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "api/video/video_timing.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "modules/rtp_rtcp/source/rtp_header_extensions.h"
#include "modules/rtp_rtcp/source/rtp_packet.h"

namespace webrtc {
// Class to hold rtp packet with metadata for sender side.
// The metadata is not send over the wire, but packet sender may use it to
// create rtp header extensions or other data that is sent over the wire.
class RtpPacketToSend : public RtpPacket {
 public:
  // RtpPacketToSend::Type is deprecated. Use RtpPacketMediaType directly.
  using Type = RtpPacketMediaType;

  explicit RtpPacketToSend(const ExtensionManager* extensions);
  RtpPacketToSend(const ExtensionManager* extensions, size_t capacity);
  RtpPacketToSend(const RtpPacketToSend& packet);
  RtpPacketToSend(RtpPacketToSend&& packet);

  RtpPacketToSend& operator=(const RtpPacketToSend& packet);
  RtpPacketToSend& operator=(RtpPacketToSend&& packet);

  ~RtpPacketToSend();

  // Time in local time base as close as it can to frame capture time.
  webrtc::Timestamp capture_time() const { return capture_time_; }
  void set_capture_time(webrtc::Timestamp time) { capture_time_ = time; }

  void set_packet_type(RtpPacketMediaType type);

  std::optional<RtpPacketMediaType> packet_type() const { return packet_type_; }

  enum class OriginalType { kAudio, kVideo };
  // Original type does not change if packet type is changed to kRetransmission.
  std::optional<OriginalType> original_packet_type() const {
    return original_packet_type_;
  }

  // If this is a retransmission, indicates the sequence number of the original
  // media packet that this packet represents. If RTX is used this will likely
  // be different from SequenceNumber().
  void set_retransmitted_sequence_number(uint16_t sequence_number) {
    retransmitted_sequence_number_ = sequence_number;
  }
  std::optional<uint16_t> retransmitted_sequence_number() const {
    return retransmitted_sequence_number_;
  }

  // If this is a retransmission, indicates the SSRC of the original
  // media packet that this packet represents.
  void set_original_ssrc(uint32_t ssrc) { original_ssrc_ = ssrc; }
  std::optional<uint32_t> original_ssrc() const { return original_ssrc_; }

  void set_allow_retransmission(bool allow_retransmission) {
    allow_retransmission_ = allow_retransmission;
  }
  bool allow_retransmission() const { return allow_retransmission_; }

  // An application can attach arbitrary data to an RTP packet using
  // `additional_data`. The additional data does not affect WebRTC processing.
  scoped_refptr<RefCountedBase> additional_data() const {
    return additional_data_;
  }
  void set_additional_data(scoped_refptr<RefCountedBase> data) {
    additional_data_ = std::move(data);
  }

  void set_packetization_finish_time(webrtc::Timestamp time) {
    SetExtension<VideoTimingExtension>(
        VideoSendTiming::GetDeltaCappedMs(time - capture_time_),
        VideoTimingExtension::kPacketizationFinishDeltaOffset);
  }

  void set_pacer_exit_time(webrtc::Timestamp time) {
    SetExtension<VideoTimingExtension>(
        VideoSendTiming::GetDeltaCappedMs(time - capture_time_),
        VideoTimingExtension::kPacerExitDeltaOffset);
  }

  void set_network_time(webrtc::Timestamp time) {
    SetExtension<VideoTimingExtension>(
        VideoSendTiming::GetDeltaCappedMs(time - capture_time_),
        VideoTimingExtension::kNetworkTimestampDeltaOffset);
  }

  void set_network2_time(webrtc::Timestamp time) {
    SetExtension<VideoTimingExtension>(
        VideoSendTiming::GetDeltaCappedMs(time - capture_time_),
        VideoTimingExtension::kNetwork2TimestampDeltaOffset);
  }

  // Indicates if packet is the first packet of a video frame.
  void set_first_packet_of_frame(bool is_first_packet) {
    is_first_packet_of_frame_ = is_first_packet;
  }
  bool is_first_packet_of_frame() const { return is_first_packet_of_frame_; }

  // Indicates if packet contains payload for a video key-frame.
  void set_is_key_frame(bool is_key_frame) { is_key_frame_ = is_key_frame; }
  bool is_key_frame() const { return is_key_frame_; }

  // Indicates if packets should be protected by FEC (Forward Error Correction).
  void set_fec_protect_packet(bool protect) { fec_protect_packet_ = protect; }
  bool fec_protect_packet() const { return fec_protect_packet_; }

  // Indicates if packet is using RED encapsulation, in accordance with
  // https://tools.ietf.org/html/rfc2198
  void set_is_red(bool is_red) { is_red_ = is_red; }
  bool is_red() const { return is_red_; }

  // The amount of time spent in the send queue, used for totalPacketSendDelay.
  // https://w3c.github.io/webrtc-stats/#dom-rtcoutboundrtpstreamstats-totalpacketsenddelay
  void set_time_in_send_queue(TimeDelta time_in_send_queue) {
    time_in_send_queue_ = time_in_send_queue;
  }
  std::optional<TimeDelta> time_in_send_queue() const {
    return time_in_send_queue_;
  }
  // A sequence number guaranteed to be monotically increasing by one for all
  // packets where transport feedback is expected.
  std::optional<int64_t> transport_sequence_number() const {
    return transport_sequence_number_;
  }
  void set_transport_sequence_number(int64_t transport_sequence_number) {
    transport_sequence_number_ = transport_sequence_number;
  }
  // Transport is capable of handling explicit congestion notification and the
  // RTP packet should be sent as ect(1)
  // https://www.rfc-editor.org/rfc/rfc9331.html
  bool send_as_ect1() const { return send_as_ect1_; }
  void set_send_as_ect1() { send_as_ect1_ = true; }

 private:
  webrtc::Timestamp capture_time_ = webrtc::Timestamp::Zero();
  std::optional<RtpPacketMediaType> packet_type_;
  std::optional<OriginalType> original_packet_type_;
  std::optional<uint32_t> original_ssrc_;
  std::optional<int64_t> transport_sequence_number_;
  bool allow_retransmission_ = false;
  std::optional<uint16_t> retransmitted_sequence_number_;
  scoped_refptr<RefCountedBase> additional_data_;
  bool is_first_packet_of_frame_ = false;
  bool is_key_frame_ = false;
  bool fec_protect_packet_ = false;
  bool is_red_ = false;
  bool send_as_ect1_ = false;
  std::optional<TimeDelta> time_in_send_queue_;
};

}  // namespace webrtc
#endif  // MODULES_RTP_RTCP_SOURCE_RTP_PACKET_TO_SEND_H_
