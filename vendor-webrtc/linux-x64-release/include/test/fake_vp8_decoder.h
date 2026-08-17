/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_FAKE_VP8_DECODER_H_
#define TEST_FAKE_VP8_DECODER_H_

#include <stdint.h>

#include "api/video/encoded_image.h"
#include "api/video_codecs/video_decoder.h"
#include "modules/video_coding/include/video_codec_interface.h"

namespace webrtc {
namespace test {

class FakeVp8Decoder : public VideoDecoder {
 public:
  FakeVp8Decoder();
  ~FakeVp8Decoder() override {}

  bool Configure(const Settings& settings) override;

  int32_t Decode(const EncodedImage& input, int64_t render_time_ms) override;

  int32_t RegisterDecodeCompleteCallback(
      DecodedImageCallback* callback) override;

  int32_t Release() override;

  DecoderInfo GetDecoderInfo() const override;
  const char* ImplementationName() const override;

  static constexpr char kImplementationName[] = "fake_vp8_decoder";

 private:
  DecodedImageCallback* callback_;
  int width_;
  int height_;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_FAKE_VP8_DECODER_H_
