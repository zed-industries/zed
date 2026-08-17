#include "nvidia_decoder_factory.h"

#include <modules/video_coding/codecs/h264/include/h264.h>

#include <memory>

#include "cuda_context.h"
#include "h264_decoder_impl.h"
#include "h265_decoder_impl.h"
#include "rtc_base/logging.h"

namespace webrtc {

constexpr char kSdpKeyNameCodecImpl[] = "implementation_name";
constexpr char kCodecName[] = "NvCodec";

static int GetCudaDeviceCapabilityMajorVersion(CUcontext context) {
  cuCtxSetCurrent(context);

  CUdevice device;
  cuCtxGetDevice(&device);

  int major;
  cuDeviceGetAttribute(&major, CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR,
                       device);

  return major;
}

std::vector<SdpVideoFormat> SupportedNvDecoderCodecs(CUcontext context) {
  std::vector<SdpVideoFormat> supportedFormats;

  // HardwareGeneration Kepler is 3.x
  // https://docs.nvidia.com/deploy/cuda-compatibility/index.html#faq
  // Kepler support h264 profile Main, Highprofile up to Level4.1
  // https://docs.nvidia.com/video-technologies/video-codec-sdk/nvdec-video-decoder-api-prog-guide/index.html#video-decoder-capabilities__table_o3x_fms_3lb
  if (GetCudaDeviceCapabilityMajorVersion(context) <= 3) {
    supportedFormats = {
        CreateH264Format(webrtc::H264Profile::kProfileHigh,
                         webrtc::H264Level::kLevel4_1, "1"),
        CreateH264Format(webrtc::H264Profile::kProfileMain,
                         webrtc::H264Level::kLevel4_1, "1"),
    };
  } else {
    supportedFormats = {
        // Constrained Baseline Profile does not support NvDecoder, but WebRTC
        // uses this profile by default,
        // so it must be returned in this method.
        CreateH264Format(webrtc::H264Profile::kProfileConstrainedBaseline,
                         webrtc::H264Level::kLevel5_1, "1"),
        CreateH264Format(webrtc::H264Profile::kProfileBaseline,
                         webrtc::H264Level::kLevel5_1, "1"),
        CreateH264Format(webrtc::H264Profile::kProfileHigh,
                         webrtc::H264Level::kLevel5_1, "1"),
        CreateH264Format(webrtc::H264Profile::kProfileMain,
                         webrtc::H264Level::kLevel5_1, "1"),
        SdpVideoFormat("H265"),
        SdpVideoFormat("HEVC"),
    };
  }

  for (auto& format : supportedFormats) {
    format.parameters.emplace(kSdpKeyNameCodecImpl, kCodecName);
  }

  return supportedFormats;
}

NvidiaVideoDecoderFactory::NvidiaVideoDecoderFactory()
    : cu_context_(livekit_ffi::CudaContext::GetInstance()) {
  if (cu_context_->Initialize()) {
    supported_formats_ = SupportedNvDecoderCodecs(cu_context_->GetContext());
  } else {
    RTC_LOG(LS_ERROR) << "Failed to initialize CUDA context.";
  }
  RTC_LOG(LS_INFO) << "NvidiaVideoDecoderFactory created with "
                   << supported_formats_.size() << " supported formats.";
}

NvidiaVideoDecoderFactory::~NvidiaVideoDecoderFactory() {}

bool NvidiaVideoDecoderFactory::IsSupported() {
  if (!livekit_ffi::CudaContext::IsAvailable()) {
    RTC_LOG(LS_WARNING) << "Cuda Context is not available.";
    return false;
  }

  std::cout << "Nvidia Decoder is supported." << std::endl;
  return true;
}

std::unique_ptr<VideoDecoder> NvidiaVideoDecoderFactory::Create(
    const Environment& env,
    const SdpVideoFormat& format) {
  // Check if the requested format is supported.
  for (const auto& supported_format : supported_formats_) {
    if (format.IsSameCodec(supported_format)) {
      // If the format is supported, create and return the decoder.
      if (!cu_context_) {
        cu_context_ = livekit_ffi::CudaContext::GetInstance();
        if (!cu_context_->Initialize()) {
          RTC_LOG(LS_ERROR) << "Failed to initialize CUDA context.";
          return nullptr;
        }
      }
      if (format.name == "H264") {
        RTC_LOG(LS_INFO) << "Using NVIDIA HW decoder (NVDEC) for H264";
        return std::make_unique<NvidiaH264DecoderImpl>(cu_context_->GetContext());
      }
      if (format.name == "H265" || format.name == "HEVC") {
        RTC_LOG(LS_INFO) << "Using NVIDIA HW decoder (NVDEC) for H265/HEVC";
        return std::make_unique<NvidiaH265DecoderImpl>(cu_context_->GetContext());
      }
    }
  }
  return nullptr;
}

std::vector<SdpVideoFormat> NvidiaVideoDecoderFactory::GetSupportedFormats()
    const {
  return supported_formats_;
}

}  // namespace webrtc
