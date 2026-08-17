// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_VIDEO_DECODER_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_VIDEO_DECODER_FACTORY_H_

#include "base/memory/raw_ptr.h"
#include "base/task/sequenced_task_runner.h"
#include "third_party/blink/renderer/platform/peerconnection/gpu_codec_support_waiter.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/webrtc/api/video_codecs/video_decoder_factory.h"
#include "third_party/webrtc/modules/video_coding/include/video_codec_interface.h"
#include "ui/gfx/color_space.h"

namespace webrtc {
class VideoDecoder;
}  // namespace webrtc

namespace media {
class GpuVideoAcceleratorFactories;
}  // namespace media

namespace blink {

class PLATFORM_EXPORT RTCVideoDecoderFactory
    : public webrtc::VideoDecoderFactory {
 public:
  explicit RTCVideoDecoderFactory(
      media::GpuVideoAcceleratorFactories* gpu_factories,
      const gfx::ColorSpace& render_color_space);
  // Temporary constructor that is used to log codecs that potentially can be HW
  // accelerated regardless of the state of feature flags. Please note that this
  // constructor must only be used for the purpose of logging.
  RTCVideoDecoderFactory(media::GpuVideoAcceleratorFactories* gpu_factories,
                         const gfx::ColorSpace& render_color_space,
                         bool override_disabled_profiles);
  RTCVideoDecoderFactory(const RTCVideoDecoderFactory&) = delete;
  RTCVideoDecoderFactory& operator=(const RTCVideoDecoderFactory&) = delete;
  ~RTCVideoDecoderFactory() override;

  // Runs on Chrome_libJingle_WorkerThread. The child thread is blocked while
  // this runs.
  std::unique_ptr<webrtc::VideoDecoder> Create(
      const webrtc::Environment& env,
      const webrtc::SdpVideoFormat& format) override;

  std::vector<webrtc::SdpVideoFormat> GetSupportedFormats() const override;

  webrtc::VideoDecoderFactory::CodecSupport QueryCodecSupport(
      const webrtc::SdpVideoFormat& format,
      bool reference_scaling) const override;

 private:
  void CheckAndWaitDecoderSupportStatusIfNeeded() const;
  raw_ptr<media::GpuVideoAcceleratorFactories> gpu_factories_;

  gfx::ColorSpace render_color_space_;

  std::unique_ptr<GpuCodecSupportWaiter> gpu_codec_support_waiter_;

  // Temporary variable to enable logging of codecs that potentially can be HW
  // accelerated. If set to true, H265 will be reported as supported regardless
  // of the state of the feature flag.
  const bool override_disabled_profiles_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_VIDEO_DECODER_FACTORY_H_
