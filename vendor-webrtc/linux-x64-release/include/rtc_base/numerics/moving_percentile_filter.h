/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_NUMERICS_MOVING_PERCENTILE_FILTER_H_
#define RTC_BASE_NUMERICS_MOVING_PERCENTILE_FILTER_H_

#include <stddef.h>

#include <cstddef>
#include <list>

#include "rtc_base/checks.h"
#include "rtc_base/numerics/percentile_filter.h"

namespace webrtc {

// Class to efficiently get moving percentile filter from a stream of samples.
template <typename T>
class MovingPercentileFilter {
 public:
  // Construct filter. `percentile` defines what percentile to track and
  // `window_size` is how many latest samples are stored for finding the
  // percentile. `percentile` must be between 0.0 and 1.0 (inclusive) and
  // `window_size` must be greater than 0.
  MovingPercentileFilter(float percentile, size_t window_size);

  MovingPercentileFilter(const MovingPercentileFilter&) = delete;
  MovingPercentileFilter& operator=(const MovingPercentileFilter&) = delete;

  // Insert a new sample.
  void Insert(const T& value);

  // Removes all samples;
  void Reset();

  // Get percentile over the latest window.
  T GetFilteredValue() const;

  // The number of samples that are currently stored.
  size_t GetNumberOfSamplesStored() const;

 private:
  PercentileFilter<T> percentile_filter_;
  std::list<T> samples_;
  size_t samples_stored_;
  const size_t window_size_;
};

// Convenience type for the common median case.
template <typename T>
class MovingMedianFilter : public MovingPercentileFilter<T> {
 public:
  explicit MovingMedianFilter(size_t window_size)
      : MovingPercentileFilter<T>(0.5f, window_size) {}
};

template <typename T>
MovingPercentileFilter<T>::MovingPercentileFilter(float percentile,
                                                  size_t window_size)
    : percentile_filter_(percentile),
      samples_stored_(0),
      window_size_(window_size) {
  RTC_CHECK_GT(window_size, 0);
}

template <typename T>
void MovingPercentileFilter<T>::Insert(const T& value) {
  percentile_filter_.Insert(value);
  samples_.emplace_back(value);
  ++samples_stored_;
  if (samples_stored_ > window_size_) {
    percentile_filter_.Erase(samples_.front());
    samples_.pop_front();
    --samples_stored_;
  }
}

template <typename T>
T MovingPercentileFilter<T>::GetFilteredValue() const {
  return percentile_filter_.GetPercentileValue();
}

template <typename T>
void MovingPercentileFilter<T>::Reset() {
  percentile_filter_.Reset();
  samples_.clear();
  samples_stored_ = 0;
}

template <typename T>
size_t MovingPercentileFilter<T>::GetNumberOfSamplesStored() const {
  return samples_stored_;
}

}  // namespace webrtc
#endif  // RTC_BASE_NUMERICS_MOVING_PERCENTILE_FILTER_H_
