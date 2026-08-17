/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_RTP_CONFIG_H_
#define CALL_RTP_CONFIG_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>
#include <string>
#include <vector>

#include "api/rtp_headers.h"
#include "api/rtp_parameters.h"

namespace webrtc {
// Currently only VP8/VP9 specific.
struct RtpPayloadState {
  int16_t picture_id = -1;
  uint8_t tl0_pic_idx = 0;
  int64_t shared_frame_id = 0;
  int64_t frame_id = 0;
};

// Settings for LNTF (LossNotification). Still highly experimental.
struct LntfConfig {
  std::string ToString() const;

  bool enabled{false};
};

// Settings for NACK, see RFC 4585 for details.
struct NackConfig {
  NackConfig() : rtp_history_ms(0) {}
  std::string ToString() const;
  // Send side: the time RTP packets are stored for retransmissions.
  // Receive side: the time the receiver is prepared to wait for
  // retransmissions.
  // Set to '0' to disable.
  int rtp_history_ms;
};

// Settings for ULPFEC forward error correction.
// Set the payload types to '-1' to disable.
struct UlpfecConfig {
  UlpfecConfig()
      : ulpfec_payload_type(-1),
        red_payload_type(-1),
        red_rtx_payload_type(-1) {}
  std::string ToString() const;
  bool operator==(const UlpfecConfig& other) const;

  // Payload type used for ULPFEC packets.
  int ulpfec_payload_type;

  // Payload type used for RED packets.
  int red_payload_type;

  // RTX payload type for RED payload.
  int red_rtx_payload_type;
};

struct RtpStreamConfig {
  std::string ToString() const;

  uint32_t ssrc = 0;
  std::string rid;
  std::string payload_name;
  int payload_type = -1;
  bool raw_payload = false;
  struct Rtx {
    std::string ToString() const;
    // SSRC to use for the RTX stream.
    uint32_t ssrc = 0;

    // Payload type to use for the RTX stream.
    int payload_type = -1;
  };
  std::optional<Rtx> rtx;
};

static const size_t kDefaultMaxPacketSize = 1500 - 40;  // TCP over IPv4.
struct RtpConfig {
  RtpConfig();
  RtpConfig(const RtpConfig&);
  ~RtpConfig();
  std::string ToString() const;

  std::vector<uint32_t> ssrcs;

  // The Rtp Stream Ids (aka RIDs) to send in the RID RTP header extension
  // if the extension is included in the list of extensions.
  // If rids are specified, they should correspond to the `ssrcs` vector.
  // This means that:
  // 1. rids.size() == 0 || rids.size() == ssrcs.size().
  // 2. If rids is not empty, then `rids[i]` should use `ssrcs[i]`.
  std::vector<std::string> rids;

  // The value to send in the MID RTP header extension if the extension is
  // included in the list of extensions.
  std::string mid;

  // See RtcpMode for description.
  RtcpMode rtcp_mode = RtcpMode::kCompound;

  // Max RTP packet size delivered to send transport from VideoEngine.
  size_t max_packet_size = kDefaultMaxPacketSize;

  // Corresponds to the SDP attribute extmap-allow-mixed.
  bool extmap_allow_mixed = false;

  // RTP header extensions to use for this send stream.
  std::vector<RtpExtension> extensions;

  // TODO(nisse): For now, these are fixed, but we'd like to support
  // changing codec without recreating the VideoSendStream. Then these
  // fields must be removed, and association between payload type and codec
  // must move above the per-stream level. Ownership could be with
  // RtpTransportControllerSend, with a reference from RtpVideoSender, where
  // the latter would be responsible for mapping the codec type of encoded
  // images to the right payload type.
  std::string payload_name;
  int payload_type = -1;
  // Payload should be packetized using raw packetizer (payload header will
  // not be added, additional meta data is expected to be present in generic
  // frame descriptor RTP header extension).
  bool raw_payload = false;

  // Configurations for each RTP stream
  std::vector<RtpStreamConfig> stream_configs;

  // See LntfConfig for description.
  LntfConfig lntf;

  // See NackConfig for description.
  NackConfig nack;

  // See UlpfecConfig for description.
  UlpfecConfig ulpfec;

  struct Flexfec {
    Flexfec();
    Flexfec(const Flexfec&);
    ~Flexfec();
    // Payload type of FlexFEC. Set to -1 to disable sending FlexFEC.
    int payload_type = -1;

    // SSRC of FlexFEC stream.
    uint32_t ssrc = 0;

    // Vector containing a single element, corresponding to the SSRC of the
    // media stream being protected by this FlexFEC stream.
    // The vector MUST have size 1.
    //
    // TODO(brandtr): Update comment above when we support
    // multistream protection.
    std::vector<uint32_t> protected_media_ssrcs;
  } flexfec;

  // Settings for RTP retransmission payload format, see RFC 4588 for
  // details.
  struct Rtx {
    Rtx();
    Rtx(const Rtx&);
    ~Rtx();
    std::string ToString() const;
    // SSRCs to use for the RTX streams.
    std::vector<uint32_t> ssrcs;

    // Payload type to use for the RTX stream.
    int payload_type = -1;
  } rtx;

  // RTCP CNAME, see RFC 3550.
  std::string c_name;

  // Enables send packet batching from the egress RTP sender.
  bool enable_send_packet_batching = false;

  bool IsMediaSsrc(uint32_t ssrc) const;
  bool IsRtxSsrc(uint32_t ssrc) const;
  bool IsFlexfecSsrc(uint32_t ssrc) const;
  std::optional<uint32_t> GetRtxSsrcAssociatedWithMediaSsrc(
      uint32_t media_ssrc) const;
  uint32_t GetMediaSsrcAssociatedWithRtxSsrc(uint32_t rtx_ssrc) const;
  uint32_t GetMediaSsrcAssociatedWithFlexfecSsrc(uint32_t flexfec_ssrc) const;
  std::optional<std::string> GetRidForSsrc(uint32_t ssrc) const;

  // Returns send config for RTP stream by provided simulcast `index`.
  RtpStreamConfig GetStreamConfig(size_t index) const;
};
}  // namespace webrtc
#endif  // CALL_RTP_CONFIG_H_
