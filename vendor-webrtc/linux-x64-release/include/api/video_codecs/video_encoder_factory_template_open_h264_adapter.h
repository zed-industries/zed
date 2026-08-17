/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_VIDEO_ENCODER_FACTORY_TEMPLATE_OPEN_H264_ADAPTER_H_
#define API_VIDEO_CODECS_VIDEO_ENCODER_FACTORY_TEMPLATE_OPEN_H264_ADAPTER_H_

#include <memory>
#include <vector>

#include "api/environment/environment.h"
#include "api/video_codecs/scalability_mode.h"
#include "api/video_codecs/sdp_video_format.h"
#include "api/video_codecs/video_encoder.h"
#include "modules/video_coding/codecs/h264/include/h264.h"

namespace webrtc {
// TODO(bugs.webrtc.org/13573): When OpenH264 is no longer a conditional build
//                              target remove #ifdefs.
struct OpenH264EncoderTemplateAdapter {
  static std::vector<SdpVideoFormat> SupportedFormats() {
#if defined(WEBRTC_USE_H264)
    return SupportedH264Codecs(/*add_scalability_modes=*/true);
#else
    return {};
#endif
  }

  static std::unique_ptr<VideoEncoder> CreateEncoder(
      [[maybe_unused]] const Environment& env,
      [[maybe_unused]] const SdpVideoFormat& format) {
#if defined(WEBRTC_USE_H264)
    return CreateH264Encoder(env, H264EncoderSettings::Parse(format));
#else
    return nullptr;
#endif
  }

  static bool IsScalabilityModeSupported(
      [[maybe_unused]] ScalabilityMode scalability_mode) {
#if defined(WEBRTC_USE_H264)
    return H264Encoder::SupportsScalabilityMode(scalability_mode);
#else
    return false;
#endif
  }
};
}  // namespace webrtc

#endif  // API_VIDEO_CODECS_VIDEO_ENCODER_FACTORY_TEMPLATE_OPEN_H264_ADAPTER_H_
