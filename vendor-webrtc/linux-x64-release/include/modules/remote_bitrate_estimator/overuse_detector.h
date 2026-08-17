/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_REMOTE_BITRATE_ESTIMATOR_OVERUSE_DETECTOR_H_
#define MODULES_REMOTE_BITRATE_ESTIMATOR_OVERUSE_DETECTOR_H_

#include <stdint.h>

#include "api/transport/bandwidth_usage.h"

namespace webrtc {

class OveruseDetector {
 public:
  OveruseDetector();

  OveruseDetector(const OveruseDetector&) = delete;
  OveruseDetector& operator=(const OveruseDetector&) = delete;

  ~OveruseDetector() = default;

  // Update the detection state based on the estimated inter-arrival time delta
  // offset. `timestamp_delta` is the delta between the last timestamp which the
  // estimated offset is based on and the last timestamp on which the last
  // offset was based on, representing the time between detector updates.
  // `num_of_deltas` is the number of deltas the offset estimate is based on.
  // Returns the state after the detection update.
  BandwidthUsage Detect(double offset,
                        double timestamp_delta,
                        int num_of_deltas,
                        int64_t now_ms);

  // Returns the current detector state.
  BandwidthUsage State() const;

 private:
  void UpdateThreshold(double modified_offset, int64_t now_ms);

  double threshold_ = 12.5;
  int64_t last_update_ms_ = -1;
  double prev_offset_ = 0.0;
  double time_over_using_ = -1;
  int overuse_counter_ = 0;
  BandwidthUsage hypothesis_ = BandwidthUsage::kBwNormal;
};
}  // namespace webrtc

#endif  // MODULES_REMOTE_BITRATE_ESTIMATOR_OVERUSE_DETECTOR_H_
