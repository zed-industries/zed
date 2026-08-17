/*
 * Copyright 2024 The WebRTC project authors. All rights reserved.
 *
 * Use of this source code is governed by a BSD-style license
 * that can be found in the LICENSE file in the root of the source
 * tree. An additional intellectual property rights grant can be found
 * in the file PATENTS.  All contributing project authors may
 * be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_CORRUPTION_DETECTION_FRAME_INSTRUMENTATION_EVALUATION_H_
#define VIDEO_CORRUPTION_DETECTION_FRAME_INSTRUMENTATION_EVALUATION_H_

#include <optional>

#include "api/video/video_frame.h"
#include "common_video/frame_instrumentation_data.h"

namespace webrtc {

std::optional<double> GetCorruptionScore(const FrameInstrumentationData& data,
                                         const VideoFrame& frame);

}  // namespace webrtc

#endif  // VIDEO_CORRUPTION_DETECTION_FRAME_INSTRUMENTATION_EVALUATION_H_
