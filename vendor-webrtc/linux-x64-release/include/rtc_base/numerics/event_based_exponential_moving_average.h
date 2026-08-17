/*
 *  Copyright 2019 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_NUMERICS_EVENT_BASED_EXPONENTIAL_MOVING_AVERAGE_H_
#define RTC_BASE_NUMERICS_EVENT_BASED_EXPONENTIAL_MOVING_AVERAGE_H_

#include <cmath>
#include <cstdint>
#include <limits>
#include <optional>

namespace webrtc {

/**
 * This class implements exponential moving average for time series
 * estimating both value, variance and variance of estimator based on
 * https://en.wikipedia.org/w/index.php?title=Moving_average&section=9#Application_to_measuring_computer_performance
 * with the additions from nisse@ added to
 * https://en.wikipedia.org/wiki/Talk:Moving_average.
 *
 * A sample gets exponentially less weight so that it's 50%
 * after `half_time` time units.
 */
class EventBasedExponentialMovingAverage {
 public:
  // `half_time` specifies how much weight will be given to old samples,
  // see example above.
  explicit EventBasedExponentialMovingAverage(int half_time);

  void AddSample(int64_t now, int value);

  double GetAverage() const { return value_; }
  double GetVariance() const { return sample_variance_; }

  // Compute 95% confidence interval assuming that
  // - variance of samples are normal distributed.
  // - variance of estimator is normal distributed.
  //
  // The returned values specifies the distance from the average,
  // i.e if X = GetAverage(), m = GetConfidenceInterval()
  // then a there is 95% likelihood that the observed variables is inside
  // [ X +/- m ].
  double GetConfidenceInterval() const;

  // Reset
  void Reset();

  // Update the half_time.
  // NOTE: resets estimate too.
  void SetHalfTime(int half_time);

 private:
  double tau_;
  double value_ = std::nan("uninit");
  double sample_variance_ = std::numeric_limits<double>::infinity();
  // This is the ratio between variance of the estimate and variance of samples.
  double estimator_variance_ = 1;
  std::optional<int64_t> last_observation_timestamp_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::EventBasedExponentialMovingAverage;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_NUMERICS_EVENT_BASED_EXPONENTIAL_MOVING_AVERAGE_H_
