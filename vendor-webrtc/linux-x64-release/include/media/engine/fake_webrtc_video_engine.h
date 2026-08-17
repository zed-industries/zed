/*
 *  Copyright (c) 2010 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MEDIA_ENGINE_FAKE_WEBRTC_VIDEO_ENGINE_H_
#define MEDIA_ENGINE_FAKE_WEBRTC_VIDEO_ENGINE_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "api/environment/environment.h"
#include "api/fec_controller_override.h"
#include "api/video/encoded_image.h"
#include "api/video/video_frame.h"
#include "api/video/video_frame_type.h"
#include "api/video_codecs/scalability_mode.h"
#include "api/video_codecs/sdp_video_format.h"
#include "api/video_codecs/video_codec.h"
#include "api/video_codecs/video_decoder.h"
#include "api/video_codecs/video_decoder_factory.h"
#include "api/video_codecs/video_encoder.h"
#include "api/video_codecs/video_encoder_factory.h"
#include "rtc_base/event.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class FakeWebRtcVideoDecoderFactory;
class FakeWebRtcVideoEncoderFactory;

// Fake class for mocking out webrtc::VideoDecoder
class FakeWebRtcVideoDecoder : public VideoDecoder {
 public:
  explicit FakeWebRtcVideoDecoder(FakeWebRtcVideoDecoderFactory* factory);
  ~FakeWebRtcVideoDecoder();

  bool Configure(const Settings& settings) override;
  int32_t Decode(const EncodedImage&, int64_t) override;
  int32_t RegisterDecodeCompleteCallback(DecodedImageCallback*) override;
  int32_t Release() override;

  int GetNumFramesReceived() const;

 private:
  int num_frames_received_;
  FakeWebRtcVideoDecoderFactory* factory_;
};

// Fake class for mocking out webrtc::VideoDecoderFactory.
class FakeWebRtcVideoDecoderFactory : public VideoDecoderFactory {
 public:
  FakeWebRtcVideoDecoderFactory();

  std::vector<SdpVideoFormat> GetSupportedFormats() const override;
  std::unique_ptr<VideoDecoder> Create(const Environment& env,
                                       const SdpVideoFormat& format) override;

  void DecoderDestroyed(FakeWebRtcVideoDecoder* decoder);
  void AddSupportedVideoCodec(const SdpVideoFormat& format);
  void AddSupportedVideoCodecType(const std::string& name);
  int GetNumCreatedDecoders();
  const std::vector<FakeWebRtcVideoDecoder*>& decoders();

 private:
  std::vector<SdpVideoFormat> supported_codec_formats_;
  std::vector<FakeWebRtcVideoDecoder*> decoders_;
  int num_created_decoders_;
};

// Fake class for mocking out webrtc::VideoEnoder
class FakeWebRtcVideoEncoder : public VideoEncoder {
 public:
  explicit FakeWebRtcVideoEncoder(FakeWebRtcVideoEncoderFactory* factory);
  ~FakeWebRtcVideoEncoder();

  void SetFecControllerOverride(
      FecControllerOverride* fec_controller_override) override;
  int32_t InitEncode(const VideoCodec* codecSettings,
                     const VideoEncoder::Settings& settings) override;
  int32_t Encode(const VideoFrame& inputImage,
                 const std::vector<VideoFrameType>* frame_types) override;
  int32_t RegisterEncodeCompleteCallback(
      EncodedImageCallback* callback) override;
  int32_t Release() override;
  void SetRates(const RateControlParameters& parameters) override;
  VideoEncoder::EncoderInfo GetEncoderInfo() const override;

  bool WaitForInitEncode();
  VideoCodec GetCodecSettings();
  int GetNumEncodedFrames();

 private:
  Mutex mutex_;
  Event init_encode_event_;
  int num_frames_encoded_ RTC_GUARDED_BY(mutex_);
  VideoCodec codec_settings_ RTC_GUARDED_BY(mutex_);
  FakeWebRtcVideoEncoderFactory* factory_;
};

// Fake class for mocking out webrtc::VideoEncoderFactory.
class FakeWebRtcVideoEncoderFactory : public VideoEncoderFactory {
 public:
  FakeWebRtcVideoEncoderFactory();

  std::vector<SdpVideoFormat> GetSupportedFormats() const override;
  VideoEncoderFactory::CodecSupport QueryCodecSupport(
      const SdpVideoFormat& format,
      std::optional<std::string> scalability_mode) const override;
  std::unique_ptr<VideoEncoder> Create(const Environment& env,
                                       const SdpVideoFormat& format) override;

  bool WaitForCreatedVideoEncoders(int num_encoders);
  void EncoderDestroyed(FakeWebRtcVideoEncoder* encoder);
  void set_encoders_have_internal_sources(bool internal_source);
  void AddSupportedVideoCodec(const SdpVideoFormat& format);
  void AddSupportedVideoCodecType(
      const std::string& name,
      const std::vector<ScalabilityMode>& scalability_modes = {});
  int GetNumCreatedEncoders();
  const std::vector<FakeWebRtcVideoEncoder*> encoders();

 private:
  Mutex mutex_;
  Event created_video_encoder_event_;
  std::vector<SdpVideoFormat> formats_;
  std::vector<FakeWebRtcVideoEncoder*> encoders_ RTC_GUARDED_BY(mutex_);
  int num_created_encoders_ RTC_GUARDED_BY(mutex_);
  bool vp8_factory_mode_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::FakeWebRtcVideoDecoder;
using ::webrtc::FakeWebRtcVideoDecoderFactory;
using ::webrtc::FakeWebRtcVideoEncoder;
using ::webrtc::FakeWebRtcVideoEncoderFactory;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // MEDIA_ENGINE_FAKE_WEBRTC_VIDEO_ENGINE_H_
