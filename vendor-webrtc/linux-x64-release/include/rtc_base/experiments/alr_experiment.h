/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_EXPERIMENTS_ALR_EXPERIMENT_H_
#define RTC_BASE_EXPERIMENTS_ALR_EXPERIMENT_H_

#include <stdint.h>

#include <optional>

#include "absl/strings/string_view.h"
#include "api/field_trials_view.h"

namespace webrtc {
struct AlrExperimentSettings {
 public:
  float pacing_factor;
  int64_t max_paced_queue_time;
  int alr_bandwidth_usage_percent;
  int alr_start_budget_level_percent;
  int alr_stop_budget_level_percent;
  // Will be sent to the receive side for stats slicing.
  // Can be 0..6, because it's sent as a 3 bits value and there's also
  // reserved value to indicate absence of experiment.
  int group_id;

  static constexpr absl::string_view kScreenshareProbingBweExperimentName =
      "WebRTC-ProbingScreenshareBwe";
  static constexpr absl::string_view kStrictPacingAndProbingExperimentName =
      "WebRTC-StrictPacingAndProbing";

  static std::optional<AlrExperimentSettings> CreateFromFieldTrial(
      const FieldTrialsView& key_value_config,
      absl::string_view experiment_name);
  static bool MaxOneFieldTrialEnabled(const FieldTrialsView& key_value_config);

 private:
  AlrExperimentSettings() = default;
};
}  // namespace webrtc

#endif  // RTC_BASE_EXPERIMENTS_ALR_EXPERIMENT_H_
