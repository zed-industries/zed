/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_INCLUDE_REMOTE_NTP_TIME_ESTIMATOR_H_
#define MODULES_RTP_RTCP_INCLUDE_REMOTE_NTP_TIME_ESTIMATOR_H_

#include <stdint.h>

#include <optional>

#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "rtc_base/numerics/moving_percentile_filter.h"
#include "system_wrappers/include/ntp_time.h"
#include "system_wrappers/include/rtp_to_ntp_estimator.h"

namespace webrtc {

class Clock;

// RemoteNtpTimeEstimator can be used to estimate a given RTP timestamp's NTP
// time in local timebase.
// Note that it needs to be trained with at least 2 RTCP SR (by calling
// `UpdateRtcpTimestamp`) before it can be used.
class RemoteNtpTimeEstimator {
 public:
  explicit RemoteNtpTimeEstimator(Clock* clock);
  RemoteNtpTimeEstimator(const RemoteNtpTimeEstimator&) = delete;
  RemoteNtpTimeEstimator& operator=(const RemoteNtpTimeEstimator&) = delete;
  ~RemoteNtpTimeEstimator() = default;

  // Updates the estimator with round trip time `rtt` and
  // new NTP time <-> RTP timestamp mapping from an RTCP sender report.
  bool UpdateRtcpTimestamp(TimeDelta rtt,
                           NtpTime sender_send_time,
                           uint32_t rtp_timestamp);

  // Estimates the NTP timestamp in local timebase from `rtp_timestamp`.
  // Returns the NTP timestamp in ms when success. -1 if failed.
  int64_t Estimate(uint32_t rtp_timestamp) {
    NtpTime ntp_time = EstimateNtp(rtp_timestamp);
    if (!ntp_time.Valid()) {
      return -1;
    }
    return ntp_time.ToMs();
  }

  // Estimates the NTP timestamp in local timebase from `rtp_timestamp`.
  // Returns invalid NtpTime (i.e. NtpTime(0)) on failure.
  NtpTime EstimateNtp(uint32_t rtp_timestamp);

  // Estimates the offset between the remote clock and the
  // local one. This is equal to local NTP clock - remote NTP clock.
  // The offset is returned in ntp time resolution, i.e. 1/2^32 sec ~= 0.2 ns.
  // Returns nullopt on failure.
  std::optional<int64_t> EstimateRemoteToLocalClockOffset();

 private:
  Clock* clock_;
  // Offset is measured with the same precision as NtpTime: in 1/2^32 seconds ~=
  // 0.2 ns.
  MovingMedianFilter<int64_t> ntp_clocks_offset_estimator_;
  RtpToNtpEstimator rtp_to_ntp_;
  Timestamp last_timing_log_ = Timestamp::MinusInfinity();
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_INCLUDE_REMOTE_NTP_TIME_ESTIMATOR_H_
