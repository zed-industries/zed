/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_VIDEO_QUALITY_OBSERVER2_H_
#define VIDEO_VIDEO_QUALITY_OBSERVER2_H_

#include <stdint.h>

#include <optional>
#include <set>
#include <vector>

#include "api/video/video_codec_type.h"
#include "api/video/video_content_type.h"
#include "rtc_base/numerics/moving_average.h"
#include "rtc_base/numerics/sample_counter.h"

namespace webrtc {
namespace internal {
// Declared in video_receive_stream2.h.
struct VideoFrameMetaData;

// Calculates spatial and temporal quality metrics and reports them to UMA
// stats.
class VideoQualityObserver {
 public:
  // Use either VideoQualityObserver::kBlockyQpThresholdVp8 or
  // VideoQualityObserver::kBlockyQpThresholdVp9.
  VideoQualityObserver();
  ~VideoQualityObserver() = default;

  void OnDecodedFrame(uint32_t rtp_frame_timestamp,
                      std::optional<uint8_t> qp,
                      VideoCodecType codec);

  void OnRenderedFrame(const VideoFrameMetaData& frame_meta);

  void OnStreamInactive();

  uint32_t NumFreezes() const;
  uint32_t NumPauses() const;
  uint32_t TotalFreezesDurationMs() const;
  uint32_t TotalPausesDurationMs() const;
  uint32_t TotalFramesDurationMs() const;
  double SumSquaredFrameDurationsSec() const;

  // Set `screenshare` to true if the last decoded frame was for screenshare.
  void UpdateHistograms(bool screenshare);

  static const uint32_t kMinFrameSamplesToDetectFreeze;
  static const uint32_t kMinIncreaseForFreezeMs;
  static const uint32_t kAvgInterframeDelaysWindowSizeFrames;

 private:
  enum Resolution {
    Low = 0,
    Medium = 1,
    High = 2,
  };

  int64_t last_frame_rendered_ms_;
  int64_t num_frames_rendered_;
  int64_t first_frame_rendered_ms_;
  int64_t last_frame_pixels_;
  bool is_last_frame_blocky_;
  // Decoded timestamp of the last delayed frame.
  int64_t last_unfreeze_time_ms_;
  MovingAverage render_interframe_delays_;
  double sum_squared_interframe_delays_secs_;
  // An inter-frame delay is counted as a freeze if it's significantly longer
  // than average inter-frame delay.
  SampleCounter freezes_durations_;
  SampleCounter pauses_durations_;
  // Time between freezes.
  SampleCounter smooth_playback_durations_;
  // Counters for time spent in different resolutions. Time between each two
  // Consecutive frames is counted to bin corresponding to the first frame
  // resolution.
  std::vector<int64_t> time_in_resolution_ms_;
  // Resolution of the last decoded frame. Resolution enum is used as an index.
  Resolution current_resolution_;
  int num_resolution_downgrades_;
  // Similar to resolution, time spent in high-QP video.
  int64_t time_in_blocky_video_ms_;
  bool is_paused_;

  // Set of decoded frames with high QP value.
  std::set<int64_t> blocky_frames_;
};

}  // namespace internal
}  // namespace webrtc

#endif  // VIDEO_VIDEO_QUALITY_OBSERVER2_H_
