/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_CODECS_VP8_VP8_SCALABILITY_H_
#define MODULES_VIDEO_CODING_CODECS_VP8_VP8_SCALABILITY_H_

#include "api/video_codecs/scalability_mode.h"

namespace webrtc {

inline constexpr ScalabilityMode kVP8SupportedScalabilityModes[] = {
    ScalabilityMode::kL1T1, ScalabilityMode::kL1T2, ScalabilityMode::kL1T3};
bool VP8SupportsScalabilityMode(ScalabilityMode scalability_mode);

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_CODECS_VP8_VP8_SCALABILITY_H_
