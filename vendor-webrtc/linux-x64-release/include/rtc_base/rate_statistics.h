/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_RATE_STATISTICS_H_
#define RTC_BASE_RATE_STATISTICS_H_

#include <stddef.h>
#include <stdint.h>

#include <deque>
#include <memory>
#include <optional>

#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Class to estimate rates based on counts in a sequence of 1-millisecond
// intervals.

// This class uses int64 for all its numbers because some rates can be very
// high; for instance, a 20 Mbit/sec video stream can wrap a 32-bit byte
// counter in 14 minutes.

// Note that timestamps used in Update(), Rate() and SetWindowSize() must never
// decrease for two consecutive calls.
// TODO(bugs.webrtc.org/11600): Migrate from int64_t to Timestamp.

class RTC_EXPORT RateStatistics {
 public:
  static constexpr float kBpsScale = 8000.0f;

  // max_window_size_ms = Maximum window size in ms for the rate estimation.
  //                      Initial window size is set to this, but may be changed
  //                      to something lower by calling SetWindowSize().
  // scale = coefficient to convert counts/ms to desired unit
  //         ex: kBpsScale (8000) for bits/s if count represents bytes.
  RateStatistics(int64_t max_window_size_ms, float scale);

  RateStatistics(const RateStatistics& other);

  RateStatistics(RateStatistics&& other);

  ~RateStatistics();

  // Reset instance to original state.
  void Reset();

  // Update rate with a new data point, moving averaging window as needed.
  void Update(int64_t count, int64_t now_ms);

  // Note that despite this being a const method, it still updates the internal
  // state (moves averaging window), but it doesn't make any alterations that
  // are observable from the other methods, as long as supplied timestamps are
  // from a monotonic clock. Ie, it doesn't matter if this call moves the
  // window, since any subsequent call to Update or Rate would still have moved
  // the window as much or more.
  std::optional<int64_t> Rate(int64_t now_ms) const;

  // Update the size of the averaging window. The maximum allowed value for
  // window_size_ms is max_window_size_ms as supplied in the constructor.
  bool SetWindowSize(int64_t window_size_ms, int64_t now_ms);

 private:
  void EraseOld(int64_t now_ms);

  struct Bucket {
    explicit Bucket(int64_t timestamp);
    int64_t sum;              // Sum of all samples in this bucket.
    int num_samples;          // Number of samples in this bucket.
    const int64_t timestamp;  // Timestamp this bucket corresponds to.
  };
  // All buckets within the time window, ordered by time.
  std::deque<Bucket> buckets_;

  // Total count recorded in all buckets.
  int64_t accumulated_count_;

  // Timestamp of the first data point seen, or -1 of none seen.
  int64_t first_timestamp_;

  // True if accumulated_count_ has ever grown too large to be
  // contained in its integer type.
  bool overflow_ = false;

  // The total number of samples in the buckets.
  int num_samples_;

  // To convert counts/ms to desired units
  const float scale_;

  // The window sizes, in ms, over which the rate is calculated.
  const int64_t max_window_size_ms_;
  int64_t current_window_size_ms_;
};
}  // namespace webrtc

#endif  // RTC_BASE_RATE_STATISTICS_H_
