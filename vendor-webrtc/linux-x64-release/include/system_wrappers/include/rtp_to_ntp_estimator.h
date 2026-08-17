/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SYSTEM_WRAPPERS_INCLUDE_RTP_TO_NTP_ESTIMATOR_H_
#define SYSTEM_WRAPPERS_INCLUDE_RTP_TO_NTP_ESTIMATOR_H_

#include <stdint.h>

#include <list>
#include <optional>

#include "rtc_base/checks.h"
#include "rtc_base/numerics/sequence_number_unwrapper.h"
#include "system_wrappers/include/ntp_time.h"

namespace webrtc {

// Converts an RTP timestamp to the NTP domain.
// The class needs to be trained with (at least 2) RTP/NTP timestamp pairs from
// RTCP sender reports before the convertion can be done.
class RtpToNtpEstimator {
 public:
  static constexpr int kMaxInvalidSamples = 3;

  RtpToNtpEstimator() = default;
  RtpToNtpEstimator(const RtpToNtpEstimator&) = delete;
  RtpToNtpEstimator& operator=(const RtpToNtpEstimator&) = delete;
  ~RtpToNtpEstimator() = default;

  enum UpdateResult { kInvalidMeasurement, kSameMeasurement, kNewMeasurement };
  // Updates measurements with RTP/NTP timestamp pair from a RTCP sender report.
  UpdateResult UpdateMeasurements(NtpTime ntp, uint32_t rtp_timestamp);

  // Converts an RTP timestamp to the NTP domain.
  // Returns invalid NtpTime (i.e. NtpTime(0)) on failure.
  NtpTime Estimate(uint32_t rtp_timestamp) const;

  // Returns estimated rtp_timestamp frequency, or 0 on failure.
  double EstimatedFrequencyKhz() const;

 private:
  // Estimated parameters from RTP and NTP timestamp pairs in `measurements_`.
  // Defines linear estimation: NtpTime (in units of 1s/2^32) =
  //   `Parameters::slope` * rtp_timestamp + `Parameters::offset`.
  struct Parameters {
    double slope;
    double offset;
  };

  // RTP and NTP timestamp pair from a RTCP SR report.
  struct RtcpMeasurement {
    NtpTime ntp_time;
    int64_t unwrapped_rtp_timestamp;
  };

  void UpdateParameters();

  int consecutive_invalid_samples_ = 0;
  std::list<RtcpMeasurement> measurements_;
  std::optional<Parameters> params_;
  mutable RtpTimestampUnwrapper unwrapper_;
};
}  // namespace webrtc

#endif  // SYSTEM_WRAPPERS_INCLUDE_RTP_TO_NTP_ESTIMATOR_H_
