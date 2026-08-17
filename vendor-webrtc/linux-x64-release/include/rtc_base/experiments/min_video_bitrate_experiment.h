/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_EXPERIMENTS_MIN_VIDEO_BITRATE_EXPERIMENT_H_
#define RTC_BASE_EXPERIMENTS_MIN_VIDEO_BITRATE_EXPERIMENT_H_

#include <optional>

#include "api/field_trials_view.h"
#include "api/units/data_rate.h"
#include "api/video/video_codec_type.h"

namespace webrtc {

extern const int kDefaultMinVideoBitrateBps;

// Return the experiment-driven minimum video bitrate.
// If no experiment is effective, returns nullopt.
std::optional<DataRate> GetExperimentalMinVideoBitrate(
    const FieldTrialsView& field_trials,
    VideoCodecType type);

}  // namespace webrtc

#endif  // RTC_BASE_EXPERIMENTS_MIN_VIDEO_BITRATE_EXPERIMENT_H_
