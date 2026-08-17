/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_BITRATE_TRACKER_H_
#define RTC_BASE_BITRATE_TRACKER_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>

#include "api/units/data_rate.h"
#include "api/units/data_size.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "rtc_base/rate_statistics.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {
// Class to estimate bitrates over running window.
// Timestamps used in Update(), Rate() and SetWindowSize() must never
// decrease for two consecutive calls.
// This class is thread unsafe.
class RTC_EXPORT BitrateTracker {
 public:
  // max_window_sizes = Maximum window size for the rate estimation.
  //                    Initial window size is set to this, but may be changed
  //                    to something lower by calling SetWindowSize().
  explicit BitrateTracker(TimeDelta max_window_size);

  BitrateTracker(const BitrateTracker&) = default;
  BitrateTracker(BitrateTracker&&) = default;
  BitrateTracker& operator=(const BitrateTracker&) = delete;
  BitrateTracker& operator=(BitrateTracker&&) = delete;

  ~BitrateTracker() = default;

  // Resets instance to original state.
  void Reset() { impl_.Reset(); }

  // Updates bitrate with a new data point, moving averaging window as needed.
  void Update(int64_t bytes, Timestamp now);
  void Update(DataSize size, Timestamp now) { Update(size.bytes(), now); }

  // Returns bitrate, moving averaging window as needed.
  // Returns nullopt when bitrate can't be measured.
  std::optional<DataRate> Rate(Timestamp now) const;

  // Update the size of the averaging window. The maximum allowed value for
  // `window_size` is `max_window_size` as supplied in the constructor.
  bool SetWindowSize(TimeDelta window_size, Timestamp now);

 private:
  RateStatistics impl_;
};
}  // namespace webrtc

#endif  // RTC_BASE_BITRATE_TRACKER_H_
