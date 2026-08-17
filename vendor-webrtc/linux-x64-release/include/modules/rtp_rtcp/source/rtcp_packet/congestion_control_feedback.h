/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_RTP_RTCP_SOURCE_RTCP_PACKET_CONGESTION_CONTROL_FEEDBACK_H_
#define MODULES_RTP_RTCP_SOURCE_RTCP_PACKET_CONGESTION_CONTROL_FEEDBACK_H_

#include <cstddef>
#include <cstdint>
#include <vector>

#include "api/array_view.h"
#include "api/units/time_delta.h"
#include "modules/rtp_rtcp/source/rtcp_packet/common_header.h"
#include "modules/rtp_rtcp/source/rtcp_packet/rtpfb.h"
#include "rtc_base/network/ecn_marking.h"

namespace webrtc {
namespace rtcp {

// Congestion control feedback message as specified in
// https://www.rfc-editor.org/rfc/rfc8888.html
class CongestionControlFeedback : public Rtpfb {
 public:
  struct PacketInfo {
    uint32_t ssrc = 0;
    uint16_t sequence_number = 0;
    //  Time offset from report timestamp. Minus infinity if the packet has not
    //  been received.
    TimeDelta arrival_time_offset = TimeDelta::MinusInfinity();
    EcnMarking ecn = EcnMarking::kNotEct;
  };

  static constexpr uint8_t kFeedbackMessageType = 11;

  // `Packets` MUST be sorted in sequence_number order per SSRC. There MUST not
  // be missing sequence numbers between `Packets`. `Packets` MUST not include
  // duplicate sequence numbers.
  CongestionControlFeedback(std::vector<PacketInfo> packets,
                            uint32_t report_timestamp_compact_ntp);
  CongestionControlFeedback() = default;

  bool Parse(const CommonHeader& packet);

  ArrayView<const PacketInfo> packets() const { return packets_; }

  uint32_t report_timestamp_compact_ntp() const {
    return report_timestamp_compact_ntp_;
  }

  // Serialize the packet.
  bool Create(uint8_t* packet,
              size_t* position,
              size_t max_length,
              PacketReadyCallback callback) const override;
  size_t BlockLength() const override;

 private:
  std::vector<PacketInfo> packets_;
  uint32_t report_timestamp_compact_ntp_ = 0;
};

}  // namespace rtcp
}  // namespace webrtc

#endif  //  MODULES_RTP_RTCP_SOURCE_RTCP_PACKET_CONGESTION_CONTROL_FEEDBACK_H_
