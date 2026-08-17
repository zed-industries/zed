/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_REMOTE_BITRATE_ESTIMATOR_OVERUSE_ESTIMATOR_H_
#define MODULES_REMOTE_BITRATE_ESTIMATOR_OVERUSE_ESTIMATOR_H_

#include <stdint.h>

#include <deque>

#include "api/transport/bandwidth_usage.h"

namespace webrtc {

class OveruseEstimator {
 public:
  OveruseEstimator();

  OveruseEstimator(const OveruseEstimator&) = delete;
  OveruseEstimator& operator=(const OveruseEstimator&) = delete;

  ~OveruseEstimator() = default;

  // Update the estimator with a new sample. The deltas should represent deltas
  // between timestamp groups as defined by the InterArrival class.
  // `current_hypothesis` should be the hypothesis of the over-use detector at
  // this time.
  void Update(int64_t t_delta,
              double ts_delta,
              int size_delta,
              BandwidthUsage current_hypothesis,
              int64_t now_ms);

  // Returns the estimated noise/jitter variance in ms^2.
  double var_noise() const { return var_noise_; }

  // Returns the estimated inter-arrival time delta offset in ms.
  double offset() const { return offset_; }

  // Returns the number of deltas which the current over-use estimator state is
  // based on.
  int num_of_deltas() const { return num_of_deltas_; }

 private:
  double UpdateMinFramePeriod(double ts_delta);
  void UpdateNoiseEstimate(double residual, double ts_delta, bool stable_state);

  int num_of_deltas_ = 0;
  double slope_ = 8.0 / 512.0;
  double offset_ = 0;
  double prev_offset_ = 0;
  double E_[2][2] = {{100.0, 0.0}, {0.0, 1e-1}};
  double process_noise_[2] = {1e-13, 1e-3};
  double avg_noise_ = 0.0;
  double var_noise_ = 50.0;
  std::deque<double> ts_delta_hist_;
};
}  // namespace webrtc

#endif  // MODULES_REMOTE_BITRATE_ESTIMATOR_OVERUSE_ESTIMATOR_H_
