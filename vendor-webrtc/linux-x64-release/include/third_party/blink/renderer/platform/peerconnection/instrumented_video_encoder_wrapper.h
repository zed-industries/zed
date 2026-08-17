// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_INSTRUMENTED_VIDEO_ENCODER_WRAPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_INSTRUMENTED_VIDEO_ENCODER_WRAPPER_H_

#include <memory>

#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/sequence_checker.h"
#include "base/task/sequenced_task_runner.h"
#include "base/thread_annotations.h"
#include "third_party/blink/renderer/platform/peerconnection/video_encoder_state_observer.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/webrtc/api/video_codecs/video_encoder.h"

namespace blink {

// InstrumentedVideoEncoderWrapper is webrtc::VideoEncoder that
// - delegates webrtc::VideoEncoder call to |wrapped_encoder_|,
// - delegates webrtc::EncodedImageCallback call to |callback_|,
// - notifies the wrapped encoder state within these calls to |state_observer_|.
class PLATFORM_EXPORT InstrumentedVideoEncoderWrapper
    : public webrtc::VideoEncoder,
      public webrtc::EncodedImageCallback {
 public:
  InstrumentedVideoEncoderWrapper(
      int id,
      std::unique_ptr<webrtc::VideoEncoder> wrapped_encoder,
      VideoEncoderStateObserver* state_observer);
  ~InstrumentedVideoEncoderWrapper() override;

  // webrtc::VideoEncoder implementations.
  int InitEncode(const webrtc::VideoCodec* codec_settings,
                 const webrtc::VideoEncoder::Settings& settings) override;
  int32_t RegisterEncodeCompleteCallback(
      webrtc::EncodedImageCallback* callback) override;
  int32_t Release() override;
  int32_t Encode(
      const webrtc::VideoFrame& frame,
      const std::vector<webrtc::VideoFrameType>* frame_types) override;
  void SetRates(const RateControlParameters& parameters) override;
  void SetFecControllerOverride(
      webrtc::FecControllerOverride* fec_controller_override) override;
  void OnPacketLossRateUpdate(float packet_loss_rate) override;
  void OnRttUpdate(int64_t rtt_ms) override;
  void OnLossNotification(const LossNotification& loss_notification) override;
  webrtc::VideoEncoder::EncoderInfo GetEncoderInfo() const override;

  // webrtc::EncodedImageCallback implementations.
  webrtc::EncodedImageCallback::Result OnEncodedImage(
      const webrtc::EncodedImage& encoded_image,
      const webrtc::CodecSpecificInfo* codec_specific_info) override;
  void OnDroppedFrame(webrtc::EncodedImageCallback::DropReason reason) override;

 private:
  void ReportEncodeResult(
      const VideoEncoderStateObserver::EncodeResult& result);

  const int id_;
  const raw_ptr<VideoEncoderStateObserver> state_observer_
      GUARDED_BY_CONTEXT(encoder_sequence_);
  const scoped_refptr<base::SequencedTaskRunner> encoder_sequence_runner_;
  const std::unique_ptr<webrtc::VideoEncoder> wrapped_encoder_;

  raw_ptr<webrtc::EncodedImageCallback> callback_;

  // WebRTC encoder sequence.
  SEQUENCE_CHECKER(encoder_sequence_);

  // WeakPtr of this, bound to |encoder_sequence_runner_|.
  base::WeakPtrFactory<InstrumentedVideoEncoderWrapper> weak_this_factory_{
      this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_INSTRUMENTED_VIDEO_ENCODER_WRAPPER_H_
