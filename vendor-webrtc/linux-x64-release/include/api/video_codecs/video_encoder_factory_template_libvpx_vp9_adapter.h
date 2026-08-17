/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_VIDEO_ENCODER_FACTORY_TEMPLATE_LIBVPX_VP9_ADAPTER_H_
#define API_VIDEO_CODECS_VIDEO_ENCODER_FACTORY_TEMPLATE_LIBVPX_VP9_ADAPTER_H_

#include <memory>
#include <vector>

#include "api/environment/environment.h"
#include "api/video_codecs/scalability_mode.h"
#include "api/video_codecs/sdp_video_format.h"
#include "api/video_codecs/video_encoder.h"
#include "api/video_codecs/vp9_profile.h"
#include "modules/video_coding/codecs/vp9/include/vp9.h"

namespace webrtc {
struct LibvpxVp9EncoderTemplateAdapter {
  static std::vector<SdpVideoFormat> SupportedFormats() {
    return SupportedVP9Codecs(/*add_scalability_modes=*/true);
  }

  static std::unique_ptr<VideoEncoder> CreateEncoder(
      const Environment& env,
      const SdpVideoFormat& format) {
    return CreateVp9Encoder(env,
                            {.profile = ParseSdpForVP9Profile(format.parameters)
                                            .value_or(VP9Profile::kProfile0)});
  }

  static bool IsScalabilityModeSupported(ScalabilityMode scalability_mode) {
    return VP9Encoder::SupportsScalabilityMode(scalability_mode);
  }
};
}  // namespace webrtc

#endif  // API_VIDEO_CODECS_VIDEO_ENCODER_FACTORY_TEMPLATE_LIBVPX_VP9_ADAPTER_H_
