/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_TIMING_TIMESTAMP_EXTRAPOLATOR_H_
#define MODULES_VIDEO_CODING_TIMING_TIMESTAMP_EXTRAPOLATOR_H_

#include <stdint.h>

#include <optional>

#include "api/units/timestamp.h"
#include "rtc_base/numerics/sequence_number_unwrapper.h"

namespace webrtc {

// Not thread safe.
class TimestampExtrapolator {
 public:
  explicit TimestampExtrapolator(Timestamp start);
  ~TimestampExtrapolator();
  void Update(Timestamp now, uint32_t ts90khz);
  std::optional<Timestamp> ExtrapolateLocalTime(uint32_t timestamp90khz) const;
  void Reset(Timestamp start);

 private:
  void CheckForWrapArounds(uint32_t ts90khz);
  bool DelayChangeDetection(double error);

  double w_[2];
  double p_[2][2];
  Timestamp start_;
  Timestamp prev_;
  std::optional<int64_t> first_unwrapped_timestamp_;
  RtpTimestampUnwrapper unwrapper_;
  std::optional<int64_t> prev_unwrapped_timestamp_;
  int packet_count_;
  double detector_accumulator_pos_;
  double detector_accumulator_neg_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_TIMING_TIMESTAMP_EXTRAPOLATOR_H_
