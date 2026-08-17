/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_ADAPTATION_BALANCED_CONSTRAINT_H_
#define VIDEO_ADAPTATION_BALANCED_CONSTRAINT_H_

#include <optional>
#include <string>

#include "api/field_trials_view.h"
#include "api/sequence_checker.h"
#include "call/adaptation/adaptation_constraint.h"
#include "call/adaptation/degradation_preference_provider.h"
#include "rtc_base/experiments/balanced_degradation_settings.h"
#include "rtc_base/system/no_unique_address.h"

namespace webrtc {

class BalancedConstraint : public AdaptationConstraint {
 public:
  BalancedConstraint(
      DegradationPreferenceProvider* degradation_preference_provider,
      const FieldTrialsView& field_trials);
  ~BalancedConstraint() override = default;

  void OnEncoderTargetBitrateUpdated(
      std::optional<uint32_t> encoder_target_bitrate_bps);

  // AdaptationConstraint implementation.
  std::string Name() const override { return "BalancedConstraint"; }
  bool IsAdaptationUpAllowed(
      const VideoStreamInputState& input_state,
      const VideoSourceRestrictions& restrictions_before,
      const VideoSourceRestrictions& restrictions_after) const override;

 private:
  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequence_checker_;
  std::optional<uint32_t> encoder_target_bitrate_bps_
      RTC_GUARDED_BY(&sequence_checker_);
  const BalancedDegradationSettings balanced_settings_;
  const DegradationPreferenceProvider* degradation_preference_provider_;
};

}  // namespace webrtc

#endif  // VIDEO_ADAPTATION_BALANCED_CONSTRAINT_H_
