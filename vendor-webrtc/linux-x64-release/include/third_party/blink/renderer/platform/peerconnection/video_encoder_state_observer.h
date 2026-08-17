// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_VIDEO_ENCODER_STATE_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_VIDEO_ENCODER_STATE_OBSERVER_H_

#include <optional>

#include "base/time/time.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace webrtc {
class VideoCodec;
}  // namespace webrtc

namespace blink {
// VideoEncoderStateObserver is the interface used by
// blink::InstrumentedVideoEncoderWrapper to notify the state of its wrapping
// encoder.
class PLATFORM_EXPORT VideoEncoderStateObserver {
 public:
  struct EncodeResult final {
    int width;
    int height;
    int keyframe;
    std::optional<int> spatial_index;

    uint32_t rtp_timestamp;
    base::TimeTicks encode_end_time;

    bool is_hardware_accelerated;
  };

  virtual ~VideoEncoderStateObserver() = default;

  // The encoder with |encoder_id| is created with |config|.
  virtual void OnEncoderCreated(int encoder_id,
                                const webrtc::VideoCodec& config) = 0;
  // The encoder with |encoder_id| is destroyed.
  virtual void OnEncoderDestroyed(int encoder_id) = 0;
  // The active spatial layers on the encoder with |encoder_id| is updated to
  // |active_spatial_layers|.
  virtual void OnRatesUpdated(int encoder_id,
                              const Vector<bool>& active_spatial_layers) = 0;
  // Encode() of the encoder with |encoder_id| is about to be performed for
  // the frame whose timestamp is |rtp_timestamp|.
  virtual void OnEncode(int encoder_id, uint32_t rtp_timestamp) = 0;
  // The encoder with |encoder_id| completes the encode and its result is
  // |result|.
  virtual void OnEncodedImage(int encoder_id, const EncodeResult& result) = 0;
};
}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_VIDEO_ENCODER_STATE_OBSERVER_H_
