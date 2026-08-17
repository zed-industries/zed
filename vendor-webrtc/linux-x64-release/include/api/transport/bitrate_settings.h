/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TRANSPORT_BITRATE_SETTINGS_H_
#define API_TRANSPORT_BITRATE_SETTINGS_H_

#include <optional>

#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Configuration of send bitrate. The `start_bitrate_bps` value is
// used for multiple purposes, both as a prior in the bandwidth
// estimator, and for initial configuration of the encoder. We may
// want to create separate apis for those, and use a smaller struct
// with only the min and max constraints.
struct RTC_EXPORT BitrateSettings {
  BitrateSettings();
  ~BitrateSettings();
  BitrateSettings(const BitrateSettings&);
  // 0 <= min <= start <= max should hold for set parameters.
  std::optional<int> min_bitrate_bps;
  std::optional<int> start_bitrate_bps;
  std::optional<int> max_bitrate_bps;
};

// TODO(srte): BitrateConstraints and BitrateSettings should be merged.
// Both represent the same kind data, but are using different default
// initializer and representation of unset values.
struct BitrateConstraints {
  int min_bitrate_bps = 0;
  int start_bitrate_bps = kDefaultStartBitrateBps;
  int max_bitrate_bps = -1;

 private:
  static constexpr int kDefaultStartBitrateBps = 300000;
};

}  // namespace webrtc

#endif  // API_TRANSPORT_BITRATE_SETTINGS_H_
