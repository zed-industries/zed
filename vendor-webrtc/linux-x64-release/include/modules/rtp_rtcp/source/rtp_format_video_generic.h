/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_RTP_RTCP_SOURCE_RTP_FORMAT_VIDEO_GENERIC_H_
#define MODULES_RTP_RTCP_SOURCE_RTP_FORMAT_VIDEO_GENERIC_H_

#include <stdint.h>

#include <cstddef>
#include <vector>

#include "api/array_view.h"
#include "modules/rtp_rtcp/source/rtp_format.h"

namespace webrtc {

class RtpPacketToSend;
struct RTPVideoHeader;

namespace RtpFormatVideoGeneric {
inline constexpr uint8_t kKeyFrameBit = 0x01;
inline constexpr uint8_t kFirstPacketBit = 0x02;
// If this bit is set, there will be an extended header contained in this
// packet. This was added later so old clients will not send this.
inline constexpr uint8_t kExtendedHeaderBit = 0x04;
}  // namespace RtpFormatVideoGeneric

class RtpPacketizerGeneric : public RtpPacketizer {
 public:
  // Initialize with payload from encoder.
  // The payload_data must be exactly one encoded generic frame.
  // Packets returned by `NextPacket` will contain the generic payload header.
  RtpPacketizerGeneric(ArrayView<const uint8_t> payload,
                       PayloadSizeLimits limits,
                       const RTPVideoHeader& rtp_video_header);
  // Initialize with payload from encoder.
  // The payload_data must be exactly one encoded generic frame.
  // Packets returned by `NextPacket` will contain raw payload without the
  // generic payload header.
  RtpPacketizerGeneric(ArrayView<const uint8_t> payload,
                       PayloadSizeLimits limits);

  ~RtpPacketizerGeneric() override;

  RtpPacketizerGeneric(const RtpPacketizerGeneric&) = delete;
  RtpPacketizerGeneric& operator=(const RtpPacketizerGeneric&) = delete;

  size_t NumPackets() const override;

  // Get the next payload.
  // Write payload and set marker bit of the `packet`.
  // Returns true on success, false otherwise.
  bool NextPacket(RtpPacketToSend* packet) override;

 private:
  // Fills header_ and header_size_ members.
  void BuildHeader(const RTPVideoHeader& rtp_video_header);

  uint8_t header_[3];
  size_t header_size_;
  ArrayView<const uint8_t> remaining_payload_;
  std::vector<int> payload_sizes_;
  std::vector<int>::const_iterator current_packet_;
};
}  // namespace webrtc
#endif  // MODULES_RTP_RTCP_SOURCE_RTP_FORMAT_VIDEO_GENERIC_H_
