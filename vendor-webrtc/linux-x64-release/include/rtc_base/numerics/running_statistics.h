/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_NUMERICS_RUNNING_STATISTICS_H_
#define API_NUMERICS_RUNNING_STATISTICS_H_

#include <algorithm>
#include <cmath>
#include <cstdint>
#include <optional>

#include "rtc_base/checks.h"
#include "rtc_base/numerics/math_utils.h"

namespace webrtc {
namespace webrtc_impl {

// tl;dr: Robust and efficient online computation of statistics,
//        using Welford's method for variance. [1]
//
// This should be your go-to class if you ever need to compute
// min, max, mean, variance and standard deviation.
// If you need to get percentiles, please use webrtc::SamplesStatsCounter.
//
// Please note RemoveSample() won't affect min and max.
// If you want a full-fledged moving window over N last samples,
// please use webrtc::RollingAccumulator.
//
// The measures return std::nullopt if no samples were fed (Size() == 0),
// otherwise the returned optional is guaranteed to contain a value.
//
// [1]
// https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm

// The type T is a scalar which must be convertible to double.
// Rationale: we often need greater precision for measures
//            than for the samples themselves.
template <typename T>
class RunningStatistics {
 public:
  // Update stats ////////////////////////////////////////////

  // Add a value participating in the statistics in O(1) time.
  void AddSample(T sample) {
    max_ = std::max(max_, sample);
    min_ = std::min(min_, sample);
    sum_ += sample;
    ++size_;
    // Welford's incremental update.
    const double delta = sample - mean_;
    mean_ += delta / size_;
    const double delta2 = sample - mean_;
    cumul_ += delta * delta2;
  }

  // Remove a previously added value in O(1) time.
  // Nb: This doesn't affect min or max.
  // Calling RemoveSample when Size()==0 is incorrect.
  void RemoveSample(T sample) {
    RTC_DCHECK_GT(Size(), 0);
    // In production, just saturate at 0.
    if (Size() == 0) {
      return;
    }
    // Since samples order doesn't matter, this is the
    // exact reciprocal of Welford's incremental update.
    --size_;
    const double delta = sample - mean_;
    mean_ -= delta / size_;
    const double delta2 = sample - mean_;
    cumul_ -= delta * delta2;
  }

  // Merge other stats, as if samples were added one by one, but in O(1).
  void MergeStatistics(const RunningStatistics<T>& other) {
    if (other.size_ == 0) {
      return;
    }
    max_ = std::max(max_, other.max_);
    min_ = std::min(min_, other.min_);
    const int64_t new_size = size_ + other.size_;
    const double new_mean =
        (mean_ * size_ + other.mean_ * other.size_) / new_size;
    // Each cumulant must be corrected.
    //   * from: sum((x_i - mean_)²)
    //   * to:   sum((x_i - new_mean)²)
    auto delta = [new_mean](const RunningStatistics<T>& stats) {
      return stats.size_ * (new_mean * (new_mean - 2 * stats.mean_) +
                            stats.mean_ * stats.mean_);
    };
    cumul_ = cumul_ + delta(*this) + other.cumul_ + delta(other);
    mean_ = new_mean;
    size_ = new_size;
  }

  // Get Measures ////////////////////////////////////////////

  // Returns number of samples involved via AddSample() or MergeStatistics(),
  // minus number of times RemoveSample() was called.
  int64_t Size() const { return size_; }

  // Returns minimum among all seen samples, in O(1) time.
  // This isn't affected by RemoveSample().
  std::optional<T> GetMin() const {
    if (size_ == 0) {
      return std::nullopt;
    }
    return min_;
  }

  // Returns maximum among all seen samples, in O(1) time.
  // This isn't affected by RemoveSample().
  std::optional<T> GetMax() const {
    if (size_ == 0) {
      return std::nullopt;
    }
    return max_;
  }

  // Returns sum in O(1) time.
  std::optional<double> GetSum() const {
    if (size_ == 0) {
      return std::nullopt;
    }
    return sum_;
  }

  // Returns mean in O(1) time.
  std::optional<double> GetMean() const {
    if (size_ == 0) {
      return std::nullopt;
    }
    return mean_;
  }

  // Returns unbiased sample variance in O(1) time.
  std::optional<double> GetVariance() const {
    if (size_ == 0) {
      return std::nullopt;
    }
    return cumul_ / size_;
  }

  // Returns unbiased standard deviation in O(1) time.
  std::optional<double> GetStandardDeviation() const {
    if (size_ == 0) {
      return std::nullopt;
    }
    return std::sqrt(*GetVariance());
  }

 private:
  int64_t size_ = 0;  // Samples seen.
  T min_ = infinity_or_max<T>();
  T max_ = minus_infinity_or_min<T>();
  double mean_ = 0;
  double cumul_ = 0;  // Variance * size_, sometimes noted m2.
  double sum_ = 0;
};

}  // namespace webrtc_impl
}  // namespace webrtc

#endif  // API_NUMERICS_RUNNING_STATISTICS_H_
