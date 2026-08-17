/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_INPUT_VOLUME_STATS_REPORTER_H_
#define MODULES_AUDIO_PROCESSING_AGC2_INPUT_VOLUME_STATS_REPORTER_H_

#include <optional>

#include "rtc_base/gtest_prod_util.h"
#include "system_wrappers/include/metrics.h"

namespace webrtc {

// Input volume statistics calculator. Computes aggregate stats based on the
// framewise input volume observed by `UpdateStatistics()`. Periodically logs
// the statistics into a histogram.
class InputVolumeStatsReporter {
 public:
  enum class InputVolumeType {
    kApplied = 0,
    kRecommended = 1,
  };

  explicit InputVolumeStatsReporter(InputVolumeType input_volume_type);
  InputVolumeStatsReporter(const InputVolumeStatsReporter&) = delete;
  InputVolumeStatsReporter operator=(const InputVolumeStatsReporter&) = delete;
  ~InputVolumeStatsReporter();

  // Updates the stats based on `input_volume`. Periodically logs the stats into
  // a histogram.
  void UpdateStatistics(int input_volume);

 private:
  FRIEND_TEST_ALL_PREFIXES(InputVolumeStatsReporterTest,
                           CheckVolumeUpdateStatsForEmptyStats);
  FRIEND_TEST_ALL_PREFIXES(InputVolumeStatsReporterTest,
                           CheckVolumeUpdateStatsAfterNoVolumeChange);
  FRIEND_TEST_ALL_PREFIXES(InputVolumeStatsReporterTest,
                           CheckVolumeUpdateStatsAfterVolumeIncrease);
  FRIEND_TEST_ALL_PREFIXES(InputVolumeStatsReporterTest,
                           CheckVolumeUpdateStatsAfterVolumeDecrease);
  FRIEND_TEST_ALL_PREFIXES(InputVolumeStatsReporterTest,
                           CheckVolumeUpdateStatsAfterReset);

  // Stores input volume update stats to enable calculation of update rate and
  // average update separately for volume increases and decreases.
  struct VolumeUpdateStats {
    int num_decreases = 0;
    int num_increases = 0;
    int sum_decreases = 0;
    int sum_increases = 0;
  } volume_update_stats_;

  // Returns a copy of the stored statistics. Use only for testing.
  VolumeUpdateStats volume_update_stats() const { return volume_update_stats_; }

  // Computes aggregate stat and logs them into a histogram.
  void LogVolumeUpdateStats() const;

  // Histograms.
  struct Histograms {
    metrics::Histogram* const on_volume_change;
    metrics::Histogram* const decrease_rate;
    metrics::Histogram* const decrease_average;
    metrics::Histogram* const increase_rate;
    metrics::Histogram* const increase_average;
    metrics::Histogram* const update_rate;
    metrics::Histogram* const update_average;
    bool AllPointersSet() const {
      return !!on_volume_change && !!decrease_rate && !!decrease_average &&
             !!increase_rate && !!increase_average && !!update_rate &&
             !!update_average;
    }
  } histograms_;

  // True if the stats cannot be logged.
  const bool cannot_log_stats_;

  int log_volume_update_stats_counter_ = 0;
  std::optional<int> previous_input_volume_ = std::nullopt;
};

// Updates the histogram that keeps track of recommended input volume changes
// required in order to match the target level in the input volume adaptation
// process.
void UpdateHistogramOnRecommendedInputVolumeChangeToMatchTarget(int volume);

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_INPUT_VOLUME_STATS_REPORTER_H_
