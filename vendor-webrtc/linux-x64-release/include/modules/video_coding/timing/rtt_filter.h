/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_TIMING_RTT_FILTER_H_
#define MODULES_VIDEO_CODING_TIMING_RTT_FILTER_H_

#include <stdint.h>

#include "absl/container/inlined_vector.h"
#include "api/units/time_delta.h"

namespace webrtc {

class RttFilter {
 public:
  RttFilter();
  RttFilter(const RttFilter&) = delete;
  RttFilter& operator=(const RttFilter&) = delete;

  // Resets the filter.
  void Reset();
  // Updates the filter with a new sample.
  void Update(TimeDelta rtt);
  // A getter function for the current RTT level.
  TimeDelta Rtt() const;

 private:
  // The size of the drift and jump memory buffers
  // and thus also the detection threshold for these
  // detectors in number of samples.
  static constexpr int kMaxDriftJumpCount = 5;
  using BufferList = absl::InlinedVector<TimeDelta, kMaxDriftJumpCount>;

  // Detects RTT jumps by comparing the difference between
  // samples and average to the standard deviation.
  // Returns true if the long time statistics should be updated
  // and false otherwise
  bool JumpDetection(TimeDelta rtt);

  // Detects RTT drifts by comparing the difference between
  // max and average to the standard deviation.
  // Returns true if the long time statistics should be updated
  // and false otherwise
  bool DriftDetection(TimeDelta rtt);

  // Computes the short time average and maximum of the vector buf.
  void ShortRttFilter(const BufferList& buf);

  bool got_non_zero_update_;
  TimeDelta avg_rtt_;
  // Variance units are TimeDelta^2. Store as ms^2.
  int64_t var_rtt_;
  TimeDelta max_rtt_;
  uint32_t filt_fact_count_;
  bool last_jump_positive_ = false;
  BufferList jump_buf_;
  BufferList drift_buf_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_TIMING_RTT_FILTER_H_
