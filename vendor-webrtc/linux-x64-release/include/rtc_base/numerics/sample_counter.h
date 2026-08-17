/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_NUMERICS_SAMPLE_COUNTER_H_
#define RTC_BASE_NUMERICS_SAMPLE_COUNTER_H_

#include <stdint.h>

#include <optional>

namespace webrtc {

// Simple utility class for counting basic statistics (max./avg./variance) on
// stream of samples.
class SampleCounter {
 public:
  SampleCounter();
  ~SampleCounter();
  void Add(int sample);
  std::optional<int> Avg(int64_t min_required_samples) const;
  std::optional<int> Max() const;
  std::optional<int> Min() const;
  std::optional<int64_t> Sum(int64_t min_required_samples) const;
  int64_t NumSamples() const;
  void Reset();
  // Adds all the samples from the `other` SampleCounter as if they were all
  // individually added using `Add(int)` method.
  void Add(const SampleCounter& other);

 protected:
  int64_t sum_ = 0;
  int64_t num_samples_ = 0;
  std::optional<int> max_;
  std::optional<int> min_;
};

class SampleCounterWithVariance : public SampleCounter {
 public:
  SampleCounterWithVariance();
  ~SampleCounterWithVariance();
  void Add(int sample);
  std::optional<int64_t> Variance(int64_t min_required_samples) const;
  void Reset();
  // Adds all the samples from the `other` SampleCounter as if they were all
  // individually added using `Add(int)` method.
  void Add(const SampleCounterWithVariance& other);

 private:
  int64_t sum_squared_ = 0;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::SampleCounter;
using ::webrtc::SampleCounterWithVariance;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES
#endif  // RTC_BASE_NUMERICS_SAMPLE_COUNTER_H_
