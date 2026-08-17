#include "nvidia_encoder_factory.h"

#include <memory>

#include "cuda_context.h"
#include "h264_encoder_impl.h"
#include "h265_encoder_impl.h"
#include "rtc_base/logging.h"

namespace webrtc {

NvidiaVideoEncoderFactory::NvidiaVideoEncoderFactory() {
  std::map<std::string, std::string> baselineParameters = {
      {"profile-level-id", "42e01f"},
      {"level-asymmetry-allowed", "1"},
      {"packetization-mode", "1"},
  };
  supported_formats_.push_back(SdpVideoFormat("H264", baselineParameters));

  // Advertise HEVC/H265 with default parameters.
  supported_formats_.push_back(SdpVideoFormat("H265"));
  // Some stacks use 'HEVC' name.
  supported_formats_.push_back(SdpVideoFormat("HEVC"));

  /*std::map<std::string, std::string> highParameters = {
      {"profile-level-id", "4d0032"},
      {"level-asymmetry-allowed", "1"},
      {"packetization-mode", "1"},
  };

  supported_formats_.push_back(SdpVideoFormat("H264", highParameters));
  */
}

NvidiaVideoEncoderFactory::~NvidiaVideoEncoderFactory() {}

bool NvidiaVideoEncoderFactory::IsSupported() {
  if (!livekit_ffi::CudaContext::IsAvailable()) {
    RTC_LOG(LS_WARNING) << "Cuda Context is not available.";
    return false;
  }

  std::cout << "Nvidia Encoder is supported." << std::endl;
  return true;
}

std::unique_ptr<VideoEncoder> NvidiaVideoEncoderFactory::Create(
    const Environment& env,
    const SdpVideoFormat& format) {
  // Check if the requested format is supported.
  for (const auto& supported_format : supported_formats_) {
    if (format.IsSameCodec(supported_format)) {
      if (!cu_context_) {
        cu_context_ = livekit_ffi::CudaContext::GetInstance();
        if (!cu_context_->Initialize()) {
          RTC_LOG(LS_ERROR) << "Failed to initialize CUDA context.";
          return nullptr;
        }
      }

      if (format.name == "H264") {
        RTC_LOG(LS_INFO) << "Using NVIDIA HW encoder (NVENC) for H264";
        return std::make_unique<NvidiaH264EncoderImpl>(
            env, cu_context_->GetContext(), CU_MEMORYTYPE_DEVICE,
            NV_ENC_BUFFER_FORMAT_IYUV, format);
      }

      if (format.name == "H265" || format.name == "HEVC") {
        RTC_LOG(LS_INFO) << "Using NVIDIA HW encoder (NVENC) for H265/HEVC";
        return std::make_unique<NvidiaH265EncoderImpl>(
            env, cu_context_->GetContext(), CU_MEMORYTYPE_DEVICE,
            NV_ENC_BUFFER_FORMAT_IYUV, format);
      }
    }
  }
  return nullptr;
}
std::vector<SdpVideoFormat> NvidiaVideoEncoderFactory::GetSupportedFormats()
    const {
  return supported_formats_;
}

std::vector<SdpVideoFormat> NvidiaVideoEncoderFactory::GetImplementations()
    const {
  return supported_formats_;
}

}  // namespace webrtc
