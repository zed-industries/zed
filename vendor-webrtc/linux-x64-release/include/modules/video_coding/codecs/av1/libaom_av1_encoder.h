/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_VIDEO_CODING_CODECS_AV1_LIBAOM_AV1_ENCODER_H_
#define MODULES_VIDEO_CODING_CODECS_AV1_LIBAOM_AV1_ENCODER_H_

#include <map>
#include <memory>

#include "absl/base/nullability.h"
#include "api/environment/environment.h"
#include "api/video_codecs/video_encoder.h"

namespace webrtc {

struct LibaomAv1EncoderSettings {
  // A map of max pixel count --> cpu speed.
  std::map<int, int> max_pixel_count_to_cpu_speed;
};
absl_nonnull std::unique_ptr<VideoEncoder> CreateLibaomAv1Encoder(
    const Environment& env,
    LibaomAv1EncoderSettings settings = {});

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_CODECS_AV1_LIBAOM_AV1_ENCODER_H_
