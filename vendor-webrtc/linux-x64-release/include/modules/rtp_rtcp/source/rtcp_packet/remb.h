/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_RTCP_PACKET_REMB_H_
#define MODULES_RTP_RTCP_SOURCE_RTCP_PACKET_REMB_H_

#include <cstddef>
#include <cstdint>
#include <vector>

#include "modules/rtp_rtcp/source/rtcp_packet/psfb.h"

namespace webrtc {
namespace rtcp {
class CommonHeader;

// Receiver Estimated Max Bitrate (REMB) (draft-alvestrand-rmcat-remb).
class Remb : public Psfb {
 public:
  static constexpr size_t kMaxNumberOfSsrcs = 0xff;

  Remb();
  Remb(const Remb&);
  ~Remb() override;

  // Parse assumes header is already parsed and validated.
  bool Parse(const CommonHeader& packet);

  bool SetSsrcs(std::vector<uint32_t> ssrcs);
  void SetBitrateBps(int64_t bitrate_bps) { bitrate_bps_ = bitrate_bps; }

  int64_t bitrate_bps() const { return bitrate_bps_; }
  const std::vector<uint32_t>& ssrcs() const { return ssrcs_; }

  size_t BlockLength() const override;

  bool Create(uint8_t* packet,
              size_t* index,
              size_t max_length,
              PacketReadyCallback callback) const override;

 private:
  static constexpr uint32_t kUniqueIdentifier = 0x52454D42;  // 'R' 'E' 'M' 'B'.

  // Media ssrc is unused, shadow base class setter and getter.
  void SetMediaSsrc(uint32_t);
  uint32_t media_ssrc() const;

  int64_t bitrate_bps_;
  std::vector<uint32_t> ssrcs_;
};
}  // namespace rtcp
}  // namespace webrtc
#endif  // MODULES_RTP_RTCP_SOURCE_RTCP_PACKET_REMB_H_
