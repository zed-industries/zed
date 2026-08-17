/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_ENCODER_BITRATE_ADJUSTER_H_
#define VIDEO_ENCODER_BITRATE_ADJUSTER_H_

#include <memory>

#include "api/field_trials_view.h"
#include "api/units/time_delta.h"
#include "api/video/encoded_image.h"
#include "api/video/video_bitrate_allocation.h"
#include "api/video_codecs/video_encoder.h"
#include "system_wrappers/include/clock.h"
#include "video/encoder_overshoot_detector.h"
#include "video/rate_utilization_tracker.h"

namespace webrtc {

class EncoderBitrateAdjuster {
 public:
  // Size of sliding window used to track overshoot rate.
  static constexpr TimeDelta kWindowSize = TimeDelta::Seconds(3);
  // Minimum number of frames since last layout change required to trust the
  // overshoot statistics. Otherwise falls back to default utilization.
  // By layout change, we mean any simulcast/spatial/temporal layer being either
  // enabled or disabled.
  static constexpr size_t kMinFramesSinceLayoutChange = 30;
  // Default utilization, before reliable metrics are available, is set to 20%
  // overshoot. This is conservative so that badly misbehaving encoders don't
  // build too much queue at the very start.
  static constexpr double kDefaultUtilizationFactor = 1.2;

  EncoderBitrateAdjuster(const VideoCodec& codec_settings,
                         const FieldTrialsView& field_trials,
                         Clock& clock);
  ~EncoderBitrateAdjuster();

  // Adjusts the given rate allocation to make it paceable within the target
  // rates.
  VideoBitrateAllocation AdjustRateAllocation(
      const VideoEncoder::RateControlParameters& rates);

  // Updated overuse detectors with data about the encoder, specifically about
  // the temporal layer frame rate allocation.
  void OnEncoderInfo(const VideoEncoder::EncoderInfo& encoder_info);

  // Updates the overuse detectors according to the encoded image size.
  // `stream_index` is the spatial or simulcast index.
  // TODO(https://crbug.com/webrtc/14891): If we want to support a mix of
  // simulcast and SVC we'll also need to consider the case where we have both
  // simulcast and spatial indices.
  void OnEncodedFrame(DataSize size, int stream_index, int temporal_index);

  void Reset();

 private:
  const bool utilize_bandwidth_headroom_;
  const bool use_newfangled_headroom_adjustment_;

  VideoEncoder::RateControlParameters current_rate_control_parameters_;
  // FPS allocation of temporal layers, per simulcast/spatial layer. Represented
  // as a Q8 fraction; 0 = 0%, 255 = 100%. See
  // VideoEncoder::EncoderInfo.fps_allocation.
  absl::InlinedVector<uint8_t, kMaxTemporalStreams>
      current_fps_allocation_[kMaxSpatialLayers];

  // Frames since layout was changed, mean that any simulcast, spatial or
  // temporal layer was either disabled or enabled.
  size_t frames_since_layout_change_;
  std::unique_ptr<EncoderOvershootDetector>
      overshoot_detectors_[kMaxSpatialLayers][kMaxTemporalStreams];

  // Per spatial layer track of average media utilization.
  std::unique_ptr<RateUtilizationTracker>
      media_rate_trackers_[kMaxSpatialLayers];

  // Minimum bitrates allowed, per spatial layer.
  uint32_t min_bitrates_bps_[kMaxSpatialLayers];

  // Codec type used for encoding.
  VideoCodecType codec_;

  // Codec mode: { kRealtimeVideo, kScreensharing }.
  VideoCodecMode codec_mode_;

  Clock& clock_;
};

}  // namespace webrtc

#endif  // VIDEO_ENCODER_BITRATE_ADJUSTER_H_
