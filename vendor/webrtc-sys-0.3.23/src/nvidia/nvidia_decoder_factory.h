

#ifndef NVIDIA_VIDEO_DECODER_FACTORY_H_
#define NVIDIA_VIDEO_DECODER_FACTORY_H_

#include <vector>

#include "api/environment/environment.h"
#include "api/video_codecs/sdp_video_format.h"
#include "api/video_codecs/video_decoder_factory.h"
#include "cuda_context.h"

namespace webrtc {

class NvidiaVideoDecoderFactory : public VideoDecoderFactory {
 public:
  NvidiaVideoDecoderFactory();
  ~NvidiaVideoDecoderFactory() override;

  static bool IsSupported();

  std::vector<webrtc::SdpVideoFormat> GetSupportedFormats() const override;
  std::unique_ptr<VideoDecoder> Create(
      const Environment& env,
      const SdpVideoFormat& format) override;

 private:
  std::vector<SdpVideoFormat> supported_formats_;
  livekit_ffi::CudaContext* cu_context_;
};

}  // namespace webrtc

#endif  // NVIDIA_VIDEO_DECODER_FACTORY_H_
