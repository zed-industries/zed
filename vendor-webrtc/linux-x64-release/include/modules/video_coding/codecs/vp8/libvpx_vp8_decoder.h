/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_CODECS_VP8_LIBVPX_VP8_DECODER_H_
#define MODULES_VIDEO_CODING_CODECS_VP8_LIBVPX_VP8_DECODER_H_

#include <cstdint>
#include <memory>
#include <optional>

#include "api/environment/environment.h"
#include "api/video/color_space.h"
#include "api/video/encoded_image.h"
#include "api/video_codecs/video_decoder.h"
#include "common_video/include/video_frame_buffer_pool.h"
#include "vpx/vpx_decoder.h"
#include "vpx/vpx_image.h"

namespace webrtc {

class LibvpxVp8Decoder : public VideoDecoder {
 public:
  explicit LibvpxVp8Decoder(const Environment& env);
  ~LibvpxVp8Decoder() override;

  bool Configure(const Settings& settings) override;
  int Decode(const EncodedImage& input_image,
             int64_t /*render_time_ms*/) override;

  // TODO(bugs.webrtc.org/15444): Remove once all subclasses have been migrated
  // to expecting calls Decode without a missing_frames param.
  int Decode(const EncodedImage& input_image,
             bool missing_frames,
             int64_t /*render_time_ms*/) override;

  int RegisterDecodeCompleteCallback(DecodedImageCallback* callback) override;
  int Release() override;

  DecoderInfo GetDecoderInfo() const override;
  const char* ImplementationName() const override;

  struct DeblockParams {
    DeblockParams() : max_level(6), degrade_qp(1), min_qp(0) {}
    DeblockParams(int max_level, int degrade_qp, int min_qp)
        : max_level(max_level), degrade_qp(degrade_qp), min_qp(min_qp) {}
    int max_level;   // Deblocking strength: [0, 16].
    int degrade_qp;  // If QP value is below, start lowering `max_level`.
    int min_qp;      // If QP value is below, turn off deblocking.
  };

 private:
  class QpSmoother;

  int ReturnFrame(const vpx_image_t* img,
                  uint32_t timeStamp,
                  int qp,
                  const webrtc::ColorSpace* explicit_color_space);
  const bool use_postproc_;

  VideoFrameBufferPool buffer_pool_;
  DecodedImageCallback* decode_complete_callback_;
  bool inited_;
  vpx_codec_ctx_t* decoder_;
  int last_frame_width_;
  int last_frame_height_;
  bool key_frame_required_;
  const std::optional<DeblockParams> deblock_params_;
  const std::unique_ptr<QpSmoother> qp_smoother_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_CODECS_VP8_LIBVPX_VP8_DECODER_H_
