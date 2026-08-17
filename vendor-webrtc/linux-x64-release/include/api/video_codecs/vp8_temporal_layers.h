/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_VP8_TEMPORAL_LAYERS_H_
#define API_VIDEO_CODECS_VP8_TEMPORAL_LAYERS_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <vector>

#include "api/fec_controller_override.h"
#include "api/video_codecs/video_encoder.h"
#include "api/video_codecs/vp8_frame_buffer_controller.h"
#include "api/video_codecs/vp8_frame_config.h"

namespace webrtc {

// Two different flavors of temporal layers are currently available:
// kFixedPattern uses a fixed repeating pattern of 1-4 layers.
// kBitrateDynamic can allocate frames dynamically to 1 or 2 layers, based on
// the bitrate produced.
// TODO(eladalon): Remove this enum.
enum class Vp8TemporalLayersType { kFixedPattern, kBitrateDynamic };

// This interface defines a way of getting the encoder settings needed to
// realize a temporal layer structure.
class Vp8TemporalLayers final : public Vp8FrameBufferController {
 public:
  Vp8TemporalLayers(
      std::vector<std::unique_ptr<Vp8FrameBufferController>>&& controllers,
      FecControllerOverride* fec_controller_override);
  ~Vp8TemporalLayers() override = default;

  void SetQpLimits(size_t stream_index, int min_qp, int max_qp) override;

  size_t StreamCount() const override;

  bool SupportsEncoderFrameDropping(size_t stream_index) const override;

  void OnRatesUpdated(size_t stream_index,
                      const std::vector<uint32_t>& bitrates_bps,
                      int framerate_fps) override;

  Vp8EncoderConfig UpdateConfiguration(size_t stream_index) override;

  Vp8FrameConfig NextFrameConfig(size_t stream_index,
                                 uint32_t rtp_timestamp) override;

  void OnEncodeDone(size_t stream_index,
                    uint32_t rtp_timestamp,
                    size_t size_bytes,
                    bool is_keyframe,
                    int qp,
                    CodecSpecificInfo* info) override;

  void OnFrameDropped(size_t stream_index, uint32_t rtp_timestamp) override;

  void OnPacketLossRateUpdate(float packet_loss_rate) override;

  void OnRttUpdate(int64_t rtt_ms) override;

  void OnLossNotification(
      const VideoEncoder::LossNotification& loss_notification) override;

 private:
  std::vector<std::unique_ptr<Vp8FrameBufferController>> controllers_;
};

}  // namespace webrtc

#endif  // API_VIDEO_CODECS_VP8_TEMPORAL_LAYERS_H_
