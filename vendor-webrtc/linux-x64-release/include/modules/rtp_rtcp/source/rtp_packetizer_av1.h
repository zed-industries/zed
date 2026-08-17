/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_RTP_PACKETIZER_AV1_H_
#define MODULES_RTP_RTCP_SOURCE_RTP_PACKETIZER_AV1_H_

#include <stddef.h>
#include <stdint.h>

#include <vector>

#include "api/array_view.h"
#include "api/video/video_frame_type.h"
#include "modules/rtp_rtcp/source/rtp_format.h"

namespace webrtc {

class RtpPacketizerAv1 : public RtpPacketizer {
 public:
  RtpPacketizerAv1(ArrayView<const uint8_t> payload,
                   PayloadSizeLimits limits,
                   VideoFrameType frame_type,
                   bool is_last_frame_in_picture);
  ~RtpPacketizerAv1() override = default;

  size_t NumPackets() const override { return packets_.size() - packet_index_; }
  bool NextPacket(RtpPacketToSend* packet) override;

 private:
  struct Obu {
    uint8_t header;
    uint8_t extension_header;  // undefined if (header & kXbit) == 0
    ArrayView<const uint8_t> payload;
    int size;  // size of the header and payload combined.
  };
  struct Packet {
    explicit Packet(int first_obu_index) : first_obu(first_obu_index) {}
    // Indexes into obus_ vector of the first and last obus that should put into
    // the packet.
    int first_obu;
    int num_obu_elements = 0;
    int first_obu_offset = 0;
    int last_obu_size;
    // Total size consumed by the packet.
    int packet_size = 0;
  };

  // Parses the payload into serie of OBUs.
  static std::vector<Obu> ParseObus(ArrayView<const uint8_t> payload);
  // Returns the number of additional bytes needed to store the previous OBU
  // element if an additonal OBU element is added to the packet.
  static int AdditionalBytesForPreviousObuElement(const Packet& packet);
  static std::vector<Packet> PacketizeInternal(ArrayView<const Obu> obus,
                                               PayloadSizeLimits limits);
  // Packetize and try to distribute the payload evenly across packets.
  static std::vector<Packet> Packetize(ArrayView<const Obu> obus,
                                       PayloadSizeLimits limits);

  uint8_t AggregationHeader() const;

  const VideoFrameType frame_type_;
  const std::vector<Obu> obus_;
  const std::vector<Packet> packets_;
  const bool is_last_frame_in_picture_;
  size_t packet_index_ = 0;
};

}  // namespace webrtc
#endif  // MODULES_RTP_RTCP_SOURCE_RTP_PACKETIZER_AV1_H_
