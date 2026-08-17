/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_VIDEO_DECODER_FACTORY_TEMPLATE_OPEN_H264_ADAPTER_H_
#define API_VIDEO_CODECS_VIDEO_DECODER_FACTORY_TEMPLATE_OPEN_H264_ADAPTER_H_

#include <memory>
#include <vector>

#include "api/video_codecs/sdp_video_format.h"
#include "api/video_codecs/video_decoder.h"
#include "modules/video_coding/codecs/h264/include/h264.h"

namespace webrtc {
// TODO(bugs.webrtc.org/13573): When OpenH264 is no longer a conditional build
//                              target remove #ifdefs.
struct OpenH264DecoderTemplateAdapter {
  static std::vector<SdpVideoFormat> SupportedFormats() {
#if defined(WEBRTC_USE_H264)

    return SupportedH264DecoderCodecs();
#else
    return {};
#endif
  }

  static std::unique_ptr<VideoDecoder> CreateDecoder(
      const SdpVideoFormat& /* format */) {
#if defined(WEBRTC_USE_H264)

    return H264Decoder::Create();
#else
    return nullptr;
#endif
  }
};
}  // namespace webrtc

#endif  // API_VIDEO_CODECS_VIDEO_DECODER_FACTORY_TEMPLATE_OPEN_H264_ADAPTER_H_
