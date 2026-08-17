/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_LIBAOM_AV1_ENCODER_FACTORY_H_
#define API_VIDEO_CODECS_LIBAOM_AV1_ENCODER_FACTORY_H_

#include <map>
#include <memory>
#include <string>

#include "api/video_codecs/video_encoder_factory_interface.h"
#include "api/video_codecs/video_encoder_interface.h"

namespace webrtc {
class LibaomAv1EncoderFactory final : VideoEncoderFactoryInterface {
 public:
  std::string CodecName() const override;
  std::string ImplementationName() const override;
  std::map<std::string, std::string> CodecSpecifics() const override;

  Capabilities GetEncoderCapabilities() const override;
  std::unique_ptr<VideoEncoderInterface> CreateEncoder(
      const StaticEncoderSettings& settings,
      const std::map<std::string, std::string>& encoder_specific_settings)
      override;
};
}  // namespace webrtc
#endif  // API_VIDEO_CODECS_LIBAOM_AV1_ENCODER_FACTORY_H_
