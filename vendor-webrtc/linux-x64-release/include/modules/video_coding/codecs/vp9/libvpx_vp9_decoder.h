/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 *
 */

#ifndef MODULES_VIDEO_CODING_CODECS_VP9_LIBVPX_VP9_DECODER_H_
#define MODULES_VIDEO_CODING_CODECS_VP9_LIBVPX_VP9_DECODER_H_

#ifdef RTC_ENABLE_VP9

#include <cstdint>

#include "api/video/color_space.h"
#include "api/video/encoded_image.h"
#include "api/video_codecs/video_decoder.h"
#include "modules/video_coding/codecs/vp9/include/vp9.h"
#include "modules/video_coding/codecs/vp9/vp9_frame_buffer_pool.h"
#include "vpx/vpx_codec.h"
#include "vpx/vpx_image.h"

namespace webrtc {

class LibvpxVp9Decoder : public VP9Decoder {
 public:
  LibvpxVp9Decoder();
  virtual ~LibvpxVp9Decoder();

  bool Configure(const Settings& settings) override;

  int Decode(const EncodedImage& input_image,
             int64_t /*render_time_ms*/) override;

  int RegisterDecodeCompleteCallback(DecodedImageCallback* callback) override;

  int Release() override;

  DecoderInfo GetDecoderInfo() const override;
  const char* ImplementationName() const override;

 private:
  int ReturnFrame(const vpx_image_t* img,
                  uint32_t timestamp,
                  int qp,
                  const webrtc::ColorSpace* explicit_color_space);

  // Memory pool used to share buffers between libvpx and webrtc.
  Vp9FrameBufferPool libvpx_buffer_pool_;
  DecodedImageCallback* decode_complete_callback_;
  bool inited_;
  vpx_codec_ctx_t* decoder_;
  bool key_frame_required_;
  Settings current_settings_;
};
}  // namespace webrtc

#endif  // RTC_ENABLE_VP9

#endif  // MODULES_VIDEO_CODING_CODECS_VP9_LIBVPX_VP9_DECODER_H_
