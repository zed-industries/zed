/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_NUMERICS_HISTOGRAM_PERCENTILE_COUNTER_H_
#define RTC_BASE_NUMERICS_HISTOGRAM_PERCENTILE_COUNTER_H_

#include <stddef.h>
#include <stdint.h>

#include <map>
#include <optional>
#include <vector>

namespace webrtc {
// Calculates percentiles on the stream of data. Use `Add` methods to add new
// values. Use `GetPercentile` to get percentile of the currently added values.
class HistogramPercentileCounter {
 public:
  // Values below `long_tail_boundary` are stored as the histogram in an array.
  // Values above - in a map.
  explicit HistogramPercentileCounter(uint32_t long_tail_boundary);
  ~HistogramPercentileCounter();
  void Add(uint32_t value);
  void Add(uint32_t value, size_t count);
  void Add(const HistogramPercentileCounter& other);
  // Argument should be from 0 to 1.
  std::optional<uint32_t> GetPercentile(float fraction);

 private:
  std::vector<size_t> histogram_low_;
  std::map<uint32_t, size_t> histogram_high_;
  const uint32_t long_tail_boundary_;
  size_t total_elements_;
  size_t total_elements_low_;
};
}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::HistogramPercentileCounter;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES
#endif  // RTC_BASE_NUMERICS_HISTOGRAM_PERCENTILE_COUNTER_H_
