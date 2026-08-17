/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_RTCP_PACKET_NACK_H_
#define MODULES_RTP_RTCP_SOURCE_RTCP_PACKET_NACK_H_

#include <cstddef>
#include <cstdint>
#include <vector>

#include "modules/rtp_rtcp/source/rtcp_packet/rtpfb.h"

namespace webrtc {
namespace rtcp {
class CommonHeader;

class Nack : public Rtpfb {
 public:
  static constexpr uint8_t kFeedbackMessageType = 1;
  Nack();
  Nack(const Nack&);
  ~Nack() override;

  // Parse assumes header is already parsed and validated.
  bool Parse(const CommonHeader& packet);

  void SetPacketIds(const uint16_t* nack_list, size_t length);
  void SetPacketIds(std::vector<uint16_t> nack_list);
  const std::vector<uint16_t>& packet_ids() const { return packet_ids_; }

  size_t BlockLength() const override;

  bool Create(uint8_t* packet,
              size_t* index,
              size_t max_length,
              PacketReadyCallback callback) const override;

 private:
  static constexpr size_t kNackItemLength = 4;
  struct PackedNack {
    uint16_t first_pid;
    uint16_t bitmask;
  };

  void Pack();    // Fills packed_ using packed_ids_. (used in SetPacketIds).
  void Unpack();  // Fills packet_ids_ using packed_. (used in Parse).

  std::vector<PackedNack> packed_;
  std::vector<uint16_t> packet_ids_;
};

}  // namespace rtcp
}  // namespace webrtc
#endif  // MODULES_RTP_RTCP_SOURCE_RTCP_PACKET_NACK_H_
