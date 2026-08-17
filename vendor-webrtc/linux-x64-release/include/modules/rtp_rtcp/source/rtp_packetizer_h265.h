/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_RTP_PACKETIZER_H265_H_
#define MODULES_RTP_RTCP_SOURCE_RTP_PACKETIZER_H265_H_

#include <cstddef>
#include <cstdint>
#include <deque>
#include <queue>

#include "api/array_view.h"
#include "modules/rtp_rtcp/source/rtp_format.h"
#include "modules/rtp_rtcp/source/rtp_packet_to_send.h"

namespace webrtc {

class RtpPacketizerH265 : public RtpPacketizer {
 public:
  // Initialize with payload from encoder.
  // The payload_data must be exactly one encoded H.265 frame.
  // For H265 we only support tx-mode SRST.
  RtpPacketizerH265(ArrayView<const uint8_t> payload, PayloadSizeLimits limits);

  RtpPacketizerH265(const RtpPacketizerH265&) = delete;
  RtpPacketizerH265& operator=(const RtpPacketizerH265&) = delete;

  ~RtpPacketizerH265() override;

  size_t NumPackets() const override;

  // Get the next payload with H.265 payload header.
  // Write payload and set marker bit of the `packet`.
  // Returns true on success or false if there was no payload to packetize.
  bool NextPacket(RtpPacketToSend* rtp_packet) override;

 private:
  struct PacketUnit {
    ArrayView<const uint8_t> source_fragment;
    bool first_fragment = false;
    bool last_fragment = false;
    bool aggregated = false;
    uint16_t header = 0;
  };
  std::deque<ArrayView<const uint8_t>> input_fragments_;
  std::queue<PacketUnit> packets_;

  bool GeneratePackets();
  bool PacketizeFu(size_t fragment_index);
  int PacketizeAp(size_t fragment_index);

  void NextAggregatePacket(RtpPacketToSend* rtp_packet);
  void NextFragmentPacket(RtpPacketToSend* rtp_packet);

  const PayloadSizeLimits limits_;
  size_t num_packets_left_ = 0;
};
}  // namespace webrtc
#endif  // MODULES_RTP_RTCP_SOURCE_RTP_PACKETIZER_H265_H_
