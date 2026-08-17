/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_NUMERICS_MOVING_AVERAGE_H_
#define RTC_BASE_NUMERICS_MOVING_AVERAGE_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>
#include <vector>

namespace webrtc {

// Calculates average over fixed size window. If there are less than window
// size elements, calculates average of all inserted so far elements.
//
class MovingAverage {
 public:
  // Maximum supported window size is 2^32 - 1.
  explicit MovingAverage(size_t window_size);
  ~MovingAverage();
  // MovingAverage is neither copyable nor movable.
  MovingAverage(const MovingAverage&) = delete;
  MovingAverage& operator=(const MovingAverage&) = delete;

  // Adds new sample. If the window is full, the oldest element is pushed out.
  void AddSample(int sample);

  // Returns rounded down average of last `window_size` elements or all
  // elements if there are not enough of them. Returns nullopt if there were
  // no elements added.
  std::optional<int> GetAverageRoundedDown() const;

  // Same as above but rounded to the closest integer.
  std::optional<int> GetAverageRoundedToClosest() const;

  // Returns unrounded average over the window.
  std::optional<double> GetUnroundedAverage() const;

  // Resets to the initial state before any elements were added.
  void Reset();

  // Returns number of elements in the window.
  size_t Size() const;

 private:
  // Total number of samples added to the class since last reset.
  size_t count_ = 0;
  // Sum of the samples in the moving window.
  int64_t sum_ = 0;
  // Circular buffer for all the samples in the moving window.
  // Size is always `window_size`
  std::vector<int> history_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::MovingAverage;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES
#endif  // RTC_BASE_NUMERICS_MOVING_AVERAGE_H_
