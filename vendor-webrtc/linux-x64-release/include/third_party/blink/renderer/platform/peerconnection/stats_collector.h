// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_STATS_COLLECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_STATS_COLLECTOR_H_

#include "base/functional/callback.h"
#include "base/time/time.h"
#include "media/base/video_codecs.h"
#include "third_party/blink/renderer/platform/peerconnection/linear_histogram.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

class PLATFORM_EXPORT StatsCollector {
 public:
  struct StatsKey {
    bool is_decode;
    media::VideoCodecProfile codec_profile;
    int pixel_size;
    bool hw_accelerated;
  };

  struct VideoStats {
    int frame_count;
    int key_frame_count;
    float p99_processing_time_ms;
  };

  // Only report data if at least 100 samples were collected. This is the
  // minimum number of samples needed for the 99th percentile to be meaningful.
  static constexpr int kMinSamplesThreshold = 100;
  // Stop collecting data after 18000 samples (10 minutes at 30 fps).
  static constexpr int kMaxSamplesThreshold = 10 * 60 * 30;

  // This callback is used to store processing stats.
  using StoreProcessingStatsCB =
      base::RepeatingCallback<void(const StatsKey&, const VideoStats&)>;

  StatsCollector(bool is_decode,
                 media::VideoCodecProfile codec_profile,
                 StoreProcessingStatsCB stats_callback);

  bool active_stats_collection() const {
    return static_cast<bool>(processing_time_ms_histogram_);
  }
  bool stats_collection_finished() const { return stats_collection_finished_; }
  size_t samples_collected() const {
    DCHECK(processing_time_ms_histogram_);
    return processing_time_ms_histogram_->NumValues();
  }

  void StartStatsCollection();
  void ClearStatsCollection();
  void AddProcessingTime(int pixel_size,
                         bool is_hardware_accelerated,
                         const float processing_time_ms,
                         const size_t new_keyframes,
                         const base::TimeTicks& now);
  void ReportStats() const;

 private:
  const bool is_decode_;
  const media::VideoCodecProfile codec_profile_;
  const StoreProcessingStatsCB stats_callback_;
  // Tracks the processing time in ms as well as the number of processed frames.
  std::unique_ptr<LinearHistogram> processing_time_ms_histogram_;
  // Tracks the total number of processed key frames.
  size_t number_of_keyframes_;  // Initialized in ClearStatsCollection().
  StatsKey current_stats_key_;  // Initialized in ClearStatsCollection().
  base::TimeTicks last_report_;
  bool stats_collection_finished_{false};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_STATS_COLLECTOR_H_
