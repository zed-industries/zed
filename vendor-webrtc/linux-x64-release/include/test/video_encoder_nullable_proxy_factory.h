/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_VIDEO_ENCODER_NULLABLE_PROXY_FACTORY_H_
#define TEST_VIDEO_ENCODER_NULLABLE_PROXY_FACTORY_H_

#include <memory>
#include <vector>

#include "api/environment/environment.h"
#include "api/video_codecs/video_encoder.h"
#include "api/video_codecs/video_encoder_factory.h"
#include "test/video_encoder_proxy_factory.h"

namespace webrtc {
namespace test {

class VideoEncoderNullableProxyFactory final : public VideoEncoderProxyFactory {
 public:
  explicit VideoEncoderNullableProxyFactory(
      VideoEncoder* encoder,
      EncoderSelectorInterface* encoder_selector)
      : VideoEncoderProxyFactory(encoder, encoder_selector) {}

  ~VideoEncoderNullableProxyFactory() override = default;

  std::unique_ptr<VideoEncoder> Create(const Environment& env,
                                       const SdpVideoFormat& format) override {
    if (!encoder_) {
      return nullptr;
    }
    return VideoEncoderProxyFactory::Create(env, format);
  }
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_VIDEO_ENCODER_NULLABLE_PROXY_FACTORY_H_
