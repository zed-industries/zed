/*
 * Copyright 2024 The WebRTC project authors. All rights reserved.
 *
 * Use of this source code is governed by a BSD-style license
 * that can be found in the LICENSE file in the root of the source
 * tree. An additional intellectual property rights grant can be found
 * in the file PATENTS.  All contributing project authors may
 * be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_CORRUPTION_DETECTION_GENERIC_MAPPING_FUNCTIONS_H_
#define VIDEO_CORRUPTION_DETECTION_GENERIC_MAPPING_FUNCTIONS_H_

#include "api/video/corruption_detection_filter_settings.h"
#include "api/video/video_codec_type.h"

namespace webrtc {

// TODO: bugs.webrtc.org/358039777 - Remove when downstream usage is gone.
using FilterSettings = CorruptionDetectionFilterSettings;

CorruptionDetectionFilterSettings GetCorruptionFilterSettings(
    int qp,
    VideoCodecType codec_type);

}  // namespace webrtc

#endif  // VIDEO_CORRUPTION_DETECTION_GENERIC_MAPPING_FUNCTIONS_H_
