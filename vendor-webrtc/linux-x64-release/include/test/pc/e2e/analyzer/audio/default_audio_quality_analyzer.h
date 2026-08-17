/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_ANALYZER_AUDIO_DEFAULT_AUDIO_QUALITY_ANALYZER_H_
#define TEST_PC_E2E_ANALYZER_AUDIO_DEFAULT_AUDIO_QUALITY_ANALYZER_H_

#include <map>
#include <string>

#include "absl/strings/string_view.h"
#include "api/numerics/samples_stats_counter.h"
#include "api/test/audio_quality_analyzer_interface.h"
#include "api/test/metrics/metrics_logger.h"
#include "api/test/track_id_stream_info_map.h"
#include "api/units/time_delta.h"
#include "rtc_base/synchronization/mutex.h"

namespace webrtc {
namespace webrtc_pc_e2e {

struct AudioStreamStats {
  SamplesStatsCounter expand_rate;
  SamplesStatsCounter accelerate_rate;
  SamplesStatsCounter preemptive_rate;
  SamplesStatsCounter speech_expand_rate;
  SamplesStatsCounter average_jitter_buffer_delay_ms;
  SamplesStatsCounter preferred_buffer_size_ms;
  SamplesStatsCounter energy;
};

class DefaultAudioQualityAnalyzer : public AudioQualityAnalyzerInterface {
 public:
  explicit DefaultAudioQualityAnalyzer(
      test::MetricsLogger* const metrics_logger);

  void Start(std::string test_case_name,
             TrackIdStreamInfoMap* analyzer_helper) override;
  void OnStatsReports(
      absl::string_view pc_label,
      const scoped_refptr<const RTCStatsReport>& report) override;
  void Stop() override;

  // Returns audio quality stats per stream label.
  std::map<std::string, AudioStreamStats> GetAudioStreamsStats() const;

 private:
  struct StatsSample {
    uint64_t total_samples_received = 0;
    uint64_t concealed_samples = 0;
    uint64_t removed_samples_for_acceleration = 0;
    uint64_t inserted_samples_for_deceleration = 0;
    uint64_t silent_concealed_samples = 0;
    TimeDelta jitter_buffer_delay = TimeDelta::Zero();
    TimeDelta jitter_buffer_target_delay = TimeDelta::Zero();
    uint64_t jitter_buffer_emitted_count = 0;
    double total_samples_duration = 0.0;
    double total_audio_energy = 0.0;
  };

  std::string GetTestCaseName(const std::string& stream_label) const;

  test::MetricsLogger* const metrics_logger_;

  std::string test_case_name_;
  TrackIdStreamInfoMap* analyzer_helper_;

  mutable Mutex lock_;
  std::map<std::string, AudioStreamStats> streams_stats_ RTC_GUARDED_BY(lock_);
  std::map<std::string, TrackIdStreamInfoMap::StreamInfo> stream_info_
      RTC_GUARDED_BY(lock_);
  std::map<std::string, StatsSample> last_stats_sample_ RTC_GUARDED_BY(lock_);
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // TEST_PC_E2E_ANALYZER_AUDIO_DEFAULT_AUDIO_QUALITY_ANALYZER_H_
