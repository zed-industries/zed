// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_VIDEO_ENCODER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_VIDEO_ENCODER_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <vector>

#include "base/feature_list.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/synchronization/lock.h"
#include "media/base/video_decoder_config.h"
#include "media/media_buildflags.h"
#include "media/video/video_encode_accelerator.h"
#include "third_party/blink/public/common/features.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/webrtc/api/video/video_bitrate_allocation.h"
#include "third_party/webrtc/modules/video_coding/include/video_codec_interface.h"
#include "ui/gfx/geometry/size.h"

#if BUILDFLAG(RTC_USE_H265)
#include "third_party/blink/renderer/platform/peerconnection/h265_parameter_sets_tracker.h"
#endif  // BUILDFLAG(RTC_USE_H265)

namespace base {
class SequencedTaskRunner;
}

namespace media {
class GpuVideoAcceleratorFactories;
class MojoVideoEncoderMetricsProviderFactory;
struct VideoEncoderInfo;
}  // namespace media

namespace blink {

namespace features {
PLATFORM_EXPORT BASE_DECLARE_FEATURE(kWebRtcScreenshareSwEncoding);
PLATFORM_EXPORT BASE_DECLARE_FEATURE(kForcingSoftwareIncludes360);
PLATFORM_EXPORT BASE_DECLARE_FEATURE(kKeepEncoderInstanceOnRelease);
}

// RTCVideoEncoder uses a media::VideoEncodeAccelerator to implement a
// webrtc::VideoEncoder class for WebRTC.  Internally, VEA methods are
// trampolined to a private RTCVideoEncoder::Impl instance.  The Impl class runs
// on the worker thread queried from the |gpu_factories_|, which is presently
// the media thread.  RTCVideoEncoder is sychronized by webrtc::VideoSender.
// webrtc::VideoEncoder methods do not run concurrently. RtcVideoEncoder needs
// to synchronize RegisterEncodeCompleteCallback and encode complete callback.
class PLATFORM_EXPORT RTCVideoEncoder : public webrtc::VideoEncoder {
 public:
  RTCVideoEncoder(media::VideoCodecProfile profile,
                  bool is_constrained_h264,
                  media::GpuVideoAcceleratorFactories* gpu_factories,
                  scoped_refptr<media::MojoVideoEncoderMetricsProviderFactory>
                      encoder_metrics_provider_factory);
  RTCVideoEncoder(const RTCVideoEncoder&) = delete;
  RTCVideoEncoder& operator=(const RTCVideoEncoder&) = delete;
  ~RTCVideoEncoder() override;

  // webrtc::VideoEncoder implementation.
  // They run on |webrtc_sequence_checker_|. Tasks are posted to |impl_| using
  // the appropriate VEA methods.
  int InitEncode(const webrtc::VideoCodec* codec_settings,
                 const webrtc::VideoEncoder::Settings& settings) override;
  int32_t Encode(
      const webrtc::VideoFrame& input_image,
      const std::vector<webrtc::VideoFrameType>* frame_types) override;
  int32_t RegisterEncodeCompleteCallback(
      webrtc::EncodedImageCallback* callback) override;
  int32_t Release() override;
  void SetRates(
      const webrtc::VideoEncoder::RateControlParameters& parameters) override;
  EncoderInfo GetEncoderInfo() const override;

  void SetErrorCallbackForTesting(
      WTF::CrossThreadOnceClosure error_callback_for_testing) {
    error_callback_for_testing_ = std::move(error_callback_for_testing);
  }
#if BUILDFLAG(RTC_USE_H265)
  void SetH265ParameterSetsTrackerForTesting(
      std::unique_ptr<H265ParameterSetsTracker> tracker);
#endif

 private:
  class Impl;

  int32_t InitializeEncoder(
      const media::VideoEncodeAccelerator::Config& vea_config);
  void UpdateEncoderInfo(
      media::VideoEncoderInfo encoder_info,
      std::vector<webrtc::VideoFrameBuffer::Type> preferred_pixel_formats);
  void SetError(uint32_t impl_id);
  void ReleaseImpl();

  bool CodecSettingsUsableForFrameSizeChange(
      const webrtc::VideoCodec& codec_settings) const;

  int32_t DrainEncoderAndUpdateFrameSize(
      const gfx::Size& input_visible_size,
      const webrtc::VideoEncoder::RateControlParameters& params,
      const media::SVCInterLayerPredMode& inter_layer_pred,
      const std::vector<media::VideoEncodeAccelerator::Config::SpatialLayer>&
          spatial_layers);

  const media::VideoCodecProfile profile_;

  const bool is_constrained_h264_;

  webrtc::VideoCodec codec_settings_;

  // Factory for creating VEAs, shared memory buffers, etc.
  const raw_ptr<media::GpuVideoAcceleratorFactories> gpu_factories_;

  scoped_refptr<media::MojoVideoEncoderMetricsProviderFactory>
      encoder_metrics_provider_factory_;

  // Task runner that the video accelerator runs on.
  const scoped_refptr<base::SequencedTaskRunner> gpu_task_runner_;

  // TODO(b/258782303): Remove this lock if |encoder_info_| is called on
  // webrtc sequence.
  mutable base::Lock lock_;
  // Default values are set in RTCVideoEncoder constructor. Some of its
  // variables are updated in UpdateEncoderInfo() in webrtc encoder thread.
  // There is a minor data race that GetEncoderInfo() can be called in a
  // different thread from webrtc encoder thread.
  webrtc::VideoEncoder::EncoderInfo encoder_info_ GUARDED_BY(lock_);

  // The sequence on which the webrtc::VideoEncoder functions are executed.
  SEQUENCE_CHECKER(webrtc_sequence_checker_);

  bool has_error_ GUARDED_BY_CONTEXT(webrtc_sequence_checker_){false};

  // Execute in SetError(). This can be valid only in testing.
  WTF::CrossThreadOnceClosure error_callback_for_testing_;

  // The RTCVideoEncoder::Impl that does all the work.
  std::unique_ptr<Impl> impl_;
  // |impl_id_| starts from 0 and increases by 1 when creating a new instance of
  // Impl.
  uint32_t impl_id_ = 0;

  // This weak pointer is bound to |gpu_task_runner_|.
  base::WeakPtr<Impl> weak_impl_;

  bool impl_initialized_;
  bool frame_size_change_supported_{false};

  // |weak_this_| is bound to |webrtc_sequence_checker_|.
  base::WeakPtr<RTCVideoEncoder> weak_this_;
  base::WeakPtrFactory<RTCVideoEncoder> weak_this_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_VIDEO_ENCODER_H_
