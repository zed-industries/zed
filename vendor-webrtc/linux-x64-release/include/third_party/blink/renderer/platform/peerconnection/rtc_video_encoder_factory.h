// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_VIDEO_ENCODER_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_VIDEO_ENCODER_FACTORY_H_

#include <vector>

#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/peerconnection/gpu_codec_support_waiter.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/webrtc/api/video_codecs/video_encoder_factory.h"
#include "third_party/webrtc/modules/video_coding/include/video_codec_interface.h"

namespace media {
class GpuVideoAcceleratorFactories;
class MojoVideoEncoderMetricsProviderFactory;
}  // namespace media

namespace blink {

// This class creates RTCVideoEncoder instances (each wrapping a
// media::VideoEncodeAccelerator) on behalf of the WebRTC stack.
class PLATFORM_EXPORT RTCVideoEncoderFactory
    : public webrtc::VideoEncoderFactory {
 public:
  RTCVideoEncoderFactory(
      media::GpuVideoAcceleratorFactories* gpu_factories,
      scoped_refptr<media::MojoVideoEncoderMetricsProviderFactory>
          encoder_metrics_provider_factory);
  // Temporary constructor that is used to log codecs that potentially can be HW
  // accelerated regardless of the state of feature flags. Please note that this
  // constructor must only be used for the purpose of logging.
  RTCVideoEncoderFactory(
      media::GpuVideoAcceleratorFactories* gpu_factories,
      scoped_refptr<media::MojoVideoEncoderMetricsProviderFactory>
          encoder_metrics_provider_factory,
      bool override_disabled_profiles);
  RTCVideoEncoderFactory(const RTCVideoEncoderFactory&) = delete;
  RTCVideoEncoderFactory& operator=(const RTCVideoEncoderFactory&) = delete;
  ~RTCVideoEncoderFactory() override;

  // webrtc::VideoEncoderFactory implementation.
  std::unique_ptr<webrtc::VideoEncoder> Create(
      const webrtc::Environment& env,
      const webrtc::SdpVideoFormat& format) override;
  std::vector<webrtc::SdpVideoFormat> GetSupportedFormats() const override;
  webrtc::VideoEncoderFactory::CodecSupport QueryCodecSupport(
      const webrtc::SdpVideoFormat& format,
      std::optional<std::string> scalability_mode) const override;

  // Some platforms don't allow hardware encoding for certain profiles. Tests
  // exercising VP9 or AV1 likely want to clear this list.
  void clear_disabled_profiles_for_testing() { disabled_profiles_.clear(); }

 private:
  void CheckAndWaitEncoderSupportStatusIfNeeded() const;

  raw_ptr<media::GpuVideoAcceleratorFactories> gpu_factories_;

  scoped_refptr<media::MojoVideoEncoderMetricsProviderFactory>
      encoder_metrics_provider_factory_;

  GpuCodecSupportWaiter gpu_codec_support_waiter_;

  // List of profiles that RTCVideoEncoderFactory will refuse to create an
  // encoder for even if the underlying GPU factories has support.
  std::vector<media::VideoCodecProfile> disabled_profiles_
      ALLOW_DISCOURAGED_TYPE("Matches webrtc API");
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_VIDEO_ENCODER_FACTORY_H_
