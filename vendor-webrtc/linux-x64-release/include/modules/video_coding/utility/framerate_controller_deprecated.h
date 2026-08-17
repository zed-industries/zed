/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_UTILITY_FRAMERATE_CONTROLLER_DEPRECATED_H_
#define MODULES_VIDEO_CODING_UTILITY_FRAMERATE_CONTROLLER_DEPRECATED_H_

#include <stdint.h>

#include <optional>

#include "rtc_base/rate_statistics.h"

namespace webrtc {

// Please use webrtc::FramerateController instead.
class FramerateControllerDeprecated {
 public:
  explicit FramerateControllerDeprecated(float target_framerate_fps);

  void SetTargetRate(float target_framerate_fps);
  float GetTargetRate();

  // Advices user to drop next frame in order to reach target framerate.
  bool DropFrame(uint32_t timestamp_ms) const;

  void AddFrame(uint32_t timestamp_ms);

  void Reset();

 private:
  std::optional<float> Rate(uint32_t timestamp_ms) const;

  std::optional<float> target_framerate_fps_;
  std::optional<uint32_t> last_timestamp_ms_;
  uint32_t min_frame_interval_ms_;
  RateStatistics framerate_estimator_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_UTILITY_FRAMERATE_CONTROLLER_DEPRECATED_H_
