// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_INSTRUMENTED_SIMULCAST_ADAPTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_INSTRUMENTED_SIMULCAST_ADAPTER_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/webrtc/api/video_codecs/video_encoder.h"
#include "third_party/webrtc/media/engine/simulcast_encoder_adapter.h"

namespace webrtc {
class VideoEncoderFactory;
}  // namespace webrtc
namespace blink {
class VideoEncoderStateObserver;

// InstrumentedSimulcastAdapter is webrtc::SimulcastEncoderAdapter with
// customized factories. It doesn't override webrtc::SimulcastEncoderAdapter
// functions except destructor.
// The factory, |primary_factory_adapter_| or |secondary_factory_adapter_|,
// creates blink::InstrumenedVideoEncoderWrapper which wraps an encoder created
// by |primary_encoder_factory| or |secondate_encoder_factory| and has a
// reference to |encoder_state_observer_|. Therefore, |encoder_state_observer_|
// observes all the encoder states used in webrtc::SimulcastEncoderAdapter. This
// class owns |encoder_state_observer_| and thus must outlive the created
// encoders.
class PLATFORM_EXPORT InstrumentedSimulcastAdapter
    : public webrtc::SimulcastEncoderAdapter {
 public:
  static std::unique_ptr<InstrumentedSimulcastAdapter> Create(
      const webrtc::Environment& env,
      webrtc::VideoEncoderFactory* primary_encoder_factory,
      webrtc::VideoEncoderFactory* secondate_encoder_factory,
      std::unique_ptr<VideoEncoderStateObserver> encoder_state_observer,
      const webrtc::SdpVideoFormat& format);

  ~InstrumentedSimulcastAdapter() override;

 private:
  class EncoderFactoryAdapter;
  InstrumentedSimulcastAdapter(
      const webrtc::Environment& env,
      std::unique_ptr<EncoderFactoryAdapter> primary_factory_adapter,
      std::unique_ptr<EncoderFactoryAdapter> secondary_factory_adapter,
      std::unique_ptr<VideoEncoderStateObserver> encoder_state_observer,
      const webrtc::SdpVideoFormat& format);

  const std::unique_ptr<VideoEncoderStateObserver> encoder_state_observer_;
  const std::unique_ptr<EncoderFactoryAdapter> primary_factory_adapter_;
  const std::unique_ptr<EncoderFactoryAdapter> secondary_factory_adapter_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_INSTRUMENTED_SIMULCAST_ADAPTER_H_
