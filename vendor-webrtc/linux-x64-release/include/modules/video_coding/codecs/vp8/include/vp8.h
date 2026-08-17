/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_CODECS_VP8_INCLUDE_VP8_H_
#define MODULES_VIDEO_CODING_CODECS_VP8_INCLUDE_VP8_H_

#include <memory>
#include <vector>

#include "absl/base/nullability.h"
#include "api/environment/environment.h"
#include "api/video_codecs/video_decoder.h"
#include "api/video_codecs/video_encoder.h"

namespace webrtc {

struct Vp8EncoderSettings {
  // Allows for overriding the resolution/bitrate limits exposed through
  // VideoEncoder::GetEncoderInfo(). No override is done if empty.
  std::vector<VideoEncoder::ResolutionBitrateLimits> resolution_bitrate_limits;
};
absl_nonnull std::unique_ptr<VideoEncoder> CreateVp8Encoder(
    const Environment& env,
    Vp8EncoderSettings settings = {});

std::unique_ptr<VideoDecoder> CreateVp8Decoder(const Environment& env);

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_CODECS_VP8_INCLUDE_VP8_H_
