/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_INCLUDE_VIDEO_CODEC_INITIALIZER_H_
#define MODULES_VIDEO_CODING_INCLUDE_VIDEO_CODEC_INITIALIZER_H_

#include <vector>

#include "api/field_trials_view.h"
#include "api/video_codecs/video_codec.h"
#include "video/config/video_encoder_config.h"

namespace webrtc {

class VideoCodecInitializer {
 public:
  // Takes a VideoEncoderConfig and the VideoStream configuration and
  // translates them into the old school VideoCodec type.
  static VideoCodec SetupCodec(const FieldTrialsView& field_trials,
                               const VideoEncoderConfig& config,
                               const std::vector<VideoStream>& streams);
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_INCLUDE_VIDEO_CODEC_INITIALIZER_H_
