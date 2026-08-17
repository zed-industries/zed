// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_OFFER_OPTIONS_PLATFORM_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_OFFER_OPTIONS_PLATFORM_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class RTCOfferOptionsPlatform final
    : public GarbageCollected<RTCOfferOptionsPlatform> {
 public:
  RTCOfferOptionsPlatform(int32_t offer_to_receive_video,
                          int32_t offer_to_receive_audio,
                          bool voice_activity_detection,
                          bool ice_restart)
      : offer_to_receive_video_(offer_to_receive_video),
        offer_to_receive_audio_(offer_to_receive_audio),
        voice_activity_detection_(voice_activity_detection),
        ice_restart_(ice_restart) {}

  int32_t OfferToReceiveVideo() const { return offer_to_receive_video_; }
  int32_t OfferToReceiveAudio() const { return offer_to_receive_audio_; }
  bool VoiceActivityDetection() const { return voice_activity_detection_; }
  bool IceRestart() const { return ice_restart_; }

  void Trace(Visitor* visitor) const {}

 private:
  int32_t offer_to_receive_video_;
  int32_t offer_to_receive_audio_;
  bool voice_activity_detection_;
  bool ice_restart_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_OFFER_OPTIONS_PLATFORM_H_
