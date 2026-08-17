// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_VIDEO_ENCODER_STATE_OBSERVER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_VIDEO_ENCODER_STATE_OBSERVER_IMPL_H_

#include <memory>
#include <optional>

#include "base/atomic_ref_count.h"
#include "base/containers/flat_map.h"
#include "base/location.h"
#include "base/sequence_checker.h"
#include "third_party/blink/renderer/platform/peerconnection/stats_collector.h"
#include "third_party/blink/renderer/platform/peerconnection/video_encoder_state_observer.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/webrtc/api/video_codecs/video_codec.h"

namespace blink {
// VideoEncoderStateObserverImpl collects the encode stats for the top spatial
// layer in SVC encoding, top stream in simulcast or the vanilla stream
// otherwise. It doesn't collect stats if multiple encoders are running. This is
// constructed in webrtc worker sequence. After construction, all the operations
// including destructor must be done in the webrtc encoder sequence.
class PLATFORM_EXPORT VideoEncoderStateObserverImpl
    : public VideoEncoderStateObserver,
      public StatsCollector {
 public:
  struct TopLayerInfo {
    int encoder_id;
    int spatial_id;
    int pixel_rate;
  };

  VideoEncoderStateObserverImpl(
      media::VideoCodecProfile profile,
      const StatsCollector::StoreProcessingStatsCB& store_processing_stats_cb);
  ~VideoEncoderStateObserverImpl() override;

  // VideoEncoderStateObserver implementation.
  void OnEncoderCreated(int encoder_id,
                        const webrtc::VideoCodec& config) override;
  void OnEncoderDestroyed(int encoder_id) override;
  void OnRatesUpdated(int encoder_id,
                      const Vector<bool>& active_spatial_layers) override;
  void OnEncode(int encoder_id, uint32_t rtp_timestamp) override;
  void OnEncodedImage(int encoder_id, const EncodeResult& result) override;

  std::optional<TopLayerInfo> FindHighestActiveEncoding() const;

 private:
  class EncoderState;

  EncoderState* GetEncoderState(
      int encoder_id,
      base::Location location = base::Location::Current())
      VALID_CONTEXT_REQUIRED(encoder_sequence_);
  void UpdateStatsCollection(base::TimeTicks now)
      VALID_CONTEXT_REQUIRED(encoder_sequence_);

  base::flat_map<int, std::unique_ptr<EncoderState>> encoder_state_by_id_
      GUARDED_BY_CONTEXT(encoder_sequence_);
  std::optional<TopLayerInfo> top_encoder_info_
      GUARDED_BY_CONTEXT(encoder_sequence_);

  base::TimeTicks last_update_stats_collection_time_
      GUARDED_BY_CONTEXT(encoder_sequence_);

  // WebRTC encoder sequence.
  SEQUENCE_CHECKER(encoder_sequence_);
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_VIDEO_ENCODER_STATE_OBSERVER_IMPL_H_
