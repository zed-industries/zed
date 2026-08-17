/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_RESOLUTION_H_
#define API_VIDEO_RESOLUTION_H_

#include <utility>

#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// A struct representing a video resolution in pixels.
struct RTC_EXPORT Resolution {
  int width = 0;
  int height = 0;

  // Helper methods.
  int PixelCount() const { return width * height; }
  std::pair<int, int> ToPair() const { return std::make_pair(width, height); }
};

inline bool operator==(const Resolution& lhs, const Resolution& rhs) {
  return lhs.width == rhs.width && lhs.height == rhs.height;
}

inline bool operator!=(const Resolution& lhs, const Resolution& rhs) {
  return !(lhs == rhs);
}

}  // namespace webrtc

#endif  // API_VIDEO_RESOLUTION_H_
