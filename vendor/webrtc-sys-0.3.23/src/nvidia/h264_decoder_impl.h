#ifndef WEBRTC_NVIDIA_H264_DECODER_IMPL_H_
#define WEBRTC_NVIDIA_H264_DECODER_IMPL_H_

#include <api/video_codecs/h264_profile_level_id.h>
#include <api/video_codecs/sdp_video_format.h>
#include <api/video_codecs/video_decoder.h>
#include <api/video_codecs/video_decoder_factory.h>
#include <api/video_codecs/video_encoder.h>
#include <api/video_codecs/video_encoder_factory.h>
#include <common_video/h264/h264_bitstream_parser.h>
#include <common_video/h264/pps_parser.h>
#include <common_video/h264/sps_parser.h>
#include <common_video/include/video_frame_buffer_pool.h>
#include <cuda.h>
#include <media/base/codec.h>

#include "NvDecoder/NvDecoder.h"

namespace webrtc {

class H264BitstreamParserEx : public ::webrtc::H264BitstreamParser {
 public:
  std::optional<SpsParser::SpsState> sps() { return sps_; }
  std::optional<PpsParser::PpsState> pps() { return pps_; }
};

class NvidiaH264DecoderImpl : public VideoDecoder {
 public:
  NvidiaH264DecoderImpl(CUcontext context);
  NvidiaH264DecoderImpl(const NvidiaH264DecoderImpl&) = delete;
  NvidiaH264DecoderImpl& operator=(const NvidiaH264DecoderImpl&) = delete;
  ~NvidiaH264DecoderImpl() override;

  bool Configure(const Settings& settings) override;
  int32_t Decode(const EncodedImage& input_image,
                 bool missing_frames,
                 int64_t render_time_ms) override;
  int32_t RegisterDecodeCompleteCallback(
      DecodedImageCallback* callback) override;
  int32_t Release() override;
  DecoderInfo GetDecoderInfo() const override;

 private:
  CUcontext cu_context_;
  std::unique_ptr<NvDecoder> decoder_;
  bool is_configured_decoder_;

  Settings settings_;

  DecodedImageCallback* decoded_complete_callback_ = nullptr;
  webrtc::VideoFrameBufferPool buffer_pool_;
  H264BitstreamParserEx h264_bitstream_parser_;
};

}  // end namespace webrtc

#endif  // WEBRTC_NVIDIA_H264_DECODER_IMPL_H_