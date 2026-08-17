/*
 * Copyright 2024 The WebRTC project authors. All rights reserved.
 *
 * Use of this source code is governed by a BSD-style license
 * that can be found in the LICENSE file in the root of the source
 * tree. An additional intellectual property rights grant can be found
 * in the file PATENTS.  All contributing project authors may
 * be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_VIDEO_INCLUDE_CORRUPTION_SCORE_CALCULATOR_H_
#define COMMON_VIDEO_INCLUDE_CORRUPTION_SCORE_CALCULATOR_H_

#include <optional>

#include "api/video/video_frame.h"
#include "common_video/frame_instrumentation_data.h"

namespace webrtc {

// Allow classes to have their own implementations of how to calculate a score
// for automatic corruption detection.
class CorruptionScoreCalculator {
 public:
  virtual ~CorruptionScoreCalculator() = default;

  virtual std::optional<double> CalculateCorruptionScore(
      const VideoFrame& frame,
      const FrameInstrumentationData& frame_instrumentation_data) = 0;
};

}  // namespace webrtc

#endif  // COMMON_VIDEO_INCLUDE_CORRUPTION_SCORE_CALCULATOR_H_
