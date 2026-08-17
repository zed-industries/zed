/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 *
 */

#ifndef MODULES_VIDEO_CODING_CODECS_VP9_INCLUDE_VP9_H_
#define MODULES_VIDEO_CODING_CODECS_VP9_INCLUDE_VP9_H_

#include <memory>
#include <vector>

#include "absl/base/nullability.h"
#include "api/environment/environment.h"
#include "api/video_codecs/scalability_mode.h"
#include "api/video_codecs/sdp_video_format.h"
#include "api/video_codecs/video_decoder.h"
#include "api/video_codecs/video_encoder.h"
#include "api/video_codecs/vp9_profile.h"

namespace webrtc {

// Returns a vector with all supported internal VP9 profiles that we can
// negotiate in SDP, in order of preference.
std::vector<SdpVideoFormat> SupportedVP9Codecs(
    bool add_scalability_modes = false);

// Returns a vector with all supported internal VP9 decode profiles in order of
// preference. These will be availble for receive-only connections.
std::vector<SdpVideoFormat> SupportedVP9DecoderCodecs();

struct Vp9EncoderSettings {
  VP9Profile profile = VP9Profile::kProfile0;
};
absl_nonnull std::unique_ptr<VideoEncoder> CreateVp9Encoder(
    const Environment& env,
    Vp9EncoderSettings settings = {});

class VP9Encoder {
 public:
  static bool SupportsScalabilityMode(ScalabilityMode scalability_mode);
};

class VP9Decoder : public VideoDecoder {
 public:
  static std::unique_ptr<VP9Decoder> Create();

  ~VP9Decoder() override {}
};
}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_CODECS_VP9_INCLUDE_VP9_H_
