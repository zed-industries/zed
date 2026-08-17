/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_NUMERICS_MOVING_MAX_COUNTER_H_
#define RTC_BASE_NUMERICS_MOVING_MAX_COUNTER_H_

#include <stdint.h>

#include <deque>
#include <limits>
#include <optional>
#include <utility>

#include "rtc_base/checks.h"

namespace webrtc {

// Implements moving max: can add samples to it and calculate maximum over some
// fixed moving window.
//
// Window size is configured at constructor.
// Samples can be added with `Add()` and max over current window is returned by
// `MovingMax`. `current_time_ms` in successive calls to Add and MovingMax
// should never decrease as if it's a wallclock time.
template <class T>
class MovingMaxCounter {
 public:
  explicit MovingMaxCounter(int64_t window_length_ms);

  MovingMaxCounter(const MovingMaxCounter&) = delete;
  MovingMaxCounter& operator=(const MovingMaxCounter&) = delete;

  // Advances the current time, and adds a new sample. The new current time must
  // be at least as large as the old current time.
  void Add(const T& sample, int64_t current_time_ms);
  // Advances the current time, and returns the maximum sample in the time
  // window ending at the current time. The new current time must be at least as
  // large as the old current time.
  std::optional<T> Max(int64_t current_time_ms);
  void Reset();

 private:
  // Throws out obsolete samples.
  void RollWindow(int64_t new_time_ms);
  const int64_t window_length_ms_;
  // This deque stores (timestamp, sample) pairs in chronological order; new
  // pairs are only ever added at the end. However, because they can't affect
  // the Max() calculation, pairs older than window_length_ms_ are discarded,
  // and if an older pair has a sample that's smaller than that of a younger
  // pair, the older pair is discarded. As a result, the sequence of timestamps
  // is strictly increasing, and the sequence of samples is strictly decreasing.
  std::deque<std::pair<int64_t, T>> samples_;
#if RTC_DCHECK_IS_ON
  int64_t last_call_time_ms_ = std::numeric_limits<int64_t>::min();
#endif
};

template <class T>
MovingMaxCounter<T>::MovingMaxCounter(int64_t window_length_ms)
    : window_length_ms_(window_length_ms) {}

template <class T>
void MovingMaxCounter<T>::Add(const T& sample, int64_t current_time_ms) {
  RollWindow(current_time_ms);
  // Remove samples that will never be maximum in any window: newly added sample
  // will always be in all windows the previous samples are. Thus, smaller or
  // equal samples could be removed. This will maintain the invariant - deque
  // contains strictly decreasing sequence of values.
  while (!samples_.empty() && samples_.back().second <= sample) {
    samples_.pop_back();
  }
  // Add the new sample but only if there's no existing sample at the same time.
  // Due to checks above, the already existing element will be larger, so the
  // new sample will never be the maximum in any window.
  if (samples_.empty() || samples_.back().first < current_time_ms) {
    samples_.emplace_back(std::make_pair(current_time_ms, sample));
  }
}

template <class T>
std::optional<T> MovingMaxCounter<T>::Max(int64_t current_time_ms) {
  RollWindow(current_time_ms);
  std::optional<T> res;
  if (!samples_.empty()) {
    res.emplace(samples_.front().second);
  }
  return res;
}

template <class T>
void MovingMaxCounter<T>::Reset() {
  samples_.clear();
}

template <class T>
void MovingMaxCounter<T>::RollWindow(int64_t new_time_ms) {
#if RTC_DCHECK_IS_ON
  RTC_DCHECK_GE(new_time_ms, last_call_time_ms_);
  last_call_time_ms_ = new_time_ms;
#endif
  const int64_t window_begin_ms = new_time_ms - window_length_ms_;
  auto it = samples_.begin();
  while (it != samples_.end() && it->first < window_begin_ms) {
    ++it;
  }
  samples_.erase(samples_.begin(), it);
}

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::MovingMaxCounter;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_NUMERICS_MOVING_MAX_COUNTER_H_
