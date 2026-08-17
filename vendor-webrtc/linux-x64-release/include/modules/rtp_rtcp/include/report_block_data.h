/*
 *  Copyright 2019 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_INCLUDE_REPORT_BLOCK_DATA_H_
#define MODULES_RTP_RTCP_INCLUDE_REPORT_BLOCK_DATA_H_

#include <cstddef>
#include <cstdint>

#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "modules/rtp_rtcp/source/rtcp_packet/report_block.h"

namespace webrtc {

// Represents fields and derived information received in RTCP report block
// attached to RTCP sender report or RTCP receiver report, as described in
// https://www.rfc-editor.org/rfc/rfc3550#section-6.4.1
class ReportBlockData {
 public:
  ReportBlockData() = default;

  ReportBlockData(const ReportBlockData&) = default;
  ReportBlockData& operator=(const ReportBlockData&) = default;

  // The SSRC identifier for the originator of this report block,
  // i.e. remote receiver of the RTP stream.
  uint32_t sender_ssrc() const { return sender_ssrc_; }

  // The SSRC identifier of the source to which the information in this
  // reception report block pertains, i.e. local sender of the RTP stream.
  uint32_t source_ssrc() const { return source_ssrc_; }

  // The fraction of RTP data packets from 'source_ssrc()' lost since the
  // previous report block was sent.
  // Fraction loss in range [0.0, 1.0].
  float fraction_lost() const {
    return static_cast<float>(fraction_lost_raw()) / 256.0f;
  }

  // Fraction loss as was written in the raw packet: range is [0, 255] where 0
  // represents no loss, and 255 represents 99.6% loss (255/256 * 100%).
  uint8_t fraction_lost_raw() const { return fraction_lost_raw_; }

  // The total number of RTP data packets from 'source_ssrc()' that have been
  // lost since the beginning of reception.  This number is defined to be the
  // number of packets expected less the number of packets actually received,
  // where the number of packets received includes any which are late or
  // duplicates. Thus, packets that arrive late are not counted as lost, and the
  // loss may be negative if there are duplicates.
  int cumulative_lost() const { return cumulative_lost_; }

  // The low 16 bits contain the highest sequence number received in an RTP data
  // packet from 'source_ssrc()', and the most significant 16 bits extend that
  // sequence number with the corresponding count of sequence number cycles.
  uint32_t extended_highest_sequence_number() const {
    return extended_highest_sequence_number_;
  }

  // An estimate of the statistical variance of the RTP data packet interarrival
  // time, measured in RTP timestamp units. The interarrival jitter J is defined
  // to be the mean deviation (smoothed absolute value) of the difference D in
  // packet spacing at the receiver compared to the sender for a pair of
  // packets.
  uint32_t jitter() const { return jitter_; }

  // Jitter converted to common time units.
  TimeDelta jitter(int rtp_clock_rate_hz) const;

  // Time in utc epoch (Jan 1st, 1970) the report block was received.
  // TODO: bugs.webrtc.org/370535296 - Remove the utc timestamp when linked
  // issue is fixed.
  Timestamp report_block_timestamp_utc() const {
    return report_block_timestamp_utc_;
  }

  // Monotonic time when the report block was received.
  Timestamp report_block_timestamp() const { return report_block_timestamp_; }

  // Round Trip Time measurments for given (sender_ssrc, source_ssrc) pair.
  // Min, max, sum, number of measurements are since beginning of the call.
  TimeDelta last_rtt() const { return last_rtt_; }
  TimeDelta sum_rtts() const { return sum_rtt_; }
  size_t num_rtts() const { return num_rtts_; }
  bool has_rtt() const { return num_rtts_ != 0; }

  void set_sender_ssrc(uint32_t ssrc) { sender_ssrc_ = ssrc; }
  void set_source_ssrc(uint32_t ssrc) { source_ssrc_ = ssrc; }
  void set_fraction_lost_raw(uint8_t lost) { fraction_lost_raw_ = lost; }
  void set_cumulative_lost(int lost) { cumulative_lost_ = lost; }
  void set_extended_highest_sequence_number(uint32_t sn) {
    extended_highest_sequence_number_ = sn;
  }
  void set_jitter(uint32_t jitter) { jitter_ = jitter; }
  // TODO: bugs.webrtc.org/370535296 - Remove the utc timestamp when linked
  // issue is fixed.
  void set_report_block_timestamp_utc(Timestamp arrival_time) {
    report_block_timestamp_utc_ = arrival_time;
  }
  void set_report_block_timestamp(Timestamp arrival_time) {
    report_block_timestamp_ = arrival_time;
  }

  void SetReportBlock(uint32_t sender_ssrc,
                      const rtcp::ReportBlock& report_block,
                      Timestamp report_block_timestamp_utc,
                      Timestamp report_block_timestamp);
  void AddRoundTripTimeSample(TimeDelta rtt);

 private:
  uint32_t sender_ssrc_ = 0;
  uint32_t source_ssrc_ = 0;
  uint8_t fraction_lost_raw_ = 0;
  int32_t cumulative_lost_ = 0;
  uint32_t extended_highest_sequence_number_ = 0;
  uint32_t jitter_ = 0;
  // TODO: bugs.webrtc.org/370535296 - Remove the utc timestamp when linked
  // issue is fixed.
  Timestamp report_block_timestamp_utc_ = Timestamp::Zero();
  Timestamp report_block_timestamp_ = Timestamp::Zero();
  TimeDelta last_rtt_ = TimeDelta::Zero();
  TimeDelta sum_rtt_ = TimeDelta::Zero();
  size_t num_rtts_ = 0;
};

class ReportBlockDataObserver {
 public:
  virtual ~ReportBlockDataObserver() = default;

  virtual void OnReportBlockDataUpdated(ReportBlockData report_block_data) = 0;
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_INCLUDE_REPORT_BLOCK_DATA_H_
