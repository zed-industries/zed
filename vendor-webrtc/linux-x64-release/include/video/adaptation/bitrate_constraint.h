/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_ADAPTATION_BITRATE_CONSTRAINT_H_
#define VIDEO_ADAPTATION_BITRATE_CONSTRAINT_H_

#include <optional>
#include <string>

#include "api/sequence_checker.h"
#include "call/adaptation/adaptation_constraint.h"
#include "call/adaptation/encoder_settings.h"
#include "call/adaptation/video_source_restrictions.h"
#include "call/adaptation/video_stream_input_state.h"
#include "rtc_base/system/no_unique_address.h"

namespace webrtc {

class BitrateConstraint : public AdaptationConstraint {
 public:
  BitrateConstraint();
  ~BitrateConstraint() override = default;

  void OnEncoderSettingsUpdated(
      std::optional<EncoderSettings> encoder_settings);
  void OnEncoderTargetBitrateUpdated(
      std::optional<uint32_t> encoder_target_bitrate_bps);

  // AdaptationConstraint implementation.
  std::string Name() const override { return "BitrateConstraint"; }
  bool IsAdaptationUpAllowed(
      const VideoStreamInputState& input_state,
      const VideoSourceRestrictions& restrictions_before,
      const VideoSourceRestrictions& restrictions_after) const override;

 private:
  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequence_checker_;
  std::optional<EncoderSettings> encoder_settings_
      RTC_GUARDED_BY(&sequence_checker_);
  std::optional<uint32_t> encoder_target_bitrate_bps_
      RTC_GUARDED_BY(&sequence_checker_);
};

}  // namespace webrtc

#endif  // VIDEO_ADAPTATION_BITRATE_CONSTRAINT_H_
