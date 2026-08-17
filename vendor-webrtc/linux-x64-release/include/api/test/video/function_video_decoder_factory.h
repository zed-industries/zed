/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_VIDEO_FUNCTION_VIDEO_DECODER_FACTORY_H_
#define API_TEST_VIDEO_FUNCTION_VIDEO_DECODER_FACTORY_H_

#include <functional>
#include <memory>
#include <utility>
#include <vector>

#include "api/environment/environment.h"
#include "api/video_codecs/sdp_video_format.h"
#include "api/video_codecs/video_decoder.h"
#include "api/video_codecs/video_decoder_factory.h"

namespace webrtc {
namespace test {

// A decoder factory producing decoders by calling a supplied create function.
class FunctionVideoDecoderFactory final : public VideoDecoderFactory {
 public:
  explicit FunctionVideoDecoderFactory(
      std::function<std::unique_ptr<VideoDecoder>()> create)
      : create_([create = std::move(create)](const Environment&,
                                             const SdpVideoFormat&) {
          return create();
        }) {}
  explicit FunctionVideoDecoderFactory(
      std::function<std::unique_ptr<VideoDecoder>(const Environment&,
                                                  const SdpVideoFormat&)>
          create)
      : create_(std::move(create)) {}
  FunctionVideoDecoderFactory(
      std::function<std::unique_ptr<VideoDecoder>()> create,
      std::vector<SdpVideoFormat> sdp_video_formats)
      : create_([create = std::move(create)](const Environment&,
                                             const SdpVideoFormat&) {
          return create();
        }),
        sdp_video_formats_(std::move(sdp_video_formats)) {}

  std::vector<SdpVideoFormat> GetSupportedFormats() const override {
    return sdp_video_formats_;
  }

  std::unique_ptr<VideoDecoder> Create(const Environment& env,
                                       const SdpVideoFormat& format) override {
    return create_(env, format);
  }

 private:
  const std::function<std::unique_ptr<VideoDecoder>(const Environment& env,
                                                    const SdpVideoFormat&)>
      create_;
  const std::vector<SdpVideoFormat> sdp_video_formats_;
};

}  // namespace test
}  // namespace webrtc

#endif  // API_TEST_VIDEO_FUNCTION_VIDEO_DECODER_FACTORY_H_
