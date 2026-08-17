/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_UTILITY_SIMULCAST_UTILITY_H_
#define MODULES_VIDEO_CODING_UTILITY_SIMULCAST_UTILITY_H_

#include <stdint.h>

#include "api/video_codecs/video_codec.h"
#include "rtc_base/system/rtc_export.h"
#include "video/config/video_encoder_config.h"

namespace webrtc {

class RTC_EXPORT SimulcastUtility {
 public:
  static uint32_t SumStreamMaxBitrate(int streams, const VideoCodec& codec);
  static int NumberOfSimulcastStreams(const VideoCodec& codec);
  static bool ValidSimulcastParameters(const VideoCodec& codec,
                                       int num_streams);
  static int NumberOfTemporalLayers(const VideoCodec& codec, int spatial_id);
  // TODO(sprang): Remove this hack when ScreenshareLayers is gone.
  static bool IsConferenceModeScreenshare(const VideoCodec& codec);
  static bool IsConferenceModeScreenshare(
      const VideoEncoderConfig& encoder_config);
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_UTILITY_SIMULCAST_UTILITY_H_
