/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_REPORT_BLOCK_STATS_H_
#define VIDEO_REPORT_BLOCK_STATS_H_

#include <stdint.h>

#include <map>

namespace webrtc {

// TODO(nisse): Usefulness of this class is somewhat unclear. The inputs are
// cumulative counters, from which we compute deltas, and then accumulate the
// deltas. May be needed on the send side, to handle wraparound in the short
// counters received over RTCP, but should not be needed on the receive side
// where we can use large enough types for all counters we need.

// Helper class for rtcp statistics.
class ReportBlockStats {
 public:
  ReportBlockStats();
  ~ReportBlockStats();

  // Updates stats and stores report block.
  void Store(uint32_t ssrc,
             int packets_lost,
             uint32_t extended_highest_sequence_number);

  // Returns the total fraction of lost packets (or -1 if less than two report
  // blocks have been stored).
  int FractionLostInPercent() const;

 private:
  // The information from an RTCP report block that we need.
  struct Report {
    uint32_t extended_highest_sequence_number;
    int32_t packets_lost;
  };

  // The total number of packets/lost packets.
  uint32_t num_sequence_numbers_;
  uint32_t num_lost_sequence_numbers_;

  // Map holding the last stored report (mapped by the source SSRC).
  std::map<uint32_t, Report> prev_reports_;
};

}  // namespace webrtc

#endif  // VIDEO_REPORT_BLOCK_STATS_H_
