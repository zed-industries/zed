/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_NUMERICS_SAMPLES_STATS_COUNTER_H_
#define API_NUMERICS_SAMPLES_STATS_COUNTER_H_

#include <stddef.h>
#include <stdint.h>

#include <map>
#include <string>
#include <vector>

#include "api/array_view.h"
#include "api/units/timestamp.h"
#include "rtc_base/checks.h"
#include "rtc_base/numerics/running_statistics.h"

namespace webrtc {

// This class extends RunningStatistics by providing GetPercentile() method,
// while slightly adapting the interface.
class SamplesStatsCounter {
 public:
  struct StatsSample {
    double value;
    Timestamp time;
    // Sample's specific metadata.
    std::map<std::string, std::string> metadata;
  };

  SamplesStatsCounter();
  explicit SamplesStatsCounter(size_t expected_samples_count);
  ~SamplesStatsCounter();
  SamplesStatsCounter(const SamplesStatsCounter&);
  SamplesStatsCounter& operator=(const SamplesStatsCounter&);
  SamplesStatsCounter(SamplesStatsCounter&&);
  SamplesStatsCounter& operator=(SamplesStatsCounter&&);

  // Adds sample to the stats in amortized O(1) time.
  void AddSample(double value);
  void AddSample(StatsSample sample);

  // Adds samples from another counter.
  void AddSamples(const SamplesStatsCounter& other);

  // Returns if there are any values in O(1) time.
  bool IsEmpty() const { return samples_.empty(); }
  // Returns the amount of samples added into counter in O(1) time.
  int64_t NumSamples() const { return stats_.Size(); }

  // Returns min in O(1) time. This function may not be called if there are no
  // samples.
  double GetMin() const {
    RTC_DCHECK(!IsEmpty());
    return *stats_.GetMin();
  }
  // Returns max in O(1) time. This function may not be called if there are no
  // samples.
  double GetMax() const {
    RTC_DCHECK(!IsEmpty());
    return *stats_.GetMax();
  }
  // Returns sum in O(1) time. This function may not be called if there are
  // no samples.
  double GetSum() const {
    RTC_DCHECK(!IsEmpty());
    return *stats_.GetSum();
  }
  // Returns average in O(1) time. This function may not be called if there are
  // no samples.
  double GetAverage() const {
    RTC_DCHECK(!IsEmpty());
    return *stats_.GetMean();
  }
  // Returns variance in O(1) time. This function may not be called if there are
  // no samples.
  double GetVariance() const {
    RTC_DCHECK(!IsEmpty());
    return *stats_.GetVariance();
  }
  // Returns standard deviation in O(1) time. This function may not be called if
  // there are no samples.
  double GetStandardDeviation() const {
    RTC_DCHECK(!IsEmpty());
    return *stats_.GetStandardDeviation();
  }
  // Returns percentile in O(nlogn) on first call and in O(1) after, if no
  // additions were done. This function may not be called if there are no
  // samples.
  //
  // `percentile` has to be in [0; 1]. 0 percentile is the min in the array and
  // 1 percentile is the max in the array.
  double GetPercentile(double percentile);
  // Returns array view with all samples added into counter. There are no
  // guarantees of order, so samples can be in different order comparing to in
  // which they were added into counter. Also return value will be invalidate
  // after call to any non const method.
  ArrayView<const StatsSample> GetTimedSamples() const { return samples_; }
  std::vector<double> GetSamples() const {
    std::vector<double> out;
    out.reserve(samples_.size());
    for (const auto& sample : samples_) {
      out.push_back(sample.value);
    }
    return out;
  }

 private:
  webrtc_impl::RunningStatistics<double> stats_;
  std::vector<StatsSample> samples_;
  bool sorted_ = false;
};

// Multiply all sample values on `value` and return new SamplesStatsCounter
// with resulted samples. Doesn't change origin SamplesStatsCounter.
SamplesStatsCounter operator*(const SamplesStatsCounter& counter, double value);
inline SamplesStatsCounter operator*(double value,
                                     const SamplesStatsCounter& counter) {
  return counter * value;
}
// Divide all sample values on `value` and return new SamplesStatsCounter with
// resulted samples. Doesn't change origin SamplesStatsCounter.
SamplesStatsCounter operator/(const SamplesStatsCounter& counter, double value);

}  // namespace webrtc

#endif  // API_NUMERICS_SAMPLES_STATS_COUNTER_H_
