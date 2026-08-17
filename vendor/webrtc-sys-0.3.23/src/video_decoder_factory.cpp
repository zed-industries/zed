/*
 * Copyright 2025 LiveKit, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "livekit/video_decoder_factory.h"

#include <modules/video_coding/codecs/av1/av1_svc_config.h>
#include "api/environment/environment.h"
#include "api/video_codecs/av1_profile.h"
#include "api/video_codecs/sdp_video_format.h"
#include "livekit/objc_video_factory.h"
#include "media/base/media_constants.h"
#include "modules/video_coding/codecs/h264/include/h264.h"
#include "modules/video_coding/codecs/vp8/include/vp8.h"
#include "modules/video_coding/codecs/vp9/include/vp9.h"
#include "rtc_base/logging.h"

#if defined(RTC_DAV1D_IN_INTERNAL_DECODER_FACTORY)
#include "modules/video_coding/codecs/av1/dav1d_decoder.h"  // nogncheck
#endif

#ifdef WEBRTC_ANDROID
#include "livekit/android.h"
#endif

#if defined(USE_NVIDIA_VIDEO_CODEC)
#include "nvidia/nvidia_decoder_factory.h"
#endif

namespace livekit_ffi {

VideoDecoderFactory::VideoDecoderFactory() {
#ifdef __APPLE__
  factories_.push_back(livekit_ffi::CreateObjCVideoDecoderFactory());
#endif

#ifdef WEBRTC_ANDROID
  factories_.push_back(CreateAndroidVideoDecoderFactory());
#endif

#if defined(USE_NVIDIA_VIDEO_CODEC)
  if (webrtc::NvidiaVideoDecoderFactory::IsSupported()) {
    factories_.push_back(std::make_unique<webrtc::NvidiaVideoDecoderFactory>());
  }
#endif
}

std::vector<webrtc::SdpVideoFormat> VideoDecoderFactory::GetSupportedFormats()
    const {
  std::vector<webrtc::SdpVideoFormat> formats;

  for (const auto& factory : factories_) {
    auto supported_formats = factory->GetSupportedFormats();
    formats.insert(formats.end(), supported_formats.begin(),
                   supported_formats.end());
  }

  formats.push_back(webrtc::SdpVideoFormat(cricket::kVp8CodecName));
  for (const webrtc::SdpVideoFormat& format :
       webrtc::SupportedVP9DecoderCodecs())
    formats.push_back(format);
  for (const webrtc::SdpVideoFormat& h264_format :
       webrtc::SupportedH264DecoderCodecs())
    formats.push_back(h264_format);

  formats.push_back(webrtc::SdpVideoFormat(
      webrtc::SdpVideoFormat::AV1Profile0(),
      webrtc::LibaomAv1EncoderSupportedScalabilityModes()));
  return formats;
}

VideoDecoderFactory::CodecSupport VideoDecoderFactory::QueryCodecSupport(
    const webrtc::SdpVideoFormat& format,
    bool reference_scaling) const {
  if (reference_scaling) {
    webrtc::VideoCodecType codec =
        webrtc::PayloadStringToCodecType(format.name);
    if (codec != webrtc::kVideoCodecVP9 && codec != webrtc::kVideoCodecAV1) {
      return {/*is_supported=*/false, /*is_power_efficient=*/false};
    }
  }

  CodecSupport codec_support;
  codec_support.is_supported = format.IsCodecInList(GetSupportedFormats());
  return codec_support;
}

std::unique_ptr<webrtc::VideoDecoder> VideoDecoderFactory::Create(
    const webrtc::Environment& env, const webrtc::SdpVideoFormat& format) {
  for (const auto& factory : factories_) {
    for (const auto& supported_format : factory->GetSupportedFormats()) {
      if (supported_format.IsSameCodec(format))
        return factory->Create(env, format);
    }
  }

  if (absl::EqualsIgnoreCase(format.name, cricket::kVp8CodecName))
    return webrtc::CreateVp8Decoder(env);
  if (absl::EqualsIgnoreCase(format.name, cricket::kVp9CodecName))
    return webrtc::VP9Decoder::Create();
  if (absl::EqualsIgnoreCase(format.name, cricket::kH264CodecName))
    return webrtc::H264Decoder::Create();


#if defined(RTC_DAV1D_IN_INTERNAL_DECODER_FACTORY)
  if (absl::EqualsIgnoreCase(format.name, cricket::kAv1CodecName)) {
    return webrtc::CreateDav1dDecoder();
  }
#endif


  RTC_LOG(LS_ERROR) << "No VideoDecoder found for " << format.name;
  return nullptr;
}

}  // namespace livekit_ffi
