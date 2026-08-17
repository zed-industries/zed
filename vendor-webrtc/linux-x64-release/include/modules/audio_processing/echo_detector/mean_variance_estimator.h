/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_ECHO_DETECTOR_MEAN_VARIANCE_ESTIMATOR_H_
#define MODULES_AUDIO_PROCESSING_ECHO_DETECTOR_MEAN_VARIANCE_ESTIMATOR_H_

namespace webrtc {

// This class iteratively estimates the mean and variance of a signal.
class MeanVarianceEstimator {
 public:
  void Update(float value);
  float std_deviation() const;
  float mean() const;
  void Clear();

 private:
  // Estimate of the expected value of the input values.
  float mean_ = 0.f;
  // Estimate of the variance of the input values.
  float variance_ = 0.f;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_ECHO_DETECTOR_MEAN_VARIANCE_ESTIMATOR_H_
