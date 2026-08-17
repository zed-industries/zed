/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 *
 */

#ifndef MODULES_VIDEO_CODING_CODECS_H264_H264_DECODER_IMPL_H_
#define MODULES_VIDEO_CODING_CODECS_H264_H264_DECODER_IMPL_H_

// Everything declared in this header is only required when WebRTC is
// build with H264 support, please do not move anything out of the
// #ifdef unless needed and tested.
#ifdef WEBRTC_USE_H264

#if defined(WEBRTC_WIN) && !defined(__clang__)
#error "See: bugs.webrtc.org/9213#c13."
#endif

// CAVEAT: According to ffmpeg docs for avcodec_send_packet, ffmpeg requires a
// few extra padding bytes after the end of input. And in addition, docs for
// AV_INPUT_BUFFER_PADDING_SIZE says "If the first 23 bits of the additional
// bytes are not 0, then damaged MPEG bitstreams could cause overread and
// segfault."
//
// WebRTC doesn't ensure any such padding, and REQUIRES ffmpeg to be compiled
// with CONFIG_SAFE_BITSTREAM_READER, which is intended to eliminate
// out-of-bounds reads. ffmpeg docs doesn't say explicitly what effects this
// flag has on the h.264 decoder or avcodec_send_packet, though, so this is in
// some way depending on undocumented behavior. If any problems turn up, we may
// have to add an extra copy operation, to enforce padding before buffers are
// passed to ffmpeg.

extern "C" {
#include <libavcodec/avcodec.h>
}  // extern "C"

#include <memory>

#include "common_video/h264/h264_bitstream_parser.h"
#include "common_video/include/video_frame_buffer_pool.h"
#include "modules/video_coding/codecs/h264/include/h264.h"

namespace webrtc {

struct AVCodecContextDeleter {
  void operator()(AVCodecContext* ptr) const { avcodec_free_context(&ptr); }
};
struct AVFrameDeleter {
  void operator()(AVFrame* ptr) const { av_frame_free(&ptr); }
};

class H264DecoderImpl : public H264Decoder {
 public:
  H264DecoderImpl();
  ~H264DecoderImpl() override;

  bool Configure(const Settings& settings) override;
  int32_t Release() override;

  int32_t RegisterDecodeCompleteCallback(
      DecodedImageCallback* callback) override;

  // `missing_frames`, `fragmentation` and `render_time_ms` are ignored.
  int32_t Decode(const EncodedImage& input_image,
                 bool /*missing_frames*/,
                 int64_t render_time_ms = -1) override;

  const char* ImplementationName() const override;

 private:
  // Called by FFmpeg when it needs a frame buffer to store decoded frames in.
  // The `VideoFrame` returned by FFmpeg at `Decode` originate from here. Their
  // buffers are reference counted and freed by FFmpeg using `AVFreeBuffer2`.
  static int AVGetBuffer2(AVCodecContext* context,
                          AVFrame* av_frame,
                          int flags);
  // Called by FFmpeg when it is done with a video frame, see `AVGetBuffer2`.
  static void AVFreeBuffer2(void* opaque, uint8_t* data);

  bool IsInitialized() const;

  // Reports statistics with histograms.
  void ReportInit();
  void ReportError();

  // Used by ffmpeg via `AVGetBuffer2()` to allocate I420 images.
  VideoFrameBufferPool ffmpeg_buffer_pool_;
  std::unique_ptr<AVCodecContext, AVCodecContextDeleter> av_context_;
  std::unique_ptr<AVFrame, AVFrameDeleter> av_frame_;

  DecodedImageCallback* decoded_image_callback_;

  bool has_reported_init_;
  bool has_reported_error_;

  webrtc::H264BitstreamParser h264_bitstream_parser_;
};

}  // namespace webrtc

#endif  // WEBRTC_USE_H264

#endif  // MODULES_VIDEO_CODING_CODECS_H264_H264_DECODER_IMPL_H_
