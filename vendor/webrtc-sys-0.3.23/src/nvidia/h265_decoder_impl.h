#ifndef WEBRTC_NVIDIA_H265_DECODER_IMPL_H_
#define WEBRTC_NVIDIA_H265_DECODER_IMPL_H_

#include <api/video_codecs/sdp_video_format.h>
#include <api/video_codecs/video_decoder.h>
#include <api/video_codecs/video_decoder_factory.h>
#include <api/video/video_codec_type.h>
#include <common_video/include/video_frame_buffer_pool.h>
#include <cuda.h>
#include <media/base/codec.h>

#include "NvDecoder/NvDecoder.h"

namespace webrtc {

class NvidiaH265DecoderImpl : public VideoDecoder {
 public:
  explicit NvidiaH265DecoderImpl(CUcontext context);
  NvidiaH265DecoderImpl(const NvidiaH265DecoderImpl&) = delete;
  NvidiaH265DecoderImpl& operator=(const NvidiaH265DecoderImpl&) = delete;
  ~NvidiaH265DecoderImpl() override;

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
};

}  // end namespace webrtc

#endif  // WEBRTC_NVIDIA_H265_DECODER_IMPL_H_


