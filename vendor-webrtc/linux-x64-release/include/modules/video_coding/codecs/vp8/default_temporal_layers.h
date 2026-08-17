/* Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
/*
 * This file defines classes for doing temporal layers with VP8.
 */
#ifndef MODULES_VIDEO_CODING_CODECS_VP8_DEFAULT_TEMPORAL_LAYERS_H_
#define MODULES_VIDEO_CODING_CODECS_VP8_DEFAULT_TEMPORAL_LAYERS_H_

#include <stddef.h>
#include <stdint.h>

#include <array>
#include <bitset>
#include <deque>
#include <memory>
#include <optional>
#include <set>
#include <vector>

#include "absl/container/inlined_vector.h"
#include "absl/strings/string_view.h"
#include "api/transport/rtp/dependency_descriptor.h"
#include "api/video_codecs/video_encoder.h"
#include "api/video_codecs/vp8_frame_buffer_controller.h"
#include "api/video_codecs/vp8_frame_config.h"
#include "modules/video_coding/codecs/vp8/include/temporal_layers_checker.h"
#include "modules/video_coding/include/video_codec_interface.h"

namespace webrtc {

class DefaultTemporalLayers final : public Vp8FrameBufferController {
 public:
  explicit DefaultTemporalLayers(int number_of_temporal_layers);
  ~DefaultTemporalLayers() override;

  void SetQpLimits(size_t stream_index, int min_qp, int max_qp) override;

  size_t StreamCount() const override;

  bool SupportsEncoderFrameDropping(size_t stream_index) const override;

  // Returns the recommended VP8 encode flags needed. May refresh the decoder
  // and/or update the reference buffers.
  Vp8FrameConfig NextFrameConfig(size_t stream_index,
                                 uint32_t timestamp) override;

  // New target bitrate, per temporal layer.
  void OnRatesUpdated(size_t stream_index,
                      const std::vector<uint32_t>& bitrates_bps,
                      int framerate_fps) override;

  Vp8EncoderConfig UpdateConfiguration(size_t stream_index) override;

  // Callbacks methods on frame completion. OnEncodeDone() or OnFrameDropped()
  // should be called once for each NextFrameConfig() call (using the RTP
  // timestamp as ID), and the calls MUST be in the same order.
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
  static constexpr size_t kNumReferenceBuffers = 3;  // Last, golden, altref.
  struct DependencyInfo {
    DependencyInfo() = default;
    DependencyInfo(absl::string_view indication_symbols,
                   Vp8FrameConfig frame_config)
        : decode_target_indications(
              webrtc_impl::StringToDecodeTargetIndications(indication_symbols)),
          frame_config(frame_config) {}

    absl::InlinedVector<DecodeTargetIndication, 10> decode_target_indications;
    Vp8FrameConfig frame_config;
  };
  struct PendingFrame {
    PendingFrame();
    PendingFrame(uint32_t timestamp,
                 bool expired,
                 uint8_t updated_buffers_mask,
                 const DependencyInfo& dependency_info);
    uint32_t timestamp = 0;
    // Flag indicating if this frame has expired, ie it belongs to a previous
    // iteration of the temporal pattern.
    bool expired = false;
    // Bitmask of Vp8BufferReference flags, indicating which buffers this frame
    // updates.
    uint8_t updated_buffer_mask = 0;
    // The frame config returned by NextFrameConfig() for this frame.
    DependencyInfo dependency_info;
  };

  static std::vector<DependencyInfo> GetDependencyInfo(size_t num_layers);
  static std::bitset<kNumReferenceBuffers> DetermineStaticBuffers(
      const std::vector<DependencyInfo>& temporal_pattern);
  bool IsSyncFrame(const Vp8FrameConfig& config) const;
  void ValidateReferences(Vp8FrameConfig::BufferFlags* flags,
                          Vp8FrameConfig::Vp8BufferReference ref) const;
  void UpdateSearchOrder(Vp8FrameConfig* config);
  size_t NumFramesSinceBufferRefresh(
      Vp8FrameConfig::Vp8BufferReference ref) const;
  void ResetNumFramesSinceBufferRefresh(Vp8FrameConfig::Vp8BufferReference ref);
  void CullPendingFramesBefore(uint32_t timestamp);

  const size_t num_layers_;
  const std::vector<unsigned int> temporal_ids_;
  const std::vector<DependencyInfo> temporal_pattern_;
  // Per reference buffer flag indicating if it is static, meaning it is only
  // updated by key-frames.
  const std::bitset<kNumReferenceBuffers> is_static_buffer_;
  FrameDependencyStructure GetTemplateStructure(int num_layers) const;

  uint8_t pattern_idx_;
  // Updated cumulative bitrates, per temporal layer.
  std::optional<std::vector<uint32_t>> new_bitrates_bps_;

  // Status for each pending frame, in
  std::deque<PendingFrame> pending_frames_;

  // One counter per reference buffer, indicating number of frames since last
  // refresh. For non-base-layer frames (ie golden, altref buffers), this is
  // reset when the pattern loops.
  std::array<size_t, kNumReferenceBuffers> frames_since_buffer_refresh_;

  // Optional utility used to verify reference validity.
  std::unique_ptr<TemporalLayersChecker> checker_;
};

class DefaultTemporalLayersChecker : public TemporalLayersChecker {
 public:
  explicit DefaultTemporalLayersChecker(int number_of_temporal_layers);
  ~DefaultTemporalLayersChecker() override;

  bool CheckTemporalConfig(bool frame_is_keyframe,
                           const Vp8FrameConfig& frame_config) override;

 private:
  struct BufferState {
    BufferState()
        : is_updated_this_cycle(false), is_keyframe(true), pattern_idx(0) {}

    bool is_updated_this_cycle;
    bool is_keyframe;
    uint8_t pattern_idx;
  };
  const size_t num_layers_;
  std::vector<unsigned int> temporal_ids_;
  const std::vector<std::set<uint8_t>> temporal_dependencies_;
  BufferState last_;
  BufferState arf_;
  BufferState golden_;
  uint8_t pattern_idx_;
};

}  // namespace webrtc
#endif  // MODULES_VIDEO_CODING_CODECS_VP8_DEFAULT_TEMPORAL_LAYERS_H_
