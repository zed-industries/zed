/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_EXPERIMENTS_QUALITY_SCALER_SETTINGS_H_
#define RTC_BASE_EXPERIMENTS_QUALITY_SCALER_SETTINGS_H_

#include <optional>

#include "api/field_trials_view.h"
#include "rtc_base/experiments/field_trial_parser.h"

namespace webrtc {

class QualityScalerSettings final {
 public:
  explicit QualityScalerSettings(const FieldTrialsView& field_trials);

  std::optional<int> SamplingPeriodMs() const;
  std::optional<int> AverageQpWindow() const;
  std::optional<int> MinFrames() const;
  std::optional<double> InitialScaleFactor() const;
  std::optional<double> ScaleFactor() const;
  std::optional<int> InitialBitrateIntervalMs() const;
  std::optional<double> InitialBitrateFactor() const;

 private:
  FieldTrialOptional<int> sampling_period_ms_;
  FieldTrialOptional<int> average_qp_window_;
  FieldTrialOptional<int> min_frames_;
  FieldTrialOptional<double> initial_scale_factor_;
  FieldTrialOptional<double> scale_factor_;
  FieldTrialOptional<int> initial_bitrate_interval_ms_;
  FieldTrialOptional<double> initial_bitrate_factor_;
};

}  // namespace webrtc

#endif  // RTC_BASE_EXPERIMENTS_QUALITY_SCALER_SETTINGS_H_
