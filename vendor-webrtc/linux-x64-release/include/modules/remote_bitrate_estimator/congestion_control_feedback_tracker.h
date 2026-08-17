/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_REMOTE_BITRATE_ESTIMATOR_CONGESTION_CONTROL_FEEDBACK_TRACKER_H_
#define MODULES_REMOTE_BITRATE_ESTIMATOR_CONGESTION_CONTROL_FEEDBACK_TRACKER_H_

#include <cstdint>
#include <optional>
#include <vector>

#include "api/units/timestamp.h"
#include "modules/rtp_rtcp/source/rtcp_packet/congestion_control_feedback.h"
#include "modules/rtp_rtcp/source/rtp_packet_received.h"
#include "rtc_base/network/ecn_marking.h"
#include "rtc_base/numerics/sequence_number_unwrapper.h"
namespace webrtc {

// CongestionControlFeedbackTracker is reponsible for creating and keeping track
// of feedback sent for a specific SSRC when feedback is sent according to
// https://datatracker.ietf.org/doc/rfc8888/
class CongestionControlFeedbackTracker {
 public:
  CongestionControlFeedbackTracker() = default;

  void ReceivedPacket(const RtpPacketReceived& packet);

  // Adds received packets to `packet_feedback`
  // RTP sequence numbers are continous from the last created feedback unless
  // reordering has occured between feedback packets. If so, the sequence
  // number range may overlap with previousely sent feedback.
  void AddPacketsToFeedback(
      Timestamp feedback_time,
      std::vector<rtcp::CongestionControlFeedback::PacketInfo>&
          packet_feedback);

 private:
  struct PacketInfo {
    uint32_t ssrc;
    int64_t unwrapped_sequence_number = 0;
    Timestamp arrival_time;
    EcnMarking ecn = EcnMarking::kNotEct;
  };

  std::optional<int64_t> last_sequence_number_in_feedback_;
  SeqNumUnwrapper<uint16_t> unwrapper_;

  std::vector<PacketInfo> packets_;
};

}  // namespace webrtc

#endif  // MODULES_REMOTE_BITRATE_ESTIMATOR_CONGESTION_CONTROL_FEEDBACK_TRACKER_H_
