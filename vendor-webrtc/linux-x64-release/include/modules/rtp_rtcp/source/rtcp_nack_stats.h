/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_RTCP_NACK_STATS_H_
#define MODULES_RTP_RTCP_SOURCE_RTCP_NACK_STATS_H_

#include <stdint.h>

namespace webrtc {

class RtcpNackStats {
 public:
  RtcpNackStats();

  // Updates stats with requested sequence number.
  // This function should be called for each NACK request to calculate the
  // number of unique NACKed RTP packets.
  void ReportRequest(uint16_t sequence_number);

  // Gets the number of NACKed RTP packets.
  uint32_t requests() const { return requests_; }

  // Gets the number of unique NACKed RTP packets.
  uint32_t unique_requests() const { return unique_requests_; }

 private:
  uint16_t max_sequence_number_;
  uint32_t requests_;
  uint32_t unique_requests_;
};

}  // namespace webrtc
#endif  // MODULES_RTP_RTCP_SOURCE_RTCP_NACK_STATS_H_
